use crate::{
    error::AccessKeyError,
    private::{PrivateRef, SnapshotKey, TemporalKey},
};
use anyhow::{bail, Result};
use libipld_core::cid::Cid;
use serde::{Deserialize, Serialize};
use wnfs_common::HashOutput;

//--------------------------------------------------------------------------------------------------
// Type Definitions
//--------------------------------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AccessKey {
    #[serde(rename = "wnfs/share/temporal")]
    Temporal(TemporalAccessKey),
    #[serde(rename = "wnfs/share/snapshot")]
    Snapshot(SnapshotAccessKey),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TemporalAccessKey {
    #[serde(serialize_with = "crate::utils::serialize_byte_slice32")]
    #[serde(deserialize_with = "crate::utils::deserialize_byte_slice32")]
    pub(crate) label: HashOutput,
    #[serde(rename = "contentCid")]
    pub(crate) content_cid: Cid,
    #[serde(rename = "temporalKey")]
    pub(crate) temporal_key: TemporalKey,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SnapshotAccessKey {
    #[serde(serialize_with = "crate::utils::serialize_byte_slice32")]
    #[serde(deserialize_with = "crate::utils::deserialize_byte_slice32")]
    pub label: HashOutput,
    #[serde(rename = "contentCid")]
    pub content_cid: Cid,
    #[serde(rename = "snapshotKey")]
    pub snapshot_key: SnapshotKey,
}

//--------------------------------------------------------------------------------------------------
// Implementations
//--------------------------------------------------------------------------------------------------

impl AccessKey {
    pub fn get_label(&self) -> &HashOutput {
        match self {
            Self::Temporal(key) => &key.label,
            Self::Snapshot(key) => &key.label,
        }
    }

    pub fn get_temporal_key(&self) -> Option<&TemporalKey> {
        match self {
            Self::Temporal(key) => Some(&key.temporal_key),
            Self::Snapshot(_) => None,
        }
    }

    pub fn get_content_cid(&self) -> &Cid {
        match self {
            Self::Temporal(key) => &key.content_cid,
            Self::Snapshot(key) => &key.content_cid,
        }
    }

    pub(crate) fn derive_private_ref(&self) -> Result<PrivateRef> {
        // TODO(appcypher): SnapshotAccessKey currently not supported for PrivateRef.
        let Self::Temporal(key) = self else {
            bail!(AccessKeyError::UnsupportedSnapshotPrivateRefDerive)
        };

        Ok(PrivateRef::with_temporal_key(
            key.label,
            key.temporal_key.clone(),
            key.content_cid,
        ))
    }
}

impl From<&PrivateRef> for TemporalAccessKey {
    fn from(private_ref: &PrivateRef) -> Self {
        Self {
            label: private_ref.saturated_name_hash,
            content_cid: private_ref.content_cid,
            temporal_key: private_ref.temporal_key.clone(),
        }
    }
}

impl From<&PrivateRef> for SnapshotAccessKey {
    fn from(private_ref: &PrivateRef) -> Self {
        Self {
            label: private_ref.saturated_name_hash,
            content_cid: private_ref.content_cid,
            snapshot_key: private_ref.temporal_key.derive_snapshot_key(),
        }
    }
}
