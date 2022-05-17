//! Node link.

use std::rc::Rc;

use anyhow::Result;
use libipld::Cid;

use super::{PublicDirectory, PublicFile, PublicNode};
use crate::{blockstore, BlockStore};

/// A link to another node in the WNFS public file system. It can be held as a simple serialised CID or as a reference to the node itself.
///
/// The public file system is a DAG so we don't have to worry bout cyclic references.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Link {
    Cid(Cid),
    Node(Rc<PublicNode>),
}

impl Link {
    /// Creates a new directory node link.
    pub fn with_dir(dir: PublicDirectory) -> Self {
        Link::Node(Rc::new(PublicNode::Dir(Rc::new(dir))))
    }

    /// Creates a new file node link.
    pub fn with_file(file: PublicFile) -> Self {
        Link::Node(Rc::new(PublicNode::File(Rc::new(file))))
    }

    /// Resolves a CID linkin the file system to a node.
    pub async fn resolve<B: BlockStore>(&self, store: &B) -> Result<Rc<PublicNode>> {
        Ok(match self {
            Link::Cid(cid) => {
                let node = blockstore::load(store, cid, DagCborCodec).await?;
                Rc::new(node)
            }
            Link::Node(node) => Rc::clone(node),
        })
    }

    /// Stores the link in the block store and returns the CID.
    pub async fn seal<B: BlockStore>(&self, store: &mut B) -> Result<Cid> {
        Ok(match self {
            Link::Cid(cid) => *cid,
            Link::Node(node) => node.store(store).await?,
        })
    }
}

#[cfg(test)]
mod public_link_tests {
    use std::{rc::Rc};

    use chrono::Utc;
    use libipld::Cid;

    use crate::{
        public::{PublicDirectory, PublicFile, PublicNode},
        MemoryBlockStore,
    };

    use super::Link;

    #[async_std::test]
    async fn node_link_can_be_sealed() {
        let time = Utc::now();

        let userland = Cid::default();

        let file = PublicFile::new(time, userland);

        let mut store = MemoryBlockStore::default();

        let file_cid = file.store(&mut store).await.unwrap();

        let unsealed_link = Link::with_file(file);

        let sealed_cid = unsealed_link.seal(&mut store).await.unwrap();

        assert_eq!(file_cid, sealed_cid);
    }

    #[async_std::test]
    async fn cid_link_can_be_resolved() {
        let time = Utc::now();

        let dir = Rc::new(PublicDirectory::new(time));

        let mut store = MemoryBlockStore::default();

        let dir_cid = dir.store(&mut store).await.unwrap();

        let unresolved_link = Link::Cid(dir_cid);

        let resolved_node = unresolved_link.resolve(&store).await.unwrap();

        assert_eq!(Rc::new(PublicNode::Dir(dir)), resolved_node);
    }
}
