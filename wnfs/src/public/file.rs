//! Public fs file node.

use std::{collections::BTreeSet, rc::Rc};

use anyhow::Result;

use async_once_cell::OnceCell;
use chrono::{DateTime, Utc};
use libipld::Cid;
use semver::Version;
use serde::{de::Error as DeError, Deserialize, Deserializer, Serialize, Serializer};

use crate::{BlockStore, FsError, Id, Metadata, NodeType, RemembersPersistence};

/// Represents a file in the WNFS public filesystem.
///
/// # Examples
///
/// ```
/// use wnfs::PublicFile;
/// use chrono::Utc;
/// use libipld::Cid;
///
/// let file = PublicFile::new(Utc::now(), Cid::default());
///
/// println!("File: {:?}", file);
/// ```
#[derive(Debug)]
pub struct PublicFile {
    persisted_as: OnceCell<Cid>,
    pub metadata: Metadata,
    pub userland: Cid,
    pub previous: BTreeSet<Cid>,
}

#[derive(Serialize, Deserialize)]
struct PublicFileSerializable {
    r#type: NodeType,
    version: Version,
    metadata: Metadata,
    userland: Cid,
    previous: Vec<Cid>,
}

//--------------------------------------------------------------------------------------------------
// Implementations
//--------------------------------------------------------------------------------------------------

impl PublicFile {
    /// Creates a new file with provided content CID.
    ///
    /// # Examples
    ///
    /// ```
    /// use wnfs::PublicFile;
    /// use chrono::Utc;
    /// use libipld::Cid;
    ///
    /// let file = PublicFile::new(Utc::now(), Cid::default());
    ///
    /// println!("File: {:?}", file);
    /// ```
    pub fn new(time: DateTime<Utc>, content_cid: Cid) -> Self {
        Self {
            persisted_as: OnceCell::new(),
            metadata: Metadata::new(time),
            userland: content_cid,
            previous: BTreeSet::new(),
        }
    }

    /// Gets the previous value of the file.
    ///
    /// # Examples
    ///
    /// ```
    /// use wnfs::{PublicDirectory, Id};
    /// use chrono::Utc;
    ///
    /// let dir = PublicDirectory::new(Utc::now());
    ///
    /// println!("id = {}", dir.get_id());
    /// ```
    pub fn get_previous(&self) -> &BTreeSet<Cid> {
        &self.previous
    }

    /// Gets the metadata of the file
    pub fn get_metadata(&self) -> &Metadata {
        &self.metadata
    }

    /// Gets the content cid of a file
    pub fn get_content_cid(&self) -> &Cid {
        &self.userland
    }

    /// Takes care of creating previous links, in case the current
    /// file was previously `.store()`ed.
    /// In any case it'll try to give you ownership of the file if possible,
    /// otherwise it clones.
    pub(crate) fn prepare_next_revision(self: Rc<Self>) -> Self {
        let Some(previous_cid) = self.persisted_as.get().cloned() else {
            // If this revision was not yet persisted, we can
            // modify it without forcing it to be flushed to a
            // BlockStore.
            return Rc::try_unwrap(self).unwrap_or_else(|rc| (*rc).clone());
        };

        let mut cloned = Rc::try_unwrap(self).unwrap_or_else(|rc| (*rc).clone());
        // We need to reset the OnceCell.
        cloned.persisted_as = OnceCell::new();
        cloned.previous = [previous_cid].into_iter().collect();

        cloned
    }

    /// Stores file in provided block store.
    ///
    /// # Examples
    ///
    /// ```
    /// use wnfs::{PublicFile, Id, MemoryBlockStore};
    /// use chrono::Utc;
    /// use libipld::Cid;
    ///
    /// #[async_std::main]
    /// async fn main() {
    ///     let mut store = MemoryBlockStore::default();
    ///     let file = PublicFile::new(Utc::now(), Cid::default());
    ///
    ///     file.store(&mut store).await.unwrap();
    /// }
    /// ```
    pub async fn store(&self, store: &mut impl BlockStore) -> Result<Cid> {
        Ok(*self
            .persisted_as
            .get_or_try_init(async { store.put_serializable(self).await })
            .await?)
    }
}

impl Serialize for PublicFile {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        PublicFileSerializable {
            r#type: NodeType::PublicFile,
            version: Version::new(0, 2, 0),
            metadata: self.metadata.clone(),
            userland: self.userland,
            previous: self.previous.iter().cloned().collect(),
        }
        .serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for PublicFile {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let PublicFileSerializable {
            r#type,
            version,
            metadata,
            userland,
            previous,
        } = PublicFileSerializable::deserialize(deserializer)?;

        if version.major != 0 || version.minor != 2 {
            return Err(DeError::custom(FsError::UnexpectedVersion(version)));
        }

        if r#type != NodeType::PublicFile {
            return Err(DeError::custom(FsError::UnexpectedNodeType(r#type)));
        }

        Ok(Self {
            persisted_as: OnceCell::new(),
            metadata,
            userland,
            previous: previous.iter().cloned().collect(),
        })
    }
}

impl Id for PublicFile {
    fn get_id(&self) -> String {
        format!("{:p}", &self.metadata)
    }
}

impl PartialEq for PublicFile {
    fn eq(&self, other: &Self) -> bool {
        self.metadata == other.metadata
            && self.userland == other.userland
            && self.previous == other.previous
    }
}

impl Clone for PublicFile {
    fn clone(&self) -> Self {
        Self {
            persisted_as: OnceCell::new_with(self.persisted_as.get().cloned()),
            metadata: self.metadata.clone(),
            userland: self.userland,
            previous: self.previous.clone(),
        }
    }
}

impl RemembersPersistence for PublicFile {
    fn persisted_as(&self) -> &OnceCell<Cid> {
        &self.persisted_as
    }
}

//--------------------------------------------------------------------------------------------------
// Tests
//--------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use crate::{dagcbor, public::PublicFile, BlockStore, MemoryBlockStore};
    use chrono::Utc;
    use libipld::{Cid, IpldCodec};
    use std::rc::Rc;

    #[async_std::test]
    async fn serialized_public_file_can_be_deserialized() {
        let original_file = PublicFile::new(Utc::now(), Cid::default());

        let serialized_file = dagcbor::encode(&original_file).unwrap();
        let deserialized_file: PublicFile = dagcbor::decode(serialized_file.as_ref()).unwrap();

        assert_eq!(deserialized_file, original_file);
    }

    #[async_std::test]
    async fn previous_links_get_set() {
        let time = Utc::now();
        let store = &mut MemoryBlockStore::default();

        let content_cid = store
            .put_block(b"Hello World".to_vec(), IpldCodec::Raw)
            .await
            .unwrap();

        let file = Rc::new(PublicFile::new(time, content_cid));

        let previous_cid = file.store(store).await.unwrap();

        let next_file = file.prepare_next_revision();

        assert_eq!(
            next_file.previous.into_iter().collect::<Vec<_>>(),
            vec![previous_cid]
        );
    }
}
