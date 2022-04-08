//! Public file system in-memory representation.

use crate::common::{error, BlockStore, FsError, Metadata, UnixFsNodeKind};
use anyhow::Result;
use chrono::{DateTime, Utc};
use hashbrown::HashMap;
use libipld::{cbor::DagCborCodec, codec::Decode, Cid};
use std::{ops::Deref, rc::Rc};

//--------------------------------------------------------------------------------------------------
// Type Definitions
//--------------------------------------------------------------------------------------------------

/// A node in a WNFS public file system. This can either be a file or a directory.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PublicNode {
    File(PublicFile),
    Dir(PublicDirectory),
}

/// A link to another node in the WNFS public file system. It can be held as a simple serialised CID or as a reference to the node itself.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Link {
    Cid(Cid),
    Node(Rc<PublicNode>),
}

/// A directory in a WNFS public file system.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublicDirectory {
    // metadata: Metadata,
    userland: HashMap<String, Link>,
    // previous: Option<Cid>,
}

/// A file in a WNFS public file system.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublicFile {
    metadata: Metadata,
    userland: Cid,
    previous: Option<Cid>,
}

//--------------------------------------------------------------------------------------------------
// Implementations
//--------------------------------------------------------------------------------------------------

// TODO(appcypher)
impl Decode<DagCborCodec> for PublicNode {
    fn decode<R: std::io::Read + std::io::Seek>(c: DagCborCodec, r: &mut R) -> Result<Self> {
        todo!()
    }
}

impl PublicNode {
    /// Stores a WNFS node as block(s) in chosen block store.
    pub async fn store<B: BlockStore>(&self, store: &mut B) -> Cid {
        match self {
            PublicNode::File(file) => file.store(store).await,
            PublicNode::Dir(dir) => dir.store(store).await,
        }
    }

    /// Casts a node to a directory.
    ///
    /// # Panics
    ///
    /// Panics if the node is not a directory.
    pub fn as_dir(&self) -> &PublicDirectory {
        match self {
            PublicNode::Dir(dir) => dir,
            _ => unreachable!(),
        }
    }
}

impl PublicDirectory {
    /// Creates a new directory using the given metadata.
    pub fn new(time: DateTime<Utc>) -> Self {
        Self {
            // metadata: Metadata::new(time, UnixFsNodeKind::Dir),
            userland: HashMap::new(),
            // previous: None,
        }
    }

    /// Follows a path and fetches the node at the end of the path.
    pub async fn get_node<B: BlockStore>(
        &self,
        path_segments: &[String],
        store: &B,
    ) -> Result<Rc<PublicNode>> {
        if path_segments.is_empty() {
            return error(FsError::InvalidPath);
        }

        let mut working_node: Rc<PublicNode> = Rc::new(PublicNode::Dir(self.clone()));

        // Iterate over the path segments until we get the node of the last segment.
        for (index, segment) in path_segments.iter().enumerate() {
            // Cast working node to directory.
            let dir = working_node.deref().as_dir();

            // Fetch node representing path segment in working directory.
            if let Some(node) = dir.lookup_node(segment, store).await? {
                match node.as_ref() {
                    PublicNode::Dir(_) => {
                        // If the node is a directory, set it as the working node.
                        working_node = Rc::clone(&node);
                    }
                    PublicNode::File(_) => {
                        // If the node is a file, we return it if it's the last segment.
                        if index != path_segments.len() - 1 {
                            return error(FsError::InvalidPath);
                        }
                        working_node = Rc::clone(&node);
                        break;
                    }
                }

                // We continue loop after setting the working node to a directory node.
                continue;
            }

            // If the node is not found, we return an error.
            return error(FsError::NodeNotFound);
        }

        Ok(working_node)
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

    /// Stores WNFS directory as block(s) in chosen block store.
    pub async fn store<B: BlockStore>(&self, store: &mut B) -> Cid {
        todo!()
    }
}

impl PublicFile {
    /// Stores WNFS block(s) in chosen block store.
    pub async fn store<B: BlockStore>(&self, store: &mut B) -> Cid {
        todo!()
    }
}

impl Link {
    // Resolves a CID link in the file system to a node.
    pub async fn resolve<B: BlockStore>(&self, store: &B) -> Result<Rc<PublicNode>> {
        Ok(match self {
            Link::Cid(cid) => {
                let node = store.load(cid).await?;
                Rc::new(node)
            }
            Link::Node(node) => Rc::clone(node),
        })
    }

    // Adds the link to the file system.
    pub async fn seal<B: BlockStore>(&self, store: &mut B) {
        todo!()
    }
}

#[cfg(test)]
mod public_node_tests {
    use super::PublicDirectory;
    use crate::MemoryBlockStore;
    use chrono::Utc;

    #[async_std::test]
    async fn unadded_node_lookup_unsuccessful() {
        let root = PublicDirectory::new(Utc::now());
        let store = MemoryBlockStore::default();
        let node = root.lookup_node("Unknown", &store).await;
        dbg!(&node);
        assert!(node.is_ok());
        assert_eq!(node.unwrap(), None);
    }
}
