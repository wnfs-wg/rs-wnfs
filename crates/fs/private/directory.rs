use std::{collections::BTreeMap, rc::Rc};

use anyhow::{bail, Result};
use async_recursion::async_recursion;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use super::{HamtStore, INumber, Key, Namefilter, PrivateNode, PrivateNodeHeader, Rng};
use crate::{BlockStore, FsError, HashOutput, Id, Metadata, UnixFsNodeKind};

//--------------------------------------------------------------------------------------------------
// Type Definitions
//--------------------------------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct RatchetKey {
    pub(crate) encrypted: Vec<u8>,
    pub(crate) bare: Option<Key>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivateRef {
    pub(crate) saturated_name_hash: HashOutput, // Sha3-256 hash of saturated namefilter
    pub(crate) content_key: Key,                // A hash or parent skip ratchet.
    pub(crate) ratchet_key: RatchetKey,         // Ratchet key.
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivateDirectoryContent {
    pub(crate) metadata: Metadata,
    pub(crate) entries: BTreeMap<String, PrivateRef>, // TODO(appcypher): Figure caching. Ordinary link-type caching is not accurate.
}

#[derive(Debug, Clone)]
pub struct PrivateDirectory {
    pub(crate) header: Option<PrivateNodeHeader>,
    pub(crate) content: PrivateDirectoryContent,
}

//--------------------------------------------------------------------------------------------------
// Implementations
//--------------------------------------------------------------------------------------------------

impl PrivateDirectory {
    pub fn new(
        parent_bare_name: Option<Namefilter>,
        inumber: INumber,
        ratchet_seed: HashOutput,
        time: DateTime<Utc>,
    ) -> Self {
        Self {
            header: Some(PrivateNodeHeader::new(
                parent_bare_name,
                inumber,
                ratchet_seed,
            )),
            content: PrivateDirectoryContent {
                metadata: Metadata::new(time, UnixFsNodeKind::Dir),
                entries: BTreeMap::new(),
            },
        }
    }

    #[async_recursion(?Send)]
    pub async fn get_node<'a, B, R>(
        self: &Rc<Self>,
        path_segments: &[String],
        hamt: &HamtStore<'a, B, R>,
    ) -> Result<Option<PrivateNode>>
    where
        B: BlockStore,
        R: Rng,
    {
        if path_segments.is_empty() {
            bail!(FsError::InvalidPath);
        }

        match path_segments.split_first().unwrap() {
            (head, &[]) => Ok(self.lookup_node(head, hamt).await?),
            (head, tail) => {
                let node = self.lookup_node(head, hamt).await?;
                if !matches!(node, Some(PrivateNode::Dir(_))) {
                    bail!(FsError::InvalidPath);
                }

                self.get_node(tail, hamt).await
            }
        }
    }

    pub async fn lookup_node<'a, B, R>(
        self: &Rc<Self>,
        path_segment: &str,
        hamt: &HamtStore<'a, B, R>,
    ) -> Result<Option<PrivateNode>>
    where
        B: BlockStore,
        R: Rng,
    {
        Ok(match self.content.entries.get(path_segment) {
            Some(private_ref) => hamt.get(private_ref).await?,
            None => None,
        })
    }

    pub fn mkdir() {
        unimplemented!()
    }

    pub fn read() {
        unimplemented!()
    }

    pub fn write() {
        unimplemented!()
    }

    pub fn rm() {
        unimplemented!()
    }

    pub fn basic_mv() {
        unimplemented!()
    }

    pub fn ls() {
        unimplemented!()
    }

    pub fn get_history() {
        unimplemented!()
    }
}

impl Id for PrivateDirectory {
    fn get_id(&self) -> String {
        format!("{:p}", &self.header)
    }
}

impl Serialize for RatchetKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_bytes(self.encrypted.as_slice())
    }
}

impl<'de> Deserialize<'de> for RatchetKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let bytes = Vec::deserialize(deserializer)?;
        Ok(RatchetKey {
            encrypted: bytes,
            bare: None,
        })
    }
}

//--------------------------------------------------------------------------------------------------
// Tests
//--------------------------------------------------------------------------------------------------

mod private_directory_tests {}
