<div align="center">
  <a href="https://github.com/wnfs-wg" target="_blank">
    <img src="https://raw.githubusercontent.com/wnfs-wg/rs-wnfs/main/assets/logo.png" alt="WNFS Logo" width="100" height="100"></img>
  </a>

  <h1 align="center">WebNative FileSystem (WNFS)</h1>

  <p>
    <a href="https://crates.io/crates/wnfs">
      <img src="https://img.shields.io/crates/v/wnfs?label=crates" alt="Concurrency Docs">
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
      <img src="https://img.shields.io/static/v1?label=Docs&message=docs.rs&color=blue" alt="Concurrency Docs">
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

This library is designed with WebAssembly in mind. You can follow instructions on how to use it in your browser applications [here][wnfs-wasm-readme].

## Outline

- [Crates](#crates)
- [Building the Project](#building-the-project)
- [Usage](#usage)
- [Testing the Project](#testing-the-project)
- [Contributing](#contributing)
- [Getting Help](#getting-help)
- [External Resources](#external-resources)
- [License](#license)

## Crates

- [wnfs](https://github.com/wnfs-wg/rs-wnfs/tree/main/wnfs)
- [wnfs-wasm](https://github.com/wnfs-wg/rs-wnfs/tree/main/wnfs-wasm)
- [wnfs-common](https://github.com/wnfs-wg/rs-wnfs/tree/main/wnfs-common)
- [wnfs-hamt](https://github.com/wnfs-wg/rs-wnfs/tree/main/wnfs-hamt)
- [wnfs-nameaccumulator](https://github.com/wnfs-wg/rs-wnfs/tree/main/wnfs-nameaccumulator)
- [wnfs-unixfs-file](https://github.com/wnfs-wg/rs-wnfs/tree/main/wnfs-unixfs-file)

This is the dependency graph between these crates:
```mermaid
flowchart TD
    wnfs-wasm --> wnfs
    wnfs-wasm --> wnfs-nameaccumulator
    %% wnfs-bench --> wnfs
    %% wnfs-bench --> wnfs-hamt
    %% wnfs-bench --> wnfs-nameaccumulator
    wnfs --> wnfs-hamt
    wnfs --> wnfs-common
    wnfs --> wnfs-unixfs-file
    wnfs --> wnfs-nameaccumulator
    wnfs-unixfs-file --> wnfs-common
    wnfs-hamt --> wnfs-common
    wnfs-nameaccumulator -> wnfs-common
```

## Building the Project

### REQUIREMENTS

- **The Rust Toolchain**

  Follow the instructions [here][rust-toolchain-guide] to install the official Rust toolchain.

- **The WebAssembly Toolchain**

  If you are interested in compiling the project for WebAssembly, you can follow the instructions below.

  <details>
    <summary>Read more</summary>

  - Install `wasm32-unknown-unknown` target

    ```bash
    rustup target add wasm32-unknown-unknown
    ```

  - [rust-analyzer][rust-analyzer-guide] is the go-to IDE tool for Rust and if you have it set up, you may want to set the `rust-analyzer.cargo.target` [setting][vscode-settings] to `wasm32-unknown-unknown`

  - Install wasm-bindgen

    ```bash
    cargo install wasm-bindgen-cli
    ```

  - Install [playwright][playwright-guide] binaries

    ```bash
    npx playwright install
    ```

  **Architecture-specifics**

  - On ARM-based (M1 family) macOS, you might need to explicitly install the following:

    - Install wasm-bindgen

      ```bash
      cargo install -f wasm-bindgen-cli
      ```

    - Install wasm-opt

      ```bash
      brew install binaryen
      ```

  - On Arch Linux based distributions, you might need to explicitly install the following:

    - Install wasm-opt

      ```bash
      sudo pacman -S binaryen
      ```

  </details>

- **The _rs-wnfs_ Command**

  You can optionally set up the `rs-wnfs` script.

  <details>
    <summary>Read more</summary>

  - Install it using the following command:

    ```bash
    sh ./scripts/rs-wnfs.sh setup
    ```

  - This lets you run the `rs-wnfs.sh` script as a command.

    ```bash
    rs-wnfs help
    ```

  </details>

### STEPS

- Clone the repository.

  ```bash
  git clone https://github.com/wnfs-wg/rs-wnfs.git
  ```

- Change directory

  ```bash
  cd rs-wnfs
  ```

- Build the project

  Check [REQUIREMENTS](#requirements) on how to set up the `rs-wnfs` command.

  ```bash
  scripts/rs-wnfs build
  ```

- You can also build for specific crates

  ```bash
  scripts/rs-wnfs build --wasm
  ```

## Usage

WNFS does not have an opinion on where you want to persist your content or the file tree. Instead, the API takes any object that implements the asynchronous [`BlockStore`][blockstore-trait] trait. The library also avoids including system function calls that could possibly tie it to a set of platforms. Operations like time and random number generation have to be passed in via the API. This allows the library to be used in a wide variety of environments. It particularly makes virtualisation easier.

Let's see an example of working with a public filesystem. We will use the in-memory block store provided by the library.

```rust
use anyhow::Result;
use chrono::Utc;
use wnfs::{
    common::MemoryBlockStore,
    public::PublicDirectory
};

#[async_std::main]
async fn main() -> Result<()> {
    // Create a new public directory.
    let dir = &mut PublicDirectory::new_rc(Utc::now());

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
use rand_chacha::ChaCha12Rng;
use rand_core::SeedableRng;
use wnfs::{
    common::MemoryBlockStore,
    private::{
        PrivateDirectory,
        forest::{hamt::HamtForest, traits::PrivateForest},
    }
};

#[async_std::main]
async fn main() -> Result<()> {
    // Create an in-memory block store.
    let store = &MemoryBlockStore::default();

    // A random number generator.
    let rng = &mut ChaCha12Rng::from_entropy();

    // Create a private forest.
    let forest = &mut HamtForest::new_trusted_rc(rng);

    // Create a new private directory.
    let dir = &mut PrivateDirectory::new_rc(&forest.empty_name(), Utc::now(), rng);

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

## Testing the Project

- Run all tests

  ```bash
  scripts/rs-wnfs test
  ```

- Run benchmarks

  ```bash
  scripts/rs-wnfs bench
  ```

  You can also find a nice graph of the CI benchmarks [here][benchmarks].

## Contributing

### Pre-commit Hook

This library recommends using [pre-commit][pre-commit-guide] for running pre-commit hooks. Please run this before every commit and/or push.

- Once installed, Run `pre-commit install` to setup the pre-commit hooks locally. This will reduce failed CI builds.
- If you are doing interim commits locally, and for some reason if you _don't_ want pre-commit hooks to fire, you can run
  `git commit -a -m "Your message here" --no-verify`.

### Conventional Commits

This project _lightly_ follows the [Conventional Commits convention][commit-spec-site]
to help explain commit history and tie in with our release process. The full
specification can be found [here][commit-spec]. We recommend prefixing your
commits with a type of `fix`, `feat`, `docs`, `ci`, `refactor`, etc...,
structured like so:

```
<type>[optional scope]: <description>

[optional body]

[optional footer(s)]
```

## Getting Help

For usage questions, usecases, or issues reach out to us in our [Discord webnative-fs channel][webnative-discord].
We would be happy to try to answer your question or try opening a new issue on Github.

## License

This project is licensed under the [Apache License 2.0](https://github.com/wnfs-wg/rs-wnfs/blob/main/LICENSE).

[benchmarks]: https://wnfs-wg.github.io/rs-wnfs/dev/bench/
[blockstore-trait]: https://github.com/wnfs-wg/rs-wnfs/blob/main/wnfs-common/src/blockstore.rs
[commit-spec]: https://www.conventionalcommits.org/en/v1.0.0/#specification
[commit-spec-site]: https://www.conventionalcommits.org/
[hamt-wiki]: https://en.wikipedia.org/wiki/Hash_array_mapped_trie
[ipld-spec]: https://ipld.io/
[npm-ipld-tools]: https://www.npmjs.com/search?q=ipld
[playwright-guide]: https://playwright.dev/
[pre-commit-guide]: https://pre-commit.com/
[rust-analyzer-guide]: https://rust-analyzer.github.io/manual.html#installation
[rust-toolchain-guide]: https://doc.rust-lang.org/cargo/getting-started/installation.html
[shared-private-fs-talk]: https://vimeo.com/534517727
[vscode-settings]: https://code.visualstudio.com/docs/getstarted/settings#_workspace-settings
[webnative-discord]: https://discord.gg/YbT6x7Wkvk
[wnfs-examples]: wnfs/examples/
[wnfs-graph-demo]: https://calm-thin-barista.fission.app
[wnfs-spec]: https://github.com/wnfs-wg/spec
[wnfs-wasm-readme]: wnfs-wasm/README.md
