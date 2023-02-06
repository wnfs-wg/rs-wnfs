use std::fmt::Debug;

use super::{PrivateNodeHeader, SnapshotKey, TemporalKey, KEY_BYTE_SIZE};
use crate::{AesError, FsError, HashOutput, Namefilter};
use aes_kw::KekAes256;
use anyhow::Result;
use libipld::Cid;
use serde::{de::Error as DeError, ser::Error as SerError, Deserialize, Serialize};

//--------------------------------------------------------------------------------------------------
// Type Definitions
//--------------------------------------------------------------------------------------------------

/// PrivateRef holds the information to fetch a specific node from the private forest and decrypt it.
///
/// It also includes required key material to decrypt/encrypt any future revisions of the node it points to.
#[derive(Clone, PartialEq, Eq)]
pub struct PrivateRef {
    /// Sha3-256 hash of saturated namefilter.
    pub(crate) saturated_name_hash: HashOutput,
    /// Skip-ratchet-derived key.
    pub(crate) temporal_key: TemporalKey,
    /// CID that identifies the exact value in the multivalue
    pub(crate) content_cid: Cid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct PrivateRefSerializable {
    #[serde(rename = "name")]
    pub(crate) saturated_name_hash: HashOutput,
    #[serde(rename = "snapshotKey")]
    pub(crate) snapshot_key: SnapshotKey,
    #[serde(rename = "temporalKey")]
    pub(crate) temporal_key: Vec<u8>,
    #[serde(rename = "contentCid")]
    pub(crate) content_cid: Cid,
}

/// A pointer to a specific revision in the private forest
/// together with the TemporalKey to decrypt any of these
/// revisions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RevisionRef {
    pub(crate) saturated_name_hash: HashOutput,
    pub(crate) temporal_key: TemporalKey,
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
        saturated_name_hash: HashOutput,
        temporal_key: TemporalKey,
        content_cid: Cid,
    ) -> Self {
        Self {
            saturated_name_hash,
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
            saturated_name_hash: self.saturated_name_hash,
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
            saturated_name_hash: private_ref.saturated_name_hash,
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
            saturated_name_hash: self.saturated_name_hash,
            temporal_key: self.temporal_key,
        }
    }

    /// Returns the label used for identifying the revision in the PrivateForest.
    pub fn get_saturated_name_hash(&self) -> &HashOutput {
        &self.saturated_name_hash
    }
}

impl Debug for PrivateRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut sat_name_hash_str = String::from("0x");
        for byte in self.saturated_name_hash {
            sat_name_hash_str.push_str(&format!("{byte:02X}"));
        }

        f.debug_struct("PrivateRef")
            .field("saturated_name_hash", &sat_name_hash_str)
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
    /// use wnfs::{private::RevisionRef, Namefilter};
    /// use rand::{thread_rng, Rng};
    ///
    /// let rng = &mut thread_rng();
    /// let revision_ref = RevisionRef::with_seed(
    ///     Namefilter::default(),
    ///     rng.gen::<[u8; 32]>(),
    ///     rng.gen::<[u8; 32]>(),
    /// );
    ///
    /// println!("Private ref: {:?}", revision_ref);
    /// ```
    pub fn with_seed(name: Namefilter, ratchet_seed: HashOutput, inumber: HashOutput) -> Self {
        PrivateNodeHeader::with_seed(name, ratchet_seed, inumber).derive_revision_ref()
    }

    /// Turns a reivison ref into a more specific pointer, a private ref.
    ///
    /// The revision ref refers to a whole multivalue that may or may not exist
    /// or may refer to multiple private nodes.
    ///
    /// The resulting private ref refers to the given CID in the multivalue.
    pub fn as_private_ref(self, content_cid: Cid) -> PrivateRef {
        PrivateRef {
            saturated_name_hash: self.saturated_name_hash,
            temporal_key: self.temporal_key,
            content_cid,
        }
    }

    /// Returns the label used for identifying the revision in the PrivateForest.
    pub fn get_saturated_name_hash(&self) -> &HashOutput {
        &self.saturated_name_hash
    }
}

//--------------------------------------------------------------------------------------------------
// Tests
//--------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use crate::{
        utils::{self, test_setup},
        PrivateDirectory, PrivateNode,
    };
    use chrono::Utc;
    use futures::StreamExt;

    use super::RevisionRef;

    #[async_std::test]
    async fn can_create_revisionref_deterministically_with_user_provided_seeds() {
        let (ref mut forest, store, rng) = test_setup::init!(forest, mut store, mut rng);
        let ratchet_seed = utils::get_random_bytes::<32>(rng);
        let inumber = utils::get_random_bytes::<32>(rng);

        let dir = PrivateNode::from(PrivateDirectory::with_seed(
            Default::default(),
            Utc::now(),
            ratchet_seed,
            inumber,
        ));

        // Throwing away the private ref
        forest.put(&dir, store, rng).await.unwrap();

        // Creating deterministic revision ref and retrieve the content.
        let revision_ref = RevisionRef::with_seed(Default::default(), ratchet_seed, inumber);
        let retrieved_node = forest
            .get_multivalue(&revision_ref, store)
            .next()
            .await
            .unwrap()
            .unwrap();

        assert_eq!(retrieved_node, dir);
    }
}
