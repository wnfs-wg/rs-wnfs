//! Public fs file node.

use std::rc::Rc;

use anyhow::Result;

use chrono::{DateTime, Utc};
use libipld::Cid;
use semver::Version;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::{BlockStore, Id, Metadata, NodeType};

/// A file in a WNFS public file system.
///
/// # Examples
///
/// ```
/// use wnfs::{public::PublicFile, Id};
/// use chrono::Utc;
/// use libipld::Cid;
///
/// let file = PublicFile::new(Utc::now(), Cid::default());
///
/// println!("id = {}", file.get_id());
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct PublicFile {
    pub version: Version,
    pub metadata: Metadata,
    pub userland: Cid,
    pub previous: Option<Cid>,
}

#[derive(Serialize, Deserialize)]
struct PublicFileSerde {
    r#type: NodeType,
    version: Version,
    pub metadata: Metadata,
    pub userland: Cid,
    pub previous: Option<Cid>,
}

//--------------------------------------------------------------------------------------------------
// Implementations
//--------------------------------------------------------------------------------------------------

impl PublicFile {
    /// Creates a new file using the given metadata and CID.
    ///
    /// # Examples
    ///
    /// ```
    /// use wnfs::{public::PublicFile, Id};
    /// use chrono::Utc;
    /// use libipld::Cid;
    ///
    /// let file = PublicFile::new(Utc::now(), Cid::default());
    ///
    /// println!("id = {}", file.get_id());
    /// ```
    pub fn new(time: DateTime<Utc>, userland: Cid) -> Self {
        Self {
            version: Version::new(0, 2, 0),
            metadata: Metadata::new(time),
            userland,
            previous: None,
        }
    }

    /// Gets the previous value of the file.
    pub fn get_previous(self: &Rc<Self>) -> Option<Cid> {
        self.previous
    }

    /// Gets the metadata of the file
    pub fn get_metadata<'a>(self: &'a Rc<Self>) -> &'a Metadata {
        &self.metadata
    }

    /// Gets the content cid of a file
    pub fn get_content_cid(self: &Rc<Self>) -> Cid {
        self.userland
    }

    /// Stores file in provided block store.
    ///
    /// # Examples
    ///
    /// ```
    /// use wnfs::{public::PublicFile, Id, MemoryBlockStore};
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
    pub async fn store<B: BlockStore>(&self, store: &mut B) -> Result<Cid> {
        store.put_serializable(self).await
    }
}

impl Serialize for PublicFile {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        PublicFileSerde {
            r#type: NodeType::PublicFile,
            version: self.version.clone(),
            metadata: self.metadata.clone(),
            userland: self.userland,
            previous: self.previous,
        }
        .serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for PublicFile {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let PublicFileSerde {
            version,
            metadata,
            userland,
            previous,
            ..
        } = PublicFileSerde::deserialize(deserializer)?;

        Ok(Self {
            version,
            metadata,
            userland,
            previous,
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
mod public_file_tests {
    use chrono::Utc;
    use libipld::Cid;

    use crate::{dagcbor, public::PublicFile};

    #[async_std::test]
    async fn serialized_public_file_can_be_deserialized() {
        let original_file = PublicFile::new(Utc::now(), Cid::default());

        let serialized_file = dagcbor::encode(&original_file).unwrap();
        let deserialized_file: PublicFile = dagcbor::decode(serialized_file.as_ref()).unwrap();

        assert_eq!(deserialized_file, original_file);
    }
}
