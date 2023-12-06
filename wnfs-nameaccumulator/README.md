<div align="center">
  <a href="https://github.com/wnfs-wg" target="_blank">
    <img src="https://raw.githubusercontent.com/wnfs-wg/rs-wnfs/main/assets/logo.png" alt="WNFS Logo" width="100" height="100"></img>
  </a>

  <h1 align="center">wnfs-nameaccumulator</h1>

  <p>
    <a href="https://crates.io/crates/wnfs-nameaccumulator">
      <img src="https://img.shields.io/crates/v/wnfs-nameaccumulator?label=crates" alt="Docs">
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

## WNFS Name Accumulators

This library implements the cryptographic primitives necessary for WNFS to prove that its writes were valid in a way that's verifyable by third parties without read access.

Specifically, it implements 2048-bit RSA accumulators and the PoKE* and PoKCR algorithms from the paper ["Batching Techniques for Accumulators with Applications to IOPs and Stateless Blockchains"](https://eprint.iacr.org/2018/1188.pdf), as well as some WNFS-specific interfaces and serialized representations for them.

## Usage

RSA accumulators require a trusted setup. Whoever has access to the trusted setup can create arbitrary valid proofs, which would in practice let malicious actors who've only been given partial access to a WNFS access to the rest of the file system.
For this reason the trusted setup is run once upon creation of a new WNFS by the root author. The root author is naturally incentivized to throw away the toxic waste from the trusted setup.

```rust
use wnfs_nameaccumulator::{AccumulatorSetup, BatchedProofPart, BatchedProofVerification, Name, NameSegment};
use rand::thread_rng;

// Run the trutsed setup.
let rng = &mut thread_rng();
let setup = &AccumulatorSetup::trusted(rng);

// We want to prove the names for two files at
// /Docs/Note and /Pics/Image respectively
let mut name_note = Name::empty(setup);
let mut name_image = Name::empty(setup);

// Each segment is represented by a random 256-bit prime number
let root_dir_segment = NameSegment::new(rng);
let docs_dir_segment = NameSegment::new(rng);
let pics_dir_segment = NameSegment::new(rng);
let note_file_segment = NameSegment::new(rng);
let image_file_segment = NameSegment::new(rng);

name_note.add_segments([root_dir_segment.clone(), docs_dir_segment, note_file_segment]);
name_image.add_segments([root_dir_segment, pics_dir_segment, image_file_segment]);

// We can collapse these arrays of primes that represent paths into 2048-bit RSA accumulators
// with a proof that they were derived from the same "base" name, in this case the `Name::empty` above.
let (accum_note, proof_note) = name_note.as_proven_accumulator(setup);
let (accum_image, proof_image) = name_image.as_proven_accumulator(setup);

// Knowing the proofs, we can batch at least parts of the proofs together.
// This results in a single 2048-bit batched proof part and ~17-20 bytes of unbatched proof per element.
let mut batched_proof = BatchedProofPart::new();
batched_proof.add(&proof_note);
batched_proof.add(&proof_image);

// Without read access, but given the accumulated base name and the proofs,
// we can verify that the accumulated names are related to the same base name.
let name_base = Name::empty(setup).as_accumulator(setup).clone();
let mut verification = BatchedProofVerification::new(setup);
verification.add(&name_base, &accum_note, &proof_note.part)?;
verification.add(&name_base, &accum_image, &proof_image.part)?;
verification.verify(&batched_proof)?;
```

## The `rug` feature

This enables a different backend for big unsigned integer arithmetic, based on the [rug crate] (which is based on the [GNU multiprecision library], also abbreviated GMP).

It is roughly 2x faster than the `num-bigint-dig` implementation when building for release, but as a bonus is also fast in debug builds (e.g. during tests) due to rug containing a statically linked release build of GMP.

However, it doesn't work in Wasm and it should be noted that GMP is licensed as [LGPLv3].

If you depend on the `wnfs` crate, but want to use the `rug` backend for your application, then simply add a `wnfs-nameaccumulator` as a dependency and enable its `rug` feature. This will make `wnfs` use a version of `wnfs-nameaccumulator` with `rug` enabled:

```toml
wnfs-nameaccumulator = { version = "*", default-features = false, features = ["rug"] }
```


[rug crate]: https://crates.io/crates/rug
[GNU multiprecision library]: https://gmplib.org/
[LGPLv3]: https://www.gnu.org/licenses/lgpl-3.0.en.html
