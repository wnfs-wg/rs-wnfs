//! Public fs directory node.

use std::{
    cmp::Ordering,
    collections::BTreeMap,
    future::Future,
    io::{Read, Seek},
    rc::Rc,
};

use crate::{error, shared, BlockStore, FsError, Metadata, Shared, UnixFsNodeKind};
use anyhow::{bail, ensure, Result};
use async_recursion::async_recursion;
use chrono::{DateTime, Utc};
use field_names::FieldNames;
use libipld::{
    cbor::{cbor::MajorKind, decode, encode, DagCborCodec},
    codec::{Decode, Encode},
    Cid, IpldCodec,
};

use super::{Id, Link, PublicFile, PublicNode};

//--------------------------------------------------------------------------------------------------
// Type Definitions
//--------------------------------------------------------------------------------------------------

/// A directory in a WNFS public file system.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublicDirectory(Shared<PublicDirectoryInner>);

#[derive(Debug, Clone, PartialEq, Eq, FieldNames)]
struct PublicDirectoryInner {
    metadata: Metadata,
    userland: BTreeMap<String, Link>,
    previous: Option<Cid>,
}

/// Represents a directory that has possibly diverged. It is the result of operating on a directory.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OpResult<T> {
    // The root node. It is the same as the previous root node if the directory has not been diverged.
    pub root_node: Shared<PublicNode>,
    // Implementation dependent but it usually the last leaf node operated on.
    pub result: T,
    /// Whether this is a divergence or not.
    pub diverged: bool,
}

//--------------------------------------------------------------------------------------------------
// Implementations
//--------------------------------------------------------------------------------------------------

impl PublicDirectory {
    /// Creates a new directory using the given metadata.
    pub fn new(time: DateTime<Utc>) -> Self {
        Self(shared(PublicDirectoryInner {
            metadata: Metadata::new(time, UnixFsNodeKind::Dir),
            userland: BTreeMap::new(),
            previous: None,
        }))
    }

    /// Follows a path and fetches the node at the end of the path.
    ///
    /// If path is empty, this returns a cloned directory based on `self`.
    ///
    /// If `diverge` is true, this will clone the spine of the path.
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
        let mut path_nodes: Vec<(String, Shared<PublicNode>)> = if !path_segments.is_empty() {
            vec![(String::new(), Rc::clone(&root_node))]
        } else {
            vec![]
        };

        // Iterate over the path segments.
        for (index, segment) in path_segments.iter().enumerate() {
            let mut working_node_mut = working_node.as_mut().unwrap().borrow_mut();

            // Fetch node representing the path segment in the working directory.
            match working_node_mut
                .as_mut_dir()
                .lookup_node(segment, store)
                .await?
            {
                Some(found_node) => match &*found_node.borrow() {
                    // If the node is a directory, set it as the working node.
                    PublicNode::Dir(_) => {
                        path_nodes.push((segment.to_string(), Rc::clone(&found_node)));

                        drop(working_node_mut);
                        working_node = Some(Rc::clone(&found_node));
                    }
                    // If the node is a file, we return it if it's the last segment.
                    PublicNode::File(_) => {
                        if index != path_segments.len() - 1 {
                            bail!(FsError::InvalidPath);
                        }

                        drop(working_node_mut);
                        working_node = Some(Rc::clone(&found_node));
                        break;
                    }
                },
                _ => {
                    // If the node is not found, we return an none.
                    drop(working_node_mut);
                    working_node = None;
                    break;
                }
            }
        }

        // Get nodes which may have diverged.
        let (root_node, working_node) = if diverge {
            let diverged_nodes = utils::diverge_and_patch(path_nodes);
            if !diverged_nodes.is_empty() {
                (
                    Rc::clone(&diverged_nodes.first().unwrap().1),
                    Some(Rc::clone(&diverged_nodes.last().unwrap().1)),
                )
            } else {
                (root_node, working_node)
            }
        } else {
            (root_node, working_node)
        };

        Ok(OpResult {
            root_node,
            result: working_node,
            diverged: diverge,
        })
    }

    /// Looks up a node by its path name in the current directory.
    ///
    /// TODO(appcypher): What is a valid path segment identifier?
    pub async fn lookup_node<B: BlockStore>(
        &self,
        path_segment: &str,
        store: &B,
    ) -> Result<Option<Shared<PublicNode>>> {
        Ok(match self.0.borrow().userland.get(path_segment) {
            Some(link) => Some(link.resolve(store).await?),
            None => None,
        })
    }

    /// Stores a directory as block(s) in provided block store.
    ///
    /// This function can be recursive if the directory contains other directories.
    #[async_recursion(?Send)]
    pub async fn store<B: BlockStore>(&self, store: &mut B) -> Result<Cid> {
        let bytes = self.encode(store).await?;
        store.put_block(bytes, IpldCodec::DagCbor).await
    }

    /// Updates or inserts a new node at the specified path.
    ///
    /// NOTE(appcypher): This is meant for internal use only as it mutates the directory in place for performance.
    /// Ideally, this method should be called with a newly forked directory.
    pub(super) async fn upsert<Fut>(
        &mut self,
        path_segment: &str,
        update_fn: impl FnOnce(Option<Link>) -> Fut,
    ) -> Result<Option<Shared<PublicNode>>>
    where
        Fut: Future<Output = Result<Option<Link>>>,
    {
        let mut working_node = None;
        let link = self.0.borrow().userland.get(path_segment).cloned();
        match update_fn(link).await? {
            // If the link is none, we remove the node from the userland.
            None => {
                self.0.borrow_mut().userland.remove(path_segment);
            }
            // If the link is some, we insert the node into the userland.
            Some(link) => {
                if let Link::Node(node) = &link {
                    working_node = Some(Rc::clone(node));
                }
                self.0
                    .borrow_mut()
                    .userland
                    .insert(path_segment.to_string(), link);
            }
        }

        Ok(working_node)
    }

    /// Reads specified file content from the directory.
    pub async fn read<B: BlockStore>(
        &self,
        path_segments: &[String],
        store: &mut B,
    ) -> Result<OpResult<Cid>> {
        match self.get_node(path_segments, store, false).await? {
            OpResult {
                root_node,
                result: Some(node_rc),
                ..
            } => match &*node_rc.borrow() {
                PublicNode::File(file) => Ok(OpResult {
                    root_node,
                    result: file.userland,
                    diverged: false,
                }),
                _ => error(FsError::NotAFile),
            },
            _ => error(FsError::NotFound),
        }
    }

    /// Writes a file to the directory.
    ///
    /// Rather than mutate the directory directly, we create a new directory and return it.
    pub async fn write<B: BlockStore>(
        &self,
        path_segments: &[String],
        content_cid: Cid,
        time: DateTime<Utc>,
        store: &B,
    ) -> Result<OpResult<Shared<PublicNode>>> {
        // If it does not already exist, create the file's parent directory.
        let (
            OpResult {
                root_node,
                result: parent_directory,
                ..
            },
            tail,
        ) = match path_segments.split_last() {
            None => bail!(FsError::InvalidPath),
            Some((tail, parent_path_segments)) => {
                (self.mkdir(parent_path_segments, time, store).await?, tail)
            }
        };

        // Insert or create file in parent directory.
        let working_node = parent_directory
            .borrow_mut()
            .as_mut_dir()
            .upsert(tail, move |link| async move {
                // If a link is provided, it is a cue to update it.
                if let Some(link) = link {
                    let node = link.resolve(store).await?;
                    return match &mut *node.borrow_mut() {
                        PublicNode::File(file) => {
                            file.metadata = Metadata::new(time, UnixFsNodeKind::File);
                            file.userland = content_cid;
                            Ok(Some(link))
                        }
                        _ => error(FsError::DirectoryAlreadyExists),
                    };
                }

                // If nothing is provided, it is a cue to return a new file node.
                let link = Link::with_file(PublicFile::new(time, content_cid));
                Ok(Some(link))
            })
            .await?
            .unwrap();

        Ok(OpResult {
            root_node,
            result: working_node,
            diverged: true,
        })
    }

    /// Creates a new directory at the specified path.
    ///
    /// If path is empty, this returns a cloned directory based on `self`.
    ///
    /// This method acts like `mkdir -p` in Unix because it creates intermediate directories if they do not exist.
    pub async fn mkdir<B: BlockStore>(
        &self,
        path_segments: &[String],
        time: DateTime<Utc>,
        store: &B,
    ) -> Result<OpResult<Shared<PublicNode>>> {
        // Clone the directory to prevent mutation of the original directory.
        let root_node = shared(PublicNode::Dir(self.clone()));
        let mut working_node = Rc::clone(&root_node);

        // The nodes along the path specified.
        let mut path_nodes: Vec<(String, Shared<PublicNode>)> = if !path_segments.is_empty() {
            vec![(String::new(), Rc::clone(&root_node))]
        } else {
            vec![]
        };

        // Set when directory exists.
        let mut dir_exists = false;

        // Iterate over path segments.
        for (index, segment) in path_segments.iter().enumerate() {
            let mut working_node_mut = working_node.borrow_mut();

            // Fetch node representing the path segment in the working directory.
            let next_node = match working_node_mut
                .as_mut_dir()
                .lookup_node(segment, store)
                .await?
            {
                Some(found_node) => match &*found_node.borrow() {
                    // If the node is a directory, set it as the next working node.
                    PublicNode::Dir(_) => {
                        if index == path_segments.len() - 1 {
                            dir_exists = true;
                        }

                        path_nodes.push((segment.to_string(), Rc::clone(&found_node)));
                        Rc::clone(&found_node)
                    }
                    // If the node is a file, we return an error.
                    PublicNode::File(_) => {
                        return if index == path_segments.len() - 1 {
                            error(FsError::FileAlreadyExists)
                        } else {
                            error(FsError::InvalidPath)
                        }
                    }
                },
                _ => {
                    // If the node is not found, we create it.
                    let new_node_rc = shared(PublicNode::Dir(PublicDirectory::new(time)));

                    // Insert the new node into the working directory.
                    working_node_mut
                        .as_mut_dir()
                        .0
                        .borrow_mut()
                        .userland
                        .insert(segment.to_string(), Link::Node(Rc::clone(&new_node_rc)));

                    path_nodes.push((segment.to_string(), Rc::clone(&new_node_rc)));
                    new_node_rc
                }
            };

            drop(working_node_mut);
            working_node = next_node;
        }

        // Get nodes which may have diverged.
        let (root_node, working_node) = if !dir_exists {
            let diverged_nodes = utils::diverge_and_patch(path_nodes);
            if !diverged_nodes.is_empty() {
                (
                    Rc::clone(&diverged_nodes.first().unwrap().1),
                    Rc::clone(&diverged_nodes.last().unwrap().1),
                )
            } else {
                (root_node, working_node)
            }
        } else {
            (root_node, working_node)
        };

        Ok(OpResult {
            root_node,
            result: working_node,
            diverged: true,
        })
    }

    /// Returns the name and metadata of the direct children of a directory.
    pub async fn ls<B: BlockStore>(
        &self,
        path_segments: &[String],
        store: &B,
    ) -> Result<OpResult<Vec<(String, Metadata)>>> {
        let OpResult {
            root_node,
            result: node,
            ..
        } = self.get_node(path_segments, store, false).await?;

        let node = node.ok_or(FsError::NotFound)?;
        let result = match &*node.borrow() {
            PublicNode::Dir(dir) => {
                // Save the directory's children info in a vector.
                let mut result = vec![];
                for (name, link) in dir.0.borrow().userland.iter() {
                    match &*link.resolve(store).await?.borrow() {
                        PublicNode::File(file) => {
                            result.push((name.clone(), file.metadata.clone()));
                        }
                        PublicNode::Dir(dir) => {
                            result.push((name.clone(), dir.0.borrow().metadata.clone()));
                        }
                    }
                }
                result
            }
            _ => bail!(FsError::NotADirectory),
        };

        Ok(OpResult {
            root_node,
            result,
            diverged: false,
        })
    }

    /// Removes a file or directory from the directory.
    ///
    /// Rather than mutate the directory directly, we create a new directory and return it.
    pub async fn rm<B: BlockStore>(
        &self,
        path_segments: &[String],
        store: &B,
    ) -> Result<OpResult<Shared<PublicNode>>> {
        // Get node's parent directory.
        let (
            OpResult {
                result: parent_node,
                root_node,
                ..
            },
            tail,
        ) = match path_segments.split_last() {
            None => bail!(FsError::InvalidPath),
            Some((tail, parent_path_segments)) => (
                self.get_node(parent_path_segments, store, true).await?,
                tail,
            ),
        };

        let parent_node = parent_node.ok_or(FsError::NotFound)?;
        match &mut *parent_node.borrow_mut() {
            PublicNode::Dir(dir) => {
                // Remove the file from the parent directory if present.
                dir.upsert(tail, |link| async move {
                    match link {
                        Some(_) => Ok(None),
                        _ => error(FsError::NotFound),
                    }
                })
                .await?;
            }
            _ => bail!(FsError::NotADirectory),
        };

        Ok(OpResult {
            root_node,
            result: parent_node,
            diverged: true,
        })
    }

    /// Encode the directory as a CBOR object.
    pub async fn encode<B: BlockStore>(&self, store: &mut B) -> Result<Vec<u8>> {
        let mut bytes = Vec::new();

        // Write the major of the section being written.
        encode::write_u64(
            &mut bytes,
            MajorKind::Map,
            PublicDirectoryInner::FIELDS.len() as u64,
        )?;

        // Ordering the fields by name based on RFC-7049 which is also what libipld uses.
        let mut cbor_order: Vec<&'static str> = Vec::from_iter(PublicDirectoryInner::FIELDS);
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
                    self.0.borrow().metadata.encode(DagCborCodec, &mut bytes)?;
                }
                "userland" => {
                    let new_userland = {
                        let mut tmp = BTreeMap::new();
                        for (k, link) in self.0.borrow().userland.iter() {
                            let cid = link.seal(store).await?;
                            tmp.insert(k.clone(), cid);
                        }
                        tmp
                    };

                    new_userland.encode(DagCborCodec, &mut bytes)?;
                }
                "previous" => {
                    self.0.borrow().previous.encode(DagCborCodec, &mut bytes)?;
                }
                _ => unreachable!(),
            }
        }

        Ok(bytes)
    }
}

impl Id for PublicDirectory {
    fn get_id(&self) -> String {
        format!("{:p}", &self.0.borrow().metadata)
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
        let mut cbor_order: Vec<&'static str> = Vec::from_iter(PublicDirectoryInner::FIELDS);
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

        Ok(Self(shared(PublicDirectoryInner {
            metadata: metadata
                .ok_or_else(|| FsError::UndecodableCborData("Missing unix_fs".into()))?,
            userland,
            previous,
        })))
    }
}

//--------------------------------------------------------------------------------------------------
// Utilities
//--------------------------------------------------------------------------------------------------

mod utils {
    use std::rc::Rc;

    use crate::{
        public::{Link, PublicNode},
        shared, Shared,
    };

    /// Creates a new nodes from path nodes with their child references patched.
    pub fn diverge_and_patch(
        path_nodes: Vec<(String, Shared<PublicNode>)>,
    ) -> Vec<(String, Shared<PublicNode>)> {
        if path_nodes.is_empty() {
            return Vec::new();
        }

        // Create divergent nodes.
        let divergent_nodes = path_nodes
            .into_iter()
            .map(|(name, node)| (name, shared(node.borrow().clone())))
            .collect::<Vec<_>>();

        // Fix up the divergent nodes so that they are referencing the right children.
        for (index, (_, parent_node)) in divergent_nodes.iter().enumerate() {
            // If there is a next node which happens to be the child node, we fix up the reference in the parent node.
            if index < divergent_nodes.len() - 1 {
                let (child_name, child_node) = &divergent_nodes[index + 1];
                parent_node
                    .borrow_mut()
                    .as_mut_dir()
                    .0
                    .borrow_mut()
                    .userland
                    .insert(child_name.clone(), Link::Node(Rc::clone(child_node)));
            }
        }

        divergent_nodes
    }
}

//--------------------------------------------------------------------------------------------------
// Tests
//--------------------------------------------------------------------------------------------------

#[cfg(test)]
mod public_directory_tests {
    use std::io::Cursor;

    use super::*;
    use crate::{public::PublicFile, BlockStoreLookup, MemoryBlockStore};
    use chrono::Utc;

    #[async_std::test]
    async fn look_up_can_fetch_file_added_to_directory() {
        let root_dir = PublicDirectory::new(Utc::now());

        let store = MemoryBlockStore::default();

        let content_cid = Cid::default();

        let time = Utc::now();

        let OpResult { root_node, .. } = root_dir
            .write(&["text.txt".into()], content_cid, time, &store)
            .await
            .unwrap();

        let node = root_node
            .borrow()
            .as_dir()
            .lookup_node("text.txt", &store)
            .await
            .unwrap();

        assert!(node.is_some());

        assert_eq!(
            node.unwrap(),
            shared(PublicNode::File(PublicFile::new(time, content_cid)))
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
    async fn mkdir_can_create_new_directory_with_diverged_root() {
        let time = Utc::now();

        let store = MemoryBlockStore::default();

        let root_dir = PublicDirectory::new(time);

        let OpResult { root_node, .. } = root_dir
            .mkdir(&["tamedun".into(), "pictures".into()], time, &store)
            .await
            .unwrap();

        let OpResult { root_node, .. } = root_node
            .borrow_mut()
            .as_mut_dir()
            .write(
                &["tamedun".into(), "pictures".into(), "puppy.jpg".into()],
                Cid::default(),
                time,
                &store,
            )
            .await
            .unwrap();

        let OpResult { result, .. } = root_node
            .borrow()
            .as_dir()
            .get_node(
                &["tamedun".into(), "pictures".into(), "puppy.jpg".into()],
                &store,
                false,
            )
            .await
            .unwrap();

        assert!(result.is_some());
    }

    #[async_std::test]
    async fn ls_can_list_children_under_directory() {
        let time = Utc::now();

        let store = MemoryBlockStore::default();

        let root_dir = PublicDirectory::new(time);

        let OpResult { root_node, .. } = root_dir
            .mkdir(&["tamedun".into(), "pictures".into()], time, &store)
            .await
            .unwrap();

        let OpResult { root_node, .. } = root_node
            .borrow_mut()
            .as_mut_dir()
            .write(
                &["tamedun".into(), "pictures".into(), "puppy.jpg".into()],
                Cid::default(),
                time,
                &store,
            )
            .await
            .unwrap();

        let OpResult { root_node, .. } = root_node
            .borrow_mut()
            .as_mut_dir()
            .mkdir(
                &["tamedun".into(), "pictures".into(), "cats".into()],
                time,
                &store,
            )
            .await
            .unwrap();

        let OpResult { result, .. } = root_node
            .borrow()
            .as_dir()
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

        let root_dir = PublicDirectory::new(time);

        let OpResult { root_node, .. } = root_dir
            .mkdir(&["tamedun".into(), "pictures".into()], time, &store)
            .await
            .unwrap();

        let OpResult { root_node, .. } = root_node
            .borrow_mut()
            .as_mut_dir()
            .write(
                &["tamedun".into(), "pictures".into(), "puppy.jpg".into()],
                Cid::default(),
                time,
                &store,
            )
            .await
            .unwrap();

        let OpResult { root_node, .. } = root_node
            .borrow_mut()
            .as_mut_dir()
            .mkdir(
                &["tamedun".into(), "pictures".into(), "cats".into()],
                time,
                &store,
            )
            .await
            .unwrap();

        let result = root_node
            .borrow()
            .as_dir()
            .rm(&["tamedun".into(), "pictures".into()], &store)
            .await;

        assert!(result.is_ok());

        let result = result
            .unwrap()
            .root_node
            .borrow()
            .as_dir()
            .rm(&["tamedun".into(), "pictures".into()], &store)
            .await;

        assert!(result.is_err());
    }

    #[async_std::test]
    async fn read_can_fetch_userland_of_file_added_to_directory() {
        let root_dir = PublicDirectory::new(Utc::now());

        let mut store = MemoryBlockStore::default();

        let content_cid = Cid::default();

        let time = Utc::now();

        let OpResult { root_node, .. } = root_dir
            .write(&["text.txt".into()], content_cid, time, &store)
            .await
            .unwrap();

        let OpResult { result, .. } = root_node
            .borrow()
            .as_dir()
            .read(&["text.txt".into()], &mut store)
            .await
            .unwrap();

        assert_eq!(result, content_cid);
    }
}
