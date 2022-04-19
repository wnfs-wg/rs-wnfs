//! File system metadata.

use std::{
    cmp::Ordering,
    io::{Read, Seek, Write},
    str::FromStr,
};

use anyhow::{ensure, Result};
use chrono::{DateTime, Utc};
use field_names::FieldNames;
use libipld::{
    cbor::{cbor::MajorKind, decode, encode, DagCborCodec},
    codec::{Decode, Encode},
    DagCbor,
};
use semver::Version;

use crate::FsError;

//--------------------------------------------------------------------------------------------------
// Type Definitions
//--------------------------------------------------------------------------------------------------

/// The different types a UnixFS can be.
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
#[derive(Debug, Clone, PartialEq, Eq, FieldNames)]
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
        // Ensure the major kind is a map.
        let major = decode::read_major(r)?;
        ensure!(
            major.kind() == MajorKind::Map,
            FsError::UndecodableCborData("Unsupported major".into())
        );

        let _ = decode::read_uint(r, major)?;

        // Ordering the fields by name based on RFC-7049 which is also what libipld uses.
        let mut cbor_order: Vec<&'static str> = Vec::from_iter(Metadata::FIELDS);
        cbor_order.sort_unstable_by(|&a, &b| match a.len().cmp(&b.len()) {
            Ordering::Greater => Ordering::Greater,
            Ordering::Less => Ordering::Less,
            Ordering::Equal => a.cmp(b),
        });

        // Iterate over the fields.
        let mut unix_fs = None;
        let mut version = String::new();
        for field in cbor_order.iter() {
            // Decode field name.
            String::decode(c, r)?;

            // Decode field value.
            match *field {
                "unix_fs" => {
                    unix_fs = Some(UnixFsMetadata::decode(c, r)?);
                }
                "version" => {
                    version = String::decode(c, r)?;
                }
                _ => unreachable!(),
            }
        }

        Ok(Self {
            unix_fs: unix_fs
                .ok_or_else(|| FsError::UndecodableCborData("Missing unix_fs".into()))?,
            version: Version::from_str(&version)?,
        })
    }
}

impl Encode<DagCborCodec> for Metadata {
    fn encode<W: Write>(&self, c: DagCborCodec, w: &mut W) -> Result<()> {
        // Write the major of the section being written.
        encode::write_u64(w, MajorKind::Map, Metadata::FIELDS.len() as u64)?;

        // Ordering the fields by name based on RFC-7049 which is also what libipld uses.
        let mut cbor_order: Vec<&'static str> = Vec::from_iter(Metadata::FIELDS);
        cbor_order.sort_unstable_by(|&a, &b| match a.len().cmp(&b.len()) {
            Ordering::Greater => Ordering::Greater,
            Ordering::Less => Ordering::Less,
            Ordering::Equal => a.cmp(b),
        });

        // Iterate over the fields.
        for field in cbor_order.iter() {
            // Encode field name.
            field.encode(c, w)?;
            // Encode field value.
            match *field {
                "unix_fs" => {
                    self.unix_fs.encode(c, w)?;
                }
                "version" => {
                    self.version.to_string().encode(c, w)?;
                }
                _ => unreachable!(),
            }
        }

        Ok(())
    }
}

//--------------------------------------------------------------------------------------------------
// Tests
//--------------------------------------------------------------------------------------------------

#[cfg(test)]
mod metadata_tests {
    use std::io::Cursor;

    use chrono::Utc;
    use libipld::{
        cbor::DagCborCodec,
        codec::{Decode, Encode},
    };

    use crate::{Metadata, UnixFsNodeKind};

    #[async_std::test]
    async fn metadata_can_encode_decode_as_cbor() {
        let metadata = Metadata::new(Utc::now(), UnixFsNodeKind::File);

        let mut encoded_bytes = vec![];

        metadata.encode(DagCborCodec, &mut encoded_bytes).unwrap();

        let mut cursor = Cursor::new(encoded_bytes);

        let decoded_metadata = Metadata::decode(DagCborCodec, &mut cursor).unwrap();

        assert_eq!(metadata, decoded_metadata);
    }
}
