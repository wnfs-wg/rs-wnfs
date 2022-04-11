//! Public fs directory node.

use std::{
    collections::BTreeMap,
    io::{Read, Seek},
    rc::Rc,
};

use crate::{error, BlockStore, FsError, Metadata, UnixFsNodeKind};
use anyhow::Result;
use async_recursion::async_recursion;
use chrono::{DateTime, Utc};
use libipld::{
    cbor::DagCborCodec,
    codec::{Decode, Encode},
    Cid, IpldCodec,
};

use super::{Link, PublicNode};

/// A directory in a WNFS public file system.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublicDirectory {
    pub(crate) metadata: Metadata,
    pub(crate) userland: BTreeMap<String, Link>,
    pub(crate) previous: Option<Cid>,
}

impl PublicDirectory {
    /// Creates a new directory using the given metadata.
    pub fn new(time: DateTime<Utc>) -> Self {
        Self {
            metadata: Metadata::new(time, UnixFsNodeKind::Dir),
            userland: BTreeMap::new(),
            previous: None,
        }
    }

    /// Follows a path and fetches the node at the end of the path.
    pub async fn get_node<B: BlockStore>(
        &self,
        path_segments: &[String],
        store: &B,
    ) -> Result<Rc<PublicNode>> {
        if path_segments.is_empty() {
            return error(FsError::InvalidPath);
        }

        let mut working_node: Rc<PublicNode> = Rc::new(PublicNode::Dir(self.clone()));

        // Iterate over the path segments until we get the node of the last segment.
        for (index, segment) in path_segments.iter().enumerate() {
            // Cast working node to directory.
            let dir = working_node.as_dir();

            // Fetch node representing path segment in working directory.
            if let Some(node) = dir.lookup_node(segment, store).await? {
                match node.as_ref() {
                    PublicNode::Dir(_) => {
                        // If the node is a directory, set it as the working node.
                        working_node = Rc::clone(&node);
                    }
                    PublicNode::File(_) => {
                        // If the node is a file, we return it if it's the last segment.
                        if index != path_segments.len() - 1 {
                            return error(FsError::InvalidPath);
                        }
                        working_node = Rc::clone(&node);
                        break;
                    }
                }

                // We continue loop after setting the working node to a directory node.
                continue;
            }

            // If the node is not found, we return an error.
            return error(FsError::NodeNotFound);
        }

        Ok(working_node)
    }

    /// Looks up a node by its path name in the current directory.
    ///
    /// TODO(appcypher): What is a valid path segment identifier?
    pub async fn lookup_node<B: BlockStore>(
        &self,
        path_segment: &str,
        store: &B,
    ) -> Result<Option<Rc<PublicNode>>> {
        Ok(match self.userland.get(path_segment) {
            Some(link) => Some(link.resolve(store).await?),
            None => None,
        })
    }

    /// Encode the directory as a CBOR object.
    pub async fn encode<B: BlockStore>(&self, store: &mut B) -> Result<Vec<u8>> {
        let mut bytes = Vec::new();
        self.metadata.encode(DagCborCodec, &mut bytes)?;

        let new_userland = {
            let mut tmp = BTreeMap::new();
            for (k, link) in self.userland.iter() {
                let cid = link.seal(store).await?;
                tmp.insert(k.clone(), cid);
            }
            tmp
        };

        new_userland.encode(DagCborCodec, &mut bytes)?;

        self.previous.encode(DagCborCodec, &mut bytes)?;
        Ok(bytes)
    }

    /// Stores WNFS directory as block(s) in chosen block store.
    ///
    /// This function can be recursive if the directory contains other directories.
    #[async_recursion(?Send)]
    pub async fn store<B: BlockStore>(&self, store: &mut B) -> Result<Cid> {
        let bytes = self.encode(store).await?;
        store.put_block(bytes, IpldCodec::DagCbor).await
    }

    /// Writes a file to the directory.
    ///
    /// Rather than mutate the directory directly, we create a new directory and return it.
    pub async fn write<B: BlockStore>(
        &self,
        path: &[String],
        content_cid: &Cid,
        store: &mut B,
    ) -> Result<Self> {
        // TODO(appcypher): Implement this.
        todo!()
    }
}

impl Decode<DagCborCodec> for PublicDirectory {
    fn decode<R: Read + Seek>(c: DagCborCodec, r: &mut R) -> Result<Self> {
        let metadata = Metadata::decode(c, r)?;
        let userland = BTreeMap::<String, Cid>::decode(c, r)?
            .into_iter()
            .map(|(k, cid)| (k, Link::Cid(cid)))
            .collect();

        let previous = Option::<Cid>::decode(c, r)?;

        Ok(Self {
            metadata,
            userland,
            previous,
        })
    }
}

#[cfg(test)]
mod public_directory_tests {
    use std::io::Cursor;

    use super::*;
    use crate::{BlockStoreLookup, MemoryBlockStore};
    use chrono::Utc;

    // #[async_std::test]
    // async fn files_added_to_directory_looked_up_unsuccessful() {
    //     let root = PublicDirectory::new(Utc::now());

    //     let mut store = MemoryBlockStore::default();

    //     let content_cid = &Cid::default();

    //     let root = root
    //         .write(&[String::from("text.txt")], content_cid, &mut store)
    //         .await
    //         .unwrap();

    //     let node = root.lookup_node("text.txt", &store).await;

    //     assert!(node.is_ok());

    //     assert_eq!(node.unwrap(), None);
    // }

    #[async_std::test]
    async fn files_not_added_to_directory_not_looked_up_unsuccessful() {
        let root = PublicDirectory::new(Utc::now());

        let store = MemoryBlockStore::default();

        let node = root.lookup_node("Unknown", &store).await;

        assert!(node.is_ok());

        assert_eq!(node.unwrap(), None);
    }

    #[async_std::test]
    async fn directory_added_to_store_fetched_successfully() {
        let root = PublicDirectory::new(Utc::now());

        let mut store = MemoryBlockStore::default();

        let cid = root.store(&mut store).await.unwrap();

        let bytes = store.get_block(&cid).await.unwrap();

        let mut cursor = Cursor::new(bytes);

        let decoded_root = PublicDirectory::decode(DagCborCodec, &mut cursor).unwrap();

        assert_eq!(root, decoded_root);
    }

    #[async_std::test]
    async fn directory_encode_decode_successful() {
        let root = PublicDirectory::new(Utc::now());

        let mut store = MemoryBlockStore::default();

        let encoded_bytes = root.encode(&mut store).await.unwrap();

        let mut cursor = Cursor::new(encoded_bytes);

        let decoded_root = PublicDirectory::decode(DagCborCodec, &mut cursor).unwrap();

        assert_eq!(root, decoded_root);
    }
}
