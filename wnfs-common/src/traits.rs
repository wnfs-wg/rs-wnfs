use crate::BlockStore;
use anyhow::Result;
use async_trait::async_trait;

//--------------------------------------------------------------------------------------------------
// Traits
//--------------------------------------------------------------------------------------------------

/// Implements deep equality check for two types.
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
pub trait IpldEq {
    /// Checks if the two items are deeply equal.
    async fn eq(&self, other: &Self, store: &impl BlockStore) -> Result<bool>;
}
