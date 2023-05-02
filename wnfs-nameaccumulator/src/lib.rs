#![feature(once_cell)]
//! TODO(matheus23)

use anyhow::Result;
use fns::prime_digest;
use num_bigint_dig::{BigUint, RandBigInt, RandPrime};
use rand_core::RngCore;
use serde::{Deserialize, Serialize};
use sha3::{Digest, Sha3_256};
use std::{cell::OnceCell, hash::Hash, str::FromStr};

mod biguint_serde_le;
mod fns;

#[derive(Clone, Eq)]
pub struct NameAccumulator {
    /// A 2048-bit number
    state: BigUint,
    /// A cache for its serialized form
    serialized_cache: OnceCell<[u8; 256]>,
}

impl NameAccumulator {
    pub fn empty(setup: &AccumulatorSetup) -> Self {
        Self {
            state: setup.generator.clone(),
            serialized_cache: OnceCell::new(),
        }
    }

    pub fn from_state(state: BigUint) -> Self {
        Self {
            state,
            serialized_cache: OnceCell::new(),
        }
    }

    pub fn add(&mut self, segment: &NameSegment, setup: &AccumulatorSetup) {
        self.state = self.state.modpow(&segment.0, &setup.modulus);
        self.serialized_cache = OnceCell::new();
    }

    pub fn add_bytes(&mut self, bytes: impl AsRef<[u8]>, setup: &AccumulatorSetup) {
        let digest = Sha3_256::new().chain_update(bytes.as_ref());
        self.add(&NameSegment::from_digest(digest), setup)
    }

    pub fn parse_bytes(bytes: &[u8]) -> Result<Self> {
        let bytes: [u8; 256] = bytes.try_into()?;
        let state = BigUint::from_bytes_le(&bytes);
        Ok(Self {
            state,
            serialized_cache: OnceCell::from(bytes),
        })
    }
}

impl Ord for NameAccumulator {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.state.cmp(&other.state)
    }
}

impl PartialOrd for NameAccumulator {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.state.partial_cmp(&other.state)
    }
}

impl PartialEq for NameAccumulator {
    fn eq(&self, other: &Self) -> bool {
        self.state == other.state
    }
}

impl std::fmt::Debug for NameAccumulator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NameAccumulator")
            .field("state", &self.state.to_string())
            .finish()
    }
}

impl Hash for NameAccumulator {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.state.hash(state);
    }
}

impl<'de> Deserialize<'de> for NameAccumulator {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let byte_buf = serde_bytes::ByteBuf::deserialize(deserializer)?;
        let mut bytes = [0u8; 256];
        bytes.copy_from_slice(&byte_buf);
        let state = BigUint::from_bytes_le(&bytes);

        Ok(NameAccumulator {
            state,
            serialized_cache: OnceCell::from(bytes),
        })
    }
}

impl Serialize for NameAccumulator {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serde_bytes::serialize(self.as_ref(), serializer)
    }
}

impl AsRef<[u8]> for NameAccumulator {
    fn as_ref(&self) -> &[u8] {
        self.serialized_cache.get_or_init(|| {
            let vec = self.state.to_bytes_le();
            let mut bytes = [0u8; 256];
            bytes[..vec.len()].copy_from_slice(&vec);
            bytes
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub struct AccumulatorSetup {
    #[serde(with = "biguint_serde_le")]
    modulus: BigUint,
    #[serde(with = "biguint_serde_le")]
    generator: BigUint,
}

impl AccumulatorSetup {
    /// Does a trusted setup in-memory and throws away the prime factors.
    /// This requires generating two 1024-bit primes, so it's fairly slow.
    pub fn trusted(rng: &mut impl RngCore) -> Self {
        // This is a trusted setup.
        // The prime factors are so-called "toxic waste", they need to be
        // disposed immediately.
        let modulus = rng.gen_prime(1024) * rng.gen_prime(1024);
        // The generator is just some random quadratic residue.
        let generator = rng
            .gen_biguint_below(&modulus)
            .modpow(&BigUint::from(2u8), &modulus);
        Self { modulus, generator }
    }

    /// Faster than `trusted`, but depends on the 2048-bit [rsa factoring challenge]
    /// to not be broken.
    ///
    /// This is great for tests.
    ///
    /// [rsa factoring challenge]: https://en.wikipedia.org/wiki/RSA_numbers#RSA-2048
    pub fn from_rsa_factoring_challenge(rng: &mut impl RngCore) -> Self {
        let modulus = BigUint::from_str(
            "25195908475657893494027183240048398571429282126204032027777137836043662020707595556264018525880784406918290641249515082189298559149176184502808489120072844992687392807287776735971418347270261896375014971824691165077613379859095700097330459748808428401797429100642458691817195118746121515172654632282216869987549182422433637259085141865462043576798423387184774447920739934236584823824281198163815010674810451660377306056201619676256133844143603833904414952634432190114657544454178424020924616515723350778707749817125772467962926386356373289912154831438167899885040445364023527381951378636564391212010397122822120720357",
        ).unwrap();
        let generator = rng
            .gen_biguint_below(&modulus)
            .modpow(&BigUint::from(2u8), &modulus);
        Self { modulus, generator }
    }
}

/// A name accumluator segment. A name accumulator commits to a set of these.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct NameSegment(
    /// Invariant: Must be a 256-bit prime
    BigUint,
);

impl std::fmt::Debug for NameSegment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("NameSegment")
            .field(&self.0.to_string())
            .finish()
    }
}

impl Serialize for NameSegment {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut bytes = self.0.to_bytes_le();
        bytes.resize(32, 0);
        serde_bytes::serialize(&bytes, serializer)
    }
}

impl<'de> Deserialize<'de> for NameSegment {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let bytes: Vec<u8> = serde_bytes::deserialize(deserializer)?;
        Ok(NameSegment(BigUint::from_bytes_le(&bytes)))
    }
}

impl NameSegment {
    pub fn new(rng: &mut impl RngCore) -> Self {
        Self(rng.gen_prime(256))
    }

    pub fn from_digest(digest: impl Digest + Clone) -> Self {
        Self(prime_digest(digest, 32).0)
    }
}

#[cfg(test)]
mod tests {
    use crate::{AccumulatorSetup, NameAccumulator, NameSegment};
    use libipld::{
        cbor::DagCborCodec,
        prelude::{Decode, Encode},
        Ipld,
    };
    use rand::thread_rng;
    use std::io::Cursor;

    #[test]
    fn name_segment_serialize_roundtrip() {
        let rng = &mut thread_rng();
        let segment = NameSegment::new(rng);

        let ipld = libipld::serde::to_ipld(&segment).unwrap();
        let mut bytes = Vec::new();
        ipld.encode(DagCborCodec, &mut bytes).unwrap();

        let ipld = Ipld::decode(DagCborCodec, &mut Cursor::new(bytes)).unwrap();
        let segment_back = libipld::serde::from_ipld::<NameSegment>(ipld).unwrap();

        assert_eq!(segment_back, segment);
    }

    #[test]
    fn name_accumulator_serialize_roundtrip() {
        let rng = &mut thread_rng();
        let setup = &AccumulatorSetup::from_rsa_factoring_challenge(rng);
        let mut acc = NameAccumulator::empty(setup);

        acc.add(&NameSegment::new(rng), setup);
        acc.add(&NameSegment::new(rng), setup);
        acc.add(&NameSegment::new(rng), setup);

        let ipld = libipld::serde::to_ipld(&acc).unwrap();
        let mut bytes = Vec::new();
        ipld.encode(DagCborCodec, &mut bytes).unwrap();

        let ipld = Ipld::decode(DagCborCodec, &mut Cursor::new(bytes)).unwrap();
        let acc_back = libipld::serde::from_ipld::<NameAccumulator>(ipld).unwrap();

        assert_eq!(acc_back, acc);
    }
}
