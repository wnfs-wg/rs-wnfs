use std::{cmp::Ordering, io::Cursor, rc::Rc};

use anyhow::{bail, Result};
use chrono::{DateTime, Utc};
use libipld::{
    cbor::DagCborCodec,
    prelude::{Decode, Encode},
    serde as ipld_serde, Ipld,
};
use serde::{Deserialize, Serialize};
use sha3::Sha3_256;
use skip_ratchet::{Ratchet, RatchetExpSearcher};

use crate::{BlockStore, FsError, HashOutput, Id, NodeType};

use super::{
    hamt::Hasher, namefilter::Namefilter, Key, PrivateDirectory, PrivateFile, PrivateForest, Rng,
};

//--------------------------------------------------------------------------------------------------
// Type Definitions
//--------------------------------------------------------------------------------------------------

pub type INumber = HashOutput;

#[derive(Debug, Clone, PartialEq)]
pub enum PrivateNode {
    File(Rc<PrivateFile>),
    Dir(Rc<PrivateDirectory>),
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct ContentKey(pub Key);

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct RatchetKey(pub Key);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PrivateNodeHeader {
    pub(crate) bare_name: Namefilter,
    pub(crate) ratchet: Ratchet,
    pub(crate) inumber: INumber,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PrivateRef {
    /// Sha3-256 hash of saturated namefilter.
    pub(crate) saturated_name_hash: HashOutput,
    /// Sha3-256 hash of the ratchet key.
    pub(crate) content_key: ContentKey,
    /// Skip-ratchet-derived key.
    pub(crate) ratchet_key: RatchetKey,
}

//--------------------------------------------------------------------------------------------------
// Implementations
//--------------------------------------------------------------------------------------------------

impl PrivateNode {
    /// Creates node with upserted modified time.
    pub fn upsert_mtime(&self, time: DateTime<Utc>) -> Self {
        match self {
            Self::File(file) => {
                let mut file = (**file).clone();
                file.metadata.upsert_mtime(time);
                Self::File(Rc::new(file))
            }
            Self::Dir(dir) => {
                let mut dir = (**dir).clone();
                dir.metadata.upsert_mtime(time);
                Self::Dir(Rc::new(dir))
            }
        }
    }

    /// Gets the header of the node.
    pub fn get_header(&self) -> &PrivateNodeHeader {
        match self {
            Self::File(file) => &file.header,
            Self::Dir(dir) => &dir.header,
        }
    }

    /// Casts a node to a directory.
    ///
    /// # Panics
    ///
    /// Panics if the node is not a directory.
    pub fn as_dir(&self) -> Result<Rc<PrivateDirectory>> {
        Ok(match self {
            Self::Dir(dir) => Rc::clone(dir),
            _ => bail!(FsError::NotADirectory),
        })
    }

    /// Casts a node to a file.
    ///
    /// # Panics
    ///
    /// Panics if the node is not a file.
    pub fn as_file(&self) -> Result<Rc<PrivateFile>> {
        Ok(match self {
            Self::File(file) => Rc::clone(file),
            _ => bail!(FsError::NotAFile),
        })
    }

    /// Returns true if underlying node is a directory.
    pub fn is_dir(&self) -> bool {
        matches!(self, Self::Dir(_))
    }

    /// Gets the latest version of the node using exponential search.
    pub(crate) async fn search_latest<B: BlockStore>(
        &self,
        forest: &PrivateForest,
        store: &B,
    ) -> Result<PrivateNode> {
        let header = self.get_header();

        let private_ref = &header.get_private_ref()?;
        if !forest.has(private_ref, store).await? {
            return Ok(self.clone());
        }

        // Start an exponential search.
        let mut search = RatchetExpSearcher::from(header.ratchet.clone());
        let mut current_header = header.clone();

        loop {
            let current = search.current();
            current_header.ratchet = current.clone();

            let has_curr = forest
                .has(&current_header.get_private_ref()?, store)
                .await?;

            let ord = if has_curr {
                Ordering::Less
            } else {
                Ordering::Greater
            };

            if !search.step(ord) {
                break;
            }
        }

        current_header.ratchet = search.current().clone();

        let latest_private_ref = current_header.get_private_ref()?;

        match forest.get(&latest_private_ref, store).await? {
            Some(node) => Ok(node),
            None => unreachable!(),
        }
    }

    /// Serializes the node with provided Serde serialilzer.
    pub fn serialize<S, R: Rng>(&self, serializer: S, rng: &mut R) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            PrivateNode::File(file) => file.serialize(serializer, rng),
            PrivateNode::Dir(dir) => dir.serialize(serializer, rng),
        }
    }

    /// Serializes the node to dag-cbor bytes.
    pub fn serialize_to_cbor<R: Rng>(&self, rng: &mut R) -> Result<Vec<u8>> {
        let ipld = self.serialize(ipld_serde::Serializer, rng)?;
        let mut bytes = Vec::new();
        ipld.encode(DagCborCodec, &mut bytes)?;
        Ok(bytes)
    }

    /// Deserializes the node from dag-cbor bytes.
    pub fn deserialize_from_cbor(bytes: &[u8], key: &RatchetKey) -> Result<Self> {
        let ipld = Ipld::decode(DagCborCodec, &mut Cursor::new(bytes))?;
        (ipld, key).try_into()
    }
}

impl TryFrom<(Ipld, &RatchetKey)> for PrivateNode {
    type Error = anyhow::Error;

    fn try_from(pair: (Ipld, &RatchetKey)) -> Result<Self> {
        match pair {
            (Ipld::Map(map), key) => {
                let r#type: NodeType = map
                    .get("type")
                    .ok_or(FsError::MissingNodeType)?
                    .try_into()?;

                Ok(match r#type {
                    NodeType::PrivateFile => {
                        PrivateNode::from(PrivateFile::deserialize(Ipld::Map(map), key)?)
                    }
                    NodeType::PrivateDirectory => {
                        PrivateNode::from(PrivateDirectory::deserialize(Ipld::Map(map), key)?)
                    }
                    other => bail!(FsError::UnexpectedNodeType(other)),
                })
            }
            other => bail!("Expected `Ipld::Map` got {:#?}", other),
        }
    }
}

impl Id for PrivateNode {
    fn get_id(&self) -> String {
        match self {
            Self::File(file) => file.get_id(),
            Self::Dir(dir) => dir.get_id(),
        }
    }
}

impl From<PrivateFile> for PrivateNode {
    fn from(file: PrivateFile) -> Self {
        Self::File(Rc::new(file))
    }
}

impl From<PrivateDirectory> for PrivateNode {
    fn from(dir: PrivateDirectory) -> Self {
        Self::Dir(Rc::new(dir))
    }
}

impl PrivateNodeHeader {
    /// Creates a new PrivateNodeHeader.
    pub fn new(parent_bare_name: Namefilter, inumber: INumber, ratchet_seed: HashOutput) -> Self {
        Self {
            bare_name: {
                let mut namefilter = parent_bare_name;
                namefilter.add(&inumber);
                namefilter
            },
            ratchet: Ratchet::zero(ratchet_seed),
            inumber,
        }
    }

    /// Advances the ratchet.
    pub fn advance_ratchet(&mut self) {
        self.ratchet.inc();
    }

    /// Gets the private ref of the current header.
    pub fn get_private_ref(&self) -> Result<PrivateRef> {
        let ratchet_key = Key::new(self.ratchet.derive_key());
        let saturated_name_hash = Sha3_256::hash(&self.get_saturated_name_with_key(&ratchet_key));

        Ok(PrivateRef {
            saturated_name_hash,
            content_key: ContentKey(Key::new(Sha3_256::hash(&ratchet_key.as_bytes()))),
            ratchet_key: RatchetKey(ratchet_key),
        })
    }

    /// Gets the saturated namefilter for this node using the provided ratchet key.
    pub fn get_saturated_name_with_key(&self, ratchet_key: &Key) -> Namefilter {
        let mut name = self.bare_name.clone();
        name.add(&ratchet_key.as_bytes());
        name.saturate();
        name
    }

    /// Gets the saturated namefilter for this node.
    #[inline]
    pub fn get_saturated_name(&self) -> Namefilter {
        let ratchet_key = Key::new(self.ratchet.derive_key());
        self.get_saturated_name_with_key(&ratchet_key)
    }
}

//--------------------------------------------------------------------------------------------------
// Tests
//--------------------------------------------------------------------------------------------------

#[cfg(test)]
mod private_node_tests {
    use crate::{private::Rng, utils::TestRng};

    use super::*;

    #[test]
    fn serialized_private_node_can_be_deserialized() {
        let rng = &mut TestRng();
        let original_file = PrivateNode::File(Rc::new(PrivateFile::new(
            Namefilter::default(),
            rng.random_bytes::<32>(),
            rng.random_bytes::<32>(),
            Utc::now(),
            b"Lorem ipsum dolor sit amet".to_vec(),
        )));
        let private_ref = original_file.get_header().get_private_ref().unwrap();

        let bytes = original_file.serialize_to_cbor(rng).unwrap();
        let deserialized_node =
            PrivateNode::deserialize_from_cbor(&bytes, &private_ref.ratchet_key).unwrap();

        assert_eq!(original_file, deserialized_node);
    }
}
