//! File system metadata.

use chrono::{DateTime, Utc};
use semver::Version;

/// Represents the type of node in the UnixFS file system.
///
/// See https://docs.ipfs.io/concepts/file-systems/#unix-file-system-unixfs
#[derive(Debug, Clone, PartialEq, Eq)]
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
#[derive(Debug, Clone, PartialEq, Eq)]
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnixFsMetadata {
    created: DateTime<Utc>,
    modified: DateTime<Utc>,
    mode: UnixFsMode,
    kind: UnixFsNodeKind,
}

/// The metadata of a node on the WNFS file system.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Metadata {
    unixfs_metadata: UnixFsMetadata,
    version: Version,
}

impl Metadata {
    /// Creates a new metadata representing a UnixFS node.
    pub fn new(time: DateTime<Utc>, kind: UnixFsNodeKind) -> Self {
        let mode =
            if matches!(kind, UnixFsNodeKind::Dir) || matches!(kind, UnixFsNodeKind::HAMTShard) {
                UnixFsMode::OwnerReadWriteGroupOthersRead
            } else {
                UnixFsMode::OwnerReadWriteExecuteGroupOthersReadExecute
            };

        Self {
            unixfs_metadata: UnixFsMetadata {
                created: time,
                modified: time,
                mode,
                kind,
            },
            version: Version::new(1, 0, 0),
        }
    }
}

#[cfg(test)]
mod metadata_tests {}
