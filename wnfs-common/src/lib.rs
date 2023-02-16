mod async_serialize;
pub mod blockstore;
mod encoding;
mod error;
mod link;
mod metadata;
mod pathnodes;
pub mod utils;

pub use async_serialize::*;
pub use blockstore::*;
pub use encoding::*;
pub use error::*;
pub use link::*;
pub use metadata::*;
pub use pathnodes::*;

//--------------------------------------------------------------------------------------------------
// Constants
//--------------------------------------------------------------------------------------------------

pub const MAX_BLOCK_SIZE: usize = usize::pow(2, 18);

//--------------------------------------------------------------------------------------------------
// Type Definitions
//--------------------------------------------------------------------------------------------------

// TODO(matheus23) move into its own file `traits.rs`?
pub mod traits {
    use anyhow::Result;
    use async_trait::async_trait;

    use crate::BlockStore;

    /// Implements deep equality check for two types.
    #[async_trait(?Send)]
    pub trait IpldEq {
        /// Checks if the two items are deeply equal.
        async fn eq<B: BlockStore>(&self, other: &Self, store: &mut B) -> Result<bool>;
    }
}
