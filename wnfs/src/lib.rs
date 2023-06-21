//! The crate implements the [Web Native File System](https://whitepaper.fission.codes/file-system/file-system-basics) (WNFS) version 2.
//!
//! The Web Native File System is a file system written for the web.
//! It is versioned, logged, programmable, has strong-yet-flexible security, and is fully controlled by the end user.
//! Service providers can validate writes without reading the contents of the file system, and minimal metadata is leaked.
//!
//! This implementation is based off of the [typescript implementation](https://github.com/fission-suite/webnative/tree/matheus23/wnfs2/src/fs).
//! It exposes an immutable API, extending WNFS immutable nature to the in-memory representation of the file system.
#![deny(unsafe_code)]

pub mod error;
pub mod private;
pub mod public;
pub(crate) mod root_tree;
pub mod traits;
mod utils;

pub mod rand_core {
    pub use rand_core::RngCore;
}
pub mod common {
    pub use wnfs_common::*;
}
pub mod hamt {
    pub use wnfs_hamt::*;
}
pub mod namefilter {
    pub use wnfs_namefilter::*;
}

//--------------------------------------------------------------------------------------------------
// Constants
//--------------------------------------------------------------------------------------------------

const VERSION: semver::Version = semver::Version::new(0, 2, 0);

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// The result of an basic get operation.
pub(crate) enum SearchResult<T> {
    Missing(T, usize),
    NotADir(T, usize),
    Found(T),
}

//--------------------------------------------------------------------------------------------------
// Constants
//--------------------------------------------------------------------------------------------------

pub const WNFS_VERSION: semver::Version = semver::Version::new(0, 2, 0);
