//! Public node system in-memory representation.

use std::{
    io::{Cursor, Read, Seek},
    rc::Rc,
    result,
};

use anyhow::{bail, Result};
use chrono::{DateTime, Utc};
use libipld::{cbor::DagCborCodec, codec::Decode, Cid};

use super::{Id, PublicDirectory, PublicFile};
use crate::{common::BlockStore, FsError, UnixFsNodeKind};

/// A node in a WNFS public file system. This can either be a file or a directory.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PublicNode {
    File(Rc<PublicFile>),
    Dir(Rc<PublicDirectory>),
}

impl PublicNode {
    /// Checks if the reference of one node is the same as the reference of another node.
    pub(crate) fn ptr_eq(&self, other: &PublicNode) -> bool {
        match (self, other) {
            (Self::File(self_file), Self::File(other_file)) => Rc::ptr_eq(self_file, other_file),
            (Self::Dir(self_dir), Self::Dir(other_dir)) => Rc::ptr_eq(self_dir, other_dir),
            _ => false,
        }
    }

    /// Create node with updated modified time.
    pub fn update_mtime(&self, time: DateTime<Utc>) -> Self {
        match self {
            Self::File(file) => {
                let mut file = (**file).clone();
                file.metadata.unix_fs.modified = time.timestamp();
                Self::File(Rc::new(file))
            }
            Self::Dir(dir) => {
                let mut dir = (**dir).clone();
                dir.metadata.unix_fs.modified = time.timestamp();
                Self::Dir(Rc::new(dir))
            }
        }
    }

    /// Create node with updated previous pointer value.
    pub fn update_previous(&self, cid: Option<Cid>) -> Self {
        match self {
            Self::File(file) => {
                let mut file = (**file).clone();
                file.previous = cid;
                Self::File(Rc::new(file))
            }
            Self::Dir(dir) => {
                let mut dir = (**dir).clone();
                dir.previous = cid;
                Self::Dir(Rc::new(dir))
            }
        }
    }

    /// Get previous ancestor of a node.
    pub fn get_previous(&self) -> Option<Cid> {
        match self {
            Self::File(file) => file.get_previous(),
            Self::Dir(dir) => dir.get_previous(),
        }
    }

    /// Casts a node to a directory.
    ///
    /// # Panics
    ///
    /// Panics if the node is not a directory.
    pub fn as_dir(&self) -> Result<Rc<PublicDirectory>> {
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
    pub fn as_file(&self) -> Result<Rc<PublicFile>> {
        Ok(match self {
            Self::File(file) => Rc::clone(file),
            _ => bail!(FsError::NotAFile),
        })
    }

    /// Stores a WNFS node as block(s) in chosen block store.
    pub async fn store<B: BlockStore>(&self, store: &mut B) -> Result<Cid> {
        Ok(match self {
            Self::File(file) => file.store(store).await?,
            Self::Dir(dir) => dir.store(store).await?,
        })
    }

    /// Returns true if underlying node is a directory.
    pub fn is_dir(&self) -> bool {
        matches!(self, Self::Dir(_))
    }

    /// Gets the node kind.
    pub fn kind(&self) -> UnixFsNodeKind {
        match self {
            Self::File(_) => UnixFsNodeKind::File,
            Self::Dir(_) => UnixFsNodeKind::Dir,
        }
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
            Ok(file) => PublicNode::File(Rc::new(file)),
            _ => {
                // If the file decode failed, we try to decode as a directory.
                let mut cursor = Cursor::new(try_file_cursor.into_inner());
                let dir = PublicDirectory::decode(c, &mut cursor)?;
                PublicNode::Dir(Rc::new(dir))
            }
        };

        Ok(node)
    }
}

#[cfg(test)]
mod public_node_tests {
    use std::{io::Cursor, rc::Rc};

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

        assert_eq!(PublicNode::File(Rc::new(file)), decoded_file);
    }

    #[async_std::test]
    async fn encoded_public_directory_can_be_decoded() {
        let directory = PublicDirectory::new(Utc::now());

        let mut store = MemoryBlockStore::default();

        let encoded_bytes = directory.encode(&mut store).await.unwrap();

        let mut cursor = Cursor::new(encoded_bytes);

        let decoded_directory = PublicNode::decode(DagCborCodec, &mut cursor).unwrap();

        assert_eq!(PublicNode::Dir(Rc::new(directory)), decoded_directory);
    }
}
