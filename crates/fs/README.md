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
use chrono::Utc;

let dir = PublicDirectory::new(Utc::now());
println!("id = {}", dir.get_id());
```

The in-memory files and directories you create with `wnfs` will need to be sealed and stored somewhere. For that, an object that implements the BlockStore trait like [this one](https://github.com/WebNativeFileSystem/rs-wnfs/blob/8bb0fbb457051295f1ed4a4707dc230c04612658/crates/fs/common/blockstore.rs#L42-L62) can be used.

```rust
use wnfs::{PublicDirectory, MemoryBlockStore, ipld::Cid};
use chrono::Utc;

let dir = PublicDirectory::new(Utc::now());
let store = MemoryBlockStore::default();

// ...
```

The WNFS API is immutable, therefore, we need to keep track of the updated root directory after every change.

Each fs operation returns a possibly updated root directory that subsequent changes can be applied on.

```rust
// ...

let dir = Rc::new(dir);

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
        &["pictures".into(), "cats".into(), "billie.jpeg".into()],
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
```

## Building the Project

- Build project

  ```bash
  cargo build --release
  ```

## Testing the Project

- Run tests

  ```bash
  cargo test --release
  ```
