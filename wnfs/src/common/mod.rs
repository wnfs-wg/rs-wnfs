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

pub const HASH_BYTE_SIZE: usize = 32;
pub const MAX_BLOCK_SIZE: usize = usize::pow(2, 18);

//--------------------------------------------------------------------------------------------------
// Type Definitions
//--------------------------------------------------------------------------------------------------

/// The general size of digests in WNFS.
pub type HashOutput = [u8; HASH_BYTE_SIZE];
