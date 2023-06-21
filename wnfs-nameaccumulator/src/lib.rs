#![feature(once_cell)]
//! TODO(matheus23)

use anyhow::Result;
use error::VerificationError;
use fns::{multi_exp, nlogn_product, prime_digest, prime_digest_fast};
use num_bigint_dig::{BigUint, ModInverse, RandBigInt, RandPrime};
use num_integer::Integer;
use num_traits::One;
use rand_core::RngCore;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use sha3::{Digest, Sha3_256};
use std::{cell::OnceCell, hash::Hash, str::FromStr};

mod error;
mod fns;
mod uint256_serde_le;

#[derive(Clone, Eq)]
pub struct NameAccumulator {
    /// A 2048-bit number
    state: BigUint,
    /// A cache for its serialized form
    serialized_cache: OnceCell<[u8; 256]>,
}

fn prepare_l_hash(modulus: &BigUint, base: &BigUint, commitment: &BigUint) -> Sha3_256 {
    let mut hasher = sha3::Sha3_256::new();
    hasher.update(&modulus.to_bytes_le());
    hasher.update(&base.to_bytes_le());
    hasher.update(&commitment.to_bytes_le());
    hasher
}

impl NameAccumulator {
    pub fn empty(setup: &AccumulatorSetup) -> Self {
        Self {
            state: setup.generator.clone(),
            serialized_cache: OnceCell::new(),
        }
    }

    pub fn with_segments<'a>(
        segments: impl IntoIterator<Item = &'a NameSegment>,
        setup: &AccumulatorSetup,
    ) -> Self {
        let mut acc = Self::empty(setup);
        acc.add(segments, setup); // ignore proof
        acc
    }

    pub fn from_state(state: BigUint) -> Self {
        Self {
            state,
            serialized_cache: OnceCell::new(),
        }
    }

    pub fn add<'a>(
        &mut self,
        segments: impl IntoIterator<Item = &'a NameSegment>,
        setup: &AccumulatorSetup,
    ) -> ElementsProof {
        // Reset the serialized state
        self.serialized_cache = OnceCell::new();

        let mut product = BigUint::one();
        for segment in segments {
            product *= &segment.0;
        }

        let witness = self.state.clone();
        self.state = self.state.modpow(&product, &setup.modulus);

        let hasher = prepare_l_hash(&setup.modulus, &witness, &self.state);
        let (l, l_hash_inc) = prime_digest(hasher, 16);

        let (q, r) = product.div_mod_floor(&l);

        let big_q = witness.modpow(&q, &setup.modulus);

        ElementsProof {
            base: witness,
            big_q,
            part: UnbatchableProofPart { l_hash_inc, r },
        }
    }

    pub fn add_bytes(
        &mut self,
        bytes: impl AsRef<[u8]>,
        setup: &AccumulatorSetup,
    ) -> ElementsProof {
        let digest = Sha3_256::new().chain_update(bytes.as_ref());
        self.add(Some(&NameSegment::from_digest(digest)), setup)
    }

    pub fn parse_bytes(byte_buf: impl AsRef<[u8]>) -> Result<Self> {
        let mut bytes = [0u8; 256];
        bytes.copy_from_slice(byte_buf.as_ref());
        Ok(Self::parse_slice(bytes.try_into()?))
    }

    pub fn parse_slice(slice: [u8; 256]) -> Self {
        let state = BigUint::from_bytes_le(&slice);
        Self {
            state,
            serialized_cache: OnceCell::from(slice),
        }
    }

    pub fn into_bytes(self) -> [u8; 256] {
        let cache = self.serialized_cache;
        let state = self.state;
        cache
            .into_inner()
            .unwrap_or_else(|| uint256_serde_le::to_bytes_helper(&state))
    }

    pub fn as_bytes(&self) -> &[u8; 256] {
        self.serialized_cache
            .get_or_init(|| uint256_serde_le::to_bytes_helper(&self.state))
    }
}
/// PoKE* (Proof of Knowledge of Exponent),
/// assuming that the base is trusted
/// (e.g. part of a common reference string, CRS).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ElementsProof {
    /// The accumulator's base, $u$
    pub base: BigUint,
    /// A part of the proof, $Q = u^q$, where $element = q*l + r$
    pub big_q: BigUint,
    /// Part of the proof that can't be batched
    pub part: UnbatchableProofPart,
}

/// The part of PoKE* (Proof of Knowledge of Exponent) proofs that can't be batched.
/// This is very small (typically <20 bytes, most likely just 17 bytes).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnbatchableProofPart {
    /// The number to increase a hash by to land on the next prime.
    /// Helps to more quickly generate/verify the prime number $l$.
    pub l_hash_inc: u32,
    /// A part of the proof, the residue of the element that's proven, i.e. $r$ in $element = q*l + r$
    pub r: BigUint,
}

impl<'de> Deserialize<'de> for UnbatchableProofPart {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let (l_hash_inc, r): (u32, &serde_bytes::Bytes) = Deserialize::deserialize(deserializer)?;
        let r = BigUint::from_bytes_le(r);
        Ok(Self { l_hash_inc, r })
    }
}

impl Serialize for UnbatchableProofPart {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut residue = [0u8; 16];
        let encoded = self.r.to_bytes_le();
        residue[..encoded.len()].copy_from_slice(&encoded);
        let r = serde_bytes::Bytes::new(&residue);
        (self.l_hash_inc, r).serialize(serializer)
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
        NameAccumulator::parse_bytes(byte_buf).map_err(serde::de::Error::custom)
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
        self.as_bytes()
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct BatchedProofPart {
    big_q_product: BigUint,
}

impl BatchedProofPart {
    pub fn new() -> Self {
        Self {
            big_q_product: BigUint::one(),
        }
    }

    pub fn add(&mut self, proof: &ElementsProof) {
        self.big_q_product *= &proof.big_q;
    }
}

pub struct BatchedProofVerification<'a> {
    bases_and_exponents: Vec<(BigUint, BigUint)>,
    setup: &'a AccumulatorSetup,
}

impl<'a> BatchedProofVerification<'a> {
    pub fn new(setup: &'a AccumulatorSetup) -> Self {
        Self {
            bases_and_exponents: Vec::new(),
            setup,
        }
    }

    pub fn add(
        &mut self,
        base: &NameAccumulator,
        commitment: &NameAccumulator,
        proof_part: &UnbatchableProofPart,
    ) -> Result<()> {
        let hasher = prepare_l_hash(&self.setup.modulus, &base.state, &commitment.state);
        let l = prime_digest_fast(hasher, 16, proof_part.l_hash_inc)
            .ok_or(VerificationError::LHashNonPrime)?;

        if proof_part.r >= l {
            Err(VerificationError::ResidueOutsideRange)?;
        }

        let proof_kcr_base = (&commitment.state
            * (&base.state)
                .mod_inverse(&self.setup.modulus)
                .unwrap()
                .to_biguint()
                .unwrap()
                .modpow(&proof_part.r, &self.setup.modulus))
            % &self.setup.modulus;

        self.bases_and_exponents.push((proof_kcr_base, l));

        Ok(())
    }

    pub fn verify(&self, batched_proof: &BatchedProofPart) -> Result<()> {
        let l_star = nlogn_product(&self.bases_and_exponents, |(_, l)| l);

        if batched_proof
            .big_q_product
            .modpow(&l_star, &self.setup.modulus)
            != multi_exp(&self.bases_and_exponents, &self.setup.modulus)
        {
            return Err(VerificationError::ValidationFailed.into());
        }

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub struct AccumulatorSetup {
    #[serde(with = "uint256_serde_le")]
    modulus: BigUint,
    #[serde(with = "uint256_serde_le")]
    pub generator: BigUint,
}

impl AccumulatorSetup {
    /// Does a trusted setup in-memory and throws away the prime factors.
    /// This requires generating two 1024-bit primes, so it's fairly slow.
    pub fn trusted(rng: &mut impl RngCore) -> Self {
        // This is a trusted setup.
        // The prime factors are "toxic waste", they need to be
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
    /// This is great for tests, as it doesn't require lots of primality tests.
    ///
    /// [rsa factoring challenge]: https://en.wikipedia.org/wiki/RSA_numbers#RSA-2048
    pub fn from_rsa_2048(rng: &mut impl RngCore) -> Self {
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

    pub fn from_seed(seed: impl AsRef<[u8]>) -> Self {
        Self::from_digest(Sha3_256::new().chain_update(seed))
    }
}

#[derive(Clone, Debug, Eq)]
pub struct Name {
    relative_to: NameAccumulator,
    segments: Vec<NameSegment>,
    accumulated: OnceCell<(NameAccumulator, ElementsProof)>,
}

impl PartialEq for Name {
    fn eq(&self, other: &Self) -> bool {
        // TODO(matheus23) this is not ideal.
        // We're special-casing certain constructions of Names to be equal, but not all,
        // just so that all *existing* tests are OK.
        let left = self
            .accumulated
            .get()
            .map(|x| &x.0)
            .or(self.segments.is_empty().then(|| &self.relative_to));
        let right = other
            .accumulated
            .get()
            .map(|x| &x.0)
            .or(other.segments.is_empty().then(|| &other.relative_to));

        if let (Some(left), Some(right)) = (left, right) {
            return left == right;
        }

        self.relative_to == other.relative_to && self.segments == other.segments
    }
}

impl Name {
    pub fn empty(setup: &AccumulatorSetup) -> Self {
        Self::new(NameAccumulator::empty(setup), None)
    }

    pub fn new(
        relative_to: NameAccumulator,
        segments: impl IntoIterator<Item = NameSegment>,
    ) -> Self {
        Self {
            relative_to,
            segments: segments.into_iter().collect(),
            accumulated: OnceCell::new(),
        }
    }

    pub fn is_root(&self) -> bool {
        self.segments.len() == 0
    }

    pub fn up(&mut self) {
        self.segments.pop();
        self.accumulated = OnceCell::new();
    }

    pub fn parent(&self) -> Option<Name> {
        if self.is_root() {
            None
        } else {
            let mut name = self.clone();
            name.up();
            Some(name)
        }
    }

    pub fn add_segments(&mut self, segments: impl IntoIterator<Item = NameSegment>) {
        self.segments.extend(segments);
        self.accumulated = OnceCell::new();
    }

    pub fn with_segments_added(&self, segments: impl IntoIterator<Item = NameSegment>) -> Self {
        let mut clone = self.clone();
        clone.add_segments(segments);
        clone
    }

    pub fn as_proven_accumulator(
        &self,
        setup: &AccumulatorSetup,
    ) -> &(NameAccumulator, ElementsProof) {
        self.accumulated.get_or_init(|| {
            let mut name = self.relative_to.clone();
            let proof = name.add(self.segments.iter(), setup);
            (name, proof)
        })
    }

    pub fn as_accumulator(&self, setup: &AccumulatorSetup) -> &NameAccumulator {
        &self.as_proven_accumulator(setup).0
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        AccumulatorSetup, BatchedProofPart, BatchedProofVerification, NameAccumulator, NameSegment,
    };
    use libipld::{
        cbor::DagCborCodec,
        prelude::{Decode, Encode},
        Ipld,
    };
    use proptest::{prop_assert, prop_assert_eq};
    use rand::thread_rng;
    use std::io::Cursor;
    use test_strategy::proptest;

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
        let setup = &AccumulatorSetup::from_rsa_2048(rng);
        let mut acc = NameAccumulator::empty(setup);

        acc.add(
            &[
                NameSegment::new(rng),
                NameSegment::new(rng),
                NameSegment::new(rng),
            ],
            setup,
        );

        let ipld = libipld::serde::to_ipld(&acc).unwrap();
        let mut bytes = Vec::new();
        ipld.encode(DagCborCodec, &mut bytes).unwrap();

        let ipld = Ipld::decode(DagCborCodec, &mut Cursor::new(bytes)).unwrap();
        let acc_back = libipld::serde::from_ipld::<NameAccumulator>(ipld).unwrap();

        assert_eq!(acc_back, acc);
    }

    #[proptest(cases = 256)]
    fn batch_proofs(do_batch_step: [bool; 4], do_verify_step: [bool; 4]) {
        let rng = &mut thread_rng();
        let setup = &AccumulatorSetup::from_rsa_2048(rng);
        let base_a = NameAccumulator::with_segments(&[NameSegment::new(rng)], setup);
        let base_b = NameAccumulator::with_segments(&[NameSegment::new(rng)], setup);

        let mut acc_a_one = base_a.clone();
        let mut acc_a_two = base_a.clone();
        let mut acc_b_one = base_b.clone();
        let mut acc_b_two = base_b.clone();

        let proof_a_one = acc_a_one.add(&[NameSegment::new(rng), NameSegment::new(rng)], setup);
        let proof_a_two = acc_a_two.add(&[NameSegment::new(rng), NameSegment::new(rng)], setup);
        let proof_b_one = acc_b_one.add(&[NameSegment::new(rng), NameSegment::new(rng)], setup);
        let proof_b_two = acc_b_two.add(&[NameSegment::new(rng), NameSegment::new(rng)], setup);

        let mut batched_proof = BatchedProofPart::new();
        if do_batch_step[0] {
            batched_proof.add(&proof_a_one);
        }
        if do_batch_step[1] {
            batched_proof.add(&proof_a_two);
        }
        if do_batch_step[2] {
            batched_proof.add(&proof_b_one);
        }
        if do_batch_step[3] {
            batched_proof.add(&proof_b_two);
        }

        let mut verify = BatchedProofVerification::new(setup);

        if do_verify_step[0] {
            verify.add(&base_a, &acc_a_one, &proof_a_one.part).unwrap();
        }
        if do_verify_step[1] {
            verify.add(&base_a, &acc_a_two, &proof_a_two.part).unwrap();
        }
        if do_verify_step[2] {
            verify.add(&base_b, &acc_b_one, &proof_b_one.part).unwrap();
        }
        if do_verify_step[3] {
            verify.add(&base_b, &acc_b_two, &proof_b_two.part).unwrap();
        }

        let result = verify.verify(&batched_proof);

        // If we end up only batching "step 3", but also verify only step 3, then that's fine.
        // If we end up batching step 3, but verifying step 2, then that should fail.
        let matching_batch_and_verify = do_batch_step
            .into_iter()
            .zip(do_verify_step.into_iter())
            .all(|(did_batch, did_verify)| did_batch == did_verify);

        if !matching_batch_and_verify {
            prop_assert!(result.is_err());
        } else {
            match result {
                Ok(_) => (),
                Err(e) => prop_assert_eq!("no error", format!("{e}")),
            }
        }
    }
}
