//! File system metadata.

use std::{
    io::{Read, Seek, Write},
    str::FromStr,
};

use anyhow::Result;
use chrono::{DateTime, Utc};
use libipld::{
    cbor::DagCborCodec,
    codec::{Decode, Encode},
    DagCbor,
};
use semver::Version;

//--------------------------------------------------------------------------------------------------
// Type Definitions
//--------------------------------------------------------------------------------------------------

/// Represents the type of node in the UnixFS file system.
///
/// See https://docs.ipfs.io/concepts/file-systems/#unix-file-system-unixfs
#[derive(Debug, Clone, PartialEq, Eq, Copy, DagCbor)]
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
/// - https://docs.ipfs.io/concepts/file-systems/#unix-file-system-unixfs
/// - https://en.wikipedia.org/wiki/File-system_permissions#Numeric_notation
#[derive(Debug, Clone, PartialEq, Eq, DagCbor)]
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

/// The metadata of a node in the UnixFS file system.
///
/// See https://docs.ipfs.io/concepts/file-systems/#unix-file-system-unixfs
#[derive(Debug, Clone, PartialEq, Eq, DagCbor)]
pub struct UnixFsMetadata {
    pub(crate) created: i64,
    pub(crate) modified: i64,
    pub(crate) mode: UnixFsMode,
    pub(crate) kind: UnixFsNodeKind,
}

/// The metadata of a node on the WNFS file system.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Metadata {
    pub(crate) unix_fs: UnixFsMetadata,
    pub(crate) version: Version,
}

//--------------------------------------------------------------------------------------------------
// Implementations
//--------------------------------------------------------------------------------------------------

impl Metadata {
    /// Creates a new metadata representing a UnixFS node.
    pub fn new(time: DateTime<Utc>, kind: UnixFsNodeKind) -> Self {
        let mode =
            if matches!(kind, UnixFsNodeKind::Dir) || matches!(kind, UnixFsNodeKind::HAMTShard) {
                UnixFsMode::OwnerReadWriteGroupOthersRead
            } else {
                UnixFsMode::OwnerReadWriteExecuteGroupOthersReadExecute
            };

        let time = time.timestamp();

        Self {
            unix_fs: UnixFsMetadata {
                created: time,
                modified: time,
                mode,
                kind,
            },
            version: Version::new(1, 0, 0),
        }
    }
}

impl Decode<DagCborCodec> for Metadata {
    fn decode<R: Read + Seek>(c: DagCborCodec, r: &mut R) -> Result<Self> {
        let unix_fs = UnixFsMetadata::decode(c, r)?;
        let version_str = String::decode(c, r)?;
        let version = Version::from_str(&version_str)?;

        Ok(Self { unix_fs, version })
    }
}

impl Encode<DagCborCodec> for Metadata {
    fn encode<W: Write>(&self, c: DagCborCodec, w: &mut W) -> Result<()> {
        self.unix_fs.encode(c, w)?;
        self.version.to_string().encode(c, w)?;

        Ok(())
    }
}

#[cfg(test)]
mod metadata_tests {
    #[async_std::test]
    async fn metadata_encode_decode_successful() {
        // TODO(appcypher): Implement this.
    }
}
