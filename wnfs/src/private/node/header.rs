use super::TemporalKey;
use crate::private::RevisionRef;
use anyhow::Result;
use libipld::{Cid, IpldCodec};
use rand_core::{CryptoRngCore, RngCore};
use serde::{Deserialize, Serialize};
use sha3::{Digest, Sha3_256};
use skip_ratchet::Ratchet;
use std::fmt::Debug;
use wnfs_common::{dagcbor, utils, BlockStore, HashOutput, HASH_BYTE_SIZE};
use wnfs_hamt::Hasher;
use wnfs_nameaccumulator::{AccumulatorSetup, NameAccumulator, NameSegment};

//--------------------------------------------------------------------------------------------------
// Type Definitions
//--------------------------------------------------------------------------------------------------

pub type INumber = NameSegment;

/// This is the header of a private node. It contains secret information about the node which includes
/// the inumber, the ratchet, and the namefilter.
///
/// # Examples
///
/// ```
/// use wnfs::{
///     private::PrivateFile,
///     namefilter::Namefilter,
///     traits::Id
/// };
/// use chrono::Utc;
/// use rand::thread_rng;
///
/// let rng = &mut thread_rng();
/// let file = PrivateFile::new(
///     Namefilter::default(),
///     Utc::now(),
///     rng,
/// );
///
/// println!("Header: {:#?}", file.header);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PrivateNodeHeader {
    /// A unique identifier of the node.
    pub(crate) inumber: INumber,
    /// Used both for versioning and deriving keys for that enforces privacy.
    pub(crate) ratchet: Ratchet,
    /// Stores the name of this node for easier lookup.
    pub(crate) name: NameAccumulator,
}

//--------------------------------------------------------------------------------------------------
// Implementations
//--------------------------------------------------------------------------------------------------

impl PrivateNodeHeader {
    /// Creates a new PrivateNodeHeader.
    pub(crate) fn new(
        parent_name: &NameAccumulator,
        setup: &AccumulatorSetup,
        rng: &mut impl RngCore,
    ) -> Self {
        let inumber = NameSegment::new(rng);
        let ratchet_seed = utils::get_random_bytes::<HASH_BYTE_SIZE>(rng);
        Self::with_seed(parent_name, ratchet_seed, inumber, setup)
    }

    /// Creates a new PrivateNodeHeader with provided seed.
    pub(crate) fn with_seed(
        parent_name: &NameAccumulator,
        ratchet_seed: HashOutput,
        inumber: INumber,
        setup: &AccumulatorSetup,
    ) -> Self {
        // A keyed hash to use for determining the seed increments without
        // leaking info about the seed itself (from the ratchet state).
        let seed_hash = Sha3_256::new()
            .chain_update("WNFS ratchet increments")
            .chain_update(&ratchet_seed)
            .finalize();
        Self {
            name: {
                let mut name = parent_name.clone();
                name.add(&inumber, setup);
                name
            },
            ratchet: Ratchet::from_seed(&ratchet_seed, seed_hash[0], seed_hash[1]),
            inumber,
        }
    }

    /// Advances the ratchet.
    pub(crate) fn advance_ratchet(&mut self) {
        self.ratchet.inc();
    }

    /// Updates the bare name of the node.
    pub(crate) fn update_bare_name(
        &mut self,
        parent_name: &NameAccumulator,
        setup: &AccumulatorSetup,
    ) {
        self.name = parent_name.clone();
        self.name.add(&self.inumber, setup)
    }

    /// Resets the ratchet.
    pub(crate) fn reset_ratchet(&mut self, rng: &mut impl CryptoRngCore) {
        self.ratchet = Ratchet::from_rng(rng)
    }

    /// Derives the revision ref of the current header.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::rc::Rc;
    /// use wnfs::{
    ///     private::PrivateFile,
    ///     namefilter::Namefilter,
    ///     traits::Id
    /// };
    /// use chrono::Utc;
    /// use rand::thread_rng;
    ///
    /// let rng = &mut thread_rng();
    /// let file = Rc::new(PrivateFile::new(
    ///     Namefilter::default(),
    ///     Utc::now(),
    ///     rng,
    /// ));
    /// let revision_ref = file.header.derive_revision_ref();
    ///
    /// println!("Private ref: {:?}", revision_ref);
    /// ```
    pub fn derive_revision_ref(&self, setup: &AccumulatorSetup) -> RevisionRef {
        let temporal_key = self.derive_temporal_key();
        let saturated_name_hash = self.get_name_hash(setup);

        RevisionRef {
            saturated_name_hash,
            temporal_key,
        }
    }

    /// Derives the temporal key.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::rc::Rc;
    /// use wnfs::{
    ///     private::PrivateFile,
    ///     namefilter::Namefilter,
    ///     traits::Id
    /// };
    /// use chrono::Utc;
    /// use rand::thread_rng;
    ///
    /// let rng = &mut thread_rng();
    /// let file = Rc::new(PrivateFile::new(
    ///     Namefilter::default(),
    ///     Utc::now(),
    ///     rng,
    /// ));
    /// let temporal_key = file.header.derive_temporal_key();
    ///
    /// println!("Temporal Key: {:?}", temporal_key);
    /// ```
    #[inline]
    pub fn derive_temporal_key(&self) -> TemporalKey {
        TemporalKey::from(&self.ratchet)
    }

    /// Derives the key that's put into the accumulator.
    pub(crate) fn derive_revision_segment(&self, setup: &AccumulatorSetup) -> NameSegment {
        let mut hasher = sha3::Sha3_256::new();
        hasher.update(&"Revision name acc element");
        hasher.update(&self.derive_temporal_key().0.as_bytes());
        NameSegment::from_digest(hasher)
    }

    /// Gets the saturated namefilter for this node.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::rc::Rc;
    /// use wnfs::{
    ///     private::{PrivateFile, AesKey},
    ///     namefilter::Namefilter
    /// };
    /// use chrono::Utc;
    /// use rand::thread_rng;
    ///
    /// let rng = &mut thread_rng();
    /// let file = Rc::new(PrivateFile::new(
    ///     Namefilter::default(),
    ///     Utc::now(),
    ///     rng,
    /// ));
    /// let saturated_name = file.header.get_saturated_name();
    ///
    /// println!("Saturated name: {:?}", saturated_name);
    /// ```
    pub fn get_name(&self, setup: &AccumulatorSetup) -> NameAccumulator {
        let mut name = self.name.clone();
        name.add(&self.derive_revision_segment(setup), setup);
        name
    }

    /// TODO(matheus23)
    pub fn get_name_hash(&self, setup: &AccumulatorSetup) -> HashOutput {
        Sha3_256::hash(self.get_name(setup).as_ref()).into()
    }

    /// Encrypts this private node header in an block, then stores that in the given
    /// BlockStore and returns its CID.
    pub async fn store(&self, store: &mut impl BlockStore) -> Result<Cid> {
        let temporal_key = self.derive_temporal_key();
        let cbor_bytes = dagcbor::encode(self)?;
        let ciphertext = temporal_key.key_wrap_encrypt(&cbor_bytes)?;
        store.put_block(ciphertext, IpldCodec::Raw).await
    }

    /// Loads a private node header from a given CID linking to the ciphertext block
    /// to be decrypted with given key.
    pub(crate) async fn load(
        cid: &Cid,
        temporal_key: &TemporalKey,
        store: &impl BlockStore,
    ) -> Result<PrivateNodeHeader> {
        let ciphertext = store.get_block(cid).await?;
        let cbor_bytes = temporal_key.key_wrap_decrypt(&ciphertext)?;
        dagcbor::decode(&cbor_bytes)
    }
}
