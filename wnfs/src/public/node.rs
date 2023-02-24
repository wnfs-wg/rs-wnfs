//! Public node system in-memory representation.

use std::{collections::BTreeSet, rc::Rc};

use anyhow::{bail, Result};
use async_once_cell::OnceCell;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use libipld::{Cid, Ipld};
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

use super::{PublicDirectory, PublicFile};
use crate::{common::BlockStore, AsyncSerialize, FsError, Id, NodeType, RemembersCid};

//--------------------------------------------------------------------------------------------------
// Type Definitions
//--------------------------------------------------------------------------------------------------

/// Represents a node in the WNFS public file system. This can either be a file or a directory.
///
/// # Examples
///
/// ```
/// use wnfs::{PublicDirectory, PublicNode};
/// use chrono::Utc;
/// use std::rc::Rc;
///
/// let dir = Rc::new(PublicDirectory::new(Utc::now()));
/// let node = PublicNode::Dir(dir);
///
/// println!("Node: {:?}", node);
/// ```
#[derive(Debug, Clone)]
pub enum PublicNode {
    File(Rc<PublicFile>),
    Dir(Rc<PublicDirectory>),
}

//--------------------------------------------------------------------------------------------------
// Implementations
//--------------------------------------------------------------------------------------------------

impl PublicNode {
    /// Creates node with upserted modified time.
    ///
    /// # Examples
    ///
    /// ```
    /// use wnfs::{PublicDirectory, PublicNode};
    /// use chrono::{Utc, Duration, TimeZone};
    /// use std::rc::Rc;
    ///
    /// let dir = Rc::new(PublicDirectory::new(Utc::now()));
    /// let node = PublicNode::Dir(dir);
    ///
    /// let time = Utc::now();
    /// let node = node.upsert_mtime(time);
    ///
    /// let imprecise_time = Utc.timestamp_opt(time.timestamp(), 0).single();
    /// assert_eq!(
    ///     imprecise_time,
    ///     node.as_dir()
    ///         .unwrap()
    ///         .get_metadata()
    ///         .get_modified()
    /// );
    /// ```
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

    /// Creates node with updated previous pointer value.
    ///
    /// # Examples
    ///
    /// ```
    /// use wnfs::{PublicDirectory, PublicNode};
    /// use chrono::Utc;
    /// use libipld::Cid;
    /// use std::{rc::Rc, collections::BTreeSet};
    ///
    /// let dir = Rc::new(PublicDirectory::new(Utc::now()));
    /// let node = PublicNode::Dir(dir);
    ///
    /// let new_cids = [Cid::default()];
    /// let node = node.update_previous(new_cids.to_vec());
    ///
    /// assert_eq!(
    ///     &BTreeSet::from(new_cids),
    ///     node.as_dir()
    ///         .unwrap()
    ///         .get_previous()
    /// );
    /// ```
    pub fn update_previous(&self, cids: Vec<Cid>) -> Self {
        match self {
            Self::File(file) => {
                let mut file = (**file).clone();
                file.previous = cids.into_iter().collect();
                Self::File(Rc::new(file))
            }
            Self::Dir(dir) => {
                let mut dir = (**dir).clone();
                dir.previous = cids.into_iter().collect();
                Self::Dir(Rc::new(dir))
            }
        }
    }

    /// Gets previous ancestor of a node.
    ///
    /// # Examples
    ///
    /// ```
    /// use wnfs::{PublicDirectory, PublicNode};
    /// use chrono::Utc;
    /// use std::rc::Rc;
    ///
    /// let dir = Rc::new(PublicDirectory::new(Utc::now()));
    /// let node = PublicNode::Dir(dir);
    ///
    /// assert_eq!(
    ///     node.get_previous(),
    ///     node.as_dir()
    ///         .unwrap()
    ///         .get_previous()
    /// );
    /// ```
    pub fn get_previous(&self) -> &BTreeSet<Cid> {
        match self {
            Self::File(file) => file.get_previous(),
            Self::Dir(dir) => dir.get_previous(),
        }
    }

    /// Casts a node to a directory.
    ///
    /// # Examples
    ///
    /// ```
    /// use wnfs::{PublicDirectory, PublicNode};
    /// use chrono::Utc;
    /// use std::rc::Rc;
    ///
    /// let dir = Rc::new(PublicDirectory::new(Utc::now()));
    /// let node = PublicNode::Dir(Rc::clone(&dir));
    ///
    /// assert_eq!(node.as_dir().unwrap(), dir);
    /// ```
    pub fn as_dir(&self) -> Result<Rc<PublicDirectory>> {
        Ok(match self {
            Self::Dir(dir) => Rc::clone(dir),
            _ => bail!(FsError::NotADirectory),
        })
    }

    /// Casts a node to a file.
    ///
    /// # Examples
    ///
    /// ```
    /// use wnfs::{PublicFile, PublicNode};
    /// use chrono::Utc;
    /// use std::rc::Rc;
    /// use libipld::Cid;
    ///
    /// let file = Rc::new(PublicFile::new(Utc::now(), Cid::default()));
    /// let node = PublicNode::File(Rc::clone(&file));
    ///
    /// assert_eq!(node.as_file().unwrap(), file);
    /// ```
    pub fn as_file(&self) -> Result<Rc<PublicFile>> {
        Ok(match self {
            Self::File(file) => Rc::clone(file),
            _ => bail!(FsError::NotAFile),
        })
    }

    /// Serializes a node to the block store.
    pub async fn store(&self, store: &mut impl BlockStore) -> Result<Cid> {
        Ok(match self {
            Self::File(file) => file.store(store).await?,
            Self::Dir(dir) => dir.store(store).await?,
        })
    }

    /// Returns true if underlying node is a directory.
    ///
    /// # Examples
    ///
    /// ```
    /// use wnfs::{PublicDirectory, PublicNode};
    /// use chrono::Utc;
    /// use std::rc::Rc;
    ///
    /// let dir = Rc::new(PublicDirectory::new(Utc::now()));
    /// let node = PublicNode::Dir(dir);
    ///
    /// assert!(node.is_dir());
    /// ```
    pub fn is_dir(&self) -> bool {
        matches!(self, Self::Dir(_))
    }

    /// Returns true if the underlying node is a file.
    ///
    /// # Examples
    ///
    /// ```
    /// use wnfs::{PublicFile, PublicNode};
    /// use chrono::Utc;
    /// use std::rc::Rc;
    /// use libipld::Cid;
    ///
    /// let file = Rc::new(PublicFile::new(Utc::now(), Cid::default()));
    /// let node = PublicNode::File(file);
    ///
    /// assert!(node.is_file());
    /// ```
    pub fn is_file(&self) -> bool {
        matches!(self, Self::File(_))
    }
}

impl Id for PublicNode {
    fn get_id(&self) -> String {
        match self {
            PublicNode::File(file) => file.get_id(),
            PublicNode::Dir(dir) => dir.get_id(),
        }
    }
}

impl PartialEq for PublicNode {
    fn eq(&self, other: &PublicNode) -> bool {
        match (self, other) {
            (Self::File(self_file), Self::File(other_file)) => {
                Rc::ptr_eq(self_file, other_file) || self_file == other_file
            }
            (Self::Dir(self_dir), Self::Dir(other_dir)) => {
                Rc::ptr_eq(self_dir, other_dir) || self_dir == other_dir
            }
            _ => false,
        }
    }
}

impl<'de> Deserialize<'de> for PublicNode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ipld::deserialize(deserializer).and_then(|ipld| ipld.try_into().map_err(de::Error::custom))
    }
}

impl TryFrom<Ipld> for PublicNode {
    type Error = anyhow::Error;

    fn try_from(ipld: Ipld) -> Result<Self> {
        match ipld {
            Ipld::Map(map) => {
                let r#type: NodeType = map
                    .get("type")
                    .ok_or(FsError::MissingNodeType)?
                    .try_into()?;

                Ok(match r#type {
                    NodeType::PublicFile => {
                        PublicNode::from(PublicFile::deserialize(Ipld::Map(map))?)
                    }
                    NodeType::PublicDirectory => {
                        PublicNode::from(PublicDirectory::deserialize(Ipld::Map(map))?)
                    }
                    other => bail!(FsError::UnexpectedNodeType(other)),
                })
            }
            other => bail!("Expected `Ipld::Map` got {:#?}", other),
        }
    }
}

impl From<PublicFile> for PublicNode {
    fn from(file: PublicFile) -> Self {
        Self::File(Rc::new(file))
    }
}

impl From<PublicDirectory> for PublicNode {
    fn from(dir: PublicDirectory) -> Self {
        Self::Dir(Rc::new(dir))
    }
}

/// Implements async deserialization for serde serializable types.
#[async_trait(?Send)]
impl AsyncSerialize for PublicNode {
    async fn async_serialize<S, B>(&self, serializer: S, store: &mut B) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        B: BlockStore + ?Sized,
    {
        match self {
            Self::File(file) => file.serialize(serializer),
            Self::Dir(dir) => dir.async_serialize(serializer, store).await,
        }
    }
}

impl RemembersCid for PublicNode {
    fn persisted_as(&self) -> &OnceCell<Cid> {
        match self {
            PublicNode::File(file) => (*file).persisted_as(),
            PublicNode::Dir(dir) => (*dir).persisted_as(),
        }
    }
}

//--------------------------------------------------------------------------------------------------
// Tests
//--------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use chrono::Utc;
    use libipld::Cid;

    use crate::{
        dagcbor,
        public::{PublicDirectory, PublicFile, PublicNode},
        MemoryBlockStore,
    };

    #[async_std::test]
    async fn serialized_public_file_can_be_deserialized() {
        let store = &mut MemoryBlockStore::default();
        let original_node_file =
            PublicNode::File(Rc::new(PublicFile::new(Utc::now(), Cid::default())));

        let serialized_node_file = dagcbor::async_encode(&original_node_file, store)
            .await
            .unwrap();

        let deserialized_node_file: PublicNode =
            dagcbor::decode(serialized_node_file.as_ref()).unwrap();

        assert_eq!(deserialized_node_file, original_node_file);
    }

    #[async_std::test]
    async fn serialized_public_directory_can_be_deserialized() {
        let store = &mut MemoryBlockStore::default();
        let original_node_dir = PublicNode::Dir(Rc::new(PublicDirectory::new(Utc::now())));

        let serialized_node_dir = dagcbor::async_encode(&original_node_dir, store)
            .await
            .unwrap();

        let deserialized_node_dir: PublicNode =
            dagcbor::decode(serialized_node_dir.as_ref()).unwrap();

        assert_eq!(deserialized_node_dir, original_node_dir);
    }
}
