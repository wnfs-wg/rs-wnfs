use crate::types::LoadedCid;
use anyhow::Result;
use async_trait::async_trait;
use libipld::Cid;
use std::{
    fmt::{self, Debug, Display, Formatter},
    sync::Arc,
};

#[async_trait]
pub trait ContentLoader: Sync + Send + std::fmt::Debug + Clone + 'static {
    /// Loads the actual content of a given cid.
    async fn load_cid(&self, cid: &Cid, ctx: &LoaderContext) -> Result<LoadedCid>;
    /// Signal that the passend in session is not used anymore.
    async fn stop_session(&self, ctx: ContextId) -> Result<()>;
    /// Checks if the given cid is present in the local storage.
    async fn has_cid(&self, cid: &Cid) -> Result<bool>;
}

#[async_trait]
impl<T: ContentLoader> ContentLoader for Arc<T> {
    async fn load_cid(&self, cid: &Cid, ctx: &LoaderContext) -> Result<LoadedCid> {
        self.as_ref().load_cid(cid, ctx).await
    }

    async fn stop_session(&self, ctx: ContextId) -> Result<()> {
        self.as_ref().stop_session(ctx).await
    }

    async fn has_cid(&self, cid: &Cid) -> Result<bool> {
        self.as_ref().has_cid(cid).await
    }
}

#[derive(Debug, Clone)]
pub struct LoaderContext {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ContextId(pub u64);

impl Display for ContextId {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "ContextId({})", self.0)
    }
}

impl From<u64> for ContextId {
    fn from(id: u64) -> Self {
        ContextId(id)
    }
}

impl From<ContextId> for u64 {
    fn from(id: ContextId) -> Self {
        id.0
    }
}
