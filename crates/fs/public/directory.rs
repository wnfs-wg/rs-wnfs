//! Public fs directory node.

use std::{
    cmp::Ordering,
    collections::BTreeMap,
    io::{Read, Seek},
    rc::Rc,
};

use crate::{blockstore, error, BlockStore, FsError, Metadata, UnixFsNodeKind};
use anyhow::{bail, ensure, Result};
use async_recursion::async_recursion;
use async_stream::try_stream;
use chrono::{DateTime, Utc};
use field_names::FieldNames;
use futures::Stream;
use libipld::{
    cbor::{cbor::MajorKind, decode, encode, DagCborCodec},
    codec::{Decode, Encode},
    Cid, IpldCodec,
};

use super::{Id, Link, PublicFile, PublicNode};

//--------------------------------------------------------------------------------------------------
// Type Definitions
//--------------------------------------------------------------------------------------------------

/// A directory in a WNFS public file system.``
#[derive(Debug, Clone, PartialEq, Eq, FieldNames)]
pub struct PublicDirectory {
    pub(crate) metadata: Metadata,
    pub(crate) userland: BTreeMap<String, Link>,
    pub(crate) previous: Option<Cid>,
}

/// Represents a directory that has possibly diverged. It is the result of operating on a directory.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OpResult<T> {
    // The root directory.
    pub root_dir: Rc<PublicDirectory>,
    // Implementation dependent but it usually the last leaf node operated on.
    pub result: T,
}

/// Represents the directory nodes along a path.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PathNodes {
    path: Vec<(Rc<PublicDirectory>, String)>,
    tail: Rc<PublicDirectory>,
}

/// The kinds of outcome from getting a `PathNodes`.
pub enum GetNodePathResult {
    Complete(PathNodes),
    MissingLink(PathNodes, String),
    NotADirectory(PathNodes, String),
}

impl PathNodes {
    /// Creates a new `PathNodes` that is not based on an existing file tree.
    fn new(time: DateTime<Utc>, path_segments: &[String], tail: Rc<PublicDirectory>) -> Self {
        let path: Vec<(Rc<PublicDirectory>, String)> = path_segments
            .iter()
            .map(|segment| (Rc::new(PublicDirectory::new(time)), segment.clone()))
            .collect();

        Self { path, tail }
    }

    /// Constructs a diverged path nodes by fixing up links in a `PathNodes` and returning the resulting root node.
    fn reconstruct(self) -> Rc<PublicDirectory> {
        if self.path.is_empty() {
            return self.tail;
        }

        let mut working_node = self.tail;
        for (dir, segment) in self.path.iter().rev() {
            let mut dir = (**dir).clone();
            let link = Link::Node(PublicNode::Dir(working_node));
            dir.userland.insert(segment.clone(), link);
            working_node = Rc::new(dir);
        }

        working_node
    }
}

//--------------------------------------------------------------------------------------------------
// Implementations
//--------------------------------------------------------------------------------------------------

impl PublicDirectory {
    /// Creates a new directory using the given metadata.
    pub fn new(time: DateTime<Utc>) -> Self {
        Self {
            metadata: Metadata::new(time, UnixFsNodeKind::Dir),
            userland: BTreeMap::new(),
            previous: None,
        }
    }

    /// Gets the previous value of the directory.
    pub fn get_previous(self: &Rc<Self>) -> Option<Cid> {
        self.previous
    }

    /// Gets the directory nodes along a path and allows for cases where the path is nopt fully constructed.
    pub async fn get_node_path<B: BlockStore>(
        self: Rc<Self>,
        path_segments: &[String],
        store: &B,
    ) -> Result<GetNodePathResult> {
        use GetNodePathResult::*;

        let mut working_node = self;
        let mut path_nodes = Vec::with_capacity(path_segments.len());

        for segment in path_segments.iter() {
            match working_node.lookup_node(segment, store).await? {
                Some(PublicNode::Dir(ref directory)) => {
                    path_nodes.push((Rc::clone(&working_node), segment.clone()));
                    working_node = Rc::clone(directory);
                }
                Some(_) => {
                    let path_nodes = PathNodes {
                        path: path_nodes,
                        tail: Rc::clone(&working_node),
                    };

                    return Ok(NotADirectory(path_nodes, segment.clone()));
                }
                None => {
                    let path_nodes = PathNodes {
                        path: path_nodes,
                        tail: Rc::clone(&working_node),
                    };

                    return Ok(MissingLink(path_nodes, segment.clone()));
                }
            }
        }

        Ok(Complete(PathNodes {
            path: path_nodes,
            tail: Rc::clone(&working_node),
        }))
    }

    /// Gets the directory nodes along a path and also supports creating missing intermediate directories.
    pub async fn get_node_path_with_mkdir<B: BlockStore>(
        self: Rc<Self>,
        path_segments: &[String],
        time: DateTime<Utc>,
        store: &B,
    ) -> Result<PathNodes> {
        match self.get_node_path(path_segments, store).await? {
            GetNodePathResult::Complete(path_nodes) => Ok(path_nodes),
            GetNodePathResult::NotADirectory(_, _) => error(FsError::InvalidPath),
            GetNodePathResult::MissingLink(path_so_far, missing_link) => {
                let missing_path = path_segments.split_at(path_so_far.path.len() + 1).1;
                let missing_path_nodes =
                    PathNodes::new(time, missing_path, Rc::new(PublicDirectory::new(time)));

                Ok(PathNodes {
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
    pub async fn get_node<B: BlockStore>(
        self: Rc<Self>,
        path_segments: &[String],
        store: &B,
    ) -> Result<OpResult<Option<PublicNode>>> {
        let root_dir = Rc::clone(&self);

        Ok(match path_segments.split_last() {
            Some((path_segment, parent_path)) => {
                match self.get_node_path(parent_path, store).await? {
                    GetNodePathResult::Complete(parent_path_nodes) => OpResult {
                        root_dir,
                        result: parent_path_nodes
                            .tail
                            .lookup_node(path_segment, store)
                            .await?,
                    },
                    GetNodePathResult::MissingLink(_, _) => bail!(FsError::NotFound),
                    GetNodePathResult::NotADirectory(_, _) => bail!(FsError::NotFound),
                }
            }
            None => OpResult {
                root_dir,
                result: Some(PublicNode::Dir(self)),
            },
        })
    }

    /// Looks up a node by its path name in the current directory.
    pub async fn lookup_node<B: BlockStore>(
        &self,
        path_segment: &str,
        store: &B,
    ) -> Result<Option<PublicNode>> {
        Ok(match self.userland.get(path_segment) {
            Some(link) => Some(link.resolve(store).await?),
            None => None,
        })
    }

    /// Stores directory in provided block store.
    ///
    /// This function can be recursive if the directory contains other directories.
    #[async_recursion(?Send)]
    pub async fn store<B: BlockStore>(&self, store: &mut B) -> Result<Cid> {
        let bytes = self.encode(store).await?;
        store.put_block(bytes, IpldCodec::DagCbor).await
    }

    /// Reads specified file content from the directory.
    pub async fn read<B: BlockStore>(
        self: Rc<Self>,
        path_segments: &[String],
        store: &mut B,
    ) -> Result<OpResult<Cid>> {
        let root_dir = Rc::clone(&self);
        let (path, filename) = utils::split_last(path_segments)?;

        match self.get_node_path(path, store).await? {
            GetNodePathResult::Complete(node_path) => {
                match node_path.tail.lookup_node(filename, store).await? {
                    Some(PublicNode::File(file)) => Ok(OpResult {
                        root_dir,
                        result: file.userland,
                    }),
                    Some(PublicNode::Dir(_)) => error(FsError::NotAFile),
                    None => error(FsError::NotFound),
                }
            }
            _ => error(FsError::NotFound),
        }
    }

    /// Writes a file to the directory.
    pub async fn write<B: BlockStore>(
        self: Rc<Self>,
        path_segments: &[String],
        content_cid: Cid,
        time: DateTime<Utc>,
        store: &B,
    ) -> Result<OpResult<()>> {
        let (directory_path, filename) = utils::split_last(path_segments)?;

        // This will create directories if they don't exist yet
        let mut directory_path_nodes = self
            .get_node_path_with_mkdir(directory_path, time, store)
            .await?;

        let mut directory = (*directory_path_nodes.tail).clone();

        // Modify the file if it already exists, otherwise create a new file with expected content
        let file = match directory.lookup_node(filename, store).await? {
            Some(PublicNode::File(file_before)) => {
                let mut file = (*file_before).clone();
                file.userland = content_cid;
                file.metadata = Metadata::new(time, UnixFsNodeKind::File);
                file
            }
            Some(PublicNode::Dir(_)) => bail!(FsError::DirectoryAlreadyExists),
            None => PublicFile::new(time, content_cid),
        };

        // insert the file into its parent directory
        directory.userland.insert(
            filename.to_string(),
            Link::Node(PublicNode::File(Rc::new(file))),
        );
        directory_path_nodes.tail = Rc::new(directory);

        // reconstruct the file path
        Ok(OpResult {
            root_dir: directory_path_nodes.reconstruct(),
            result: (),
        })
    }

    /// Creates a new directory at the specified path.
    ///
    /// This method acts like `mkdir -p` in Unix because it creates intermediate directories if they do not exist.
    pub async fn mkdir<B: BlockStore>(
        self: Rc<Self>,
        path_segments: &[String],
        time: DateTime<Utc>,
        store: &B,
    ) -> Result<OpResult<()>> {
        let node_path_with_dirs = self
            .get_node_path_with_mkdir(path_segments, time, store)
            .await?;

        Ok(OpResult {
            root_dir: node_path_with_dirs.reconstruct(),
            result: (),
        })
    }

    /// Returns the name and metadata of the direct children of a directory.
    pub async fn ls<B: BlockStore>(
        self: Rc<Self>,
        path_segments: &[String],
        store: &B,
    ) -> Result<OpResult<Vec<(String, Metadata)>>> {
        let root_dir = Rc::clone(&self);
        match self.get_node_path(path_segments, store).await? {
            GetNodePathResult::Complete(node_path) => {
                let mut result = vec![];
                for (name, link) in node_path.tail.userland.iter() {
                    match link.resolve(store).await? {
                        PublicNode::File(file) => {
                            result.push((name.clone(), file.metadata.clone()));
                        }
                        PublicNode::Dir(dir) => {
                            result.push((name.clone(), dir.metadata.clone()));
                        }
                    }
                }
                Ok(OpResult { root_dir, result })
            }
            _ => bail!(FsError::NotFound),
        }
    }

    /// Removes a file or directory from the directory.
    pub async fn rm<B: BlockStore>(
        self: Rc<Self>,
        path_segments: &[String],
        store: &B,
    ) -> Result<OpResult<PublicNode>> {
        let (directory_path, node_name) = utils::split_last(path_segments)?;

        let mut directory_node_path = match self.get_node_path(directory_path, store).await? {
            GetNodePathResult::Complete(node_path) => node_path,
            _ => bail!(FsError::NotFound),
        };

        let mut directory = (*directory_node_path.tail).clone();

        // remove the entry from its parent directory
        let removed_node = match directory.userland.remove(node_name) {
            Some(entry) => entry.resolve(store).await?,
            None => bail!(FsError::NotFound),
        };

        directory_node_path.tail = Rc::new(directory);

        Ok(OpResult {
            root_dir: directory_node_path.reconstruct(),
            result: removed_node,
        })
    }

    /// Moves a file or directory from one path to another.
    ///
    /// This function requires stating the destination name explicitly.
    pub async fn basic_mv<B: BlockStore>(
        self: Rc<Self>,
        path_segments_from: &[String],
        path_segments_to: &[String],
        store: &B,
    ) -> Result<OpResult<()>> {
        let root_dir = Rc::clone(&self);
        let (directory_path_nodes, tail) = utils::split_last(path_segments_to)?;

        let OpResult {
            root_dir,
            result: removed_node,
        } = root_dir.rm(path_segments_from, store).await?;

        let mut path_nodes = match root_dir.get_node_path(directory_path_nodes, store).await? {
            GetNodePathResult::Complete(node_path) => node_path,
            _ => bail!(FsError::NotFound),
        };

        let mut directory = (*path_nodes.tail).clone();

        ensure!(
            !directory.userland.contains_key(tail),
            FsError::FileAlreadyExists
        );

        // TODO(appcypher): We need to update the mtime of the moved node.
        directory
            .userland
            .insert(tail.clone(), Link::Node(removed_node));

        path_nodes.tail = Rc::new(directory);

        Ok(OpResult {
            root_dir: path_nodes.reconstruct(),
            result: (),
        })
    }

    /// Encode the directory as a CBOR object.
    pub async fn encode<B: BlockStore>(&self, store: &mut B) -> Result<Vec<u8>> {
        let mut bytes = Vec::new();

        // Write the major of the section being written.
        encode::write_u64(
            &mut bytes,
            MajorKind::Map,
            PublicDirectory::FIELDS.len() as u64,
        )?;

        // Ordering the fields by name based on RFC-7049 which is also what libipld uses.
        let mut cbor_order: Vec<&'static str> = Vec::from_iter(PublicDirectory::FIELDS);
        cbor_order.sort_unstable_by(|&a, &b| match a.len().cmp(&b.len()) {
            Ordering::Greater => Ordering::Greater,
            Ordering::Less => Ordering::Less,
            Ordering::Equal => a.cmp(b),
        });

        // Iterate over the fields.
        for field in cbor_order.iter() {
            // Encode field name.
            field.encode(DagCborCodec, &mut bytes)?;
            // Encode field value.
            match *field {
                "metadata" => {
                    self.metadata.encode(DagCborCodec, &mut bytes)?;
                }
                "userland" => {
                    let new_userland = {
                        let mut tmp = BTreeMap::new();
                        for (k, link) in self.userland.iter() {
                            let cid = link.seal(store).await?;
                            tmp.insert(k.clone(), cid);
                        }
                        tmp
                    };

                    new_userland.encode(DagCborCodec, &mut bytes)?;
                }
                "previous" => {
                    self.previous.encode(DagCborCodec, &mut bytes)?;
                }
                _ => unreachable!(),
            }
        }

        Ok(bytes)
    }

    // TODO(appcypher): Make non recursive.
    /// Constructs a tree from directory with `base` as the historical ancestor.
    pub async fn base_history_on<B: BlockStore>(
        self: Rc<Self>,
        base: Rc<Self>,
        store: &mut B,
    ) -> Result<OpResult<()>> {
        if Rc::ptr_eq(&self, &base) {
            return Ok(OpResult {
                root_dir: Rc::clone(&self),
                result: (),
            });
        }

        let mut dir = (*self).clone();
        dir.previous = Some(base.store(store).await?);

        for (name, entry) in self.userland.iter() {
            if let Some(base_entry) = base.userland.get(name) {
                if let Some(new_entry) =
                    Self::base_history_on_helper(entry, base_entry, store).await?
                {
                    dir.userland.insert(name.to_string(), new_entry);
                }
            }
        }

        Ok(OpResult {
            root_dir: Rc::new(dir),
            result: (),
        })
    }

    /// Constructs a tree from directory with `base` as the historical ancestor.
    #[async_recursion(?Send)]
    pub async fn base_history_on_helper<B: BlockStore>(
        link: &Link,
        base_link: &Link,
        store: &mut B,
    ) -> Result<Option<Link>> {
        if link.partial_equal(base_link, store).await? {
            return Ok(None);
        }

        let node = link.resolve(store).await?;
        let base_node = base_link.resolve(store).await?;

        let (mut dir, dir_rc, base_dir) = match (node, base_node) {
            (PublicNode::Dir(dir_rc), PublicNode::Dir(base_dir_rc)) => {
                let mut dir = (*dir_rc).clone();
                dir.previous = Some(base_link.seal(store).await?);
                (dir, dir_rc, base_dir_rc)
            }
            (PublicNode::File(file_rc), PublicNode::File(_)) => {
                let mut file = (*file_rc).clone();
                file.previous = Some(base_link.seal(store).await?);
                return Ok(Some(Link::Node(PublicNode::File(Rc::new(file)))));
            }
            _ => {
                // One is a file and the other is a directory
                // No need to fix up previous links
                return Ok(None);
            }
        };

        for (name, entry) in dir_rc.userland.iter() {
            if let Some(base_entry) = base_dir.userland.get(name) {
                if let Some(new_entry) =
                    Self::base_history_on_helper(entry, base_entry, store).await?
                {
                    dir.userland.insert(name.to_string(), new_entry);
                }
            }
        }

        Ok(Some(Link::Node(PublicNode::Dir(Rc::new(dir)))))
    }

    /// Gets the iterator for walking the history of a directory node.
    pub fn get_history<B: BlockStore>(
        self: Rc<Self>,
        store: &B,
    ) -> impl Stream<Item = Result<Cid>> + '_ {
        let mut working_node = self;
        try_stream! {
            while let Some(cid) = working_node.get_previous() {
                working_node = Rc::new(blockstore::load(store, &cid).await?);
                yield cid;
            }
        }
    }
}

impl Id for PublicDirectory {
    fn get_id(&self) -> String {
        format!("{:p}", &self.metadata)
    }
}

// Decoding CBOR-encoded PublicDirectory from bytes.
impl Decode<DagCborCodec> for PublicDirectory {
    fn decode<R: Read + Seek>(c: DagCborCodec, r: &mut R) -> Result<Self> {
        // Ensure the major kind is a map.
        let major = decode::read_major(r)?;
        ensure!(
            major.kind() == MajorKind::Map,
            FsError::UndecodableCborData("Unsupported major".into())
        );

        // Decode the length of the map.
        let _ = decode::read_uint(r, major)?;

        // Ordering the fields by name based on RFC-7049 which is also what libipld uses.
        let mut cbor_order: Vec<&'static str> = Vec::from_iter(PublicDirectory::FIELDS);
        cbor_order.sort_unstable_by(|&a, &b| match a.len().cmp(&b.len()) {
            Ordering::Greater => Ordering::Greater,
            Ordering::Less => Ordering::Less,
            Ordering::Equal => a.cmp(b),
        });

        // Iterate over the fields.
        let mut metadata = None;
        let mut userland = BTreeMap::new();
        let mut previous = None;

        // Iterate over the fields.
        for field in cbor_order.iter() {
            // Decode field name.
            String::decode(c, r)?;

            // Decode field value.
            match *field {
                "metadata" => {
                    metadata = Some(Metadata::decode(c, r)?);
                }
                "userland" => {
                    userland = BTreeMap::<_, Cid>::decode(c, r)?
                        .into_iter()
                        .map(|(k, cid)| (k, Link::Cid(cid)))
                        .collect();
                }
                "previous" => {
                    previous = <Option<Cid>>::decode(c, r)?;
                }
                _ => unreachable!(),
            }
        }

        Ok(PublicDirectory {
            metadata: metadata
                .ok_or_else(|| FsError::UndecodableCborData("Missing unix_fs".into()))?,
            userland,
            previous,
        })
    }
}

//--------------------------------------------------------------------------------------------------
// Utilities
//--------------------------------------------------------------------------------------------------

mod utils {
    use anyhow::Result;

    use crate::{error, FsError};

    pub(super) fn split_last(path_segments: &[String]) -> Result<(&[String], &String)> {
        match path_segments.split_last() {
            Some((last, rest)) => Ok((rest, last)),
            None => error(FsError::InvalidPath),
        }
    }
}

//--------------------------------------------------------------------------------------------------
// Tests
//--------------------------------------------------------------------------------------------------

#[cfg(test)]
mod public_directory_tests {
    use std::io::Cursor;

    use super::*;
    use crate::{public::PublicFile, MemoryBlockStore};
    use chrono::Utc;

    #[async_std::test]
    async fn look_up_can_fetch_file_added_to_directory() {
        let root_dir = Rc::new(PublicDirectory::new(Utc::now()));
        let store = MemoryBlockStore::default();
        let content_cid = Cid::default();
        let time = Utc::now();

        let OpResult { root_dir, .. } = root_dir
            .write(&["text.txt".into()], content_cid, time, &store)
            .await
            .unwrap();

        let node = root_dir.lookup_node("text.txt", &store).await.unwrap();

        assert!(node.is_some());

        assert_eq!(
            node,
            Some(PublicNode::File(Rc::new(PublicFile::new(
                time,
                content_cid
            ))))
        );
    }

    #[async_std::test]
    async fn look_up_cannot_fetch_file_not_added_to_directory() {
        let root = PublicDirectory::new(Utc::now());
        let store = MemoryBlockStore::default();

        let node = root.lookup_node("Unknown", &store).await;

        assert!(node.is_ok());

        assert_eq!(node.unwrap(), None);
    }

    #[async_std::test]
    async fn directory_added_to_store_can_be_retrieved() {
        let root = PublicDirectory::new(Utc::now());
        let mut store = MemoryBlockStore::default();

        let cid = root.store(&mut store).await.unwrap();

        let bytes = store.get_block(&cid).await.unwrap();

        let mut cursor = Cursor::new(bytes.as_ref());

        let decoded_root = PublicDirectory::decode(DagCborCodec, &mut cursor).unwrap();

        assert_eq!(root, decoded_root);
    }

    #[async_std::test]
    async fn directory_can_encode_decode_as_cbor() {
        let root = PublicDirectory::new(Utc::now());
        let mut store = MemoryBlockStore::default();

        let encoded_bytes = root.encode(&mut store).await.unwrap();

        let mut cursor = Cursor::new(encoded_bytes);

        let decoded_root = PublicDirectory::decode(DagCborCodec, &mut cursor).unwrap();

        assert_eq!(root, decoded_root);
    }

    #[async_std::test]
    async fn mkdir_can_create_new_directory() {
        let time = Utc::now();
        let store = MemoryBlockStore::default();

        // on a fresh directory
        let OpResult { root_dir, .. } = Rc::new(PublicDirectory::new(time))
            // create a new dir
            .mkdir(&["tamedun".into(), "pictures".into()], time, &store)
            .await
            .unwrap();

        // get the node
        let OpResult { result, .. } = root_dir
            .get_node(&["tamedun".into(), "pictures".into()], &store)
            .await
            .unwrap();

        assert!(result.is_some());
    }

    #[async_std::test]
    async fn ls_can_list_children_under_directory() {
        let time = Utc::now();
        let store = MemoryBlockStore::default();
        let root_dir = Rc::new(PublicDirectory::new(time));

        let OpResult { root_dir, .. } = root_dir
            .mkdir(&["tamedun".into(), "pictures".into()], time, &store)
            .await
            .unwrap();

        let OpResult { root_dir, .. } = root_dir
            .write(
                &["tamedun".into(), "pictures".into(), "puppy.jpg".into()],
                Cid::default(),
                time,
                &store,
            )
            .await
            .unwrap();

        let OpResult { root_dir, .. } = root_dir
            .mkdir(
                &["tamedun".into(), "pictures".into(), "cats".into()],
                time,
                &store,
            )
            .await
            .unwrap();

        let OpResult { result, .. } = root_dir
            .ls(&["tamedun".into(), "pictures".into()], &store)
            .await
            .unwrap();

        assert_eq!(result.len(), 2);

        assert_eq!(result[0].0, String::from("cats"));

        assert_eq!(result[1].0, String::from("puppy.jpg"));

        assert_eq!(result[0].1.unix_fs.kind, UnixFsNodeKind::Dir);

        assert_eq!(result[1].1.unix_fs.kind, UnixFsNodeKind::File);
    }

    #[async_std::test]
    async fn rm_can_remove_children_from_directory() {
        let time = Utc::now();
        let store = MemoryBlockStore::default();
        let root_dir = Rc::new(PublicDirectory::new(time));

        let OpResult { root_dir, .. } = root_dir
            .mkdir(&["tamedun".into(), "pictures".into()], time, &store)
            .await
            .unwrap();

        let OpResult { root_dir, .. } = root_dir
            .write(
                &["tamedun".into(), "pictures".into(), "puppy.jpg".into()],
                Cid::default(),
                time,
                &store,
            )
            .await
            .unwrap();

        let OpResult { root_dir, .. } = root_dir
            .mkdir(
                &["tamedun".into(), "pictures".into(), "cats".into()],
                time,
                &store,
            )
            .await
            .unwrap();

        let result = root_dir
            .rm(&["tamedun".into(), "pictures".into()], &store)
            .await;

        assert!(result.is_ok());

        let result = result
            .unwrap()
            .root_dir
            .rm(&["tamedun".into(), "pictures".into()], &store)
            .await;

        assert!(result.is_err());
    }

    #[async_std::test]
    async fn read_can_fetch_userland_of_file_added_to_directory() {
        let mut store = MemoryBlockStore::default();
        let content_cid = Cid::default();
        let time = Utc::now();

        let OpResult { root_dir, .. } = Rc::new(PublicDirectory::new(time))
            .write(&["text.txt".into()], content_cid, time, &store)
            .await
            .unwrap();

        let OpResult { result, .. } = root_dir
            .read(&["text.txt".into()], &mut store)
            .await
            .unwrap();

        assert_eq!(result, content_cid);
    }

    #[async_std::test]
    async fn path_nodes_can_generates_new_path_nodes() {
        let store = MemoryBlockStore::default();
        let now = Utc::now();

        let path_nodes = PathNodes::new(
            now,
            &["Documents".into(), "Apps".into()],
            Rc::new(PublicDirectory::new(now)),
        );

        let reconstructed = path_nodes.clone().reconstruct();

        let result = reconstructed
            .get_node_path(&["Documents".into(), "Apps".into()], &store)
            .await
            .unwrap();

        match result {
            GetNodePathResult::MissingLink(_, segment) => panic!("MissingLink {segment}"),
            GetNodePathResult::NotADirectory(_, segment) => panic!("NotADirectory {segment}"),
            GetNodePathResult::Complete(path_nodes_2) => {
                assert_eq!(path_nodes.path.len(), path_nodes_2.path.len());
                assert_eq!(path_nodes.path[0].1, path_nodes_2.path[0].1);
                assert_eq!(path_nodes.path[1].1, path_nodes_2.path[1].1);
            }
        }
    }

    #[async_std::test]
    async fn base_history_on_can_create_a_new_derived_tree_pointing_to_base() {
        let time = Utc::now();
        let mut store = MemoryBlockStore::default();
        let root_dir = Rc::new(PublicDirectory::new(time));

        let OpResult {
            root_dir: base_root,
            ..
        } = root_dir
            .write(
                &["pictures".into(), "cats".into(), "tabby.jpg".into()],
                Cid::default(),
                time,
                &store,
            )
            .await
            .unwrap();

        let OpResult {
            root_dir: updated_root,
            ..
        } = Rc::clone(&base_root)
            .write(
                &["pictures".into(), "cats".into(), "luna.png".into()],
                Cid::default(),
                time,
                &store,
            )
            .await
            .unwrap();

        let OpResult {
            root_dir: derived_root,
            ..
        } = updated_root
            .base_history_on(Rc::clone(&base_root), &mut store)
            .await
            .unwrap();

        // Assert that the root node points to its old version.
        let derived_previous_cid = derived_root.get_previous();
        let base_cid = base_root.store(&mut store).await.unwrap();

        assert!(derived_previous_cid.is_some());
        assert_eq!(derived_previous_cid.unwrap(), base_cid);

        // Assert that some node that exists between versions points to its old version.
        let OpResult {
            result: derived_node,
            ..
        } = Rc::clone(&derived_root)
            .get_node(&["pictures".into(), "cats".into()], &store)
            .await
            .unwrap();

        let OpResult {
            result: base_node, ..
        } = base_root
            .get_node(&["pictures".into(), "cats".into()], &store)
            .await
            .unwrap();

        assert!(derived_node.is_some());
        assert!(base_node.is_some());

        let derived_previous_cid = derived_node.unwrap().get_previous();
        let base_cid = base_node.unwrap().store(&mut store).await.unwrap();

        assert!(derived_previous_cid.is_some());
        assert_eq!(derived_previous_cid.unwrap(), base_cid);

        // Assert that some node that doesn't exists between versions does not point to anything.
        let OpResult {
            result: derived_node,
            ..
        } = Rc::clone(&derived_root)
            .get_node(
                &["pictures".into(), "cats".into(), "luna.png".into()],
                &store,
            )
            .await
            .unwrap();

        assert!(derived_node.is_some());
        assert!(matches!(derived_node.unwrap().get_previous(), None));
    }

    #[async_std::test]
    async fn mv_can_move_sub_directory_to_another_valid_location() {
        let time = Utc::now();
        let store = MemoryBlockStore::default();
        let root_dir = Rc::new(PublicDirectory::new(time));

        let OpResult { root_dir, .. } = root_dir
            .write(
                &["pictures".into(), "cats".into(), "tabby.jpg".into()],
                Cid::default(),
                time,
                &store,
            )
            .await
            .unwrap();

        let OpResult { root_dir, .. } = root_dir
            .write(
                &["pictures".into(), "cats".into(), "luna.png".into()],
                Cid::default(),
                time,
                &store,
            )
            .await
            .unwrap();

        let OpResult { root_dir, .. } = root_dir
            .mkdir(&["images".into()], time, &store)
            .await
            .unwrap();

        let OpResult { root_dir, .. } = root_dir
            .basic_mv(
                &["pictures".into(), "cats".into()],
                &["images".into(), "cats".into()],
                &store,
            )
            .await
            .unwrap();

        let OpResult { root_dir, result } = root_dir.ls(&["images".into()], &store).await.unwrap();

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].0, String::from("cats"));

        let OpResult { result, .. } = root_dir.ls(&["pictures".into()], &store).await.unwrap();

        assert_eq!(result.len(), 0);
    }

    #[async_std::test]
    async fn mv_cannot_move_sub_directory_to_invalid_location() {
        let time = Utc::now();
        let store = MemoryBlockStore::default();
        let root_dir = Rc::new(PublicDirectory::new(time));

        let OpResult { root_dir, .. } = root_dir
            .mkdir(
                &[
                    "videos".into(),
                    "movies".into(),
                    "anime".into(),
                    "ghibli".into(),
                ],
                time,
                &store,
            )
            .await
            .unwrap();

        let result = root_dir
            .basic_mv(
                &["videos".into(), "movies".into()],
                &["videos".into(), "movies".into(), "anime".into()],
                &store,
            )
            .await;

        assert!(result.is_err());
    }

    #[async_std::test]
    async fn mv_can_rename_directories() {
        let time = Utc::now();
        let mut store = MemoryBlockStore::default();
        let root_dir = Rc::new(PublicDirectory::new(time));

        let OpResult { root_dir, .. } = root_dir
            .write(&["file.txt".into()], Cid::default(), time, &store)
            .await
            .unwrap();

        let OpResult { root_dir, .. } = root_dir
            .basic_mv(&["file.txt".into()], &["renamed.txt".into()], &store)
            .await
            .unwrap();

        let OpResult { result, .. } = root_dir
            .read(&["renamed.txt".into()], &mut store)
            .await
            .unwrap();

        assert!(result == Cid::default());
    }

    #[async_std::test]
    async fn mv_fails_moving_directories_to_files() {
        let time = Utc::now();
        let store = MemoryBlockStore::default();
        let root_dir = Rc::new(PublicDirectory::new(time));

        let OpResult { root_dir, .. } = root_dir
            .mkdir(&["movies".into(), "ghibli".into()], time, &store)
            .await
            .unwrap();

        let OpResult { root_dir, .. } = root_dir
            .write(&["file.txt".into()], Cid::default(), time, &store)
            .await
            .unwrap();

        let result = root_dir
            .basic_mv(
                &["movies".into(), "ghibli".into()],
                &["file.txt".into()],
                &store,
            )
            .await;

        assert!(result.is_err());
    }
}
