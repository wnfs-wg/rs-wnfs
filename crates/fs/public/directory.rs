//! Public fs directory node.

use std::{
    cmp::Ordering,
    collections::BTreeMap,
    future::Future,
    io::{Read, Seek},
    rc::Rc,
};

use crate::{blockstore, error, shared, BlockStore, FsError, Metadata, Shared, UnixFsNodeKind};
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

fn focus(
    path_nodes: PathTo<Rc<PublicDirectory>>,
    segment: String,
) -> PathTo<(String, Option<Rc<PublicNode>>)> {
    todo!()
}

fn map<A, B, F: FnOnce(A) -> B>(path_nodes: PathTo<A>, f: F) -> PathTo<B> {
    todo!()
}

fn reconstruct_file(path_nodes: PathTo<(String, Option<Rc<PublicNode>>)>) -> Rc<PublicDirectory> {
    todo!()
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

fn collapse<T>(path_to: PathTo<PathTo<T>>) -> PathTo<T> {
    let mut path = path_to.path.clone();
    path.extend(path_to.focus.path);
    PathTo {
        path,
        focus: path_to.focus.focus,
    }
}

fn new_path_to<T>(time: DateTime<Utc>, path: &[String], to: T) -> PathTo<T> {
    let path: Vec<(Rc<PublicDirectory>, String)> = path
        .iter()
        .map(|segment| (Rc::new(PublicDirectory::new(time)), segment.to_string()))
        .collect();
    PathTo { path, focus: to }
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

        for (index, segment) in path_segments.iter().enumerate() {
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

    // /// Follows a path and fetches the node at the end of the path.
    // ///
    // /// If path is empty, this returns a cloned directory based on `self`.
    // ///
    // /// If `diverge` is true, the path diverges. This is only used internally.
    // pub async fn get_node<B: BlockStore>(
    //     &self,
    //     path_segments: &[String],
    //     store: &B,
    //     diverge: bool,
    // ) -> Result<OpResult<Option<Shared<PublicNode>>>> {
    //     // Set working node to current directory.
    //     let root_node = shared(PublicNode::Dir(self.clone()));
    //     let mut working_node = Some(Rc::clone(&root_node));

    //     // The nodes along the path specified.
    //     let mut path_nodes: Vec<(String, Shared<PublicNode>)> = if !path_segments.is_empty() {
    //         vec![(String::new(), Rc::clone(&root_node))]
    //     } else {
    //         vec![]
    //     };

    //     // Iterate over the path segments.
    //     for (index, segment) in path_segments.iter().enumerate() {
    //         let mut working_node_mut = working_node.as_mut().unwrap().borrow_mut();

    //         // Fetch node representing the path segment in the working directory.
    //         match working_node_mut
    //             .as_mut_dir()
    //             .lookup_node(segment, store)
    //             .await?
    //         {
    //             Some(found_node) => match &*found_node.borrow() {
    //                 // If the node is a directory, set it as the working node.
    //                 PublicNode::Dir(_) => {
    //                     path_nodes.push((segment.to_string(), Rc::clone(&found_node)));

    //                     drop(working_node_mut);
    //                     working_node = Some(Rc::clone(&found_node));
    //                 }
    //                 // If the node is a file, we return it if it's the last segment.
    //                 PublicNode::File(_) => {
    //                     if index != path_segments.len() - 1 {
    //                         bail!(FsError::InvalidPath);
    //                     }

    //                     drop(working_node_mut);
    //                     working_node = Some(Rc::clone(&found_node));
    //                     break;
    //                 }
    //             },
    //             _ => {
    //                 // If the node is not found, we return an none.
    //                 drop(working_node_mut);
    //                 working_node = None;
    //                 break;
    //             }
    //         }
    //     }

    //     // Get nodes which may have diverged.
    //     let (root_node, working_node) = if diverge {
    //         let diverged_nodes = utils::diverge_and_patch(path_nodes);
    //         if !diverged_nodes.is_empty() {
    //             (
    //                 Rc::clone(&diverged_nodes.first().unwrap().1),
    //                 Some(Rc::clone(&diverged_nodes.last().unwrap().1)),
    //             )
    //         } else {
    //             (root_node, working_node)
    //         }
    //     } else {
    //         (root_node, working_node)
    //     };

    //     Ok(OpResult {
    //         root_node,
    //         result: working_node,
    //         diverged: diverge,
    //     })
    // }

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

    // /// Updates or inserts a new node at the specified path.
    // ///
    // /// NOTE(appcypher): This is meant for internal use only as it mutates the directory in place for performance.
    // /// Ideally, this method should be called with a newly forked directory.
    // pub(super) async fn upsert<Fut>(
    //     &mut self,
    //     path_segment: &str,
    //     update_fn: impl FnOnce(Option<Link>) -> Fut,
    // ) -> Result<Option<Shared<PublicNode>>>
    // where
    //     Fut: Future<Output = Result<Option<Link>>>,
    // {
    //     let mut working_node = None;
    //     let link = self.0.borrow().userland.get(path_segment).cloned();
    //     match update_fn(link).await? {
    //         // If the link is none, we remove the node from the userland.
    //         None => {
    //             self.0.borrow_mut().userland.remove(path_segment);
    //         }
    //         // If the link is some, we insert the node into the userland.
    //         Some(link) => {
    //             if let Link::Node(node) = &link {
    //                 working_node = Some(Rc::clone(node));
    //             }
    //             self.0
    //                 .borrow_mut()
    //                 .userland
    //                 .insert(path_segment.to_string(), link);
    //         }
    //     }

    //     Ok(working_node)
    // }

    // /// Reads specified file content from the directory.
    // pub async fn read<B: BlockStore>(
    //     &self,
    //     path_segments: &[String],
    //     store: &mut B,
    // ) -> Result<OpResult<Cid>> {
    //     match self.get_node(path_segments, store, false).await? {
    //         OpResult {
    //             root_node,
    //             result: Some(node_rc),
    //             ..
    //         } => match &*node_rc.borrow() {
    //             PublicNode::File(file) => Ok(OpResult {
    //                 root_node,
    //                 result: file.userland,
    //                 diverged: false,
    //             }),
    //             _ => error(FsError::NotAFile),
    //         },
    //         _ => error(FsError::NotFound),
    //     }
    // }

    // /// Writes a file to the directory.
    // ///
    // /// Rather than mutate the directory directly, we create a new directory and return it.
    // pub async fn write<B: BlockStore>(
    //     &self,
    //     path_segments: &[String],
    //     content_cid: Cid,
    //     time: DateTime<Utc>,
    //     store: &B,
    // ) -> Result<OpResult<Shared<PublicNode>>> {
    //     // If it does not already exist, create the file's parent directory.
    //     let (
    //         OpResult {
    //             root_node,
    //             result: parent_directory,
    //             ..
    //         },
    //         tail,
    //     ) = match path_segments.split_last() {
    //         None => bail!(FsError::InvalidPath),
    //         Some((tail, parent_path_segments)) => (
    //             self.mkdir(parent_path_segments, time, store, true).await?,
    //             tail,
    //         ),
    //     };

    //     // Insert or update file in parent directory.
    //     let working_node = parent_directory
    //         .borrow_mut()
    //         .as_mut_dir()
    //         .upsert(tail, move |link| async move {
    //             // If a link is provided, it is a cue to update it.
    //             if let Some(link) = link {
    //                 let node = link.resolve(store).await?;
    //                 return match &mut *node.borrow_mut() {
    //                     PublicNode::File(file) => {
    //                         file.metadata = Metadata::new(time, UnixFsNodeKind::File);
    //                         file.userland = content_cid;
    //                         Ok(Some(link))
    //                     }
    //                     _ => error(FsError::DirectoryAlreadyExists),
    //                 };
    //             }

    //             // If nothing is provided, it is a cue to return a new file node.
    //             let link = Link::with_file(PublicFile::new(time, content_cid));
    //             Ok(Some(link))
    //         })
    //         .await?
    //         .unwrap();

    //     Ok(OpResult {
    //         root_node,
    //         result: working_node,
    //         diverged: true,
    //     })
    // }

    // /// Creates a new directory at the specified path.
    // ///
    // /// If path is empty, this returns the root node.
    // ///
    // /// This method acts like `mkdir -p` in Unix because it creates intermediate directories if they do not exist.
    // ///
    // /// If `diverge_anyway` is set to true, the path diverges even if directory exists. This is only used internally.
    // pub async fn mkdir<B: BlockStore>(
    //     &self,
    //     path_segments: &[String],
    //     time: DateTime<Utc>,
    //     store: &B,
    //     diverge_anyway: bool,
    // ) -> Result<OpResult<Shared<PublicNode>>> {
    //     // Clone the directory to prevent mutation of the original directory.
    //     // TODO(appcypher): What does self.clone do here?
    //     let root_node = shared(PublicNode::Dir(self.clone()));
    //     let mut working_node = Rc::clone(&root_node);

    //     // The nodes along the path specified.
    //     let mut path_nodes: Vec<(String, Shared<PublicNode>)> =
    //         vec![(String::new(), Rc::clone(&root_node))];

    //     // Represents when the directory we are trying to create already exists.
    //     // It is set true because we know the root node exists. Just in case the path_segments is empty and we return early.
    //     let mut dir_exists = true;

    //     // Iterate over path segments.
    //     for (index, segment) in path_segments.iter().enumerate() {
    //         let mut working_node_mut = working_node.borrow_mut();

    //         // Fetch node representing the path segment in the working directory.
    //         let next_node = match working_node_mut
    //             .as_mut_dir()
    //             .lookup_node(segment, store)
    //             .await?
    //         {
    //             Some(found_node) => match &*found_node.borrow() {
    //                 // If the node is a directory, set it as the next working node.
    //                 PublicNode::Dir(_) => {
    //                     path_nodes.push((segment.to_string(), Rc::clone(&found_node)));
    //                     Rc::clone(&found_node)
    //                 }
    //                 // If the node is a file, we return an error.
    //                 PublicNode::File(_) => {
    //                     return if index == path_segments.len() - 1 {
    //                         error(FsError::FileAlreadyExists)
    //                     } else {
    //                         error(FsError::InvalidPath)
    //                     }
    //                 }
    //             },
    //             _ => {
    //                 // At the final segment, we know for sure the directory does not exist if we have to create it.
    //                 if index == path_segments.len() - 1 {
    //                     dir_exists = false;
    //                 }

    //                 // Node is not found so we create it.
    //                 let new_node_rc = shared(PublicNode::Dir(PublicDirectory::new(time)));

    //                 // Insert the new node into the working directory.
    //                 working_node_mut
    //                     .as_mut_dir()
    //                     .0
    //                     .borrow_mut()
    //                     .userland
    //                     .insert(segment.to_string(), Link::Node(Rc::clone(&new_node_rc)));

    //                 path_nodes.push((segment.to_string(), Rc::clone(&new_node_rc)));
    //                 new_node_rc
    //             }
    //         };

    //         drop(working_node_mut);
    //         working_node = next_node;
    //     }

    //     // If directory does not already exist or `diverge_anyway` is set, we create a divergent path.
    //     let (root_node, working_node) = if !dir_exists || diverge_anyway {
    //         let diverged_nodes = utils::diverge_and_patch(path_nodes);
    //         if !diverged_nodes.is_empty() {
    //             (
    //                 Rc::clone(&diverged_nodes.first().unwrap().1),
    //                 Rc::clone(&diverged_nodes.last().unwrap().1),
    //             )
    //         } else {
    //             (root_node, working_node)
    //         }
    //     } else {
    //         (root_node, working_node)
    //     };

    //     Ok(OpResult {
    //         root_node,
    //         result: working_node,
    //         diverged: true, // Wrong!
    //     })
    // }

    // /// Returns the name and metadata of the direct children of a directory.
    // pub async fn ls<B: BlockStore>(
    //     &self,
    //     path_segments: &[String],
    //     store: &B,
    // ) -> Result<OpResult<Vec<(String, Metadata)>>> {
    //     let OpResult {
    //         root_node,
    //         result: node,
    //         ..
    //     } = self.get_node(path_segments, store, false).await?;

    //     let node = node.ok_or(FsError::NotFound)?;
    //     let result = match &*node.borrow() {
    //         PublicNode::Dir(dir) => {
    //             // Save the directory's children info in a vector.
    //             let mut result = vec![];
    //             for (name, link) in dir.0.borrow().userland.iter() {
    //                 match &*link.resolve(store).await?.borrow() {
    //                     PublicNode::File(file) => {
    //                         result.push((name.clone(), file.metadata.clone()));
    //                     }
    //                     PublicNode::Dir(dir) => {
    //                         result.push((name.clone(), dir.0.borrow().metadata.clone()));
    //                     }
    //                 }
    //             }
    //             result
    //         }
    //         _ => bail!(FsError::NotADirectory),
    //     };

    //     Ok(OpResult {
    //         root_node,
    //         result,
    //         diverged: false,
    //     })
    // }

    // /// Removes a file or directory from the directory.
    // ///
    // /// Rather than mutate the directory directly, we create a new directory and return it.
    // pub async fn rm<B: BlockStore>(
    //     &self,
    //     path_segments: &[String],
    //     store: &B,
    // ) -> Result<OpResult<Shared<PublicNode>>> {
    //     // Get node's parent directory.
    //     let (
    //         OpResult {
    //             result: parent_node,
    //             root_node,
    //             ..
    //         },
    //         tail,
    //     ) = match path_segments.split_last() {
    //         None => bail!(FsError::InvalidPath),
    //         Some((tail, parent_path_segments)) => (
    //             self.get_node(parent_path_segments, store, true).await?,
    //             tail,
    //         ),
    //     };

    //     let parent_node = parent_node.ok_or(FsError::NotFound)?;
    //     match &mut *parent_node.borrow_mut() {
    //         PublicNode::Dir(dir) => {
    //             // Remove the file from the parent directory if present.
    //             dir.upsert(tail, |link| async move {
    //                 match link {
    //                     Some(_) => Ok(None),
    //                     _ => error(FsError::NotFound),
    //                 }
    //             })
    //             .await?;
    //         }
    //         _ => bail!(FsError::NotADirectory),
    //     };

    //     Ok(OpResult {
    //         root_node,
    //         result: parent_node,
    //         diverged: true,
    //     })
    // }

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
// Utilities
//--------------------------------------------------------------------------------------------------

// mod utils {
//     use std::rc::Rc;

//     use crate::{
//         public::{Link, PublicNode},
//         shared, Shared,
//     };

//     /// Creates a new nodes from path nodes with their child references patched.
//     pub fn diverge_and_patch(
//         path_nodes: Vec<(String, Shared<PublicNode>)>,
//     ) -> Vec<(String, Shared<PublicNode>)> {
//         if path_nodes.is_empty() {
//             return Vec::new();
//         }

//         // Create divergent nodes.
//         let divergent_nodes = path_nodes
//             .into_iter()
//             .map(|(name, node)| {
//                 let deep_clone = shared(match &*node.borrow() {
//                     PublicNode::Dir(dir) => PublicNode::Dir(dir.deep_clone()),
//                     PublicNode::File(file) => PublicNode::File(file.clone()),
//                 });
//                 (name, deep_clone)
//             })
//             .collect::<Vec<_>>();

//         // Fix up the divergent nodes so that they are referencing the right children.
//         for (index, (_, parent_node)) in divergent_nodes.iter().enumerate() {
//             // If there is a next node which happens to be the child node, we fix up the reference in the parent node.
//             if index < divergent_nodes.len() - 1 {
//                 let (child_name, child_node) = &divergent_nodes[index + 1];
//                 parent_node
//                     .borrow_mut()
//                     .as_mut_dir()
//                     .0
//                     .borrow_mut()
//                     .userland
//                     .insert(child_name.clone(), Link::Node(Rc::clone(child_node)));
//             }
//         }

//         divergent_nodes
//     }
// }

//--------------------------------------------------------------------------------------------------
// Tests
//--------------------------------------------------------------------------------------------------

#[cfg(test)]
mod public_directory_tests {
    use std::io::Cursor;

    use super::*;
    use crate::{public::PublicFile, MemoryBlockStore};
    use chrono::Utc;

    // #[async_std::test]
    // async fn look_up_can_fetch_file_added_to_directory() {
    //     let root_dir = PublicDirectory::new(Utc::now());

    //     let store = MemoryBlockStore::default();

    //     let content_cid = Cid::default();

    //     let time = Utc::now();

    //     let OpResult { root_node, .. } = root_dir
    //         .write(&["text.txt".into()], content_cid, time, &store)
    //         .await
    //         .unwrap();

    //     let node = root_node
    //         .borrow()
    //         .as_dir()
    //         .lookup_node("text.txt", &store)
    //         .await
    //         .unwrap();

    //     assert!(node.is_some());

    //     assert_eq!(
    //         node.unwrap(),
    //         shared(PublicNode::File(PublicFile::new(time, content_cid)))
    //     );
    // }

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

    // #[async_std::test]
    // async fn mkdir_can_create_new_directory_with_diverged_root() {
    //     let time = Utc::now();

    //     let store = MemoryBlockStore::default();

    //     let root_dir = PublicDirectory::new(time);

    //     let OpResult { root_node, .. } = root_dir
    //         .mkdir(&["tamedun".into(), "pictures".into()], time, &store, false)
    //         .await
    //         .unwrap();

    //     let OpResult { root_node, .. } = root_node
    //         .borrow_mut()
    //         .as_mut_dir()
    //         .write(
    //             &["tamedun".into(), "pictures".into(), "puppy.jpg".into()],
    //             Cid::default(),
    //             time,
    //             &store,
    //         )
    //         .await
    //         .unwrap();

    //     let OpResult { result, .. } = root_node
    //         .borrow()
    //         .as_dir()
    //         .get_node(
    //             &["tamedun".into(), "pictures".into(), "puppy.jpg".into()],
    //             &store,
    //             false,
    //         )
    //         .await
    //         .unwrap();

    //     assert!(result.is_some());
    // }

    // #[async_std::test]
    // async fn ls_can_list_children_under_directory() {
    //     let time = Utc::now();

    //     let store = MemoryBlockStore::default();

    //     let root_dir = PublicDirectory::new(time);

    //     let OpResult { root_node, .. } = root_dir
    //         .mkdir(&["tamedun".into(), "pictures".into()], time, &store, false)
    //         .await
    //         .unwrap();

    //     let OpResult { root_node, .. } = root_node
    //         .borrow_mut()
    //         .as_mut_dir()
    //         .write(
    //             &["tamedun".into(), "pictures".into(), "puppy.jpg".into()],
    //             Cid::default(),
    //             time,
    //             &store,
    //         )
    //         .await
    //         .unwrap();

    //     let OpResult { root_node, .. } = root_node
    //         .borrow_mut()
    //         .as_mut_dir()
    //         .mkdir(
    //             &["tamedun".into(), "pictures".into(), "cats".into()],
    //             time,
    //             &store,
    //             false,
    //         )
    //         .await
    //         .unwrap();

    //     let OpResult { result, .. } = root_node
    //         .borrow()
    //         .as_dir()
    //         .ls(&["tamedun".into(), "pictures".into()], &store)
    //         .await
    //         .unwrap();

    //     assert_eq!(result.len(), 2);

    //     assert_eq!(result[0].0, String::from("cats"));

    //     assert_eq!(result[1].0, String::from("puppy.jpg"));

    //     assert_eq!(result[0].1.unix_fs.kind, UnixFsNodeKind::Dir);

    //     assert_eq!(result[1].1.unix_fs.kind, UnixFsNodeKind::File);
    // }

    // #[async_std::test]
    // async fn rm_can_remove_children_from_directory() {
    //     let time = Utc::now();

    //     let store = MemoryBlockStore::default();

    //     let root_dir = PublicDirectory::new(time);

    //     let OpResult { root_node, .. } = root_dir
    //         .mkdir(&["tamedun".into(), "pictures".into()], time, &store, false)
    //         .await
    //         .unwrap();

    //     let OpResult { root_node, .. } = root_node
    //         .borrow_mut()
    //         .as_mut_dir()
    //         .write(
    //             &["tamedun".into(), "pictures".into(), "puppy.jpg".into()],
    //             Cid::default(),
    //             time,
    //             &store,
    //         )
    //         .await
    //         .unwrap();

    //     let OpResult { root_node, .. } = root_node
    //         .borrow_mut()
    //         .as_mut_dir()
    //         .mkdir(
    //             &["tamedun".into(), "pictures".into(), "cats".into()],
    //             time,
    //             &store,
    //             false,
    //         )
    //         .await
    //         .unwrap();

    //     let result = root_node
    //         .borrow()
    //         .as_dir()
    //         .rm(&["tamedun".into(), "pictures".into()], &store)
    //         .await;

    //     assert!(result.is_ok());

    //     let result = result
    //         .unwrap()
    //         .root_node
    //         .borrow()
    //         .as_dir()
    //         .rm(&["tamedun".into(), "pictures".into()], &store)
    //         .await;

    //     assert!(result.is_err());
    // }

    // #[async_std::test]
    // async fn read_can_fetch_userland_of_file_added_to_directory() {
    //     let root_dir = PublicDirectory::new(Utc::now());

    //     let mut store = MemoryBlockStore::default();

    //     let content_cid = Cid::default();

    //     let time = Utc::now();

    //     let OpResult { root_node, .. } = root_dir
    //         .write(&["text.txt".into()], content_cid, time, &store)
    //         .await
    //         .unwrap();

    //     let OpResult { result, .. } = root_node
    //         .borrow()
    //         .as_dir()
    //         .read(&["text.txt".into()], &mut store)
    //         .await
    //         .unwrap();

    //     assert_eq!(result, content_cid);
    // }

    #[async_std::test]
    async fn example_path_to() {
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
            GetNodePathResult::MissingLink(_, segment) => panic!("MissingLink {}", segment),
            GetNodePathResult::NotADirectory(_, segment) => panic!("NotADirectory {}", segment),
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
