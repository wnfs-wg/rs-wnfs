use super::{ContentKey, Key, PrivateNodeHeader, RevisionKey};
use crate::{FsError, HashOutput, Namefilter};
use anyhow::Result;
use rand_core::RngCore;
use serde::{de::Error as DeError, ser::Error as SerError, Deserialize, Serialize};

//--------------------------------------------------------------------------------------------------
// Type Definitions
//--------------------------------------------------------------------------------------------------

/// PrivateRef holds the information to fetch associated node from a HAMT and decrypt it if it is present.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrivateRef {
    /// Sha3-256 hash of saturated namefilter.
    pub(crate) saturated_name_hash: HashOutput,
    /// Sha3-256 hash of the ratchet key.
    pub(crate) content_key: ContentKey,
    /// Skip-ratchet-derived key.
    pub(crate) revision_key: RevisionKey,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct PrivateRefSerializable {
    #[serde(rename = "name")]
    pub(crate) saturated_name_hash: HashOutput,
    #[serde(rename = "contentKey")]
    pub(crate) content_key: ContentKey,
    #[serde(rename = "revisionKey")]
    pub(crate) revision_key: Vec<u8>,
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
    /// use wnfs::{private::{PrivateRef, RevisionKey}, private::Key};
    /// use rand::{thread_rng, Rng};
    ///
    /// let rng = &mut thread_rng();
    /// let private_ref = PrivateRef::with_revision_key(
    ///     rng.gen::<[u8; 32]>(),
    ///     RevisionKey::from(Key::new(rng.gen::<[u8; 32]>())),
    /// );
    ///
    /// println!("Private ref: {:?}", private_ref);
    /// ```
    pub fn with_revision_key(saturated_name_hash: HashOutput, revision_key: RevisionKey) -> Self {
        Self {
            saturated_name_hash,
            content_key: revision_key.derive_content_key(),
            revision_key,
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
    pub fn with_seed(name: Namefilter, ratchet_seed: HashOutput, inumber: HashOutput) -> Self {
        let h = PrivateNodeHeader::with_seed(name, ratchet_seed, inumber);
        h.get_private_ref()
    }

    pub(crate) fn to_serializable(
        &self,
        revision_key: &RevisionKey,
        rng: &mut impl RngCore,
    ) -> Result<PrivateRefSerializable> {
        // encrypt ratchet key
        let revision_key = revision_key
            .0
            .encrypt(&Key::generate_nonce(rng), self.revision_key.0.as_bytes())?;
        Ok(PrivateRefSerializable {
            saturated_name_hash: self.saturated_name_hash,
            content_key: self.content_key.clone(),
            revision_key,
        })
    }

    pub(crate) fn from_serializable(
        private_ref: PrivateRefSerializable,
        revision_key: &RevisionKey,
    ) -> Result<Self> {
        let revision_key = RevisionKey(Key::new(
            revision_key
                .0
                .decrypt(&private_ref.revision_key)?
                .try_into()
                .map_err(|e: Vec<u8>| {
                    FsError::InvalidDeserialization(format!(
                        "Expected 32 bytes for ratchet key, but got {}",
                        e.len()
                    ))
                })?,
        ));
        Ok(Self {
            saturated_name_hash: private_ref.saturated_name_hash,
            content_key: private_ref.content_key,
            revision_key,
        })
    }

    pub fn serialize<S>(
        &self,
        serializer: S,
        revision_key: &RevisionKey,
        rng: &mut impl RngCore,
    ) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.to_serializable(revision_key, rng)
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

//--------------------------------------------------------------------------------------------------
// Tests
//--------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use crate::{
        private::PrivateForest,
        utils::{self, test_setup},
        PrivateDirectory, PrivateNode,
    };
    use chrono::Utc;

    use super::PrivateRef;

    #[async_std::test]
    async fn can_create_privateref_deterministically_with_user_provided_seeds() {
        let (hamt, store, rng) = test_setup::init!(hamt, mut store, mut rng);
        let ratchet_seed = utils::get_random_bytes::<32>(rng);
        let inumber = utils::get_random_bytes::<32>(rng);

        let dir = PrivateNode::from(PrivateDirectory::with_seed(
            Default::default(),
            Utc::now(),
            ratchet_seed,
            inumber,
        ));

        let header = dir.get_header();
        let hamt = hamt
            .put(
                header.get_saturated_name(),
                &header.get_private_ref(),
                &dir,
                store,
                rng,
            )
            .await
            .unwrap();

        // Creating deterministic privateref.
        let private_ref = PrivateRef::with_seed(Default::default(), ratchet_seed, inumber);
        let retrieved_node = hamt
            .get(&private_ref, PrivateForest::resolve_lowest, store)
            .await
            .unwrap()
            .unwrap();

        assert_eq!(retrieved_node, dir);
    }
}
