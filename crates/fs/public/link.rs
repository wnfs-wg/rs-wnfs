//! Node link.

use std::rc::Rc;

use anyhow::Result;
use libipld::Cid;

use super::{PublicDirectory, PublicFile, PublicNode};
use crate::{wrap_link_methods, BlockStore, Link};

//--------------------------------------------------------------------------------------------------
// Type Definitions
//--------------------------------------------------------------------------------------------------

/// A link to another node in the WNFS public file system. It can be held as a simple serialised CID or as a reference to the node itself.
///
/// The public file system is a DAG so we don't have to worry bout cyclic references.
#[derive(Debug, Clone, PartialEq)]
pub struct PublicLink(pub(crate) Link<PublicNode>);

//--------------------------------------------------------------------------------------------------
// Implementations
//--------------------------------------------------------------------------------------------------

wrap_link_methods!(PublicLink<PublicNode>);

impl PublicLink {
    /// Create a new link to a node.
    pub fn new(node: PublicNode) -> Self {
        Self(Link::from(node))
    }

    /// Creates a new directory node link.
    pub fn with_dir(dir: Rc<PublicDirectory>) -> Self {
        PublicLink(Link::from(PublicNode::Dir(dir)))
    }

    /// Creates a new file node link.
    pub fn with_file(file: Rc<PublicFile>) -> Self {
        PublicLink(Link::from(PublicNode::File(file)))
    }

    /// Checks if two links are equal. It goes as far as resolving values to Cids to check if they are the same.
    pub async fn deep_eq<B: BlockStore>(&self, other: &PublicLink, store: &mut B) -> Result<bool> {
        let (link, other_link) = (&self.0, &other.0);
        Ok(match (link, other_link) {
            (
                Link::Decoded { value, .. },
                Link::Decoded {
                    value: other_value, ..
                },
            ) => {
                // We use this as as shortcut if the two nodes have the same memory address.
                value.ptr_eq(&other_value)
            }
            (Link::Encoded { cid, .. }, Link::Decoded { .. }) => {
                cid == other_link.resolve_cid(store).await?
            }
            (Link::Decoded { .. }, Link::Encoded { cid: other_cid, .. }) => {
                other_cid == self.resolve_cid(store).await?
            }
            _ => link == other_link,
        })
    }
}

// #[cfg(test)]
// mod public_link_tests {
//     use std::rc::Rc;

//     use chrono::Utc;
//     use libipld::Cid;

//     use crate::{
//         public::{PublicDirectory, PublicFile, PublicNode},
//         MemoryBlockStore,
//     };

//     use super::PublicLink;

//     #[async_std::test]
//     async fn node_link_can_be_sealed() {
//         let time = Utc::now();

//         let userland = Cid::default();

//         let file = Rc::new(PublicFile::new(time, userland));

//         let mut store = MemoryBlockStore::default();

//         let file_cid = file.store(&mut store).await.unwrap();

//         let unsealed_link = PublicLink::with_file(file);

//         let sealed_cid = unsealed_link.seal(&mut store).await.unwrap();

//         assert_eq!(file_cid, sealed_cid);
//     }

//     #[async_std::test]
//     async fn cid_link_can_be_resolved() {
//         let time = Utc::now();

//         let dir = Rc::new(PublicDirectory::new(time));

//         let mut store = MemoryBlockStore::default();

//         let dir_cid = dir.store(&mut store).await.unwrap();

//         let unresolved_link = PublicLink::Cid(dir_cid);

//         let resolved_node = unresolved_link.resolve(&store).await.unwrap();

//         assert_eq!(PublicNode::Dir(dir), resolved_node);
//     }
// }
