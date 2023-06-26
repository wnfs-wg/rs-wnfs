use super::SnapshotKey;
use crate::private::{encrypted::Encrypted, FileContent};
use libipld::Cid;
use semver::Version;
use serde::{Deserialize, Serialize};
use skip_ratchet::Ratchet;
use std::collections::BTreeMap;
use wnfs_common::{HashOutput, Metadata};
use wnfs_nameaccumulator::{NameAccumulator, NameSegment};

//--------------------------------------------------------------------------------------------------
// Type Definitions
//--------------------------------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) enum PrivateNodeContentSerializable {
    #[serde(rename = "wnfs/priv/file")]
    File(PrivateFileContentSerializable),
    #[serde(rename = "wnfs/priv/dir")]
    Dir(PrivateDirectoryContentSerializable),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct PrivateFileContentSerializable {
    pub version: Version,
    #[serde(rename = "headerCid")]
    pub header_cid: Cid,
    pub previous: Vec<(usize, Encrypted<Cid>)>,
    pub metadata: Metadata,
    pub content: FileContent,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct PrivateDirectoryContentSerializable {
    pub version: Version,
    pub previous: Vec<(usize, Encrypted<Cid>)>,
    #[serde(rename = "headerCid")]
    pub header_cid: Cid,
    pub metadata: Metadata,
    pub entries: BTreeMap<String, PrivateRefSerializable>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct PrivateNodeHeaderSerializable {
    /// A unique identifier of the node.
    pub inumber: NameSegment,
    /// Used both for versioning and deriving keys for that enforces privacy.
    pub ratchet: Ratchet,
    /// Stores the name of this node for easier lookup.
    pub name: NameAccumulator,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct PrivateRefSerializable {
    #[serde(rename = "name")]
    pub revision_name_hash: HashOutput,
    #[serde(rename = "snapshotKey")]
    pub snapshot_key: SnapshotKey,
    #[serde(rename = "temporalKey")]
    pub temporal_key: Vec<u8>,
    #[serde(rename = "contentCid")]
    pub content_cid: Cid,
}
