use std::{collections::BTreeMap, rc::Rc};

use anyhow::{bail, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha3::Sha3_256;
use skip_ratchet::Ratchet;

use super::{
    namefilter::Namefilter, ContentKey, HamtStore, INumber, Key, PrivateFile, PrivateNode,
    PrivateNodeHeader, PrivateRef, RatchetKey, Rng,
};

use crate::{
    error, private::{hamt::Hasher, EncryptedRatchetKey}, utils, BlockStore, FsError, HashOutput, Id, Metadata, OpResult,
    PathNodes, PathNodesResult, UnixFsNodeKind, HASH_BYTE_SIZE,
};

//--------------------------------------------------------------------------------------------------
// Type Definitions
//--------------------------------------------------------------------------------------------------

pub type PrivateOpResult<T> = OpResult<PrivateDirectory, T>;
pub type PrivatePathNodes = PathNodes<PrivateDirectory>;
pub type PrivatePathNodesResult = PathNodesResult<PrivateDirectory>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivateDirectoryContent {
    pub(crate) metadata: Metadata,
    pub(crate) entries: BTreeMap<String, PrivateRef>,
}

#[derive(Debug, Clone)]
pub struct PrivateDirectory {
    pub(crate) header: Option<PrivateNodeHeader>,
    pub(crate) content: PrivateDirectoryContent,
}

//--------------------------------------------------------------------------------------------------
// Implementations
//--------------------------------------------------------------------------------------------------

impl PrivateDirectory {
    /// Creates a new directory with provided details.
    pub fn new(
        parent_bare_name: Option<Namefilter>,
        inumber: INumber,
        ratchet_seed: HashOutput,
        time: DateTime<Utc>,
    ) -> Self {
        Self {
            header: Some(PrivateNodeHeader::new(
                parent_bare_name,
                inumber,
                ratchet_seed,
            )),
            content: PrivateDirectoryContent {
                metadata: Metadata::new(time, UnixFsNodeKind::Dir),
                entries: BTreeMap::new(),
            },
        }
    }

    /// Generates two random set of bytes.
    pub fn generate_double_random<R: Rng>() -> (HashOutput, HashOutput) {
        const _DOUBLE_SIZE: usize = HASH_BYTE_SIZE * 2;
        let [first, second] = unsafe {
            std::mem::transmute::<[u8; _DOUBLE_SIZE], [[u8; HASH_BYTE_SIZE]; 2]>(R::random_bytes::<
                _DOUBLE_SIZE,
            >())
        };
        (first, second)
    }

    ///  Advances the ratchet.
    pub(crate) fn advance_ratchet(&mut self) {
        self.header.as_mut().map(|header| {
            header.advance_ratchet();
        });
    }

    /// Creates a new `PathNodes` that is not based on an existing file tree.
    pub(crate) fn create_path_nodes<'a, B, R>(
        path_segments: &[String],
        time: DateTime<Utc>,
        parent_bare_name: Option<Namefilter>,
        _: &HamtStore<'a, B, R>,
    ) -> PrivatePathNodes
    where
        B: BlockStore,
        R: Rng,
    {
        let mut working_parent_bare_name = parent_bare_name;
        let (mut inumber, mut ratchet_seed) = Self::generate_double_random::<R>();

        let path: Vec<(Rc<PrivateDirectory>, String)> = path_segments
            .iter()
            .map(|segment| {
                // Create new private directory.
                let directory = Rc::new(PrivateDirectory::new(
                    std::mem::take(&mut working_parent_bare_name),
                    inumber,
                    ratchet_seed,
                    time,
                ));

                // Update seeds and the working parent bare name.
                (inumber, ratchet_seed) = Self::generate_double_random::<R>();
                working_parent_bare_name = directory
                    .header
                    .as_ref()
                    .map(|header| header.bare_name.clone());

                (directory, segment.clone())
            })
            .collect();

        PrivatePathNodes {
            path,
            tail: Rc::new(PrivateDirectory::new(
                std::mem::take(&mut working_parent_bare_name),
                inumber,
                ratchet_seed,
                time,
            )),
        }
    }

    /// Uses specified path segments and their existence in the file tree to generate `PathNodes`.
    ///
    /// Supports cases where the entire path does not exist.
    pub(crate) async fn get_path_nodes<'a, B, R>(
        self: Rc<Self>,
        path_segments: &[String],
        hamt: &HamtStore<'a, B, R>,
    ) -> Result<PrivatePathNodesResult>
    where
        B: BlockStore,
        R: Rng,
    {
        use PathNodesResult::*;
        let mut working_node = self;
        let mut path_nodes = Vec::with_capacity(path_segments.len());

        for segment in path_segments {
            match working_node.lookup_node(segment, hamt).await? {
                Some(PrivateNode::Dir(ref directory)) => {
                    path_nodes.push((Rc::clone(&working_node), segment.clone()));
                    working_node = Rc::clone(directory);
                }
                Some(_) => {
                    let path_nodes = PrivatePathNodes {
                        path: path_nodes,
                        tail: Rc::clone(&working_node),
                    };

                    return Ok(NotADirectory(path_nodes, segment.clone()));
                }
                None => {
                    let path_nodes = PrivatePathNodes {
                        path: path_nodes,
                        tail: Rc::clone(&working_node),
                    };

                    return Ok(MissingLink(path_nodes, segment.clone()));
                }
            }
        }

        Ok(Complete(PrivatePathNodes {
            path: path_nodes,
            tail: Rc::clone(&working_node),
        }))
    }

    /// Uses specified path segments to generate `PathNodes`. Creates missing directories as needed.
    pub(crate) async fn get_or_create_path_nodes<'a, B, R>(
        self: Rc<Self>,
        path_segments: &[String],
        time: DateTime<Utc>,
        hamt: &HamtStore<'a, B, R>,
    ) -> Result<PrivatePathNodes>
    where
        B: BlockStore,
        R: Rng,
    {
        use PathNodesResult::*;
        match self.get_path_nodes(path_segments, hamt).await? {
            Complete(path_nodes) => Ok(path_nodes),
            NotADirectory(_, _) => error(FsError::InvalidPath),
            MissingLink(path_so_far, missing_link) => {
                // Get remaining missing path segments.
                let missing_path = path_segments.split_at(path_so_far.path.len() + 1).1;

                // Get tail bare name from `path_so_far`.
                let parent_bare_name = path_so_far
                    .tail
                    .header
                    .as_ref()
                    .map(|header| header.bare_name.clone());

                // Create missing directories.
                let missing_path_nodes =
                    Self::create_path_nodes(missing_path, time, parent_bare_name, hamt);

                Ok(PrivatePathNodes {
                    path: [
                        path_so_far.path,
                        vec![(path_so_far.tail, missing_link)],
                        missing_path_nodes.path,
                    ]
                    .concat(),
                    tail: missing_path_nodes.tail,
                })
            }
        }
    }

    /// Fix up `PathNodes` so that parents refer to the newly updated children.
    async fn fix_up_path_nodes<'a, B, R>(
        private_ref: &PrivateRef,
        path_nodes: PrivatePathNodes,
        hamt: &mut HamtStore<'a, B, R>,
    ) -> Result<Rc<Self>>
    where
        B: BlockStore,
        R: Rng,
    {
        let mut working_child_dir = {
            let mut tmp = (*path_nodes.tail).clone();
            tmp.advance_ratchet();
            Rc::new(tmp)
        };

        for (parent_dir, segment) in path_nodes.path.iter().rev() {
            let mut parent_dir = (**parent_dir).clone();

            parent_dir.advance_ratchet();

            let child_private_ref = Self::generate_child_private_ref::<R>(
                &parent_dir.header,
                &working_child_dir.header,
            )?;

            parent_dir
                .content
                .entries
                .insert(segment.clone(), child_private_ref.clone());

            let parent_dir = Rc::new(parent_dir);

            hamt.set(
                working_child_dir
                    .header
                    .as_ref()
                    .ok_or(FsError::MissingHeader)?
                    .bare_name
                    .clone(),
                &child_private_ref,
                &PrivateNode::Dir(Rc::clone(&working_child_dir)),
            )
            .await?;

            working_child_dir = parent_dir;
        }

        hamt.set(
            working_child_dir
                .header
                .as_ref()
                .ok_or(FsError::MissingHeader)?
                .bare_name
                .clone(),
            &private_ref,
            &PrivateNode::Dir(Rc::clone(&working_child_dir)),
        )
        .await?;

        Ok(working_child_dir)
    }

    /// Generates a child entry `PrivateRef`.
    fn generate_child_private_ref<R>(
        parent_header: &Option<PrivateNodeHeader>,
        child_header: &Option<PrivateNodeHeader>,
    ) -> Result<PrivateRef>
    where
        R: Rng,
    {
        match (parent_header, child_header) {
            (Some(parent_header), Some(child_header)) => {
                let (ratchet_key, content_key) =
                    Self::generate_keys::<R>(&parent_header.ratchet, &child_header.ratchet)?;

                let saturated_name_hash = {
                    let mut name = child_header.bare_name.clone();
                    name.add(&ratchet_key.get_bare_key()?.as_bytes());
                    name.saturate();
                    Sha3_256::hash(&name.as_bytes())
                };

                Ok(PrivateRef {
                    saturated_name_hash,
                    content_key,
                    ratchet_key: Some(ratchet_key),
                })
            }
            _ => bail!(FsError::MissingHeader),
        }
    }

    /// Generates a child entry ratchet key and content key.
    fn generate_keys<R>(
        parent_ratchet: &Ratchet,
        child_ratchet: &Ratchet,
    ) -> Result<(RatchetKey, ContentKey)>
    where
        R: Rng,
    {
        use RatchetKey::*;

        let child_ratchet_key_bare = Key::from(child_ratchet.derive_key());
        let child_content_key = Key::from(Sha3_256::hash(&child_ratchet_key_bare.as_bytes()));
        let parent_ratchet_key = Key::from(parent_ratchet.derive_key());

        Ok((Encrypted(EncryptedRatchetKey {
            encrypted: parent_ratchet_key.encrypt(
                &Key::generate_nonce::<R>(),
                child_ratchet_key_bare.as_bytes(),
            )?,
            bare: Some(child_ratchet_key_bare),
        }), child_content_key))
    }

    /// Follows a path and fetches the node at the end of the path.
    pub async fn get_node<'a, B, R>(
        self: Rc<Self>,
        path_segments: &[String],
        hamt: &HamtStore<'a, B, R>,
    ) -> Result<PrivateOpResult<Option<PrivateNode>>>
    where
        B: BlockStore,
        R: Rng,
    {
        use PathNodesResult::*;
        let root_dir = Rc::clone(&self);

        Ok(match path_segments.split_last() {
            Some((path_segment, parent_path)) => {
                match self.get_path_nodes(parent_path, hamt).await? {
                    Complete(parent_path_nodes) => PrivateOpResult {
                        root_dir,
                        result: parent_path_nodes
                            .tail
                            .lookup_node(path_segment, hamt)
                            .await?,
                    },
                    MissingLink(_, _) => bail!(FsError::NotFound),
                    NotADirectory(_, _) => bail!(FsError::NotFound),
                }
            }
            None => PrivateOpResult {
                root_dir,
                result: Some(PrivateNode::Dir(self)),
            },
        })
    }

    /// Reads specified file content from the directory.
    pub async fn read<'a, B, R>(
        self: Rc<Self>,
        path_segments: &[String],
        hamt: &HamtStore<'a, B, R>,
    ) -> Result<PrivateOpResult<Vec<u8>>>
    where
        B: BlockStore,
        R: Rng,
    {
        let root_dir = Rc::clone(&self);
        let (path, filename) = utils::split_last(path_segments)?;

        match self.get_path_nodes(path, hamt).await? {
            PathNodesResult::Complete(node_path) => {
                match node_path.tail.lookup_node(filename, hamt).await? {
                    Some(PrivateNode::File(file)) => Ok(PrivateOpResult {
                        root_dir,
                        result: file.content.content.clone(),
                    }),
                    Some(PrivateNode::Dir(_)) => error(FsError::NotAFile),
                    None => error(FsError::NotFound),
                }
            }
            _ => error(FsError::NotFound),
        }
    }

    /// Writes a file to the directory.
    pub async fn write<'a, B, R>(
        self: Rc<Self>,
        path_segments: &[String],
        private_ref: &PrivateRef,
        time: DateTime<Utc>,
        content: Vec<u8>,
        hamt: &mut HamtStore<'a, B, R>,
    ) -> Result<PrivateOpResult<()>>
    where
        B: BlockStore,
        R: Rng,
    {
        let (directory_path, filename) = utils::split_last(path_segments)?;

        // This will create directories if they don't exist yet
        let mut directory_path_nodes = self
            .get_or_create_path_nodes(directory_path, time, hamt)
            .await?;

        let mut directory = (*directory_path_nodes.tail).clone();

        // Modify the file if it already exists, otherwise create a new file with expected content
        let file = match directory.lookup_node(filename, hamt).await? {
            Some(PrivateNode::File(file_before)) => {
                let mut file = (*file_before).clone();
                file.content.content = content;
                file.content.metadata = Metadata::new(time, UnixFsNodeKind::File);
                file
            }
            Some(PrivateNode::Dir(_)) => bail!(FsError::DirectoryAlreadyExists),
            None => {
                let (inumber, ratchet_seed) = Self::generate_double_random::<R>();
                PrivateFile::new(
                    directory.header.as_ref().map(|h| h.bare_name.clone()),
                    inumber,
                    ratchet_seed,
                    time,
                    content,
                )
            }
        };

        // Insert the file into its parent directory
        let child_private_ref = Self::generate_child_private_ref::<R>(&directory.header, &file.header)?;

        directory
            .content
            .entries
            .insert(filename.to_string(), child_private_ref);

        directory_path_nodes.tail = Rc::new(directory);

        // Fix up the file path
        Ok(PrivateOpResult {
            root_dir: Self::fix_up_path_nodes(private_ref, directory_path_nodes, hamt).await?, //
            result: (),
        })
    }

    /// Looks up a node by its path name in the current directory.
    pub async fn lookup_node<'a, B, R>(
        &self,
        path_segment: &str,
        hamt: &HamtStore<'a, B, R>,
    ) -> Result<Option<PrivateNode>>
    where
        B: BlockStore,
        R: Rng,
    {
        Ok(match self.content.entries.get(path_segment) {
            Some(private_ref) => hamt.get(private_ref).await?,
            None => None,
        })
    }

    /// Creates a new directory at the specified path.
    pub async fn mkdir<'a, B, R>(
        self: Rc<Self>,
        path_segments: &[String],
        private_ref: &PrivateRef,
        time: DateTime<Utc>,
        hamt: &mut HamtStore<'a, B, R>,
    ) -> Result<PrivateOpResult<()>>
    where
        B: BlockStore,
        R: Rng,
    {
        let path_nodes = self
            .get_or_create_path_nodes(path_segments, time, hamt)
            .await?;

        Ok(PrivateOpResult {
            root_dir: Self::fix_up_path_nodes(private_ref, path_nodes, hamt).await?,
            result: (),
        })
    }

    /// Returns names and metadata of directory's immediate children.
    pub async fn ls<'a, B, R>(
        self: Rc<Self>,
        path_segments: &[String],
        hamt: &HamtStore<'a, B, R>,
    ) -> Result<PrivateOpResult<Vec<(String, Metadata)>>>
    where
        B: BlockStore,
        R: Rng,
    {
        let root_dir = Rc::clone(&self);
        match self.get_path_nodes(path_segments, hamt).await? {
            PathNodesResult::Complete(path_nodes) => {
                let mut result = vec![];
                for (name, private_ref) in path_nodes.tail.content.entries.iter() {
                    match hamt.get(private_ref).await? {
                        Some(PrivateNode::File(file)) => {
                            result.push((name.clone(), file.content.metadata.clone()));
                        }
                        Some(PrivateNode::Dir(dir)) => {
                            result.push((name.clone(), dir.content.metadata.clone()));
                        }
                        _ => bail!(FsError::NotFound),
                    }
                }
                Ok(PrivateOpResult { root_dir, result })
            }
            _ => bail!(FsError::NotFound),
        }
    }

    /// Removes a file or directory from the directory.
    pub async fn rm<'a, B, R>(
        self: Rc<Self>,
        path_segments: &[String],
        private_ref: &PrivateRef,
        hamt: &mut HamtStore<'a, B, R>,
    ) -> Result<PrivateOpResult<PrivateNode>>
    where
        B: BlockStore,
        R: Rng,
    {
        let (directory_path, node_name) = utils::split_last(path_segments)?;

        let mut directory_path_nodes = match self.get_path_nodes(directory_path, hamt).await? {
            PrivatePathNodesResult::Complete(node_path) => node_path,
            _ => bail!(FsError::NotFound),
        };

        let mut directory = (*directory_path_nodes.tail).clone();

        // Remove the entry from its parent directory
        let removed_node = match directory.content.entries.remove(node_name) {
            Some(ref private_ref) => hamt.get(private_ref).await?.unwrap(),
            None => bail!(FsError::NotFound),
        };

        directory_path_nodes.tail = Rc::new(directory);

        Ok(PrivateOpResult {
            root_dir: Self::fix_up_path_nodes(private_ref, directory_path_nodes, hamt).await?,
            result: removed_node,
        })
    }
}

impl Id for PrivateDirectory {
    fn get_id(&self) -> String {
        format!("{:p}", &self.header)
    }
}

//--------------------------------------------------------------------------------------------------
// Tests
//--------------------------------------------------------------------------------------------------

#[cfg(test)]
mod private_directory_tests {
    // use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn i64_abs_is_never_negative(a: i64) {
            // This actually fails if a == i64::MIN, but randomly picking one
            // specific value out of 2⁶⁴ is overwhelmingly unlikely.
            assert!(a.abs() >= 0);
            println!("{}", a);
        }
    }
}
