use super::TemporalKey;
use crate::{error::FsError, private::RevisionRef};
use anyhow::Result;
use libipld::{Cid, IpldCodec};
use rand_core::{CryptoRngCore, RngCore};
use serde::{Deserialize, Serialize};
use sha3::{Digest, Sha3_256};
use skip_ratchet::Ratchet;
use std::fmt::Debug;
use wnfs_common::{dagcbor, utils, BlockStore, HashOutput, HASH_BYTE_SIZE};
use wnfs_hamt::Hasher;
use wnfs_nameaccumulator::{AccumulatorSetup, Name, NameAccumulator, NameSegment};

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
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrivateNodeHeader {
    /// A unique identifier of the node.
    pub(crate) inumber: INumber,
    /// Used both for versioning and deriving keys for that enforces privacy.
    pub(crate) ratchet: Ratchet,
    /// Stores the name of this node for easier lookup.
    pub(crate) name: Name,
}

#[derive(Serialize, Deserialize)]
pub struct PrivateNodeHeaderSerializable {
    /// A unique identifier of the node.
    inumber: INumber,
    /// Used both for versioning and deriving keys for that enforces privacy.
    ratchet: Ratchet,
    /// Stores the name of this node for easier lookup.
    name: NameAccumulator,
}

//--------------------------------------------------------------------------------------------------
// Implementations
//--------------------------------------------------------------------------------------------------

impl PrivateNodeHeader {
    /// Creates a new PrivateNodeHeader.
    pub(crate) fn new(parent_name: &Name, rng: &mut impl RngCore) -> Self {
        let inumber = NameSegment::new(rng);
        let ratchet_seed = utils::get_random_bytes::<HASH_BYTE_SIZE>(rng);
        Self::with_seed(parent_name, ratchet_seed, inumber)
    }

    /// Creates a new PrivateNodeHeader with provided seed.
    pub(crate) fn with_seed(
        parent_name: &Name,
        ratchet_seed: HashOutput,
        inumber: INumber,
    ) -> Self {
        // A keyed hash to use for determining the seed increments without
        // leaking info about the seed itself (from the ratchet state).
        let seed_hash = Sha3_256::new()
            .chain_update("WNFS ratchet increments")
            .chain_update(ratchet_seed)
            .finalize();
        Self {
            name: parent_name.with_segments_added(Some(inumber.clone())),
            ratchet: Ratchet::from_seed(&ratchet_seed, seed_hash[0], seed_hash[1]),
            inumber,
        }
    }

    /// Advances the ratchet.
    pub(crate) fn advance_ratchet(&mut self) {
        self.ratchet.inc();
    }

    /// Updates the bare name of the node.
    pub(crate) fn update_bare_name(&mut self, parent_name: &Name) {
        self.name = parent_name.clone();
        self.name.add_segments(Some(self.inumber.clone()));
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
    pub fn get_name(&self) -> Name {
        self.name
            .with_segments_added(Some(self.derive_temporal_key().to_revision_segment()))
    }

    /// TODO(matheus23)
    pub fn get_name_hash(&self, setup: &AccumulatorSetup) -> HashOutput {
        Sha3_256::hash(&self.get_name().into_accumulator(setup))
    }

    /// Encrypts this private node header in an block, then stores that in the given
    /// BlockStore and returns its CID.
    pub async fn store(
        &self,
        store: &mut impl BlockStore,
        setup: &AccumulatorSetup,
    ) -> Result<Cid> {
        let temporal_key = self.derive_temporal_key();
        let cbor_bytes = dagcbor::encode(&self.to_serializable(setup))?;
        let ciphertext = temporal_key.key_wrap_encrypt(&cbor_bytes)?;
        store.put_block(ciphertext, IpldCodec::Raw).await
    }

    pub(crate) fn to_serializable(
        &self,
        setup: &AccumulatorSetup,
    ) -> PrivateNodeHeaderSerializable {
        PrivateNodeHeaderSerializable {
            inumber: self.inumber.clone(),
            ratchet: self.ratchet.clone(),
            name: self.name.as_accumulator(setup).clone(),
        }
    }

    pub(crate) fn from_serializable(serializable: PrivateNodeHeaderSerializable) -> Self {
        Self {
            inumber: serializable.inumber,
            ratchet: serializable.ratchet,
            name: Name::new(serializable.name, []),
        }
    }

    /// Loads a private node header from a given CID linking to the ciphertext block
    /// to be decrypted with given key.
    pub(crate) async fn load(
        cid: &Cid,
        temporal_key: &TemporalKey,
        store: &impl BlockStore,
        mounted_relative_to: Option<Name>,
        setup: &AccumulatorSetup,
    ) -> Result<Self> {
        let ciphertext = store.get_block(cid).await?;
        let cbor_bytes = temporal_key.key_wrap_decrypt(&ciphertext)?;
        let decoded = dagcbor::decode::<PrivateNodeHeaderSerializable>(&cbor_bytes)?;
        let mut header = Self::from_serializable(decoded);
        if let Some(parent_name) = mounted_relative_to {
            let name = parent_name.with_segments_added([header.inumber.clone()]);
            let mounted_acc = name.as_accumulator(setup);
            let name_acc = header.name.as_accumulator(setup);
            if mounted_acc != name_acc {
                return Err(FsError::MountPointAndDeserializedNameMismatch(
                    mounted_acc.as_bytes().clone(),
                    name_acc.as_bytes().clone(),
                )
                .into());
            }
            header.name = name;
        }
        Ok(header)
    }
}
