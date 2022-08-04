use std::{collections::BTreeMap, rc::Rc};

use anyhow::{bail, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use super::{HamtStore, INumber, Key, Namefilter, PrivateNode, PrivateNodeHeader, Rng};
use crate::{
    error, BlockStore, FsError, HashOutput, Id, Metadata, OpResult, PathNodes,
    PathNodesReconstruct, PathNodesResult, UnixFsNodeKind, HASH_BYTE_SIZE,
};

//--------------------------------------------------------------------------------------------------
// Type Definitions
//--------------------------------------------------------------------------------------------------

pub type PrivateOpResult<T> = OpResult<PrivateDirectory, T>;
pub type PrivatePathNodes = PathNodes<PrivateDirectory>;
pub type PrivatePathNodesResult = PathNodesResult<PrivateDirectory>;

#[derive(Debug, Clone)]
pub struct RatchetKey {
    pub(crate) encrypted: Vec<u8>,
    pub(crate) bare: Option<Key>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivateRef {
    pub(crate) saturated_name_hash: HashOutput, // Sha3-256 hash of saturated namefilter
    pub(crate) content_key: Key,                // A hash or parent skip ratchet.
    pub(crate) ratchet_key: RatchetKey,         // Ratchet key.
}

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

    /// Creates a new `PathNodes` that is not based on an existing file tree.
    pub(crate) fn create_path_nodes<'a, B, R>(
        time: DateTime<Utc>,
        path_segments: &[String],
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
                    Self::create_path_nodes(time, missing_path, parent_bare_name, hamt);

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

    /// Looks up a node by its path name in the current directory.
    pub async fn lookup_node<'a, B, R>(
        self: &Rc<Self>,
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
    pub fn mkdir<'a, B, R>(
        self: &Rc<Self>,
        path_segment: &str,
        hamt: &HamtStore<'a, B, R>,
    ) -> Result<PrivateOpResult<()>>
    where
        B: BlockStore,
        R: Rng,
    {
        Ok(PrivateOpResult {
            root_dir: self.clone(),
            result: (),
        })
    }

    /// Returns names and metadata of directory's immediate children.
    pub fn ls() {
        unimplemented!()
    }

}

impl PathNodesReconstruct for PrivateDirectory {
    type NodeType = Self;

    fn reconstruct(path_nodes: PathNodes<Self::NodeType>) -> Rc<Self::NodeType> {
        if path_nodes.path.is_empty() {
            return path_nodes.tail;
        }

        let mut working_dir = path_nodes.tail;
        for (dir, segment) in path_nodes.path.iter().rev() {
            let mut dir = (**dir).clone();
            // TODO(appcypher): Fix
            // Create private ref for link
            // Set in hamt store.
            // Then set private_ref in directory.
            working_dir = Rc::new(dir);
        }

        working_dir
    }
}

impl Id for PrivateDirectory {
    fn get_id(&self) -> String {
        format!("{:p}", &self.header)
    }
}

impl Serialize for RatchetKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_bytes(self.encrypted.as_slice())
    }
}

impl<'de> Deserialize<'de> for RatchetKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let bytes = Vec::deserialize(deserializer)?;
        Ok(RatchetKey {
            encrypted: bytes,
            bare: None,
        })
    }
}

//--------------------------------------------------------------------------------------------------
// Tests
//--------------------------------------------------------------------------------------------------

#[cfg(test)]
mod private_directory_tests {
    use super::*;
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
