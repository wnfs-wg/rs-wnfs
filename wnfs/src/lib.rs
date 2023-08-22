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
    pub use rand_core::CryptoRngCore;
}
pub mod common {
    pub use wnfs_common::*;
}
pub mod hamt {
    pub use wnfs_hamt::*;
}
pub mod nameaccumulator {
    pub use wnfs_nameaccumulator::*;
}

//--------------------------------------------------------------------------------------------------
// Constants
//--------------------------------------------------------------------------------------------------

/// The version of the WNFS data format that this library outputs
pub const WNFS_VERSION: semver::Version = semver::Version::new(1, 0, 0);

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
// Functions
//--------------------------------------------------------------------------------------------------

/// Whether given WNFS data format version can be read by this library
pub fn is_readable_wnfs_version(version: &semver::Version) -> bool {
    get_wnfs_version_req().matches(version)
}

/// The WNFS data format version requirement for this version of the library
pub fn get_wnfs_version_req() -> semver::VersionReq {
    use semver::*;
    VersionReq {
        comparators: vec![Comparator {
            op: Op::Exact,
            major: WNFS_VERSION.major,
            minor: Some(WNFS_VERSION.minor),
            patch: None,
            pre: Prerelease::EMPTY,
        }],
    }
}
