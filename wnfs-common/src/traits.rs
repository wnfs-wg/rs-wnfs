use crate::BlockStore;
use anyhow::Result;
use async_trait::async_trait;

//--------------------------------------------------------------------------------------------------
// Traits
//--------------------------------------------------------------------------------------------------

/// Implements deep equality check for two types.
#[async_trait(?Send)]
pub trait IpldEq {
    /// Checks if the two items are deeply equal.
    async fn eq(&self, other: &Self, store: &mut impl BlockStore) -> Result<bool>;
}
