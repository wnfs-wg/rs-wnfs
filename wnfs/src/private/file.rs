use anyhow::Result;
use chrono::{DateTime, Utc};
use libipld::{cbor::DagCborCodec, prelude::Encode, Cid};
use rand_core::RngCore;
use semver::Version;
use serde::{de::Error as DeError, ser::Error as SerError, Deserialize, Deserializer, Serialize};

use crate::{dagcbor, BlockStore, Id, Metadata, NodeType};

use super::{namefilter::Namefilter, Key, PrivateNodeHeader, RevisionKey};

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
#[derive(Debug, Clone, PartialEq)]
pub struct PrivateFile {
    pub version: Version,
    pub header: PrivateNodeHeader,
    pub metadata: Metadata,
    pub content: Vec<u8>, // TODO(appcypher): Support linked file content.
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PrivateFileSerializable {
    pub r#type: NodeType,
    pub version: Version,
    pub header: Vec<u8>,
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
            version: Version::new(0, 2, 0),
            header: PrivateNodeHeader::new(parent_bare_name, rng),
            metadata: Metadata::new(time),
            content,
        }
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
            metadata: self.metadata.clone(),
            content: self.content.clone(),
        })
        .serialize(serializer)
    }

    /// Deserializes the file with provided Serde deserializer and key.
    pub(crate) fn deserialize<'de, D>(deserializer: D, key: &RevisionKey) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let PrivateFileSerializable {
            version,
            metadata,
            header,
            content,
            ..
        } = PrivateFileSerializable::deserialize(deserializer)?;

        Ok(Self {
            version,
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
        // TODO(matheus23) revisit
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
    }
}

impl Id for PrivateFile {
    fn get_id(&self) -> String {
        format!("{:p}", &self.header)
    }
}
