<div align="center">
  <a href="https://github.com/WebNativeFileSystem" target="_blank">
    <img src="https://raw.githubusercontent.com/WebNativeFileSystem/rs-wnfs/main/assets/logo.svg" alt="Fission Logo" width="100" height="100"></img>
  </a>

  <h1 align="center">WebNative FileSystem (WNFS)</h1>

  <p>
    <a href="https://crates.io/crates/wnfs">
      <img src="https://img.shields.io/crates/v/wnfs?label=crates" alt="Concurrency Docs">
    </a>
    <a href="https://codecov.io/gh/WebNativeFileSystem/rs-wnfs">
      <img src="https://codecov.io/gh/WebNativeFileSystem/rs-wnfs/branch/main/graph/badge.svg?token=95YHXFMFF4" alt="Code Coverage"/>
    </a>
    <a href="https://github.com/WebNativeFileSystem/rs-wnfs/actions?query=">
      <img src="https://github.com/WebNativeFileSystem/rs-wnfs/actions/workflows/checks.yaml/badge.svg" alt="Build Status">
    </a>
    <a href="https://github.com/WebNativeFileSystem/rs-wnfs/blob/main/LICENSE">
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

This crate is a Rust implementation of the primitives for creating and manipulating IPLD graphs that encode WNFS.

A goal of the project is to be easily compiled to WebAssembly to be used in the browsers or other environments.

## Outline

- [Usage](#usage)
- [Building the Project](#building-the-project)
- [Testing the Project](#testing-the-project)

## Usage

Creating a new public directory.

```rust
use wnfs::{PublicDirectory, Id};

use async_std::main;
use chrono::Utc;

#[async_std::main]
async fn main() {
  let dir = PublicDirectory::new(Utc::now());
  println!("id = {}", dir.get_id());
}
```

The in-memory files and directories you create with `wnfs` will need to be sealed and stored somewhere. For that, a type that implements the BlockStore trait like [this one](https://github.com/WebNativeFileSystem/rs-wnfs/blob/8bb0fbb457051295f1ed4a4707dc230c04612658/crates/fs/common/blockstore.rs#L42-L62) can be used.

```rust
use wnfs::{MemoryBlockStore, PublicDirectory, OpResult, ipld::Cid};

use async_std::main;
use chrono::Utc;

use std::rc::Rc;
// ...
```

The WNFS API is immutable, therefore, we need to keep track of the updated root directory after every change.

Each fs operation returns a possibly updated root directory that subsequent changes can be applied on.

```rust
// ...
#[async_std::main]
async fn main() {
    let time = Utc::now();
    let dir = Rc::new(PublicDirectory::new(time));
    let store = MemoryBlockStore::default();

    // Create a /pictures/cats directory.
    let OpResult { root_dir, .. } = dir
        .mkdir(&["pictures".into(), "cats".into()], time, &store)
        .await
        .unwrap();

    // Get a sample CIDv1.
    let cid = Cid::default();

    // Add a file to /pictures/cats.
    let OpResult { root_dir, .. } = root_dir
        .write(
            &["pictures".into(), "cats".into(), "tabby.png".into()],
            cid,
            time,
            &store,
        )
        .await
        .unwrap();

    // Create and add a file to /pictures/dogs directory.
    let OpResult { root_dir, .. } = root_dir
        .write(
            &["pictures".into(), "dogs".into(), "billie.jpeg".into()],
            cid,
            time,
            &store,
        )
        .await
        .unwrap();

    // Delete /pictures/cats directory.
    let OpResult { root_dir, .. } = root_dir
        .rm(&["pictures".into(), "cats".into()], &store)
        .await
        .unwrap();

    // List all files in /pictures directory.
    let OpResult { result, .. } = root_dir
        .ls(&["pictures".into()], &store)
        .await
        .unwrap();

    println!("Files in /pictures: {:#?}", result);
}
```

## Building the Project

#### REQUIREMENTS

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

  - Install playwrigth binaries

    ```bash
    npx playwright install
    ```

  On ARM-based (M1 family) macOS, you might need to explicitly install the following:

  - Install wasm-bindgen

    ```bash
    cargo install -f wasm-bindgen-cli
    ```

  - Install wasm-opt

    ```bash
    brew install binaryen
    ```

  </details>

- **The _rs-wnfs_ Command**

  You can optionally set up the `rs-wnfs` script.

  <details>
    <summary>Read more</summary>

  - Install it using the following command:

    ```bash
    sh script/rs-wnfs.sh setup
    ```

  - This lets you run the `rs-wnfs.sh` script as a command.

    ```bash
    rs-wnfs help
    ```

  </details>

#### STEPS

- Clone the repository.

  ```bash
  git https://github.com/WebNativeFileSystem/rs-wnfs.git
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

## Testing the Project

- Run all tests

  ```bash
  rs-wnfs test --all
  ```

- Show code coverage

  ```bash
  rs-wnfs coverage
  ```
