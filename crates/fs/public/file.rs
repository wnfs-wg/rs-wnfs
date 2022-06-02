//! Public fs file node.

use std::rc::Rc;

use anyhow::Result;
use chrono::{DateTime, Utc};
use libipld::{cbor::DagCborCodec, prelude::Encode, Cid, DagCbor, IpldCodec};
use serde::{Serialize, Deserialize};

use crate::{BlockStore, Metadata, UnixFsNodeKind};

use super::Id;

/// A file in a WNFS public file system.
///
/// # Examples
///
/// ```
/// use wnfs::{PublicFile, Id};
/// use chrono::Utc;
/// use libipld::Cid;
///
/// let file = PublicFile::new(Utc::now(), Cid::default());
///
/// println!("id = {}", file.get_id());
/// ```
#[derive(Debug, Clone, PartialEq, Eq, DagCbor, Serialize, Deserialize)]
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
    /// use wnfs::{PublicFile, Id};
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
    pub async fn store<B: BlockStore>(&self, store: &mut B) -> Result<Cid> {
        let bytes = {
            let mut tmp = vec![];
            self.encode(DagCborCodec, &mut tmp)?;
            tmp
        };
        store.put_block(bytes, IpldCodec::DagCbor).await
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
    use std::io::Cursor;

    use chrono::Utc;
    use libipld::prelude::Decode;

    use super::*;

    #[async_std::test]
    async fn file_can_encode_decode_as_cbor() {
        let file = PublicFile::new(Utc::now(), Cid::default());

        let mut encoded_bytes = vec![];

        file.encode(DagCborCodec, &mut encoded_bytes).unwrap();

        let mut cursor = Cursor::new(encoded_bytes);

        let decoded_file = PublicFile::decode(DagCborCodec, &mut cursor).unwrap();

        assert_eq!(file, decoded_file);
    }
}
