use std::{io::Cursor, rc::Rc};

use anyhow::{anyhow, bail, Result};
use libipld::{
    cbor::DagCborCodec,
    codec::{Decode, Encode},
    serde as ipld_serde, Ipld,
};
use serde::{Deserialize, Serialize, Serializer};
use skip_ratchet::Ratchet;

use crate::{FsError, HashOutput, Id, Metadata};

use super::{
    Namefilter, PrivateDirectory, PrivateDirectoryContent, PrivateFile, PrivateFileContent,
};

//--------------------------------------------------------------------------------------------------
// Type Definitions
//--------------------------------------------------------------------------------------------------

pub type INumber = Vec<u8>;

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

//--------------------------------------------------------------------------------------------------
// Implementations
//--------------------------------------------------------------------------------------------------

impl PrivateNodeHeader {
    /// Creates a new PrivateNodeHeader.
    pub fn new(
        parent_bare_name: Option<Namefilter>,
        inumber: INumber,
        ratchet_seed: HashOutput,
    ) -> Self {
        Self {
            bare_name: {
                let mut namefilter = parent_bare_name.unwrap_or_default();
                namefilter.add(&inumber);
                namefilter
            },
            ratchet: Ratchet::zero(ratchet_seed),
            inumber,
        }
    }
}

impl PrivateNode {
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
            Some(bytes) => Some(Ipld::decode(DagCborCodec, &mut Cursor::new(bytes))?),
            None => None,
        };

        let header: Option<PrivateNodeHeader> = match header_ipld {
            Some(ipld) => Some(ipld_serde::from_ipld(ipld)?),
            None => None,
        };

        let content_ipld = Ipld::decode(DagCborCodec, &mut Cursor::new(content_bytes))?;
        Ipld::deserialize(content_ipld)
            .and_then(|ipld| Ok(Self::deserialize_content(ipld, header).unwrap()))
            .map_err(|e| anyhow!(e))
    }

    pub fn deserialize_content(
        content_ipld: Ipld,
        header: Option<PrivateNodeHeader>,
    ) -> Result<Self> {
        match content_ipld {
            Ipld::Map(map) => {
                let metadata_ipld = map
                    .get("metadata")
                    .ok_or("Missing metadata field")
                    .map_err(|e| anyhow!(e))?;

                let metadata: Metadata =
                    metadata_ipld.try_into().map_err(|e: String| anyhow!(e))?;

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
