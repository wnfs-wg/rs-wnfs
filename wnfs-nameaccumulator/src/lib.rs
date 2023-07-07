//! This library implements the cryptographic primitives necessary for WNFS to prove that its writes were valid in a way that's verifyable by third parties without read access.
//!
//! Specifically, it implements 2048-bit RSA accumulators and the PoKE* and PoKCR algorithms from the paper ["Batching Techniques for Accumulators with Applications to IOPs and Stateless Blockchains"](https://eprint.iacr.org/2018/1188.pdf), as well as some WNFS-specific interfaces and serialized representations for them.

mod error;
mod fns;
mod name;
mod uint256_serde_be;

pub use name::*;
