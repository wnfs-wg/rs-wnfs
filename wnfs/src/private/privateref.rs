use std::fmt::Debug;

use super::{ContentKey, PrivateNodeHeader, RevisionKey, KEY_BYTE_SIZE};
use crate::{AesError, FsError, HashOutput, Namefilter};
use aes_kw::KekAes256;
use anyhow::Result;
use libipld::Cid;
use serde::{de::Error as DeError, ser::Error as SerError, Deserialize, Serialize};

//--------------------------------------------------------------------------------------------------
// Type Definitions
//--------------------------------------------------------------------------------------------------

/// PrivateRef holds the information to fetch associated node from a private forest and decrypt it if it is present.
///
/// It also includes required key material to decrypt/encrypt the revision it points to
/// as well as any future revisions.
#[derive(Clone, PartialEq, Eq)]
pub struct PrivateRef {
    /// Sha3-256 hash of saturated namefilter.
    pub(crate) saturated_name_hash: HashOutput,
    /// Skip-ratchet-derived key.
    pub(crate) revision_key: RevisionKey,
    /// TODO(matheus23) docs
    pub(crate) content_cid: Cid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct PrivateRefSerializable {
    #[serde(rename = "name")]
    pub(crate) saturated_name_hash: HashOutput,
    #[serde(rename = "contentKey")]
    pub(crate) content_key: ContentKey,
    #[serde(rename = "revisionKey")]
    pub(crate) revision_key: Vec<u8>,
    #[serde(rename = "contentCid")]
    pub(crate) content_cid: Cid,
}

/// TODO(matheus23) docs
/// This is outside spec: Just a pointer to a revision without
/// disambiguating the actual content block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RevisionRef {
    pub saturated_name_hash: HashOutput,
    pub revision_key: RevisionKey,
}

//--------------------------------------------------------------------------------------------------
// Implementations
//--------------------------------------------------------------------------------------------------

impl PrivateRef {
    /// Creates a PrivateRef from provided saturated name and revision key.
    ///
    /// # Examples
    ///
    /// ```
    /// use wnfs::{private::{PrivateRef, RevisionKey}, private::AesKey};
    /// use rand::{thread_rng, Rng};
    ///
    /// let rng = &mut thread_rng();
    /// let private_ref = PrivateRef::with_revision_key(
    ///     rng.gen::<[u8; 32]>(),
    ///     RevisionKey::from(AesKey::new(rng.gen::<[u8; 32]>())),
    /// );
    ///
    /// println!("Private ref: {:?}", private_ref);
    /// ```
    pub fn with_revision_key(
        saturated_name_hash: HashOutput,
        revision_key: RevisionKey,
        content_cid: Cid,
    ) -> Self {
        Self {
            saturated_name_hash,
            revision_key,
            content_cid,
        }
    }

    /// Creates a PrivateRef from provided namefilter, ratchet seed and inumber.
    ///
    /// # Examples
    ///
    /// ```
    /// use wnfs::{private::PrivateRef, Namefilter};
    /// use rand::{thread_rng, Rng};
    ///
    /// let rng = &mut thread_rng();
    /// let private_ref = PrivateRef::with_seed(
    ///     Namefilter::default(),
    ///     rng.gen::<[u8; 32]>(),
    ///     rng.gen::<[u8; 32]>(),
    /// );
    ///
    /// println!("Private ref: {:?}", private_ref);
    /// ```
    pub fn with_seed(
        name: Namefilter,
        ratchet_seed: HashOutput,
        inumber: HashOutput,
        content_cid: Cid,
    ) -> Self {
        PrivateNodeHeader::with_seed(name, ratchet_seed, inumber).derive_private_ref(content_cid)
    }

    pub(crate) fn to_serializable(
        &self,
        parent_revision_key: &RevisionKey,
    ) -> Result<PrivateRefSerializable> {
        let content_key = self.revision_key.derive_content_key();
        // encrypt ratchet key
        let revision_key_as_kek = KekAes256::from(parent_revision_key.0.clone().bytes());
        let revision_key_wrapped = revision_key_as_kek
            .wrap_with_padding_vec(self.revision_key.0.as_bytes())
            .map_err(|e| AesError::UnableToEncrypt(format!("{e}")))?;

        Ok(PrivateRefSerializable {
            saturated_name_hash: self.saturated_name_hash,
            content_key,
            revision_key: revision_key_wrapped,
            content_cid: self.content_cid,
        })
    }

    pub(crate) fn from_serializable(
        private_ref: PrivateRefSerializable,
        parent_revision_key: &RevisionKey,
    ) -> Result<Self> {
        // TODO: Move key wrapping & unwrapping logic to impl RevisionKey
        let revision_key_as_kek = KekAes256::from(parent_revision_key.0.clone().bytes());

        let revision_key_raw: [u8; KEY_BYTE_SIZE] = revision_key_as_kek
            .unwrap_with_padding_vec(&private_ref.revision_key)
            .map_err(|e| AesError::UnableToDecrypt(format!("{e}")))?
            .try_into()
            .map_err(|e: Vec<u8>| {
                FsError::InvalidDeserialization(format!(
                    "Expected 32 bytes for ratchet key, but got {}",
                    e.len()
                ))
            })?;

        let revision_key = revision_key_raw.into();

        Ok(Self {
            saturated_name_hash: private_ref.saturated_name_hash,
            revision_key,
            content_cid: private_ref.content_cid,
        })
    }

    pub fn serialize<S>(&self, serializer: S, revision_key: &RevisionKey) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.to_serializable(revision_key)
            .map_err(SerError::custom)?
            .serialize(serializer)
    }

    pub fn deserialize<'de, D>(
        deserializer: D,
        revision_key: &RevisionKey,
    ) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let private_ref = PrivateRefSerializable::deserialize(deserializer)?;
        PrivateRef::from_serializable(private_ref, revision_key).map_err(DeError::custom)
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
            .field("revision_key", &self.revision_key.0)
            .field("content_cid", &self.content_cid)
            .finish()
    }
}

impl RevisionRef {
    /// Creates a PrivateRef from provided namefilter, ratchet seed and inumber.
    ///
    /// # Examples
    ///
    /// ```
    /// use wnfs::{private::PrivateRef, Namefilter};
    /// use rand::{thread_rng, Rng};
    ///
    /// let rng = &mut thread_rng();
    /// let private_ref = PrivateRef::with_seed(
    ///     Namefilter::default(),
    ///     rng.gen::<[u8; 32]>(),
    ///     rng.gen::<[u8; 32]>(),
    /// );
    ///
    /// println!("Private ref: {:?}", private_ref);
    /// ```
    pub fn with_seed(name: Namefilter, ratchet_seed: HashOutput, inumber: HashOutput) -> Self {
        PrivateNodeHeader::with_seed(name, ratchet_seed, inumber).derive_revision_ref()
    }

    /// TODO(matheus23) docs
    pub fn as_private_ref(self, content_cid: Cid) -> PrivateRef {
        PrivateRef {
            saturated_name_hash: self.saturated_name_hash,
            revision_key: self.revision_key,
            content_cid,
        }
    }
}

//--------------------------------------------------------------------------------------------------
// Tests
//--------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use crate::{
        private::PrivateNodeHeader,
        utils::{self, test_setup},
        PrivateDirectory, PrivateNode,
    };
    use chrono::Utc;

    #[async_std::test]
    async fn can_create_privateref_deterministically_with_user_provided_seeds() {
        let (forest, store, rng) = test_setup::init!(forest, mut store, mut rng);
        let ratchet_seed = utils::get_random_bytes::<32>(rng);
        let inumber = utils::get_random_bytes::<32>(rng);

        let dir = PrivateNode::from(PrivateDirectory::with_seed(
            Default::default(),
            Utc::now(),
            ratchet_seed,
            inumber,
        ));

        // Throwing away the private ref
        let (forest, _) = forest.put(&dir, store, rng).await.unwrap();

        // TODO(matheus23) refactor this (!!!)

        // Creating deterministic header and retrieve the content.
        let header = PrivateNodeHeader::with_seed(Default::default(), ratchet_seed, inumber);
        let saturated_name_hash = header.get_saturated_name_hash();
        let retrieved_node_cids = forest
            .get_encrypted(&saturated_name_hash, store)
            .await
            .unwrap()
            .unwrap();

        let mut result = None;
        for cid in retrieved_node_cids {
            match PrivateNode::load(&header.derive_private_ref(*cid), store).await {
                Ok(node) => {
                    result = Some(node);
                    break;
                }
                // TODO(matheus23) only match AES errors
                Err(_) => {}
            }
        }

        let retrieved_node = result.unwrap();

        assert_eq!(retrieved_node, dir);
    }
}
