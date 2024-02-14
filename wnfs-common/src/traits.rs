use crate::{utils::CondSend, BlockStore};
use anyhow::Result;
use futures::Future;

//--------------------------------------------------------------------------------------------------
// Traits
//--------------------------------------------------------------------------------------------------

/// Implements deep equality check for two types.
pub trait IpldEq {
    /// Checks if the two items are deeply equal.
    fn eq(
        &self,
        other: &Self,
        store: &impl BlockStore,
    ) -> impl Future<Output = Result<bool>> + CondSend;
}
