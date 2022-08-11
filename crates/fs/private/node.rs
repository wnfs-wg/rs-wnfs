use std::{io::Cursor, rc::Rc};

use anyhow::{anyhow, bail, Result};
use chrono::{DateTime, Utc};
use libipld::{
    cbor::DagCborCodec,
    codec::{Decode, Encode},
    serde as ipld_serde, Ipld,
};
use serde::{Deserialize, Serialize, Serializer};
use sha3::Sha3_256;
use skip_ratchet::Ratchet;

use crate::{FsError, HashOutput, Id, Metadata};

use super::{
    hamt::Hasher, namefilter::Namefilter, Key, PrivateDirectory, PrivateDirectoryContent,
    PrivateFile, PrivateFileContent,
};

use log::debug;

//--------------------------------------------------------------------------------------------------
// Type Definitions
//--------------------------------------------------------------------------------------------------

pub type INumber = HashOutput;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentKey(pub Key);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RatchetKey(pub Key);

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PrivateNodeHeader {
    pub(crate) bare_name: Namefilter,
    pub(crate) ratchet: Ratchet,
    pub(crate) inumber: INumber,
}

#[derive(Debug, Clone)]
pub enum PrivateNode {
    File(Rc<PrivateFile>),
    Dir(Rc<PrivateDirectory>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivateRef {
    pub(crate) saturated_name_hash: HashOutput, // Sha3-256 hash of saturated namefilter
    pub(crate) content_key: ContentKey,         // A hash of ratchet key.
    pub(crate) ratchet_key: RatchetKey,         // Encrypted ratchet key.
}

//--------------------------------------------------------------------------------------------------
// Implementations
//--------------------------------------------------------------------------------------------------

impl PrivateNodeHeader {
    /// Creates a new PrivateNodeHeader.
    pub fn new(parent_bare_name: Namefilter, inumber: INumber, ratchet_seed: HashOutput) -> Self {
        Self {
            bare_name: {
                let mut namefilter = parent_bare_name;
                namefilter.add(&inumber);
                namefilter
            },
            ratchet: Ratchet::zero(ratchet_seed),
            inumber,
        }
    }

    /// Advances the ratchet.
    pub fn advance_ratchet(&mut self) {
        self.ratchet.inc();
    }

    /// Gets the private ref of the current header.
    pub fn get_private_ref(&self) -> Result<PrivateRef> {
        let ratchet_key = Key::new(self.ratchet.derive_key());
        let saturated_name_hash = {
            let mut name = self.bare_name.clone();
            name.add(&ratchet_key.as_bytes());
            name.saturate();
            Sha3_256::hash(&name.as_bytes())
        };

        Ok(PrivateRef {
            saturated_name_hash,
            content_key: ContentKey(Key::new(Sha3_256::hash(&ratchet_key.as_bytes()))),
            ratchet_key: RatchetKey(ratchet_key),
        })
    }

    /// Gets the saturated namefilter for this node.
    pub fn get_saturated_name(&self) -> Namefilter {
        let ratchet_key = Key::new(self.ratchet.derive_key());
        let mut name = self.bare_name.clone();
        name.add(&ratchet_key.as_bytes());
        name.saturate();
        name
    }
}

impl PrivateNode {
    /// Creates node with updated modified time.
    pub fn update_mtime(&self, time: DateTime<Utc>) -> Self {
        match self {
            Self::File(file) => {
                let mut file = (**file).clone();
                file.content.metadata.unix_fs.modified = time.timestamp();
                Self::File(Rc::new(file))
            }
            Self::Dir(dir) => {
                let mut dir = (**dir).clone();
                dir.content.metadata.unix_fs.modified = time.timestamp();
                Self::Dir(Rc::new(dir))
            }
        }
    }

    pub fn header(&self) -> &PrivateNodeHeader {
        match self {
            Self::File(file) => &file.header,
            Self::Dir(dir) => &dir.header,
        }
    }

    pub fn serialize_header<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            PrivateNode::File(file) => file.header.serialize(serializer),
            PrivateNode::Dir(dir) => dir.header.serialize(serializer),
        }
    }

    pub fn serialize_content<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            PrivateNode::File(file) => file.content.serialize(serializer),
            PrivateNode::Dir(dir) => dir.content.serialize(serializer),
        }
    }

    pub fn serialize_as_cbor(&self) -> Result<(Vec<u8>, Vec<u8>)> {
        let header_ipld = self.serialize_header(ipld_serde::Serializer)?;
        let content_ipld = self.serialize_content(ipld_serde::Serializer)?;

        debug!("serialize: header_ipld: {:?}", header_ipld);
        debug!("serialize: content_ipld: {:?}", content_ipld);

        let mut header_bytes = Vec::new();
        let mut content_bytes = Vec::new();

        header_ipld.encode(DagCborCodec, &mut header_bytes)?;
        content_ipld.encode(DagCborCodec, &mut content_bytes)?;

        Ok((header_bytes, content_bytes))
    }

    pub fn deserialize_from_cbor(
        header_bytes: &Option<Vec<u8>>,
        content_bytes: &[u8],
    ) -> Result<Self> {
        let header_ipld = match header_bytes {
            Some(bytes) => Ipld::decode(DagCborCodec, &mut Cursor::new(bytes))?,
            None => bail!(FsError::MissingHeader),
        };

        debug!("deserialize: header_ipld: {:?}", header_ipld);

        let header: PrivateNodeHeader = ipld_serde::from_ipld(header_ipld)?;

        debug!("deserialize: header: {:?}", header);

        let content_ipld = Ipld::decode(DagCborCodec, &mut Cursor::new(content_bytes))?;

        debug!("deserialize: content_ipld: {:?}", content_ipld);

        Self::deserialize_content(content_ipld, header)
    }

    pub fn deserialize_content(content_ipld: Ipld, header: PrivateNodeHeader) -> Result<Self> {
        match content_ipld {
            Ipld::Map(map) => {
                let metadata_ipld = map
                    .get("metadata")
                    .ok_or("Missing metadata field")
                    .map_err(|e| anyhow!(e))?;

                debug!("map: metadata: {:?}", metadata_ipld);
                let metadata: Metadata =
                    metadata_ipld.try_into().map_err(|e: String| anyhow!(e))?;

                debug!("map: map: {:?}", map);

                Ok(if metadata.is_file() {
                    let content = PrivateFileContent::deserialize(Ipld::Map(map))?;
                    PrivateNode::from(PrivateFile { header, content })
                } else {
                    let content = PrivateDirectoryContent::deserialize(Ipld::Map(map))?;
                    PrivateNode::from(PrivateDirectory { header, content })
                })
            }
            other => bail!(FsError::InvalidDeserialization(format!(
                "Expected `Ipld::Map` got {:?}",
                other
            ))),
        }
    }
}

impl Id for PrivateNode {
    fn get_id(&self) -> String {
        match self {
            Self::File(file) => file.get_id(),
            Self::Dir(dir) => dir.get_id(),
        }
    }
}

impl From<PrivateFile> for PrivateNode {
    fn from(file: PrivateFile) -> Self {
        Self::File(Rc::new(file))
    }
}

impl From<PrivateDirectory> for PrivateNode {
    fn from(dir: PrivateDirectory) -> Self {
        Self::Dir(Rc::new(dir))
    }
}

//--------------------------------------------------------------------------------------------------
// Tests
//--------------------------------------------------------------------------------------------------

mod private_node_tests {}
