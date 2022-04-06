use crate::common::Metadata;
use libipld::Cid;
use std::collections::HashMap;

pub enum PublicNode {
    File(PublicFile),
    Dir(PublicDirKind),
}

pub enum PublicDirKind {
    Link(PublicDir<LinkKind>),
    Cid(PublicDir<Cid>), // Persisted. TODO: what does that mean?
}

pub enum LinkKind {
    Cid(Cid),
    Node(PublicNode),
}

pub struct PublicDir<T> {
    metadata: Metadata,
    userland: HashMap<String, T>, // TODO: Hashbrown?
    previous: Option<Cid>,
}

pub struct PublicFile {
    metadata: Metadata,
    userland: Cid,
    previous: Option<Cid>,
}

pub enum OperationContext {
    AbortContext,
    BlockStore,
}

type PublicDirWithNodes = PublicDir<LinkKind>;

pub async fn lookup_node(path: &str, dir: PublicDirWithNodes, ) -> Cid {
    todo!("implement lookup_node")
}
