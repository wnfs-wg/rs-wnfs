<div align="center">
  <a href="https://github.com/wnfs-wg" target="_blank">
    <img src="https://raw.githubusercontent.com/wnfs-wg/rs-wnfs/main/assets/logo.png" alt="WNFS Logo" width="100" height="100"></img>
  </a>

  <h1 align="center">WebNative FileSystem (WNFS)</h1>

  <p>
    <a href="https://crates.io/crates/wnfs">
      <img src="https://img.shields.io/crates/v/wnfs?label=crates" alt="Docs">
    </a>
    <a href="https://codecov.io/gh/wnfs-wg/rs-wnfs">
      <img src="https://codecov.io/gh/wnfs-wg/rs-wnfs/branch/main/graph/badge.svg?token=95YHXFMFF4" alt="Code Coverage"/>
    </a>
    <a href="https://github.com/wnfs-wg/rs-wnfs/actions?query=">
      <img src="https://github.com/wnfs-wg/rs-wnfs/actions/workflows/checks.yaml/badge.svg" alt="Build Status">
    </a>
    <a href="https://github.com/wnfs-wg/rs-wnfs/blob/main/LICENSE">
      <img src="https://img.shields.io/badge/License-Apache%202.0-blue.svg" alt="License">
    </a>
    <a href="https://docs.rs/wnfs">
      <img src="https://img.shields.io/static/v1?label=Docs&message=docs.rs&color=blue" alt="Docs">
    </a>
    <a href="https://discord.gg/zAQBDEq">
      <img src="https://img.shields.io/static/v1?label=Discord&message=join%20us!&color=mediumslateblue" alt="Discord">
    </a>
  </p>
</div>

<div align="center"><sub>:warning: Work in progress :warning:</sub></div>

##

This is a Rust implementation of [the WebNative FileSystem (WNFS) specification][wnfs-spec]. WNFS is a versioned content-addressable distributed filesystem with private and public sub systems. The private filesystem is encrypted so that only users with the right keys can access its contents. It is designed to prevent inferring metadata like the structure of the file tree. The other part of the WNFS filesystem is a simpler public filesystem that is not encrypted and can be accessed by anyone with the right address.

WNFS also features collaborative editing of file trees, where multiple users can edit the same tree at the same time.

WNFS file trees can serialize and be deserialized from [IPLD graphs][ipld-spec] with an extensible metadata section. This allows WNFS to be understood by other [IPLD-based tools][npm-ipld-tools] and systems.

## Usage

WNFS does not have an opinion on where you want to persist your content or the file tree. Instead, the API takes any object that implements the asynchronous [`BlockStore`][blockstore-trait] trait. The library also avoids including system function calls that could possibly tie it to a set of platforms. Operations like time and random number generation have to be passed in via the API. This allows the library to be used in a wide variety of environments. It particularly makes virtualisation easier.

Let's see an example of working with a public filesystem. We will use the in-memory block store provided by the library.

```rust
use anyhow::Result;
use chrono::Utc;
use std::rc::Rc;
use wnfs::public::PublicDirectory;
use wnfs_common::MemoryBlockStore;

#[async_std::main]
async fn main() -> Result<()> {
    // Create a new public directory.
    let dir = &mut Rc::new(PublicDirectory::new(Utc::now()));

    // Create an in-memory block store.
    let store = &MemoryBlockStore::default();

    // Add a /pictures/cats subdirectory.
    dir.mkdir(&["pictures".into(), "cats".into()], Utc::now(), store)
        .await?;

    // Store the the file tree in the in-memory block store.
    dir.store(store).await?;

    // List all files in /pictures directory.
    let result = dir.ls(&["pictures".into()], store).await?;

    println!("Files in /pictures: {:#?}", result);

    Ok(())
}
```

Here we create a root directory `dir` and subsequently add a `/pictures/cats` subdirectory to it. As mentioned earlier, system-level operations like time are passed in from the API. In this case, we use the `Utc::now()` function from the [chrono][chrono-crate] crate to get the current time.

`PublicDirectory` gets wrapped in `Rc` here because it lets us pass it around without worrying about ownership and lifetimes. Making the Rc `&mut` futher allows us to relinquish ownership to the interior `PublicDirectory` and point to a new one when needed (essentially for every write). This immutable way of handling changes has cool benefits like tracking and rolling back changes. It also makes collaborative editing easier to implement and reason about. You can find more examples in the [`wnfs/examples/`][wnfs-examples] folder.

That's the public filesystem, the private filesystem, on the other hand, is a bit more involved. The [Hash Array Mapped Trie (HAMT)][hamt-wiki] is where we store the private filesystem tree and some other information related to it. HAMT allows for effective storage and retrieval of encrypted and obfuscated filesystem trees and `PrivateForest` is basically a HAMT that can contain multiple file trees with hash for keys and CIDs for values.

```rust
use anyhow::Result;
use chrono::Utc;
use rand::thread_rng;
use std::rc::Rc;
use wnfs::private::{
    PrivateDirectory,
    forest::{hamt::HamtForest, traits::PrivateForest},
};
use wnfs_common::MemoryBlockStore;

#[async_std::main]
async fn main() -> Result<()> {
    // Create an in-memory block store.
    let store = &MemoryBlockStore::default();

    // A random number generator.
    let rng = &mut thread_rng();

    // Create a private forest.
    let forest = &mut Rc::new(HamtForest::new_trusted(rng));

    // Create a new private directory.
    let dir = &mut Rc::new(PrivateDirectory::new(
        &forest.empty_name(),
        Utc::now(),
        rng,
    ));

    // Add a file to /pictures/cats directory.
    dir.mkdir(
        &["pictures".into(), "cats".into()],
        true,
        Utc::now(),
        forest,
        store,
        rng,
    )
    .await?;

    // Add a file to /pictures/dogs/billie.jpg file.
    dir.write(
        &["pictures".into(), "dogs".into(), "billie.jpg".into()],
        true,
        Utc::now(),
        b"Hello! This is billie".to_vec(),
        forest,
        store,
        rng,
    )
    .await?;

    // List all files in /pictures directory.
    let result = dir.ls(&["pictures".into()], true, forest, store).await?;

    println!("Files in /pictures: {:#?}", result);

    Ok(())
}
```

This example introduces a few new concepts. The first is the `HamtForest` which is a HAMT that can contain multiple file trees and implements the `PrivateForest` interface needed for persisting private file systems.

The second is the `Name` (returned from `forest.empty_name()`) and `NameAccumulator` that lets us identify nodes in the filesystem, and are suitable for offspring proving.

Finally, we have the random number generator, `rng`, that the library uses for generating new keys and other random values needed for the protocol.

Check the [`wnfs/examples/`][wnfs-examples] folder for more examples.


[blockstore-trait]: https://github.com/wnfs-wg/rs-wnfs/blob/main/wnfs-common/src/blockstore.rs
[hamt-wiki]: https://en.wikipedia.org/wiki/Hash_array_mapped_trie
[ipld-spec]: https://ipld.io/
[npm-ipld-tools]: https://www.npmjs.com/search?q=ipld
[wnfs-examples]: https://github.com/wnfs-wg/rs-wnfs/tree/main/wnfs/examples
[wnfs-graph-demo]: https://calm-thin-barista.fission.app
[wnfs-spec]: https://github.com/wnfs-wg/spec
