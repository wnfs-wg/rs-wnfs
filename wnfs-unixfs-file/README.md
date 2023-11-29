<div align="center">
  <a href="https://github.com/wnfs-wg" target="_blank">
    <img src="https://raw.githubusercontent.com/wnfs-wg/rs-wnfs/main/assets/logo.png" alt="WNFS Logo" width="100" height="100"></img>
  </a>

  <h1 align="center">wnfs-unixfs-file</h1>

  <p>
    <a href="https://crates.io/crates/wnfs-unixfs-file">
      <img src="https://img.shields.io/crates/v/wnfs-unixfs-file?label=crates" alt="Docs">
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

This Rust crate provides an implementation of UnixFs files. WNFS uses the UnixFs file encoding purely to chunk big byte arrays into multiple blocks and produce a single CID for them to link to from WNFS sttuctures.

This crate is a fork from beetle (previously "iroh")'s [iroh-unixfs crate](https://github.com/n0-computer/beetle/tree/3e137cb2bc18e1d458c3f72d5e817b03d9537d5d/iroh-unixfs).

Major changes relative to that implementation include:
- Removed prost code generation, instead it's some hard-coded structs with prost annotations
- Removed support for any UnixFs structures other than files (no directories, directory shards or symlinks)
- Removed parallelization for hashing to make the crate async runtime-independent (so it can be used in wasm with wasm-bindgen-futures!)
- Doesn't hard-code use of SHA-256 anymore
- Integrated with the wnfs-common `BlockStore` trait

## Usage

```rs
use wnfs_unixfs_file::builder::FileBuilder;
use wnfs_common::MemoryBlockStore;
use tokio::io::AsyncReadExt;

// Where data is stored
let store = &MemoryBlockStore::new();

// Encoding byte arrays, getting a CID
let data = vec![1u8; 1_000_000]; // 1MiB of ones
let root_cid = FileBuilder::new()
    .content_bytes(data)
    .build()?
    .store(store)
    .await?;

// Taking a CID, reading back a byte array:
let file = UnixFsFile::load(&root_cid, store).await?;
println!("filesize: {}", file.filesize());
let mut buffer = Vec::new();
let mut reader = file.into_content_reader(store, None)?;
reader.read_to_end(&mut buffer).await?;
// buffer now has 1 million ones

// You can also seek
use tokio::io::AsyncSeekExt;
use std::io::SeekFrom;
let mut reader = file.into_content_reader(store, None)?;
reader.seek(SeekFrom::Start(10_000)).await?;
let mut slice = [0u8; 10_000];
reader.read_exact(&mut slice).await?;
// slice now has 10_000 ones
```
