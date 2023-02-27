//! File system metadata.

use anyhow::{bail, Result};
use chrono::{DateTime, TimeZone, Utc};
use libipld::Ipld;
use serde::{de::Error as DeError, Deserialize, Deserializer, Serialize, Serializer};
use std::{collections::BTreeMap, convert::TryInto};

//--------------------------------------------------------------------------------------------------
// Type Definitions
//--------------------------------------------------------------------------------------------------

/// The type of file system node.
#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum NodeType {
    PublicFile,
    PublicDirectory,
    PrivateFile,
    PrivateDirectory,
    TemporalSharePointer,
    SnapshotSharePointer,
}

impl ToString for NodeType {
    fn to_string(&self) -> String {
        match self {
            NodeType::PublicFile => "wnfs/pub/file",
            NodeType::PublicDirectory => "wnfs/pub/dir",
            NodeType::PrivateFile => "wnfs/priv/file",
            NodeType::PrivateDirectory => "wnfs/priv/dir",
            NodeType::TemporalSharePointer => "wnfs/share/temporal",
            NodeType::SnapshotSharePointer => "wnfs/share/snapshot",
        }
        .to_string()
    }
}

/// The metadata of a node in the WNFS file system.
///
/// # Examples
///
/// ```
/// use wnfs_common::Metadata;
/// use chrono::Utc;
///
/// let metadata = Metadata::new(Utc::now());
///
/// println!("{:?}", metadata);
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Metadata(pub BTreeMap<String, Ipld>);

//--------------------------------------------------------------------------------------------------
// Implementations
//--------------------------------------------------------------------------------------------------

impl Metadata {
    /// Creates a new metadata.
    ///
    /// # Examples
    ///
    /// ```
    /// use wnfs_common::Metadata;
    /// use chrono::Utc;
    ///
    /// let metadata = Metadata::new(Utc::now());
    ///
    /// println!("{:?}", metadata);
    /// ```
    pub fn new(time: DateTime<Utc>) -> Self {
        let time = time.timestamp();
        Self(BTreeMap::from([
            ("created".into(), time.into()),
            ("modified".into(), time.into()),
        ]))
    }

    /// Updates modified time.
    ///
    /// # Examples
    ///
    /// ```
    /// use wnfs_common::Metadata;
    /// use chrono::{Utc, TimeZone, Duration};
    ///
    /// let mut metadata = Metadata::new(Utc::now());
    /// let time = Utc::now() + Duration::days(1);
    ///
    /// metadata.upsert_mtime(time);
    ///
    /// let imprecise_time = Utc.timestamp_opt(time.timestamp(), 0).single();
    /// assert_eq!(metadata.get_modified(), imprecise_time);
    /// ```
    pub fn upsert_mtime(&mut self, time: DateTime<Utc>) {
        self.0.insert("modified".into(), time.timestamp().into());
    }

    /// Returns the created time.
    ///
    /// # Examples
    ///
    /// ```
    /// use wnfs_common::Metadata;
    /// use chrono::{Utc, TimeZone};
    ///
    /// let time = Utc::now();
    /// let metadata = Metadata::new(time);
    ///
    /// let imprecise_time = Utc.timestamp_opt(time.timestamp(), 0).single();
    /// assert_eq!(metadata.get_created(), imprecise_time);
    /// ```
    ///
    /// Will return `None` if there's no created metadata on the
    /// node or if it's not a second-based POSIX timestamp integer.
    pub fn get_created(&self) -> Option<DateTime<Utc>> {
        self.0.get("created").and_then(|ipld| match ipld {
            Ipld::Integer(i) => Utc.timestamp_opt(i64::try_from(*i).ok()?, 0).single(),
            _ => None,
        })
    }

    /// Returns the modified time.
    ///
    /// # Examples
    ///
    /// ```
    /// use wnfs_common::Metadata;
    /// use chrono::{Utc, TimeZone};
    ///
    /// let time = Utc::now();
    /// let metadata = Metadata::new(time);
    ///
    /// let imprecise_time = Utc.timestamp_opt(time.timestamp(), 0).single();
    /// assert_eq!(metadata.get_modified(), imprecise_time);
    /// ```
    ///
    /// Will return `None` if there's no created metadata on the
    /// node or if it's not a second-based POSIX timestamp integer.
    pub fn get_modified(&self) -> Option<DateTime<Utc>> {
        self.0.get("modified").and_then(|ipld| match ipld {
            Ipld::Integer(i) => Utc.timestamp_opt(i64::try_from(*i).ok()?, 0).single(),
            _ => None,
        })
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
            "wnfs/share/temporal" => NodeType::TemporalSharePointer,
            "wnfs/share/snapshot" => NodeType::SnapshotSharePointer,
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
            NodeType::TemporalSharePointer => "wnfs/share/temporal".into(),
            NodeType::SnapshotSharePointer => "wnfs/share/snapshot".into(),
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
mod tests {
    use crate::{dagcbor, Metadata};
    use chrono::Utc;

    #[async_std::test]
    async fn metadata_can_encode_decode_as_cbor() {
        let metadata = Metadata::new(Utc::now());

        let encoded_metadata = dagcbor::encode(&metadata).unwrap();
        let decoded_metadata = dagcbor::decode::<Metadata>(encoded_metadata.as_ref()).unwrap();

        assert_eq!(metadata, decoded_metadata);
    }
}
