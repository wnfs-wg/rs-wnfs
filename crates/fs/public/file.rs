//! Public fs file node.

use std::rc::Rc;

use anyhow::Result;

use chrono::{DateTime, Utc};
use libipld::Cid;
use serde::{Deserialize, Serialize};

use crate::{BlockStore, Id, Metadata, UnixFsNodeKind};

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
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublicFile {
    pub(crate) metadata: Metadata,
    pub(crate) userland: Cid,
    pub(crate) previous: Option<Cid>,
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
            metadata: Metadata::new(time, UnixFsNodeKind::File),
            userland,
            previous: None,
        }
    }

    // Gets the previous value of the file.
    pub fn get_previous(self: &Rc<Self>) -> Option<Cid> {
        self.previous
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
