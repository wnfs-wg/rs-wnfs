use libipld_core::cid::Cid;
use semver::Version;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use wnfs_common::Metadata;

//--------------------------------------------------------------------------------------------------
// Type Definitions
//--------------------------------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PublicNodeSerializable {
    #[serde(rename = "wnfs/pub/file")]
    File(PublicFileSerializable),
    #[serde(rename = "wnfs/pub/dir")]
    Dir(PublicDirectorySerializable),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublicFileSerializable {
    pub version: Version,
    pub metadata: Metadata,
    pub userland: Cid,
    pub previous: Vec<Cid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublicDirectorySerializable {
    pub version: Version,
    pub metadata: Metadata,
    pub userland: BTreeMap<String, Cid>,
    pub previous: Vec<Cid>,
}
