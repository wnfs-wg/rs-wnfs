//! The crate implements the [Web Native File System](https://whitepaper.fission.codes/file-system/file-system-basics) (WNFS) version 2.
//!
//! The Web Native File System is a file system written for the web.
//! It is versioned, logged, programmable, has strong-yet-flexible security, and is fully controlled by the end user.
//! Service providers can validate writes without reading the contents of the file system, and minimal metadata is leaked.
//!
//! This implementation is based off of the [typescript implementation](https://github.com/fission-suite/webnative/tree/matheus23/wnfs2/src/fs).
//! It exposes an immutable API, extending WNFS immutable nature to the in-memory representation of the file system.

mod common;
pub mod private;
pub mod public;
mod traits;

pub use common::*;
pub use private::{
    namefilter::Namefilter, PrivateDirectory, PrivateFile, PrivateNode, PrivateOpResult,
};
pub use public::{PublicDirectory, PublicFile, PublicNode, PublicOpResult};
pub use traits::*;

//--------------------------------------------------------------------------------------------------
// Re-exports
//--------------------------------------------------------------------------------------------------

pub mod ipld {
    pub use libipld::{
        cbor::DagCborCodec,
        codec::{Codec, Decode, Encode},
        Cid, IpldCodec,
    };
}

pub mod rng {
    pub use rand_core::RngCore;
}
