<div align="center">
  <a href="https://github.com/wnfs-wg" target="_blank">
    <img src="https://raw.githubusercontent.com/wnfs-wg/rs-wnfs/main/assets/logo.png" alt="WNFS Logo" width="100" height="100"></img>
  </a>

  <h1 align="center">wnfs-namefilter</h1>

  <p>
    <a href="https://crates.io/crates/wnfs-namefilter">
      <img src="https://img.shields.io/crates/v/wnfs-namefilter?label=crates" alt="Docs">
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

Namefilters are 2048-bit Bloom filters. They can hold 47 path segments with one-in-a-billion false positive rate using 30 hashes.

Many Bloom filter implementations are optimized for speed, not consistency. We have chosen the XXH32 (i.e. 32-bit) algorithm. It is about as fast as XXH64 for 256-bit (i.e. small) data.
XXH32 is very portable. It can be implemented within JavaScript's number system (at time of writing, ES2021 and earlier). It also can be natively implemented on any 32-bit machine, or on common 64-bit machines with a 32-bit compatability mode, such as AMD64.
However, for every element inserted into the Bloom filters we need k = 30 different hash functions.

We get these from the first two XXH32 invocations with seeds 0 and 1 and the enhanced double hashing scheme (§5.2, Algorithm 2) to generate more hash functions from the first two. In our case the enhanced double hashing scheme operates on 32-bit unsigned integer arithmetic. The resulting hashes are taken modulo m = 2048 to convert these hashes into an index in the accumulator.

## Usage

```rust
use wnfs_namefilter::Namefilter;

let mut filter = Namefilter::default();

filter.add(&[0xF5u8; 32]);
filter.saturate();

assert!(filter.contains(&[0xF5u8; 32]));
```
