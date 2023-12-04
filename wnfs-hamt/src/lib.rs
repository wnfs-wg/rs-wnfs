//! This Rust crate provides an implementation of a [Hash Array Mapped Trie (HAMT)](https://en.wikipedia.org/wiki/Hash_array_mapped_trie) based on IPLD.
//!
//! HAMT is a data structure that hashes keys and uses increments of the hash at each level to
//! determine placement of the entry or child node in the tree structure.
//!
//! The number of bits used for index calculation at each level is determined by the bitWidth.
//! Each node can hold up to 2^bitWidth elements, which are stored in an array. Entries are stored in key-sorted order in buckets.
//! If a bucket already contains the maximum number of elements, a new child node is created and entries are inserted into the new node.
//!
//! The data elements array is only allocated to store actual entries, and a map bitfield is used to determine if an index exists in the data array.
//!
//! The implementation is based on [fvm_ipld_hamt](https://github.com/filecoin-project/ref-fvm/tree/master/ipld/hamt) with some modifications for async blockstore access and immutability-by-default.

mod constants;
mod diff;
mod error;
mod hamt;
mod hash;
mod merge;
mod node;
mod pointer;
pub mod serializable;

pub(crate) use constants::*;
pub use diff::*;
pub use hamt::*;
pub use hash::*;
pub use merge::*;
pub use node::*;
pub use pointer::*;

#[cfg(any(test, feature = "test_utils"))]
pub mod strategies;
