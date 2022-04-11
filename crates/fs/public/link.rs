//! Node link.

use std::rc::Rc;

use anyhow::Result;
use libipld::{cbor::DagCborCodec, Cid};

use super::PublicNode;
use crate::BlockStore;

/// A link to another node in the WNFS public file system. It can be held as a simple serialised CID or as a reference to the node itself.
///
/// The public file system is a DAG so we don't have to worry bout cyclic references.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Link {
    Cid(Cid),
    Node(Rc<PublicNode>),
}

impl Link {
    // Resolves a CID link in the file system to a node.
    pub async fn resolve<B: BlockStore>(&self, store: &B) -> Result<Rc<PublicNode>> {
        Ok(match self {
            Link::Cid(cid) => {
                let node = store.load(cid, DagCborCodec).await?;
                Rc::new(node)
            }
            Link::Node(node) => Rc::clone(node),
        })
    }

    // Stores the link in the block store and returns the CID.
    pub async fn seal<B: BlockStore>(&self, store: &mut B) -> Result<Cid> {
        Ok(match self {
            Link::Cid(cid) => *cid,
            Link::Node(node) => node.store(store).await?,
        })
    }
}

#[cfg(test)]
mod public_link_tests {
    #[async_std::test]
    async fn node_link_sealed_successfully() {
        // TODO(appcypher): Implement.
    }

    #[async_std::test]
    async fn cid_link_resolved_successfully() {
        // TODO(appcypher): Implement.
    }
}
