use super::{PrivateNodeHeader, PrivateRefSerializable, TemporalKey, KEY_BYTE_SIZE};
use crate::error::{AesError, FsError};
use aes_kw::KekAes256;
use anyhow::Result;
use libipld::Cid;
use serde::{de::Error as DeError, ser::Error as SerError, Deserialize, Serialize};
use std::fmt::Debug;
use wnfs_common::HashOutput;
use wnfs_nameaccumulator::{AccumulatorSetup, Name, NameSegment};

//--------------------------------------------------------------------------------------------------
// Type Definitions
//--------------------------------------------------------------------------------------------------

/// PrivateRef holds the information to fetch a specific node from the private forest and decrypt it.
///
/// It also includes required key material to decrypt/encrypt any future revisions of the node it points to.
#[derive(Clone, PartialEq, Eq)]
pub struct PrivateRef {
    /// Sha3-256 hash of the revision name. Used as the label for identifying revisions of PrivateNodes in the PrivateForest.
    pub revision_name_hash: HashOutput,
    /// Skip-ratchet-derived key. Gives read access to the revision pointed to and any newer revisions.
    pub temporal_key: TemporalKey,
    /// CID that identifies the exact value in the multivalue.
    pub content_cid: Cid,
}

/// A pointer to a specific revision in the private forest
/// together with the TemporalKey to decrypt any of these
/// revisions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RevisionRef {
    /// Sha3-256 hash of the revision name. Used as the label for private nodes in the private forest.
    pub revision_name_hash: HashOutput,
    /// Skip-ratchet-derived key. Gives read access to the revision pointed to and any newer revisions.
    pub temporal_key: TemporalKey,
}

//--------------------------------------------------------------------------------------------------
// Implementations
//--------------------------------------------------------------------------------------------------

impl PrivateRef {
    /// Creates a PrivateRef from provided saturated name and temporal key.
    ///
    /// # Examples
    ///
    /// ```
    /// use wnfs::{private::{PrivateRef, TemporalKey}, private::AesKey};
    /// use rand::{thread_rng, Rng};
    ///
    /// let content_cid = Default::default();
    /// let rng = &mut thread_rng();
    /// let private_ref = PrivateRef::with_temporal_key(
    ///     rng.gen::<[u8; 32]>(),
    ///     TemporalKey::from(AesKey::new(rng.gen::<[u8; 32]>())),
    ///     content_cid,
    /// );
    ///
    /// println!("Private ref: {:?}", private_ref);
    /// ```
    pub fn with_temporal_key(
        revision_name_hash: HashOutput,
        temporal_key: TemporalKey,
        content_cid: Cid,
    ) -> Self {
        Self {
            revision_name_hash,
            temporal_key,
            content_cid,
        }
    }

    pub(crate) fn to_serializable(
        &self,
        parent_temporal_key: &TemporalKey,
    ) -> Result<PrivateRefSerializable> {
        let snapshot_key = self.temporal_key.derive_snapshot_key();
        // encrypt ratchet key
        let temporal_key_as_kek = KekAes256::from(parent_temporal_key.0.clone().bytes());
        let temporal_key_wrapped = temporal_key_as_kek
            .wrap_with_padding_vec(self.temporal_key.0.as_bytes())
            .map_err(|e| AesError::UnableToEncrypt(format!("{e}")))?;

        Ok(PrivateRefSerializable {
            revision_name_hash: self.revision_name_hash,
            snapshot_key,
            temporal_key: temporal_key_wrapped,
            content_cid: self.content_cid,
        })
    }

    pub(crate) fn from_serializable(
        private_ref: PrivateRefSerializable,
        parent_temporal_key: &TemporalKey,
    ) -> Result<Self> {
        // TODO: Move key wrapping & unwrapping logic to impl TemporalKey
        let temporal_key_as_kek = KekAes256::from(parent_temporal_key.0.clone().bytes());

        let temporal_key_raw: [u8; KEY_BYTE_SIZE] = temporal_key_as_kek
            .unwrap_with_padding_vec(&private_ref.temporal_key)
            .map_err(|e| AesError::UnableToDecrypt(format!("{e}")))?
            .try_into()
            .map_err(|e: Vec<u8>| {
                FsError::InvalidDeserialization(format!(
                    "Expected 32 bytes for ratchet key, but got {}",
                    e.len()
                ))
            })?;

        let temporal_key = temporal_key_raw.into();

        Ok(Self {
            revision_name_hash: private_ref.revision_name_hash,
            temporal_key,
            content_cid: private_ref.content_cid,
        })
    }

    pub fn serialize<S>(&self, serializer: S, temporal_key: &TemporalKey) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.to_serializable(temporal_key)
            .map_err(SerError::custom)?
            .serialize(serializer)
    }

    pub fn deserialize<'de, D>(
        deserializer: D,
        temporal_key: &TemporalKey,
    ) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let private_ref = PrivateRefSerializable::deserialize(deserializer)?;
        PrivateRef::from_serializable(private_ref, temporal_key).map_err(DeError::custom)
    }

    /// Returns a revision ref that refers to all other multivalues
    /// next to this private ref's value.
    pub fn as_revision_ref(self) -> RevisionRef {
        RevisionRef {
            revision_name_hash: self.revision_name_hash,
            temporal_key: self.temporal_key,
        }
    }
}

impl Debug for PrivateRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut sat_name_hash_str = String::from("0x");
        for byte in self.revision_name_hash {
            sat_name_hash_str.push_str(&format!("{byte:02X}"));
        }

        f.debug_struct("PrivateRef")
            .field("revision_name_hash", &sat_name_hash_str)
            .field("temporal_key", &self.temporal_key.0)
            .field("content_cid", &self.content_cid)
            .finish()
    }
}

impl RevisionRef {
    /// Creates a RevisionRef from provided namefilter, ratchet seed and inumber.
    ///
    /// # Examples
    ///
    /// ```
    /// use wnfs::private::RevisionRef;
    /// use wnfs_nameaccumulator::{AccumulatorSetup, Name, NameSegment};
    /// use rand::{thread_rng, Rng};
    ///
    /// let rng = &mut thread_rng();
    /// // Usually stored in the PrivateForest:
    /// let setup = &AccumulatorSetup::from_rsa_2048(rng);
    /// let revision_ref = RevisionRef::with_seed(
    ///     &Name::empty(setup),
    ///     rng.gen::<[u8; 32]>(),
    ///     NameSegment::new(rng),
    ///     setup,
    /// );
    ///
    /// println!("Private ref: {:?}", revision_ref);
    /// ```
    pub fn with_seed(
        name: &Name,
        ratchet_seed: HashOutput,
        inumber: NameSegment,
        setup: &AccumulatorSetup,
    ) -> Self {
        PrivateNodeHeader::with_seed(name, ratchet_seed, inumber).derive_revision_ref(setup)
    }

    /// Turns a reivison ref into a more specific pointer, a private ref.
    ///
    /// The revision ref refers to a whole multivalue that may or may not exist
    /// or may refer to multiple private nodes.
    ///
    /// The resulting private ref refers to the given CID in the multivalue.
    pub fn as_private_ref(self, content_cid: Cid) -> PrivateRef {
        PrivateRef {
            revision_name_hash: self.revision_name_hash,
            temporal_key: self.temporal_key,
            content_cid,
        }
    }
}

//--------------------------------------------------------------------------------------------------
// Tests
//--------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::RevisionRef;
    use crate::private::{
        forest::{hamt::HamtForest, traits::PrivateForest},
        PrivateDirectory, PrivateNode,
    };
    use chrono::Utc;
    use futures::StreamExt;
    use proptest::test_runner::{RngAlgorithm, TestRng};
    use std::rc::Rc;
    use wnfs_common::{utils, MemoryBlockStore};
    use wnfs_nameaccumulator::NameSegment;

    #[async_std::test]
    async fn can_create_revisionref_deterministically_with_user_provided_seeds() {
        let rng = &mut TestRng::deterministic_rng(RngAlgorithm::ChaCha);
        let store = &mut MemoryBlockStore::default();
        let forest = &mut Rc::new(HamtForest::new_rsa_2048(rng));
        let ratchet_seed = utils::get_random_bytes::<32>(rng);
        let inumber = NameSegment::new(rng);

        let dir = PrivateNode::from(PrivateDirectory::with_seed(
            &forest.empty_name(),
            Utc::now(),
            ratchet_seed,
            inumber.clone(),
        ));

        // Throwing away the private ref
        dir.store(forest, store, rng).await.unwrap();

        // Creating deterministic revision ref and retrieve the content.
        let setup = forest.get_accumulator_setup();
        let revision_ref =
            RevisionRef::with_seed(&forest.empty_name(), ratchet_seed, inumber, setup);
        let retrieved_node = forest
            .get_multivalue(&revision_ref, store, None)
            .next()
            .await
            .unwrap()
            .unwrap();

        assert_eq!(retrieved_node, dir);
    }
}
