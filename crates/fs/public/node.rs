//! Public file system in-memory representation.

use std::{
    collections::BTreeMap,
    io::{Read, Seek},
};

use anyhow::Result;
use libipld::{cbor::DagCborCodec, codec::Decode, Cid};

use super::{Link, PublicDirectory, PublicFile};
use crate::{common::BlockStore, Metadata, UnixFsNodeKind};

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
}

impl Decode<DagCborCodec> for PublicNode {
    fn decode<R: Read + Seek>(c: DagCborCodec, r: &mut R) -> Result<Self> {
        let metadata = Metadata::decode(c, r)?;
        let node = if matches!(metadata.unix_fs.kind, UnixFsNodeKind::File) {
            let userland = Cid::decode(c, r)?;
            let previous = <Option<Cid>>::decode(c, r)?;

            PublicNode::File(PublicFile {
                metadata,
                userland,
                previous,
            })
        } else {
            let userland = BTreeMap::<String, Cid>::decode(c, r)?
                .into_iter()
                .map(|(k, cid)| (k, Link::Cid(cid)))
                .collect();

            let previous = <Option<Cid>>::decode(c, r)?;

            PublicNode::Dir(PublicDirectory {
                metadata,
                userland,
                previous,
            })
        };

        Ok(node)
    }
}

#[cfg(test)]
mod public_node_tests {
    use std::io::Cursor;

    use chrono::Utc;
    use libipld::{cbor::DagCborCodec, codec::Decode, Cid, prelude::Encode};

    use crate::{
        public::{PublicDirectory, PublicFile, PublicNode},
        MemoryBlockStore,
    };

    // #[async_std::test]
    // async fn encoded_public_file_decoded_successfully() {
    //     let file = PublicFile::new(Utc::now(), Cid::default());

    //     dbg!(&file);

    //     let mut encoded_bytes = vec![];

    //     file.encode(DagCborCodec, &mut encoded_bytes).unwrap();

    //     dbg!(format!("{:02x?}", encoded_bytes));

    //     let mut cursor = Cursor::new(encoded_bytes);

    //     let decoded_file = PublicNode::decode(DagCborCodec, &mut cursor).unwrap();

    //     assert_eq!(PublicNode::File(file), decoded_file);
    // }

    #[async_std::test]
    async fn encoded_public_directory_decoded_successfully() {
        let directory = PublicDirectory::new(Utc::now());

        let mut store = MemoryBlockStore::default();

        let encoded_bytes = directory.encode(&mut store).await.unwrap();

        let mut cursor = Cursor::new(encoded_bytes);

        let decoded_directory = PublicNode::decode(DagCborCodec, &mut cursor).unwrap();

        assert_eq!(PublicNode::Dir(directory), decoded_directory);
    }

    #[async_std::test]
    async fn public_directory_casted_successfully() {
        let directory = PublicDirectory::new(Utc::now());

        let node = PublicNode::Dir(directory.clone());

        assert_eq!(node.as_dir(), &directory);
    }
}
