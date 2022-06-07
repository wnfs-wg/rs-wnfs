use libipld::Cid;

use super::PrivateNode;

/// A link to another node in the WNFS public file system. It can be held as a simple serialised CID or as a reference to the node itself.
///
/// The public file system is a DAG so we don't have to worry bout cyclic references.
#[derive(Debug, Clone)]
pub enum Link {
    Cid(Cid),
    Node(PrivateNode),
}
