//! This library implements the cryptographic primitives necessary for WNFS to prove that its writes were valid in a way that's verifyable by third parties without read access.
//!
//! Specifically, it implements 2048-bit RSA accumulators and the PoKE* and PoKCR algorithms from the paper ["Batching Techniques for Accumulators with Applications to IOPs and Stateless Blockchains"](https://eprint.iacr.org/2018/1188.pdf), as well as some WNFS-specific interfaces and serialized representations for them.

mod error;
mod fns;
#[cfg(any(feature = "rug", feature = "num-bigint-dig"))]
mod name;
mod traits;
mod uint256_serde_be;

#[cfg(not(feature = "rug"))]
#[cfg(not(feature = "num-bigint-dig"))]
compile_error!("no backend for big numbers, enable either the 'rug' or 'num-bigint-dig' feature.");

#[cfg(any(feature = "rug", feature = "num-bigint-dig"))]
pub use name::*;
pub use traits::*;
