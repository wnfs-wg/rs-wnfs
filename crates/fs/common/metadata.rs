//! File system metadata.

use std::collections::BTreeMap;

use anyhow::{bail, Result};
use chrono::{DateTime, Utc};
use libipld::Ipld;
use serde::{de::Error as DeError, Deserialize, Deserializer, Serialize, Serializer};

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

/// The different types a UnixFS can be.
///
/// See <https://docs.ipfs.io/concepts/file-systems/#unix-file-system-unixfs>
#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum UnixFsNodeKind {
    Raw,
    File,
    Dir,
    Metadata,
    SymLink,
    HAMTShard,
}

/// Mode represents the Unix permissions for a UnixFS node.
///
/// See
/// - <https://docs.ipfs.io/concepts/file-systems/#unix-file-system-unixfs>
/// - <https://en.wikipedia.org/wiki/File-system_permissions#Numeric_notation>
#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum UnixFsMode {
    NoPermissions = 0,
    OwnerReadWriteExecute = 700,
    OwnerGroupReadWriteExecute = 770,
    AllReadWriteExecute = 777,
    AllExecute = 111,
    AllWrite = 222,
    AllWriteExecute = 333,
    AllRead = 444,
    AllReadExecute = 555,
    AllReadWrite = 666,
    OwnerReadWriteExecuteGroupRead = 740,
    OwnerReadWriteExecuteGroupOthersReadExecute = 755,
    OwnerReadWriteGroupOthersRead = 644,
}

/// The metadata of a node on the WNFS file system.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Metadata(BTreeMap<String, Ipld>);

//--------------------------------------------------------------------------------------------------
// Implementations
//--------------------------------------------------------------------------------------------------

impl Metadata {
    /// Creates a new metadata representing a UnixFS node.
    pub fn new(time: DateTime<Utc>, kind: UnixFsNodeKind) -> Self {
        let time: Ipld = time.timestamp().into();
        let mode =
            if matches!(kind, UnixFsNodeKind::Dir) || matches!(kind, UnixFsNodeKind::HAMTShard) {
                (UnixFsMode::OwnerReadWriteGroupOthersRead as u32).into()
            } else {
                (UnixFsMode::OwnerReadWriteExecuteGroupOthersReadExecute as u32).into()
            };

        Self(BTreeMap::from([(
            "unix_fs_meta".into(),
            Ipld::Map(BTreeMap::from([
                ("created".into(), time.clone()),
                ("modified".into(), time),
                ("mode".into(), mode),
                ("kind".into(), String::from(&kind).into()),
            ])),
        )]))
    }

    pub fn get_unix_fs(&self) -> Option<&BTreeMap<String, Ipld>> {
        match self.0.get("unix_fs_meta") {
            Some(Ipld::Map(map)) => Some(map),
            _ => None,
        }
    }

    /// Updates modified time.
    pub fn update_mtime(&mut self, time: DateTime<Utc>) {
        if let Some(Ipld::Map(map)) = self.0.get_mut("unix_fs_meta") {
            map.insert("modified".into(), time.timestamp().into());
        }
    }
}

impl TryFrom<&Ipld> for NodeType {
    type Error = anyhow::Error;

    fn try_from(ipld: &Ipld) -> Result<Self> {
        match ipld {
            Ipld::String(s) => NodeType::try_from(s.as_str()),
            other => bail!("Expected `Ipld::Integer` got {:#?}", other),
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
        serializer.serialize_str(&String::from(self))
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

impl From<&UnixFsNodeKind> for String {
    fn from(kind: &UnixFsNodeKind) -> Self {
        match kind {
            UnixFsNodeKind::Raw => "raw".into(),
            UnixFsNodeKind::File => "file".into(),
            UnixFsNodeKind::Dir => "dir".into(),
            UnixFsNodeKind::Metadata => "metadata".into(),
            UnixFsNodeKind::SymLink => "symlink".into(),
            UnixFsNodeKind::HAMTShard => "hamt-shard".into(),
        }
    }
}

//--------------------------------------------------------------------------------------------------
// Tests
//--------------------------------------------------------------------------------------------------

#[cfg(test)]
mod metadata_tests {
    use chrono::Utc;

    use crate::{dagcbor, Metadata, UnixFsNodeKind};

    #[async_std::test]
    async fn metadata_can_encode_decode_as_cbor() {
        let metadata = Metadata::new(Utc::now(), UnixFsNodeKind::File);

        let encoded_metadata = dagcbor::encode(&metadata).unwrap();
        let decoded_metadata = dagcbor::decode::<Metadata>(encoded_metadata.as_ref()).unwrap();

        assert_eq!(metadata, decoded_metadata);
    }
}
