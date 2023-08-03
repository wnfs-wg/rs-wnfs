//! The crate implements the [Web Native File System](https://whitepaper.fission.codes/file-system/file-system-basics) (WNFS) version 2.
//!
//! The Web Native File System is a file system written for the web.
//! It is versioned, logged, programmable, has strong-yet-flexible security, and is fully controlled by the end user.
//! Service providers can validate writes without reading the contents of the file system, and minimal metadata is leaked.
//!
//! This implementation is based off of the [typescript implementation](https://github.com/fission-suite/webnative/tree/matheus23/wnfs2/src/fs) but designed with immutability in mind.
//!
//! Let's see an example of working with a public filesystem. We will use the in-memory block store provided by the library.
//!
//! ```rust
//! use anyhow::Result;
//! use chrono::Utc;
//! use std::rc::Rc;
//! use wnfs::{common::MemoryBlockStore, public::PublicDirectory};
//!
//! #[async_std::main]
//! async fn main() -> Result<()> {
//!     // Create a new public directory.
//!     let dir = &mut Rc::new(PublicDirectory::new(Utc::now()));
//!
//!     // Create an in-memory block store.
//!     let store = &MemoryBlockStore::default();
//!
//!     // Add a /pictures/cats subdirectory.
//!     dir.mkdir(&["pictures".into(), "cats".into()], Utc::now(), store)
//!         .await?;
//!
//!     // Store the the file tree in the in-memory block store.
//!     dir.store(store).await?;
//!
//!     // List all files in /pictures directory.
//!     let result = dir.ls(&["pictures".into()], store).await?;
//!
//!     println!("Files in /pictures: {:#?}", result);
//!
//!     Ok(())
//! }
//! ```
//!
//! Here we create a root directory `dir` and subsequently add a `/pictures/cats` subdirectory to it. As mentioned earlier, system-level operations like time are passed in from the API. In this case, we use the `Utc::now()` function from the [chrono][chrono-crate] crate to get the current time.
//!
//! `PublicDirectory` gets wrapped in `Rc` here because it lets us pass it around without worrying about ownership and lifetimes. Making the Rc `&mut` futher allows us to relinquish ownership to the interior `PublicDirectory` and point to a new one when needed (essentially for every write). This immutable way of handling changes has cool benefits like tracking and rolling back changes. It also makes collaborative editing easier to implement and reason about. You can find more examples in the [`wnfs/examples/`][wnfs-examples] folder.
//!
//! That's the public filesystem, the private filesystem, on the other hand, is a bit more involved. The [Hash Array Mapped Trie (HAMT)][hamt-wiki] is where we store the private filesystem tree and some other information related to it. HAMT allows for effective storage and retrieval of encrypted and obfuscated filesystem trees and `PrivateForest` is basically a HAMT that can contain multiple file trees with hash for keys and CIDs for values.
//!
//! ```rust
//! use anyhow::Result;
//! use chrono::Utc;
//! use rand::thread_rng;
//! use std::rc::Rc;
//! use wnfs::private::{
//!     PrivateDirectory,
//!     common::MemoryBlockStore,
//!     forest::{hamt::HamtForest, traits::PrivateForest},
//! };
//!
//! #[async_std::main]
//! async fn main() -> Result<()> {
//!     // Create an in-memory block store.
//!     let store = &MemoryBlockStore::default();
//!
//!     // A random number generator.
//!     let rng = &mut thread_rng();
//!
//!     // Create a private forest.
//!     let forest = &mut Rc::new(HamtForest::new_trusted(rng));
//!
//!     // Create a new private directory.
//!     let dir = &mut Rc::new(PrivateDirectory::new(
//!         &forest.empty_name(),
//!         Utc::now(),
//!         rng,
//!     ));
//!
//!     // Add a file to /pictures/cats directory.
//!     dir.mkdir(
//!         &["pictures".into(), "cats".into()],
//!         true,
//!         Utc::now(),
//!         forest,
//!         store,
//!         rng,
//!     )
//!     .await?;
//!
//!     // Add a file to /pictures/dogs/billie.jpg file.
//!     dir.write(
//!         &["pictures".into(), "dogs".into(), "billie.jpg".into()],
//!         true,
//!         Utc::now(),
//!         b"Hello! This is billie".to_vec(),
//!         forest,
//!         store,
//!         rng,
//!     )
//!     .await?;
//!
//!     // List all files in /pictures directory.
//!     let result = dir.ls(&["pictures".into()], true, forest, store).await?;
//!
//!     println!("Files in /pictures: {:#?}", result);
//!
//!     Ok(())
//! }
//! ```
//!
//! This example introduces a few new concepts. The first is the `HamtForest` which is a HAMT that can contain multiple file trees and implements the `PrivateForest` interface needed for persisting private file systems.
//!
//! The second is the `Name` (returned from `forest.empty_name()`) and `NameAccumulator` that lets us identify nodes in the filesystem, and are suitable for offspring proving.
//!
//! Finally, we have the random number generator, `rng`, that the library uses for generating new keys and other random values needed for the protocol.
//!
//! Check the [`wnfs/examples/`](wnfs-examples) folder for more examples.
//!
//!
//! [blockstore-trait]: https://github.com/wnfs-wg/rs-wnfs/blob/main/wnfs-common/src/blockstore.rs
//! [hamt-wiki]: https://en.wikipedia.org/wiki/Hash_array_mapped_trie
//! [ipld-spec]: https://ipld.io/
//! [wnfs-examples]: https://github.com/wnfs-wg/rs-wnfs/tree/main/wnfs/examples
//! [wnfs-graph-demo]: https://calm-thin-barista.fission.app
//! [wnfs-spec]: https://github.com/wnfs-wg/spec

#![deny(unsafe_code)]

pub mod error;
pub mod private;
pub mod public;
pub(crate) mod root_tree;
pub mod traits;
mod utils;

pub mod rand_core {
    //! Re-exports of rand-core lib.
    pub use rand_core::CryptoRngCore;
}
pub mod common {
    //! Re-exports of wnfs-common lib.
    pub use wnfs_common::*;
}
pub mod hamt {
    //! Re-exports of wnfs-hamt lib.
    pub use wnfs_hamt::*;
}
pub mod nameaccumulator {
    //! Re-exports of wnfs-nameaccumulator lib.
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
