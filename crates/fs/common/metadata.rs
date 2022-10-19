//! File system metadata.

use std::{collections::BTreeMap, convert::TryInto};

use anyhow::{bail, Result};
use chrono::{DateTime, TimeZone, Utc};
use libipld::Ipld;
use serde::{de::Error as DeError, Deserialize, Deserializer, Serialize, Serializer};

use crate::FsError;

//--------------------------------------------------------------------------------------------------
// Type Definitions
//--------------------------------------------------------------------------------------------------

/// The type of node.
#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum NodeType {
    PublicFile,
    PublicDirectory,
    PrivateFile,
    PrivateDirectory,
}

/// The metadata of a node on the WNFS file system.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Metadata(pub BTreeMap<String, Ipld>);

//--------------------------------------------------------------------------------------------------
// Implementations
//--------------------------------------------------------------------------------------------------

impl Metadata {
    /// Creates a new metadata representing a UnixFS node.
    pub fn new(time: DateTime<Utc>) -> Self {
        let time = time.timestamp_nanos();
        Self(BTreeMap::from([
            ("created".into(), time.into()),
            ("modified".into(), time.into()),
        ]))
    }

    /// Updates modified time.
    pub fn upsert_mtime(&mut self, time: DateTime<Utc>) {
        self.0
            .insert("modified".into(), time.timestamp_nanos().into());
    }

    /// Returns the created time.
    pub fn get_created(&self) -> Result<DateTime<Utc>> {
        let time = self
            .0
            .get("created")
            .ok_or(FsError::MissingCreatedTimeMetadata)?;

        match time {
            Ipld::Integer(i) => Ok(Utc.timestamp_nanos(i64::try_from(*i)?)),
            other => bail!("Expected `Ipld::Integer` got {:#?}", other),
        }
    }

    /// Returns the modified time.
    pub fn get_modified(&self) -> Result<DateTime<Utc>> {
        let time = self
            .0
            .get("modified")
            .ok_or(FsError::MissingModifiedTimeMetadata)?;

        match time {
            Ipld::Integer(i) => Ok(Utc.timestamp_nanos(i64::try_from(*i)?)),
            other => bail!("Expected `Ipld::Integer` got {:#?}", other),
        }
    }
}

impl TryFrom<&Ipld> for NodeType {
    type Error = anyhow::Error;

    fn try_from(ipld: &Ipld) -> Result<Self> {
        match ipld {
            Ipld::String(s) => NodeType::try_from(s.as_str()),
            other => bail!("Expected `Ipld::String` got {:#?}", other),
        }
    }
}

impl TryFrom<&str> for NodeType {
    type Error = anyhow::Error;

    fn try_from(name: &str) -> Result<Self> {
        Ok(match name.to_lowercase().as_str() {
            "wnfs/priv/dir" => NodeType::PrivateDirectory,
            "wnfs/priv/file" => NodeType::PrivateFile,
            "wnfs/pub/dir" => NodeType::PublicDirectory,
            "wnfs/pub/file" => NodeType::PublicFile,
            _ => bail!("Unknown UnixFsNodeKind: {}", name),
        })
    }
}

impl From<&NodeType> for String {
    fn from(r#type: &NodeType) -> Self {
        match r#type {
            NodeType::PrivateDirectory => "wnfs/priv/dir".into(),
            NodeType::PrivateFile => "wnfs/priv/file".into(),
            NodeType::PublicDirectory => "wnfs/pub/dir".into(),
            NodeType::PublicFile => "wnfs/pub/file".into(),
        }
    }
}

impl Serialize for NodeType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        String::from(self).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for NodeType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let r#type = String::deserialize(deserializer)?;
        r#type.as_str().try_into().map_err(DeError::custom)
    }
}
//--------------------------------------------------------------------------------------------------
// Tests
//--------------------------------------------------------------------------------------------------

#[cfg(test)]
mod metadata_tests {
    use chrono::Utc;

    use crate::{dagcbor, Metadata};

    #[async_std::test]
    async fn metadata_can_encode_decode_as_cbor() {
        let metadata = Metadata::new(Utc::now());

        let encoded_metadata = dagcbor::encode(&metadata).unwrap();
        let decoded_metadata = dagcbor::decode::<Metadata>(encoded_metadata.as_ref()).unwrap();

        assert_eq!(metadata, decoded_metadata);
    }
}
