//! Public fs directory node.

use std::{
    cmp::Ordering,
    collections::BTreeMap,
    io::{Read, Seek},
    rc::Rc,
};

use crate::{error, BlockStore, FsError, Metadata, UnixFsNodeKind};
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

use super::{DeepClone, Id, Link, PublicFile, PublicNode};

//--------------------------------------------------------------------------------------------------
// Type Definitions
//--------------------------------------------------------------------------------------------------

/// A directory in a WNFS public file system.
#[derive(Debug, Clone, PartialEq, Eq, FieldNames)]
pub struct PublicDirectory {
    metadata: Metadata,
    userland: BTreeMap<String, Link>,
    previous: Option<Cid>,
}

/// Represents a directory that has possibly diverged. It is the result of operating on a directory.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OpResult<T> {
    // The root node. It is the same as the previous root node if the directory has not been diverged.
    pub root_node: Rc<PublicDirectory>,
    // Implementation dependent but it usually the last leaf node operated on.
    pub result: T,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PathTo<T> {
    path: Vec<(Rc<PublicDirectory>, String)>,
    focus: T,
}

pub enum GetNodePathResult {
    Complete(PathTo<Rc<PublicDirectory>>),
    MissingLink(PathTo<Rc<PublicDirectory>>, String),
    NotADirectory(PathTo<Rc<PublicDirectory>>, String),
}

fn reconstruct_dir(path_nodes: PathTo<Rc<PublicDirectory>>) -> Rc<PublicDirectory> {
    if path_nodes.path.is_empty() {
        return path_nodes.focus;
    }

    let mut working_node = path_nodes.focus;

    for (dir, segment) in path_nodes.path.iter().rev() {
        let mut dir = (**dir).clone();
        let link = Link::Node(Rc::new(PublicNode::Dir(working_node)));
        dir.userland.insert(segment.clone(), link);
        working_node = Rc::new(dir);
    }

    working_node
}

fn new_path_to<T>(time: DateTime<Utc>, path: &[String], to: T) -> PathTo<T> {
    let path: Vec<(Rc<PublicDirectory>, String)> = path
        .iter()
        .map(|segment| (Rc::new(PublicDirectory::new(time)), segment.to_string()))
        .collect();
    PathTo { path, focus: to }
}

fn expect_nonempty_path(path_segments: &[String]) -> Result<(&[String], &String)> {
    match path_segments.split_last() {
        Some((last, rest)) => Ok((rest, last)),
        None => error(FsError::InvalidPath),
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

    /// Updates the directory previous field
    pub fn update_previous(&mut self, previous: Option<Cid>) {
        self.0.borrow_mut().previous = previous;
    }

    // Gets the previous value of the directory.
    pub fn get_previous(&self) -> Option<Cid> {
        self.0.borrow().previous
    }

    /// Follows a path and fetches the node at the end of the path.
    ///
    /// If path is empty, this returns a cloned directory based on `self`.
    ///
    /// If `diverge` is true, the path diverges. This is only used internally.
    pub async fn get_node<B: BlockStore>(
        &self,
        path_segments: &[String],
        store: &B,
        diverge: bool,
    ) -> Result<OpResult<Option<Shared<PublicNode>>>> {
        // Set working node to current directory.
        let root_node = shared(PublicNode::Dir(self.clone()));
        let mut working_node = Some(Rc::clone(&root_node));

        // The nodes along the path specified.
        let mut path_nodes = {
            let mut tmp = Vec::with_capacity(path_segments.len());
            tmp.push((String::new(), Rc::clone(&root_node)));
            tmp
        };
    }

        // Iterate over the path segments.
    // root -"Documents"-> documents -"file.txt"-> filetxt
    // root.get_node_path([]) -> PathNodes(path_nodes:  , root)
    // root.get_node_path(["Documents"]) -> PathNodes([root, "Documents"], documents)
    // "Documents/Apps/flatmate.json"
    // root.get_node_path(["Documents", "Apps"])
    //  -> PathNodes([(root, "Documents"), (documents, "Apps")], apps)

    pub async fn get_node_path<B: BlockStore>(
        self: Rc<PublicDirectory>,
        path_segments: &[String],
        store: &B,
    ) -> Result<GetNodePathResult> {
        use GetNodePathResult::*;

        let mut working_node = self;
        let mut path_nodes = Vec::with_capacity(path_segments.len());

        for segment in path_segments.iter() {
            match working_node.lookup_node(segment, store).await? {
                None => {
                    let path_to = PathTo {
                        path: path_nodes,
                        focus: Rc::clone(&working_node),
                    };
                    return Ok(MissingLink(path_to, segment.to_string()));
                }
                Some(found_node) => match &*found_node {
                    PublicNode::Dir(directory) => {
                        path_nodes.push((Rc::clone(&working_node), segment.to_string()));
                        working_node = Rc::clone(directory);
                    }
                    _ => {
                        let path_to = PathTo {
                            path: path_nodes,
                            focus: Rc::clone(&working_node),
                        };
                        return Ok(NotADirectory(path_to, segment.to_string()));
                    }
                },
            }
        }

        Ok(Complete(PathTo {
            path: path_nodes,
            focus: Rc::clone(&working_node),
        }))
    }

    pub async fn get_node_path_with_mkdir<B: BlockStore>(
        self: Rc<PublicDirectory>,
        path_segments: &[String],
        mkdir_ctime: DateTime<Utc>,
        store: &B,
    ) -> Result<PathTo<Rc<PublicDirectory>>> {
        match self.get_node_path(path_segments, store).await? {
            GetNodePathResult::Complete(np) => Ok(np),
            GetNodePathResult::NotADirectory(_, _) => error(FsError::InvalidPath),
            GetNodePathResult::MissingLink(path_so_far, missing_link) => {
                // path setup
                // path_so_far.path covers [0..path_so_far.path.len()]
                // missing_link is index path_so_far.path.len()
                // missing_path covers [path_so_far.path.len()+1..path_segments.len()]
                let missing_path = path_segments.split_at(path_so_far.path.len() + 1).1;
                let missing_path_to = new_path_to(
                    mkdir_ctime,
                    missing_path,
                    Rc::new(PublicDirectory::new(mkdir_ctime)),
                );
                Ok(PathTo {
                    path: [
                        path_so_far.path,
                        vec![(path_so_far.focus, missing_link)],
                        missing_path_to.path,
                    ]
                    .concat(),
                    focus: missing_path_to.focus,
                })
            }
        }
    }

    /// Follows a path and fetches the node at the end of the path.
    ///
    /// If path is empty, this returns a new node based on self.
    pub async fn get_node<B: BlockStore>(
        self: Rc<PublicDirectory>,
        path_segments: &[String],
        store: &B,
    ) -> Result<OpResult<Option<Rc<PublicNode>>>> {
        let root_node = Rc::clone(&self);

        Ok(match path_segments.split_last() {
            Some((path_segment, parent_path)) => {
                match self.get_node_path(parent_path, store).await? {
                    GetNodePathResult::Complete(parent_path_to) => OpResult {
                        root_node,
                        result: parent_path_to
                            .focus
                            .lookup_node(path_segment, store)
                            .await?,
                    },
                    GetNodePathResult::MissingLink(_, _) => bail!(FsError::NotFound),
                    GetNodePathResult::NotADirectory(_, _) => bail!(FsError::NotFound),
                }
            }
            None => OpResult {
                root_node,
                result: Some(Rc::new(PublicNode::Dir(self))),
            },
        })
    }

    /// Looks up a node by its path name in the current directory.
    ///
    /// TODO(appcypher): What is a valid path segment identifier?
    pub async fn lookup_node<B: BlockStore>(
        &self,
        path_segment: &str,
        store: &B,
    ) -> Result<Option<Rc<PublicNode>>> {
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
        self: Rc<PublicDirectory>,
        path_segments: &[String],
        store: &mut B,
    ) -> Result<OpResult<Cid>> {
        let root_node = Rc::clone(&self);
        let (path, filename) = expect_nonempty_path(path_segments)?;

        match self.get_node_path(path, store).await? {
            GetNodePathResult::Complete(node_path) => {
                match node_path.focus.lookup_node(filename, store).await? {
                    Some(node) => match &*node {
                        PublicNode::File(file) => Ok(OpResult{ root_node, result: file.userland }),
                        PublicNode::Dir(_) => error(FsError::NotAFile),
                    },
                    None => error(FsError::NotFound),
                }
            }
            _ => error(FsError::NotFound),
        }
    }

    /// Writes a file to the directory.
    ///
    /// Rather than mutate the directory directly, we create a new directory and return it.
    pub async fn write<B: BlockStore>(
        self: Rc<PublicDirectory>,
        path_segments: &[String],
        content_cid: Cid,
        time: DateTime<Utc>,
        store: &B,
    ) -> Result<OpResult<()>> {
        let (directory_path, filename) = expect_nonempty_path(path_segments)?;

        // get_node_path_with_mkdir will create directories if they don't exist yet
        let mut directory_path_nodes = self
            .get_node_path_with_mkdir(directory_path, time, store)
            .await?;

        let mut directory = (*directory_path_nodes.focus).clone();

        // Modify the file if it already exists, otherwise create a new file with expected content
        let file = match directory.lookup_node(filename, store).await? {
            Some(link) => match &*link {
                PublicNode::File(file_before) => {
                    let mut file = (**file_before).clone();
                    file.userland = content_cid;
                    file.metadata = Metadata::new(time, UnixFsNodeKind::File);
                    file
                }
                PublicNode::Dir(_) => bail!(FsError::DirectoryAlreadyExists),
            },
            None => PublicFile::new(time, content_cid),
        };

        // insert the file into its parent directory
        directory.userland.insert(
            filename.to_string(),
            Link::Node(Rc::new(PublicNode::File(Rc::new(file)))),
        );
        directory_path_nodes.focus = Rc::new(directory);

        // reconstruct the file path
        Ok(OpResult {
            root_node: reconstruct_dir(directory_path_nodes),
            result: (),
        })
    }

    /// Creates a new directory at the specified path.
    ///
    /// If path is empty, this returns the root node.
    ///
    /// This method acts like `mkdir -p` in Unix because it creates intermediate directories if they do not exist.
    ///
    /// If `diverge_anyway` is set to true, the path diverges even if directory exists. This is only used internally.
    pub async fn mkdir<B: BlockStore>(
        self: Rc<PublicDirectory>,
        path_segments: &[String],
        time: DateTime<Utc>,
        store: &B,
    ) -> Result<OpResult<()>> {
        let node_path_with_dirs = self
            .get_node_path_with_mkdir(path_segments, time, store)
            .await?;

        Ok(OpResult {
            root_node: reconstruct_dir(node_path_with_dirs),
            result: (),
        })
    }

    /// Returns the name and metadata of the direct children of a directory.
    pub async fn ls<B: BlockStore>(
        self: Rc<PublicDirectory>,
        path_segments: &[String],
        store: &B,
    ) -> Result<OpResult<Vec<(String, Metadata)>>> {
        let root_node = Rc::clone(&self);
        match self.get_node_path(path_segments, store).await? {
            GetNodePathResult::Complete(node_path) => {
                let mut result = vec![];
                for (name, link) in node_path.focus.userland.iter() {
                    match &*link.resolve(store).await? {
                        PublicNode::File(file) => {
                            result.push((name.clone(), file.metadata.clone()));
                        }
                        PublicNode::Dir(dir) => {
                            result.push((name.clone(), dir.metadata.clone()));
                        }
                    }
                }
                Ok(OpResult { root_node, result })
            }
            _ => bail!(FsError::NotFound),
        }
    }

    /// Removes a file or directory from the directory.
    ///
    /// Rather than mutate the directory directly, we create a new directory and return it.
    pub async fn rm<B: BlockStore>(
        self: Rc<PublicDirectory>,
        path_segments: &[String],
        store: &B,
    ) -> Result<OpResult<Rc<PublicNode>>> {
        let (directory_path, node_name) = expect_nonempty_path(path_segments)?;

        let mut directory_node_path = match self.get_node_path(directory_path, store).await? {
            GetNodePathResult::Complete(node_path) => node_path,
            _ => bail!(FsError::NotFound),
        };

        let mut directory = (*directory_node_path.focus).clone();

        // remove the entry from its parent directory
        let removed_node = match directory.userland.remove(node_name) {
            Some(entry) => entry.resolve(store).await?,
            None => bail!(FsError::NotFound),
        };

        directory_node_path.focus = Rc::new(directory);

        Ok(OpResult {
            root_node: reconstruct_dir(directory_node_path),
            result: removed_node,
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

    /// Constructs a tree from directory with `base` as the historical ancestor.
    pub async fn base_history_on<B: BlockStore>(
        &self,
        base: &PublicDirectory,
        store: &mut B,
    ) -> Result<OpResult<()>> {
        let root_node = shared(PublicNode::Dir(self.deep_clone()));
        let root_base_node = shared(PublicNode::Dir(base.clone()));

        let mut stack = vec![(Rc::clone(&root_node), Some(root_base_node))];

        // We use post-order depth-first traversal.
        while let Some((node, base_node)) = stack.pop() {
            // If base_node does not exist then there is nothing to do because it means node does not have an ancestor.
            let base_node = match base_node {
                Some(base_node) => base_node,
                None => continue,
            };

            // If the cids of both are the same, then nothing changed.
            let node_cid = node.borrow().store(store).await?;
            let base_node_cid = base_node.borrow().store(store).await?;
            if node_cid == base_node_cid {
                continue;
            }

            // Fix up the directory's previous field.
            node.borrow_mut().update_previous(Some(base_node_cid));

            let pair = (&mut *node.borrow_mut(), &*base_node.borrow());
            if let (PublicNode::Dir(dir), PublicNode::Dir(base_dir)) = pair {
                // Iterate through the userland of the node.
                let mut new_user_land = BTreeMap::new();
                for (name, link) in dir.0.borrow().userland.iter() {
                    // Deep clone child node.
                    let child_node = link.resolve(store).await?;
                    let child_base_node = match base_dir.0.borrow().userland.get(name) {
                        Some(link) => Some(link.resolve(store).await?),
                        None => None,
                    };

                    // Construct the new userland that the parent node will now point to.
                    new_user_land.insert(name.clone(), Link::Node(Rc::clone(&child_node)));

                    // Push child node onto the stack.
                    stack.push((child_node, child_base_node));
                }

                // Mutate dir to point to the new userland.
                dir.0.borrow_mut().userland = new_user_land;
            }
        }

        Ok(OpResult {
            root_node,
            result: (),
        })
    }

    /// Gets the iterator for walking the history of a directory node.
    pub async fn get_history<'a, B: BlockStore>(
        &self,
        store: &'a B,
    ) -> impl Stream<Item = Result<Cid>> + 'a {
        let mut working_node = self.clone();
        try_stream! {
            while let Some(cid) = {
                let tmp = working_node.0.borrow();
                tmp.previous
            } {
                working_node = blockstore::load(store, &cid).await?;
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

impl DeepClone for PublicDirectory {
    fn deep_clone(&self) -> Self {
        Self(shared(self.0.borrow().clone()))
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
        let root_node = Rc::new(PublicDirectory::new(Utc::now()));

        let store = MemoryBlockStore::default();

        let content_cid = Cid::default();

        let time = Utc::now();

        let OpResult { root_node, .. } = root_node
            .write(&["text.txt".into()], content_cid, time, &store)
            .await
            .unwrap();

        let node = root_node.lookup_node("text.txt", &store).await.unwrap();

        assert!(node.is_some());

        assert_eq!(
            node,
            Some(Rc::new(PublicNode::File(Rc::new(PublicFile::new(
                time,
                content_cid
            )))))
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
        let OpResult { root_node, .. } = Rc::new(PublicDirectory::new(time))
            // create a new dir
            .mkdir(&["tamedun".into(), "pictures".into()], time, &store)
            .await
            .unwrap();

        // get the node
        let OpResult { result, .. } = root_node
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

        let OpResult { root_node, .. } = root_dir
            .mkdir(&["tamedun".into(), "pictures".into()], time, &store)
            .await
            .unwrap();

        let OpResult { root_node, .. } = root_node
            .write(
                &["tamedun".into(), "pictures".into(), "puppy.jpg".into()],
                Cid::default(),
                time,
                &store,
            )
            .await
            .unwrap();

        let OpResult { root_node, .. } = root_node
            .mkdir(
                &["tamedun".into(), "pictures".into(), "cats".into()],
                time,
                &store,
            )
            .await
            .unwrap();

        let OpResult { result, .. } = root_node
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

        let OpResult { root_node, .. } = root_dir
            .mkdir(&["tamedun".into(), "pictures".into()], time, &store)
            .await
            .unwrap();

        let OpResult { root_node, .. } = root_node
            .write(
                &["tamedun".into(), "pictures".into(), "puppy.jpg".into()],
                Cid::default(),
                time,
                &store,
            )
            .await
            .unwrap();

        let OpResult { root_node, .. } = root_node
            .mkdir(
                &["tamedun".into(), "pictures".into(), "cats".into()],
                time,
                &store,
            )
            .await
            .unwrap();

        let result = root_node
            .rm(&["tamedun".into(), "pictures".into()], &store)
            .await;

        assert!(result.is_ok());

        let result = result
            .unwrap()
            .root_node
            .rm(&["tamedun".into(), "pictures".into()], &store)
            .await;

        assert!(result.is_err());
    }

    #[async_std::test]
    async fn read_can_fetch_userland_of_file_added_to_directory() {
        let mut store = MemoryBlockStore::default();

        let content_cid = Cid::default();

        let time = Utc::now();

        let OpResult { root_node, .. } = Rc::new(PublicDirectory::new(time))
            .write(&["text.txt".into()], content_cid, time, &store)
            .await
            .unwrap();

        let OpResult { result, .. } = root_node
            .read(&["text.txt".into()], &mut store)
            .await
            .unwrap();

        assert_eq!(result, content_cid);
    }

    #[async_std::test]
    async fn test_new_path_to_generates_path_nodes() {
        let store = MemoryBlockStore::default();
        let now = Utc::now();

        let path_to = new_path_to(
            now,
            &["Documents".into(), "Apps".into()],
            Rc::new(PublicDirectory::new(now)),
        );

        let reconstructed = reconstruct_dir(path_to.clone());

        let result = reconstructed
            .get_node_path(&["Documents".into(), "Apps".into()], &store)
            .await
            .unwrap();

        match result {
            GetNodePathResult::MissingLink(_, segment) => panic!("MissingLink {segment}"),
            GetNodePathResult::NotADirectory(_, segment) => panic!("NotADirectory {segment}"),
            GetNodePathResult::Complete(path_to_2) => {
                assert_eq!(path_to.path.len(), path_to_2.path.len());
                assert_eq!(path_to.path[0].1, path_to_2.path[0].1);
                assert_eq!(path_to.path[1].1, path_to_2.path[1].1);
            }
        }
    }

    #[async_std::test]
    async fn base_history_on_can_create_a_new_derived_tree_pointing_to_base() {
        let time = Utc::now();
        let mut store = MemoryBlockStore::default();
        let root_dir = PublicDirectory::new(time);

        let OpResult {
            root_node: base_root,
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
            root_node: updated_root,
            ..
        } = base_root
            .borrow()
            .as_dir()
            .write(
                &["pictures".into(), "cats".into(), "luna.png".into()],
                Cid::default(),
                time,
                &store,
            )
            .await
            .unwrap();

        let OpResult {
            root_node: derived_root,
            ..
        } = updated_root
            .borrow()
            .as_dir()
            .base_history_on(base_root.borrow().as_dir(), &mut store)
            .await
            .unwrap();

        // Assert that the root node points to its old version.
        let derived_previous_cid = derived_root.borrow().as_dir().get_previous();
        let base_cid = base_root.borrow().store(&mut store).await.unwrap();

        assert!(derived_previous_cid.is_some());
        assert_eq!(derived_previous_cid.unwrap(), base_cid);

        // Assert that some node that exists between versions points to its old version.
        let OpResult {
            result: derived_node,
            ..
        } = derived_root
            .borrow()
            .as_dir()
            .get_node(&["pictures".into(), "cats".into()], &store, false)
            .await
            .unwrap();

        let OpResult {
            result: base_node, ..
        } = base_root
            .borrow()
            .as_dir()
            .get_node(&["pictures".into(), "cats".into()], &store, false)
            .await
            .unwrap();

        assert!(derived_node.is_some());
        assert!(base_node.is_some());

        let derived_previous_cid = derived_node.unwrap().borrow().as_dir().get_previous();
        let base_cid = base_node.unwrap().borrow().store(&mut store).await.unwrap();

        assert!(derived_previous_cid.is_some());
        assert_eq!(derived_previous_cid.unwrap(), base_cid);

        // Assert that some node that doesn't exists between versions does not point to anything.
        let OpResult {
            result: derived_node,
            ..
        } = derived_root
            .borrow()
            .as_dir()
            .get_node(
                &["pictures".into(), "cats".into(), "luna.png".into()],
                &store,
                false,
            )
            .await
            .unwrap();

        assert!(derived_node.is_some());
        assert!(matches!(
            derived_node.unwrap().borrow().as_file().get_previous(),
            None
        ));
    }

    #[async_std::test]
    async fn mv_can_move_sub_directory_to_another_valid_location() {
        let time = Utc::now();
        let store = MemoryBlockStore::default();
        let root_dir = PublicDirectory::new(time);

        let OpResult { root_node, .. } = root_dir
            .write(
                &["pictures".into(), "cats".into(), "tabby.jpg".into()],
                Cid::default(),
                time,
                &store,
            )
            .await
            .unwrap();

        let OpResult { root_node, .. } = root_node
            .borrow()
            .as_dir()
            .write(
                &["pictures".into(), "cats".into(), "luna.png".into()],
                Cid::default(),
                time,
                &store,
            )
            .await
            .unwrap();

        let OpResult { root_node, .. } = root_node
            .borrow()
            .as_dir()
            .mkdir(&["images".into()], time, &store, false)
            .await
            .unwrap();

        let OpResult { root_node, .. } = root_node
            .borrow()
            .as_dir()
            .basic_mv(
                &["pictures".into(), "cats".into()],
                &["images".into(), "cats".into()],
                time,
                &store,
            )
            .await
            .unwrap();

        let OpResult { root_node, result } = root_node
            .borrow()
            .as_dir()
            .ls(&["images".into()], &store)
            .await
            .unwrap();

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].0, String::from("cats"));

        let OpResult { result, .. } = root_node
            .borrow()
            .as_dir()
            .ls(&["pictures".into()], &store)
            .await
            .unwrap();

        assert_eq!(result.len(), 0);
    }

    #[async_std::test]
    async fn mv_cannot_move_sub_directory_to_invalid_location() {
        let time = Utc::now();
        let store = MemoryBlockStore::default();
        let root_dir = PublicDirectory::new(time);

        let OpResult { root_node, .. } = root_dir
            .mkdir(
                &[
                    "videos".into(),
                    "movies".into(),
                    "anime".into(),
                    "ghibli".into(),
                ],
                time,
                &store,
                false,
            )
            .await
            .unwrap();

        let result = root_node
            .borrow()
            .as_dir()
            .basic_mv(
                &["videos".into(), "movies".into()],
                &["videos".into(), "movies".into(), "anime".into()],
                time,
                &store,
            )
            .await;

        assert!(result.is_err());
    }

    #[async_std::test]
    async fn mv_can_rename_directories() {
        let time = Utc::now();
        let mut store = MemoryBlockStore::default();
        let root_dir = PublicDirectory::new(time);

        let OpResult { root_node, .. } = root_dir
            .write(&["file.txt".into()], Cid::default(), time, &store)
            .await
            .unwrap();

        let OpResult { root_node, .. } = root_node
            .borrow()
            .as_dir()
            .basic_mv(&["file.txt".into()], &["renamed.txt".into()], time, &store)
            .await
            .unwrap();

        let OpResult { result, .. } = root_node
            .borrow()
            .as_dir()
            .read(&["renamed.txt".into()], &mut store)
            .await
            .unwrap();

        assert!(result == Cid::default());
    }
    #[async_std::test]
    async fn mv_fails_moving_directories_to_files() {
        let time = Utc::now();
        let store = MemoryBlockStore::default();
        let root_dir = PublicDirectory::new(time);

        let OpResult { root_node, .. } = root_dir
            .mkdir(&["movies".into(), "ghibli".into()], time, &store, false)
            .await
            .unwrap();

        let OpResult { root_node, .. } = root_node
            .borrow()
            .as_dir()
            .write(&["file.txt".into()], Cid::default(), time, &store)
            .await
            .unwrap();

        let result = root_node
            .borrow()
            .as_dir()
            .basic_mv(
                &["movies".into(), "ghibli".into()],
                &["file.txt".into()],
                time,
                &store,
            )
            .await;

        assert!(result.is_err());
    }
}
