//! Namefilters are 2048-bit Bloom filters. They can hold 47 path segments with one-in-a-billion false positive rate using 30 hashes.
//!
//! Many Bloom filter implementations are optimized for speed, not consistency. We have chosen the XXH32 (i.e. 32-bit) algorithm. It is about as fast as XXH64 for 256-bit (i.e. small) data.
//! XXH32 is very portable. It can be implemented within JavaScript's number system (at time of writing, ES2021 and earlier). It also can be natively implemented on any 32-bit machine, or on common 64-bit machines with a 32-bit compatability mode, such as AMD64.
//! However, for every element inserted into the Bloom filters we need k = 30 different hash functions.
//!
//! We get these from the first two XXH32 invocations with seeds 0 and 1 and the enhanced double hashing scheme (§5.2, Algorithm 2) to generate more hash functions from the first two. In our case the enhanced double hashing scheme operates on 32-bit unsigned integer arithmetic. The resulting hashes are taken modulo m = 2048 to convert these hashes into an index in the accumulator.

mod bloomfilter;
mod namefilter;

pub use bloomfilter::*;
pub use namefilter::*;
