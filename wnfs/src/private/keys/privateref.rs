use super::KEY_BYTE_SIZE;
use crate::{
    error::{AesError, FsError},
    private::{PrivateRefSerializable, TemporalKey},
};
use aes_kw::KekAes256;
use anyhow::Result;
use libipld_core::cid::Cid;
use serde::{de::Error as DeError, ser::Error as SerError, Deserialize, Serialize};
use std::fmt::Debug;
use wnfs_common::HashOutput;

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

// TODO(appcypher): Remove RevisionRef.
/// A pointer to a specific revision in the private forest
/// together with the TemporalKey to decrypt any of these
/// revisions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct RevisionRef {
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
    pub(crate) fn with_temporal_key(
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

    #[allow(unused)]
    pub(crate) fn serialize<S>(
        &self,
        serializer: S,
        temporal_key: &TemporalKey,
    ) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.to_serializable(temporal_key)
            .map_err(SerError::custom)?
            .serialize(serializer)
    }

    #[allow(unused)]
    pub(crate) fn deserialize<'de, D>(
        deserializer: D,
        temporal_key: &TemporalKey,
    ) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let private_ref = PrivateRefSerializable::deserialize(deserializer)?;
        PrivateRef::from_serializable(private_ref, temporal_key).map_err(DeError::custom)
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
    /// Turns a reivison ref into a more specific pointer, a private ref.
    ///
    /// The revision ref refers to a whole multivalue that may or may not exist
    /// or may refer to multiple private nodes.
    ///
    /// The resulting private ref refers to the given CID in the multivalue.
    pub(crate) fn into_private_ref(self, content_cid: Cid) -> PrivateRef {
        PrivateRef {
            revision_name_hash: self.revision_name_hash,
            temporal_key: self.temporal_key,
            content_cid,
        }
    }
}
