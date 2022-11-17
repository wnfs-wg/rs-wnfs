use anyhow::Result;
use async_once_cell::OnceCell;
use chrono::{DateTime, Utc};
use libipld::{cbor::DagCborCodec, prelude::Encode, Cid};
use rand_core::RngCore;
use semver::Version;
use serde::{de::Error as DeError, ser::Error as SerError, Deserialize, Deserializer, Serialize};
use std::{collections::BTreeSet, rc::Rc};

use crate::{dagcbor, BlockStore, Id, Metadata, NodeType};

use super::{encrypted::Encrypted, namefilter::Namefilter, Key, PrivateNodeHeader, RevisionKey};

//--------------------------------------------------------------------------------------------------
// Type Definitions
//--------------------------------------------------------------------------------------------------

/// Represents a file in the WNFS private filesystem.
///
/// # Examples
///
/// ```
/// use wnfs::{PrivateFile, Namefilter, Id};
/// use chrono::Utc;
/// use rand::thread_rng;
///
/// let rng = &mut thread_rng();
/// let file = PrivateFile::new(
///     Namefilter::default(),
///     Utc::now(),
///     b"hello world".to_vec(),
///     rng,
/// );
///
/// println!("file = {:?}", file);
/// ```
#[derive(Debug)]
pub struct PrivateFile {
    persisted_as: OnceCell<Cid>,
    pub version: Version,
    pub header: PrivateNodeHeader,
    pub previous: Option<Encrypted<BTreeSet<Cid>>>,
    pub metadata: Metadata,
    pub content: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PrivateFileSerializable {
    pub r#type: NodeType,
    pub version: Version,
    pub header: Vec<u8>,
    pub previous: Option<Encrypted<BTreeSet<Cid>>>,
    pub metadata: Metadata,
    pub content: Vec<u8>,
}

//--------------------------------------------------------------------------------------------------
// Implementations
//--------------------------------------------------------------------------------------------------

impl PrivateFile {
    /// Creates a new file using the given metadata and CID.
    ///
    /// # Examples
    ///
    /// ```
    /// use wnfs::{PrivateFile, Namefilter, Id};
    /// use chrono::Utc;
    /// use rand::thread_rng;
    ///
    /// let rng = &mut thread_rng();
    /// let file = PrivateFile::new(
    ///     Namefilter::default(),
    ///     Utc::now(),
    ///     b"hello world".to_vec(),
    ///     rng,
    /// );
    ///
    /// println!("file = {:?}", file);
    /// ```
    pub fn new<R: RngCore>(
        parent_bare_name: Namefilter,
        time: DateTime<Utc>,
        content: Vec<u8>,
        rng: &mut R,
    ) -> Self {
        Self {
            persisted_as: OnceCell::new(),
            version: Version::new(0, 2, 0),
            header: PrivateNodeHeader::new(parent_bare_name, rng),
            previous: None,
            metadata: Metadata::new(time),
            content,
        }
    }

    /// This should be called to prepare a node for modifications,
    /// if it's meant to be a successor revision of the current revision.
    ///
    /// It will store the current revision in the given `BlockStore` to
    /// retrieve its CID and put that into the `previous` links,
    /// as well as advancing the ratchet and resetting the `persisted_as` pointer.
    pub(crate) async fn prepare_next_revision<B: BlockStore>(
        self: Rc<Self>,
        store: &mut B,
        rng: &mut impl RngCore,
    ) -> Result<Self> {
        let cid = self.store(store, rng).await?;

        let mut cloned = Rc::try_unwrap(self).unwrap_or_else(|rc| (*rc).clone());
        cloned.persisted_as = OnceCell::new(); // Also done in `.clone()`, but need this to work in case try_unwrap optimizes.
        let key = cloned.header.get_private_ref()?.revision_key.0;
        let previous = Encrypted::from_value(BTreeSet::from([cid]), &key, rng)?;

        cloned.previous = Some(previous);
        cloned.header.advance_ratchet();

        Ok(cloned)
    }

    /// This prepares this file for key rotation, usually for moving or
    /// copying the file to some other place.
    ///
    /// Will reset the ratchet, so a different key is necessary for read access,
    /// will reset the inumber to reset write access,
    /// will update the bare namefilter to match the new parent's namefilter,
    /// so it inherits the write access rules from the new parent and
    /// resets the `persisted_as` pointer.
    pub(crate) fn prepare_key_rotation(
        &mut self,
        parent_bare_name: Namefilter,
        rng: &mut impl RngCore,
    ) {
        self.header.update_bare_name(parent_bare_name);
        self.header.reset_ratchet(rng);
        self.persisted_as = OnceCell::new();
    }

    /// Serializes the file with provided Serde serialilzer.
    pub(crate) fn serialize<S, R: RngCore>(
        &self,
        serializer: S,
        rng: &mut R,
    ) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let key = self
            .header
            .get_private_ref()
            .map_err(SerError::custom)?
            .revision_key;

        (PrivateFileSerializable {
            r#type: NodeType::PrivateFile,
            version: self.version.clone(),
            header: {
                let cbor_bytes = dagcbor::encode(&self.header).map_err(SerError::custom)?;
                key.0
                    .encrypt(&Key::generate_nonce(rng), &cbor_bytes)
                    .map_err(SerError::custom)?
            },
            previous: self.previous.clone(),
            metadata: self.metadata.clone(),
            content: self.content.clone(),
        })
        .serialize(serializer)
    }

    /// Deserializes the file with provided Serde deserializer and key.
    pub(crate) fn deserialize<'de, D>(
        deserializer: D,
        key: &RevisionKey,
        from_cid: Cid,
    ) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let PrivateFileSerializable {
            version,
            metadata,
            header,
            previous,
            content,
            ..
        } = PrivateFileSerializable::deserialize(deserializer)?;

        Ok(Self {
            persisted_as: OnceCell::new_with(Some(from_cid)),
            version,
            previous,
            metadata,
            header: {
                let cbor_bytes = key.0.decrypt(&header).map_err(DeError::custom)?;
                dagcbor::decode(&cbor_bytes).map_err(DeError::custom)?
            },
            content,
        })
    }

    pub(crate) async fn store<B: BlockStore>(
        &self,
        store: &mut B,
        rng: &mut impl RngCore,
    ) -> Result<Cid> {
        let cid = self
            .persisted_as
            .get_or_try_init::<anyhow::Error>(async {
                // TODO(matheus23) deduplicate when reworking serialization
                let private_ref = &self.header.get_private_ref()?;

                // Serialize node to cbor.
                let ipld = self.serialize(libipld::serde::Serializer, rng)?;
                let mut bytes = Vec::new();
                ipld.encode(DagCborCodec, &mut bytes)?;

                // Encrypt bytes with content key.
                let enc_bytes = private_ref
                    .content_key
                    .0
                    .encrypt(&Key::generate_nonce(rng), &bytes)?;

                // Store content section in blockstore and get Cid.
                store.put_block(enc_bytes, libipld::IpldCodec::Raw).await
            })
            .await?;

        Ok(*cid)
    }
}

impl PartialEq for PrivateFile {
    fn eq(&self, other: &Self) -> bool {
        self.header == other.header
            && self.version == other.version
            && self.previous == other.previous
            && self.metadata == other.metadata
            && self.content == other.content
    }
}

impl Clone for PrivateFile {
    fn clone(&self) -> Self {
        Self {
            persisted_as: OnceCell::new(),
            version: self.version.clone(),
            header: self.header.clone(),
            previous: self.previous.clone(),
            metadata: self.metadata.clone(),
            content: self.content.clone(),
        }
    }
}

impl Id for PrivateFile {
    fn get_id(&self) -> String {
        format!("{:p}", &self.header)
    }
}
