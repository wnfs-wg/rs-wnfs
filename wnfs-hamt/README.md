<div align="center">
  <a href="https://github.com/wnfs-wg" target="_blank">
    <img src="https://raw.githubusercontent.com/wnfs-wg/rs-wnfs/main/assets/logo.png" alt="WNFS Logo" width="100" height="100"></img>
  </a>

  <h1 align="center">wnfs-hamt</h1>

  <p>
    <a href="https://crates.io/crates/wnfs-hamt">
      <img src="https://img.shields.io/crates/v/wnfs-hamt?label=crates" alt="Docs">
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

This Rust crate provides an implementation of a [Hash Array Mapped Trie (HAMT)](https://en.wikipedia.org/wiki/Hash_array_mapped_trie) based on IPLD.

HAMT is a data structure that hashes keys and uses increments of the hash at each level to determine placement of the entry or child node in the tree structure.

The number of bits used for index calculation at each level is determined by the bitWidth.
Each node can hold up to 2^bitWidth elements, which are stored in an array. Entries are stored in key-sorted order in buckets.
If a bucket already contains the maximum number of elements, a new child node is created and entries are inserted into the new node.

The data elements array is only allocated to store actual entries, and a map bitfield is used to determine if an index exists in the data array.

The implementation is based on [fvm_ipld_hamt](https://github.com/filecoin-project/ref-fvm/tree/master/ipld/hamt) with some modifications for async blockstore access and immutability-by-default.

## Usage

```rust
use wnfs_hamt::Node;
use wnfs_common::MemoryBlockStore;

let store = &MemoryBlockStore::default();
let scores: Node<String, usize> = Rc::new(Node::default());

scores.set("Mandy", 30, store).await?;
let result = scores.get("Mandy", store).await?;
```
