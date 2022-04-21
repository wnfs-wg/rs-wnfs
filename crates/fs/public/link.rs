//! Node link.

use std::rc::Rc;

use anyhow::Result;
use libipld::{cbor::DagCborCodec, Cid};

use super::{PublicDirectory, PublicFile, PublicNode};
use crate::{blockstore, shared, BlockStore, Shared};

/// A link to another node in the WNFS public file system. It can be held as a simple serialised CID or as a reference to the node itself.
///
/// The public file system is a DAG so we don't have to worry bout cyclic references.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Link {
    Cid(Cid),
    Node(Shared<PublicNode>),
}

impl Link {
    /// Creates a new directory node link.
    pub fn with_dir(dir: PublicDirectory) -> Self {
        Link::Node(shared(PublicNode::Dir(dir)))
    }

    /// Creates a new file node link.
    pub fn with_file(file: PublicFile) -> Self {
        Link::Node(shared(PublicNode::File(file)))
    }

    /// Resolves a CID linkin the file system to a node.
    pub async fn resolve<B: BlockStore>(&self, store: &B) -> Result<Shared<PublicNode>> {
        Ok(match self {
            Link::Cid(cid) => {
                let node = blockstore::load(store, cid, DagCborCodec).await?;
                shared(node)
            }
            Link::Node(node) => Rc::clone(node),
        })
    }

    /// Stores the link in the block store and returns the CID.
    pub async fn seal<B: BlockStore>(&self, store: &mut B) -> Result<Cid> {
        Ok(match self {
            Link::Cid(cid) => *cid,
            Link::Node(node) => node.borrow().store(store).await?,
        })
    }
}

#[cfg(test)]
mod public_link_tests {
    use std::mem;

    use chrono::Utc;
    use libipld::Cid;

    use crate::{
        public::{PublicDirectory, PublicFile, PublicNode},
        shared, MemoryBlockStore,
    };

    use super::Link;

    #[async_std::test]
    async fn node_link_can_be_sealed() {
        let time = Utc::now();

        let userland = Cid::default();

        let file = PublicFile::new(time, userland);

        let mut store = MemoryBlockStore::default();

        let file_cid = file.store(&mut store).await.unwrap();

        let unsealed_link = Link::Node(shared(PublicNode::File(file)));

        let sealed_cid = unsealed_link.seal(&mut store).await.unwrap();

        assert_eq!(file_cid, sealed_cid);
    }

    #[async_std::test]
    async fn cid_link_can_be_resolved() {
        let time = Utc::now();

        let dir = PublicDirectory::new(time);

        let mut store = MemoryBlockStore::default();

        let dir_cid = dir.store(&mut store).await.unwrap();

        let unresolved_link = Link::Cid(dir_cid);

        let resolved_node = unresolved_link.resolve(&store).await.unwrap();

        let node = mem::replace(
            &mut *resolved_node.borrow_mut(),
            PublicNode::Dir(PublicDirectory::new(time)),
        );

        assert_eq!(dir, node.into_dir())
    }
}
