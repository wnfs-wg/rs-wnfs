//! Public fs file node.
use anyhow::Result;
use chrono::{DateTime, Utc};
use libipld::Cid;
use semver::Version;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::collections::BTreeSet;
use wnfs_common::{BlockStore, Metadata, NodeType};

use crate::Id;

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
#[derive(Debug, Clone, PartialEq)]
pub struct PublicFile {
    pub version: Version,
    pub metadata: Metadata,
    pub userland: Cid,
    pub previous: BTreeSet<Cid>,
}

#[derive(Serialize, Deserialize)]
struct PublicFileSerializable {
    r#type: NodeType,
    version: Version,
    pub metadata: Metadata,
    pub userland: Cid,
    pub previous: Vec<Cid>,
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
            version: Version::new(0, 2, 0),
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
    #[inline(always)]
    pub async fn store(&self, store: &mut impl BlockStore) -> Result<Cid> {
        store.put_serializable(self).await
    }
}

impl Serialize for PublicFile {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        PublicFileSerializable {
            r#type: NodeType::PublicFile,
            version: self.version.clone(),
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
            version,
            metadata,
            userland,
            previous,
            ..
        } = PublicFileSerializable::deserialize(deserializer)?;

        Ok(Self {
            version,
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

//--------------------------------------------------------------------------------------------------
// Tests
//--------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use crate::public::PublicFile;
    use chrono::Utc;
    use libipld::Cid;
    use wnfs_common::dagcbor;

    #[async_std::test]
    async fn serialized_public_file_can_be_deserialized() {
        let original_file = PublicFile::new(Utc::now(), Cid::default());

        let serialized_file = dagcbor::encode(&original_file).unwrap();
        let deserialized_file: PublicFile = dagcbor::decode(serialized_file.as_ref()).unwrap();

        assert_eq!(deserialized_file, original_file);
    }
}
