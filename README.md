<div align="center">
  <a href="https://github.com/wnfs-wg" target="_blank">
    <img src="assets/logo.png" alt="Fission Logo" width="100" height="100"></img>
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

This is a Rust implementation of [the WebNative FileSystem (WNFS) specification](https://github.com/wnfs-wg/spec). WNFS is a versioned content-addressable distributed filesystem with private and public sub systems. The private filesystem is encrypted so that only users with the right keys can access its contents. It is designed to prevent inferring metadata like the structure of the file tree. The other part of the WNFS filesystem is a simpler public filesystem that is not encrypted and can be accessed by anyone with the right address.

WNFS also features collaborative editing of file trees, where multiple users can edit the same tree at the same time.

WNFS file trees can serialize and be deserialized from IPLD graphs with an extensible metadata section. This allows WNFS to be understood by other IPLD-based tools and systems.

This library is designed with WebAssembly in mind. You can follow instructions on how to use it in your browser applications [here](crates/wasm/README.md).

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

- [fs / filesystem](https://github.com/wnfs-wg/rs-wnfs/tree/main/crates/fs)
- [wasm](https://github.com/wnfs-wg/rs-wnfs/tree/main/crates/wasm)

## Building the Project

### REQUIREMENTS

- **The Rust Toolchain**

  Follow the instructions [here](https://doc.rust-lang.org/cargo/getting-started/installation.html) to install the official Rust toolchain.

- **The WebAssembly Toolchain**

  If you are interested in compiling the project for WebAssembly, you can follow the instructions below.

  <details>
    <summary>Read more</summary>

  - Install `wasm32-unknown-unknown` target

    ```bash
    rustup target add wasm32-unknown-unknown
    ```

  - [rust-analyzer](https://rust-analyzer.github.io/manual.html#installation) is the go-to IDE tool for Rust and if you have it set up, you may want to set the `rust-analyzer.cargo.target` [setting](https://code.visualstudio.com/docs/getstarted/settings#_workspace-settings) to `wasm32-unknown-unknown`

  - Install wasm-pack

    ```bash
    cargo install wasm-pack
    ```

  - Install [playwright](https://playwright.dev/) binaries

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
    sudo sh scripts/rs-wnfs.sh setup
    ```

  - This lets you run the `rs-wnfs.sh` script as a command.

    ```bash
    rs-wnfs help
    ```

  </details>

### STEPS

- Clone the repository.

  ```bash
  git clone https://github.com/WebNativeFileSystem/rs-wnfs.git
  ```

- Change directory

  ```bash
  cd rs-wnfs
  ```

- Build the project

  Check [REQUIREMENTS](#requirements) on how to set up the `rs-wnfs` command.

  ```bash
  rs-wnfs build --all
  ```

- You can also build for specific crates

  ```bash
  rs-wnfs build --wasm
  ```

## Usage

WNFS does not have an opinion on where you want to persist your content or the file tree. Instead, the API expects any object that implements the async [`BlockStore`](https://github.com/wnfs-wg/rs-wnfs/blob/07d026c1ef324597da9ac7897353015dd634af16/crates/fs/common/blockstore.rs#L30-L85) interface. This implementation also defers system-level operations to the user; requiring that operations like time and random number generation be passed in from the interface. This makes for a clean wasm interface that works everywhere.

Let's see an example of working with a public directory. Here we are going to use the memory-based blockstore provided by library.

```rust
use wnfs::{MemoryBlockStore, PublicDirectory, PublicOpResult};

use chrono::Utc;

use std::rc::Rc;

#[async_std::main]
async fn main() {
    // Create a new public directory.
    let dir = Rc::new(PublicDirectory::new(Utc::now()));

    // Create a memory-based blockstore.
    let store = &mut MemoryBlockStore::default();

    // Add a /pictures/cats subdirectory.
    let PublicOpResult { root_dir, .. } = dir
        .mkdir(&["pictures".into(), "cats".into()], Utc::now(), store)
        .await
        .unwrap();

    // Store the the file tree in the memory blockstore.
    root_dir.store(store).await.unwrap();

    // Print root directory.
    println!("{:#?}", root_dir);
}
```

You may notice that we store the `root_dir` returned by the `mkdir` operation, not the `dir` we started with. That is because WNFS internal state is immutable and every operation potentially returns a new root directory. This allows us to track and rollback changes when needed. It also makes collaborative editing easier to implement and reason about. You can find more examples in the [`crates/fs/examples/`](crates/fs/examples/) folder. And there is a basic demo of the filesystem immutability [here](https://calm-thin-barista.fission.app).

The private filesystem, on the other hand, is a bit more involved. [Hash Array Mapped Trie (HAMT)](https://en.wikipedia.org/wiki/Hash_array_mapped_trie) is used as the intermediate format of private file tree before it is persisted to the blockstore. HAMT helps us hide the hierarchy of the file tree.

```rust
use wnfs::{
    private::PrivateForest, MemoryBlockStore, Namefilter, PrivateDirectory, PrivateOpResult,
};

use chrono::Utc;
use rand::thread_rng;

use std::rc::Rc;

#[async_std::main]
async fn main() {
    // Create a memory-based blockstore.
    let store = &mut MemoryBlockStore::default();

    // A random number generator the private filesystem can use.
    let rng = &mut thread_rng();

    // Create HAMT intermediate data structure.
    let hamt = Rc::new(PrivateForest::new());

    // Create a new private directory.
    let dir = Rc::new(PrivateDirectory::new(
        Namefilter::default(),
        Utc::now(),
        rng,
    ));

    // Add a file to /pictures/cats directory.
    let PrivateOpResult { root_dir, hamt, .. } = dir
        .mkdir(
            &["pictures".into(), "cats".into()],
            true,
            Utc::now(),
            hamt,
            store,
            rng,
        )
        .await
        .unwrap();

    // Add a file to /pictures/dogs/billie.jpg file.
    let PrivateOpResult { root_dir, hamt, .. } = root_dir
        .write(
            &["pictures".into(), "dogs".into(), "billie.jpeg".into()],
            true,
            Utc::now(),
            b"hello world".to_vec(),
            hamt,
            store,
            rng,
        )
        .await
        .unwrap();

    // List all files in /pictures directory.
    let PrivateOpResult { result, .. } = root_dir
        .ls(&["pictures".into()], true, hamt, store)
        .await
        .unwrap();

    println!("Files in /pictures: {:#?}", result);
}
```

Namefilters are currently how we represent the identity key of a node in the filesystem. They have nice properties, one of which is the ability to check if one node belongs to another. This is necessary in a filesystem where metadata like hierarchy needs to be hidden from observing agents. One notable caveat with namefilters is that they can only reliably store information of a file tree 47 levels deep or less so there is a plan to replace them with cryptographic accumlators in the near future.

Check the [`crates/fs/examples/`](crates/fs/examples/) folder for more examples.

## Testing the Project

- Run all tests

  ```bash
  rs-wnfs test --all
  ```

- Show code coverage

  ```bash
  rs-wnfs coverage
  ```

## Contributing

### Pre-commit Hook

This library recommends using [pre-commit](https://pre-commit.com/) for running pre-commit hooks. Please run this before every commit and/or push.

- Once installed, Run `pre-commit install` to setup the pre-commit hooks locally.  This will reduce failed CI builds.
- If you are doing interim commits locally, and for some reason if you _don't_ want pre-commit hooks to fire, you can run
  `git commit -a -m "Your message here" --no-verify`.

## Getting Help

For usage questions, usecases, or issues reach out to us in our [Discord webnative-fs channel](https://discord.com/channels/478735028319158273/877623827331428403).
We would be happy to try to answer your question or try opening a new issue on Github.

## External Resources

This is a list of links to blog posts, confereence talks, and tutorials related to WNFS in general:

### Talks

- [A Distributed File System for Secure P2P Applications](https://www.youtube.com/watch?v=-f4cH_HQU4U) by Brooklyn Zelenka (Strange Loop 2022)
- [WebNative File System](https://www.youtube.com/watch?v=3se17NAS-Lw) by Brooklyn Zelenka (IPFS bing 2022)
- [Shared Private Files Design in Webnative's WNFS](https://vimeo.com/534517727) by Brooklyn Zelenka

### Related Implementations

- [wnfs-go](https://github.com/wnfs-wg/wnfs-go)
- [WNFS (typescript) / Webnative SDK](https://github.com/fission-codes/webnative/blob/main/README.md#web-native-file-system)

## License

This project is licensed under the [Apache License 2.0](https://github.com/wnfs-wg/rs-wnfs/blob/main/LICENSE).
