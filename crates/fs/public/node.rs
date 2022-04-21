//! Public file system in-memory representation.

use std::{
    io::{Cursor, Read, Seek},
    result,
};

use anyhow::Result;
use libipld::{cbor::DagCborCodec, codec::Decode, Cid};

use super::{Id, PublicDirectory, PublicFile};
use crate::common::BlockStore;

/// A node in a WNFS public file system. This can either be a file or a directory.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PublicNode {
    File(PublicFile),
    Dir(PublicDirectory),
}

impl PublicNode {
    /// Stores a WNFS node as block(s) in chosen block store.
    pub async fn store<B: BlockStore>(&self, store: &mut B) -> Result<Cid> {
        Ok(match self {
            PublicNode::File(file) => file.store(store).await?,
            PublicNode::Dir(dir) => dir.store(store).await?,
        })
    }

    /// Casts a node to an owned directory.
    ///
    /// # Panics
    ///
    /// Panics if the node is not a directory.
    pub fn into_dir(self) -> PublicDirectory {
        match self {
            PublicNode::Dir(dir) => dir,
            _ => unreachable!(),
        }
    }

    /// Casts a node to a directory.
    ///
    /// # Panics
    ///
    /// Panics if the node is not a directory.
    pub fn as_dir(&self) -> &PublicDirectory {
        match self {
            PublicNode::Dir(dir) => dir,
            _ => unreachable!(),
        }
    }

    /// Casts a node to a mutable directory.
    ///
    /// # Panics
    ///
    /// Panics if the node is not a directory.
    pub fn as_mut_dir(&mut self) -> &mut PublicDirectory {
        match self {
            PublicNode::Dir(dir) => dir,
            _ => unreachable!(),
        }
    }

    /// Returns true if underlying node is a directory.
    pub fn is_dir(&self) -> bool {
        matches!(self, PublicNode::Dir(_))
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

impl Decode<DagCborCodec> for PublicNode {
    fn decode<R: Read + Seek>(c: DagCborCodec, r: &mut R) -> Result<Self> {
        // NOTE(appcypher): There is really no great way to seek or peek at the data behind `r :: R: Read + Seek`.
        // So we just copy the whole data behind the opaque type which allows us to cursor over the data multiple times.
        // It is not ideal but it works.
        let bytes: Vec<u8> = r.bytes().collect::<result::Result<_, _>>()?;

        // We first try to decode as a file.
        let mut try_file_cursor = Cursor::new(bytes);
        let try_file_decode = PublicFile::decode(c, &mut try_file_cursor);

        let node = match try_file_decode {
            Ok(file) => PublicNode::File(file),
            _ => {
                // If the file decode failed, we try to decode as a directory.
                let mut cursor = Cursor::new(try_file_cursor.into_inner());
                let dir = PublicDirectory::decode(c, &mut cursor)?;
                PublicNode::Dir(dir)
            }
        };

        Ok(node)
    }
}

#[cfg(test)]
mod public_node_tests {
    use std::io::Cursor;

    use chrono::Utc;
    use libipld::{cbor::DagCborCodec, codec::Decode, prelude::Encode, Cid};

    use crate::{
        public::{PublicDirectory, PublicFile, PublicNode},
        MemoryBlockStore,
    };

    #[async_std::test]
    async fn encoded_public_file_can_be_decoded() {
        let file = PublicFile::new(Utc::now(), Cid::default());

        let mut encoded_bytes = vec![];

        file.encode(DagCborCodec, &mut encoded_bytes).unwrap();

        let mut cursor = Cursor::new(encoded_bytes);

        let decoded_file = PublicNode::decode(DagCborCodec, &mut cursor).unwrap();

        assert_eq!(PublicNode::File(file), decoded_file);
    }

    #[async_std::test]
    async fn encoded_public_directory_can_be_decoded() {
        let directory = PublicDirectory::new(Utc::now());

        let mut store = MemoryBlockStore::default();

        let encoded_bytes = directory.encode(&mut store).await.unwrap();

        let mut cursor = Cursor::new(encoded_bytes);

        let decoded_directory = PublicNode::decode(DagCborCodec, &mut cursor).unwrap();

        assert_eq!(PublicNode::Dir(directory), decoded_directory);
    }

    #[async_std::test]
    async fn public_node_can_be_casted_to_public_directory() {
        let directory = PublicDirectory::new(Utc::now());

        let node = PublicNode::Dir(directory.clone());

        assert_eq!(node.as_dir(), &directory);
    }
}
