use crate::private::{encrypted::Encrypted, FileContent, PrivateRefSerializable};
use libipld::Cid;
use semver::Version;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use wnfs_common::Metadata;

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
