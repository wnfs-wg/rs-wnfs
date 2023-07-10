use crate::{
    error::VerificationError,
    fns::{multi_exp, nlogn_product, prime_digest, prime_digest_fast},
    uint256_serde_be::to_bytes_helper,
};
use anyhow::Result;
use blake3::traits::digest::Digest;
use num_bigint_dig::{BigUint, ModInverse, RandBigInt, RandPrime};
use num_integer::Integer;
use num_traits::One;
use once_cell::sync::OnceCell;
use rand_core::CryptoRngCore;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::{hash::Hash, str::FromStr};
use zeroize::Zeroize;

//--------------------------------------------------------------------------------------------------
// Type Definitions
//--------------------------------------------------------------------------------------------------

/// A WNFS name.
/// Each file or directory has a name.
/// Names consist of a set of name segments and are commited to name accumulators.
/// However, these names are based on RSA accumulators to make it possible
/// to prove a relationship between two names, e.g a file being contained in
/// a sub-directory of a directory while leaking as little information as possible.
#[derive(Clone, Debug, Eq)]
pub struct Name {
    relative_to: NameAccumulator,
    segments: Vec<NameSegment>,
    accumulated: OnceCell<(NameAccumulator, ElementsProof)>,
}

/// Represents a setup needed for RSA accumulator operation.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub struct AccumulatorSetup {
    #[serde(with = "crate::uint256_serde_be")]
    modulus: BigUint,
    #[serde(with = "crate::uint256_serde_be")]
    pub generator: BigUint,
}

/// A WNFS name represented as the RSA accumulator of all of its name segments.
#[derive(Clone, Eq)]
pub struct NameAccumulator {
    /// A 2048-bit number
    state: BigUint,
    /// A cache for its serialized form
    serialized_cache: OnceCell<[u8; 256]>,
}

/// A name accumluator segment. A name accumulator commits to a set of these.
/// They are represented as 256-bit prime numbers.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct NameSegment(
    /// Invariant: Must be a 256-bit prime
    BigUint,
);

/// PoKE* (Proof of Knowledge of Exponent),
/// assuming that the base is trusted
/// (e.g. part of a common reference string).
#[derive(Clone, PartialEq, Eq)]
pub struct ElementsProof {
    /// The accumulator's base, $u$
    pub base: BigUint,
    /// A part of the proof, $Q = u^q$, where $element = q*l + r$
    pub big_q: BigUint,
    /// Part of the proof that can't be batched
    pub part: UnbatchableProofPart,
}

/// The part of PoKE* (Proof of Knowledge of Exponent) proofs that can't be batched.
/// This is very small (serialized typically <20 bytes, most likely just 17 bytes).
#[derive(Clone, PartialEq, Eq)]
pub struct UnbatchableProofPart {
    /// The number to increase a hash by to land on the next prime.
    /// Helps to more quickly generate/verify the prime number $l$.
    pub l_hash_inc: u32,
    /// A part of the proof, the residue of the element that's proven, i.e. $r$ in $element = q*l + r$
    pub r: BigUint,
}

/// The part of a name accumulator proof that can be batched,
/// i.e. the size of this part of the proof is independent of
/// the number of elements being proven. It's always 2048-bit.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct BatchedProofPart {
    #[serde(with = "crate::uint256_serde_be")]
    big_q_product: BigUint,
}

/// Data that is kept around for verifying batch proofs.
pub struct BatchedProofVerification<'a> {
    bases_and_exponents: Vec<(BigUint, BigUint)>,
    setup: &'a AccumulatorSetup,
}

//--------------------------------------------------------------------------------------------------
// Implementations
//--------------------------------------------------------------------------------------------------

impl Name {
    /// Create the empty name
    pub fn empty(setup: &AccumulatorSetup) -> Self {
        Self::new(NameAccumulator::empty(setup), None)
    }

    /// Create a name relative to some other committed name
    /// and with given segments added to that name.
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

    /// Returns whether the name has any segments added to it or
    /// just represents an absolute path without relative segments.
    pub fn is_root(&self) -> bool {
        self.segments.is_empty()
    }

    /// Remove the last name segment, if possible.
    pub fn up(&mut self) {
        self.segments.pop();
        self.accumulated = OnceCell::new();
    }

    /// Return the parent name, if possible.
    pub fn parent(&self) -> Option<Name> {
        if self.is_root() {
            None
        } else {
            let mut name = self.clone();
            name.up();
            Some(name)
        }
    }

    pub fn get_segments(&self) -> &Vec<NameSegment> {
        &self.segments
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

    /// Returns the commited name accumulator for this name
    /// as well as a proof that related the name accumulator that
    /// this name is relative to.
    ///
    /// This proof process is memoized. Running it twice won't duplicate work.
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

    /// Return what name accumulator this name commits to.
    pub fn as_accumulator(&self, setup: &AccumulatorSetup) -> &NameAccumulator {
        &self.as_proven_accumulator(setup).0
    }
}

impl NameAccumulator {
    /// Create the empty accumulator.
    pub fn empty(setup: &AccumulatorSetup) -> Self {
        Self {
            state: setup.generator.clone(),
            serialized_cache: OnceCell::new(),
        }
    }

    /// Create an accumulator with given segments inside.
    pub fn with_segments<'a>(
        segments: impl IntoIterator<Item = &'a NameSegment>,
        setup: &AccumulatorSetup,
    ) -> Self {
        let mut acc = Self::empty(setup);
        acc.add(segments, setup); // ignore proof
        acc
    }

    /// Create an accumulator from the number it's represented as.
    ///
    /// This needs to be a 2048-bit number in the RSA group from
    /// the accumulator setup used.
    pub fn from_state(state: BigUint) -> Self {
        Self {
            state,
            serialized_cache: OnceCell::new(),
        }
    }

    /// Add a set of elements to the accumulator and return a batch
    /// elements proof that verifies the change of state of the accumulator.
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

        let hasher = poke_fiat_shamir_l_hash(&setup.modulus, &witness, &self.state);
        let (l, l_hash_inc) = prime_digest(hasher, 16);

        let (q, r) = product.div_mod_floor(&l);

        let big_q = witness.modpow(&q, &setup.modulus);

        ElementsProof {
            base: witness,
            big_q,
            part: UnbatchableProofPart { l_hash_inc, r },
        }
    }

    /// Deserialize a name accumulator from bytes.
    ///
    /// The byte array needs to be 256 bytes (2048 bits).
    pub fn parse_bytes(byte_buf: impl AsRef<[u8]>) -> Result<Self> {
        let mut bytes = [0u8; 256];
        bytes.copy_from_slice(byte_buf.as_ref());
        Ok(Self::parse_slice(bytes))
    }

    /// Deserialize a name accumulator from bytes.
    pub fn parse_slice(slice: [u8; 256]) -> Self {
        let state = BigUint::from_bytes_be(&slice);
        Self {
            state,
            serialized_cache: OnceCell::from(slice),
        }
    }

    /// Serialize a name accumulator from bytes.
    pub fn into_bytes(self) -> [u8; 256] {
        let cache = self.serialized_cache;
        let state = self.state;
        cache
            .into_inner()
            .unwrap_or_else(|| to_bytes_helper(&state))
    }

    /// Serialize a name accumulator to bytes and return a reference.
    ///
    /// This call is memoized, serializing twice won't duplicate work.
    pub fn as_bytes(&self) -> &[u8; 256] {
        self.serialized_cache
            .get_or_init(|| to_bytes_helper(&self.state))
    }
}

fn poke_fiat_shamir_l_hash(
    modulus: &BigUint,
    base: &BigUint,
    commitment: &BigUint,
) -> impl Digest + Clone {
    let mut hasher = blake3::Hasher::new();
    hasher.update(&to_bytes_helper::<256>(modulus));
    hasher.update(&to_bytes_helper::<256>(base));
    hasher.update(&to_bytes_helper::<256>(commitment));
    hasher
}

impl AccumulatorSetup {
    /// Finishes a setup given a 2048-bit RSA modulus encoded in big-endian.
    ///
    /// Remaining work is safe and doesn't require a trusted environment.
    pub fn with_modulus(modulus_big_endian: &[u8; 256], rng: &mut impl CryptoRngCore) -> Self {
        let modulus = BigUint::from_bytes_be(modulus_big_endian);
        // The generator is just some random quadratic residue.
        let generator = rng
            .gen_biguint_below(&modulus)
            .modpow(&BigUint::from(2u8), &modulus);
        Self { modulus, generator }
    }

    /// Does a trusted setup in-memory and throws away the prime factors.
    /// This requires generating two 1024-bit primes, so it's fairly slow.
    ///
    /// The toxic waste generated during this operation is zeroized as soon
    /// as possible. Keep in mind that in theory it's still possible for a
    /// priviliged process to observe the memory and copy out the toxic waste
    /// before it's deleted.
    pub fn trusted(rng: &mut impl CryptoRngCore) -> Self {
        // This is a trusted setup.
        // The prime factors are "toxic waste", they need to be
        // disposed immediately.
        let mut p = rng.gen_prime(1024);
        let mut q = rng.gen_prime(1024);
        let modulus = &p * &q;
        // Make sure to delete toxic waste
        p.zeroize();
        q.zeroize();
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
    pub fn from_rsa_2048(rng: &mut impl CryptoRngCore) -> Self {
        let modulus = BigUint::from_str(
            "25195908475657893494027183240048398571429282126204032027777137836043662020707595556264018525880784406918290641249515082189298559149176184502808489120072844992687392807287776735971418347270261896375014971824691165077613379859095700097330459748808428401797429100642458691817195118746121515172654632282216869987549182422433637259085141865462043576798423387184774447920739934236584823824281198163815010674810451660377306056201619676256133844143603833904414952634432190114657544454178424020924616515723350778707749817125772467962926386356373289912154831438167899885040445364023527381951378636564391212010397122822120720357",
        ).unwrap();
        let generator = rng
            .gen_biguint_below(&modulus)
            .modpow(&BigUint::from(2u8), &modulus);
        Self { modulus, generator }
    }
}

impl NameSegment {
    /// Create a new, random name segment
    pub fn new(rng: &mut impl CryptoRngCore) -> Self {
        Self(rng.gen_prime(256))
    }

    /// Derive a name segment by finishing a hasher state
    /// (which is repeatedly re-hashed with a counter to find a prime).
    pub fn from_digest(digest: impl Digest + Clone) -> Self {
        Self(prime_digest(digest, 32).0)
    }

    /// Derive a name segment from a seed secret
    pub fn from_seed(seed: impl AsRef<[u8]>) -> Self {
        Self::from_digest(blake3::Hasher::new().chain_update(seed))
    }
}

impl BatchedProofPart {
    /// Create a new proof batcher.
    pub fn new() -> Self {
        Self {
            big_q_product: BigUint::one(),
        }
    }

    /// Add the batchable portion of a proof of elements
    /// for a certain name accumulator to this batch proof.
    pub fn add(&mut self, proof: &ElementsProof, setup: &AccumulatorSetup) {
        self.big_q_product *= &proof.big_q;
        self.big_q_product %= &setup.modulus;
    }
}

impl<'a> BatchedProofVerification<'a> {
    /// Create a new verifier
    pub fn new(setup: &'a AccumulatorSetup) -> Self {
        Self {
            bases_and_exponents: Vec::new(),
            setup,
        }
    }

    /// Add another relation to verify.
    ///
    /// This will return an error if the unbatchable
    /// proof part is invalid, either due to its "l" hash
    /// not being prime or the residue being out of range.
    ///
    /// This can happen if the base element or commitment
    /// don't fit this proof part.
    pub fn add(
        &mut self,
        base: &NameAccumulator,
        commitment: &NameAccumulator,
        proof_part: &UnbatchableProofPart,
    ) -> Result<()> {
        let hasher = poke_fiat_shamir_l_hash(&self.setup.modulus, &base.state, &commitment.state);
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

    /// Verify the whole relation of previously added bases to their commitments using
    /// the batched proof.
    ///
    /// Will return an error if verification fails.
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

impl Serialize for NameSegment {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serde_bytes::serialize(to_bytes_helper::<32>(&self.0).as_ref(), serializer)
    }
}

impl<'de> Deserialize<'de> for NameSegment {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let bytes: Vec<u8> = serde_bytes::deserialize(deserializer)?;
        Ok(NameSegment(BigUint::from_bytes_be(&bytes)))
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

impl<'de> Deserialize<'de> for UnbatchableProofPart {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let (l_hash_inc, r): (u32, &serde_bytes::Bytes) = Deserialize::deserialize(deserializer)?;
        let r = BigUint::from_bytes_be(r);
        Ok(Self { l_hash_inc, r })
    }
}

impl Serialize for UnbatchableProofPart {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let residue = to_bytes_helper::<16>(&self.r);
        let r = serde_bytes::Bytes::new(&residue);
        (self.l_hash_inc, r).serialize(serializer)
    }
}

impl PartialEq for Name {
    fn eq(&self, other: &Self) -> bool {
        let left = self
            .accumulated
            .get()
            .map(|x| &x.0)
            .or(self.segments.is_empty().then_some(&self.relative_to));
        let right = other
            .accumulated
            .get()
            .map(|x| &x.0)
            .or(other.segments.is_empty().then_some(&other.relative_to));

        if let (Some(left), Some(right)) = (left, right) {
            return left == right;
        }

        self.relative_to == other.relative_to && self.segments == other.segments
    }
}

impl PartialEq for NameAccumulator {
    fn eq(&self, other: &Self) -> bool {
        self.state == other.state
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

impl AsRef<[u8]> for NameAccumulator {
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl Default for BatchedProofPart {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Debug for NameSegment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("NameSegment")
            .field(&self.0.to_string())
            .finish()
    }
}

impl std::fmt::Debug for AccumulatorSetup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AccumulatorSetup")
            .field("modulus", &self.modulus.to_string())
            .field("generator", &self.generator.to_string())
            .finish()
    }
}

impl std::fmt::Debug for UnbatchableProofPart {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UnbatchableProofPart")
            .field("l_hash_inc", &self.l_hash_inc)
            .field("r", &self.r.to_string())
            .finish()
    }
}

impl std::fmt::Debug for ElementsProof {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ElementsProof")
            .field("base", &self.base.to_string())
            .field("big_q", &self.big_q.to_string())
            .field("part", &self.part)
            .finish()
    }
}

impl std::fmt::Debug for BatchedProofPart {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BatchedProofPart")
            .field("big_q_product", &self.big_q_product.to_string())
            .finish()
    }
}

//--------------------------------------------------------------------------------------------------
// Tests
//--------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use crate::{
        uint256_serde_be::to_bytes_helper, AccumulatorSetup, BatchedProofPart,
        BatchedProofVerification, Name, NameAccumulator, NameSegment,
    };
    use anyhow::Result;
    use libipld::{
        cbor::DagCborCodec,
        prelude::{Decode, Encode},
        Ipld,
    };
    use num_bigint_dig::BigUint;
    use proptest::{prop_assert, prop_assert_eq};
    use rand::{thread_rng, SeedableRng};
    use rand_chacha::ChaCha12Rng;
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

    #[test]
    fn name_batched_proof_example() -> Result<()> {
        let rng = &mut thread_rng();
        let setup = &AccumulatorSetup::from_rsa_2048(rng);
        let mut name_note = Name::empty(setup);
        let mut name_image = Name::empty(setup);

        let root_dir_segment = NameSegment::new(rng);
        let docs_dir_segment = NameSegment::new(rng);
        let pics_dir_segment = NameSegment::new(rng);
        let note_file_segment = NameSegment::new(rng);
        let image_file_segment = NameSegment::new(rng);

        name_note.add_segments([
            root_dir_segment.clone(),
            docs_dir_segment,
            note_file_segment,
        ]);
        name_image.add_segments([root_dir_segment, pics_dir_segment, image_file_segment]);

        let (accum_note, proof_note) = name_note.as_proven_accumulator(setup);
        let (accum_image, proof_image) = name_image.as_proven_accumulator(setup);

        let mut batched_proof = BatchedProofPart::new();
        batched_proof.add(proof_note, setup);
        batched_proof.add(proof_image, setup);

        let name_base = Name::empty(setup).as_accumulator(setup).clone();
        let mut verification = BatchedProofVerification::new(setup);
        verification.add(&name_base, accum_note, &proof_note.part)?;
        verification.add(&name_base, accum_image, &proof_image.part)?;
        verification.verify(&batched_proof)?;

        Ok(())
    }

    #[proptest(cases = 32)]
    fn batch_proofs(do_batch_step: [bool; 4], do_verify_step: [bool; 4], seed: u64) {
        let rng = &mut ChaCha12Rng::seed_from_u64(seed);
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
            batched_proof.add(&proof_a_one, setup);
        }
        if do_batch_step[1] {
            batched_proof.add(&proof_a_two, setup);
        }
        if do_batch_step[2] {
            batched_proof.add(&proof_b_one, setup);
        }
        if do_batch_step[3] {
            batched_proof.add(&proof_b_two, setup);
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

    #[proptest]
    fn padded_biguint_encoding_roundtrips(num: u64) {
        let num = BigUint::from(num);
        let bytes = to_bytes_helper::<8>(&num);
        let parsed = BigUint::from_bytes_be(bytes.as_ref());
        prop_assert_eq!(parsed, num);
    }
}
