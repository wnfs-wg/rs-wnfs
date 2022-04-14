//! Public fs directory node.

use std::{
    cell::RefCell,
    cmp::Ordering,
    collections::BTreeMap,
    io::{Read, Seek},
    mem,
    rc::Rc,
};

use crate::{error, BlockStore, FsError, Metadata, UnixFsNodeKind};
use anyhow::Result;
use async_recursion::async_recursion;
use chrono::{DateTime, Utc};
use field_names::FieldNames;
use libipld::{
    cbor::{cbor::MajorKind, decode, encode, DagCborCodec},
    codec::{Decode, Encode},
    Cid, IpldCodec,
};

use super::{Link, PublicNode};

/// A directory in a WNFS public file system.
#[derive(Debug, Clone, PartialEq, Eq, FieldNames)]
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
    ) -> Result<Option<Rc<RefCell<PublicNode>>>> {
        if path_segments.is_empty() {
            return error(FsError::InvalidPath);
        }

        // TODO(appcypher): Any way to avoid this clone?
        let mut working_node = Some(Rc::new(RefCell::new(PublicNode::Dir(self.clone()))));

        // Iterate over the path segments until we get the node of the last segment.
        for (index, segment) in path_segments.iter().enumerate() {
            // Cast working node to directory.
            let node_rc = working_node.unwrap();
            let node_ref = node_rc.borrow();
            let dir = node_ref.as_dir();

            // Fetch node representing path segment in working directory.
            if let Some(found_node) = dir.lookup_node(segment, store).await? {
                match *found_node.borrow() {
                    // If the node is a directory, set it as the working node.
                    PublicNode::Dir(_) => {
                        working_node = Some(Rc::clone(&node_rc));
                    }
                    // If the node is a file, we return it if it's the last segment.
                    PublicNode::File(_) => {
                        if index != path_segments.len() - 1 {
                            return error(FsError::InvalidPath);
                        }
                        working_node = Some(Rc::clone(&node_rc));
                        break;
                    }
                }

                // We continue loop after setting the working node to a directory node.
                continue;
            }

            // If the node is not found, we return an none.
            return Ok(None);
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
    ) -> Result<Option<Rc<RefCell<PublicNode>>>> {
        Ok(match self.userland.get(path_segment) {
            Some(link) => Some(link.resolve(store).await?),
            None => None,
        })
    }

    /// Encode the directory as a CBOR object.
    pub async fn encode<B: BlockStore>(&self, store: &mut B) -> Result<Vec<u8>> {
        let mut bytes = Vec::new();

        // Write the major of the section being written.
        encode::write_u64(
            &mut bytes,
            MajorKind::Map,
            PublicDirectory::FIELDS.len() as u64,
        )?;

        // Ordering the fields by name based on RFC-7049 which is also what libipld uses.
        let mut cbor_order: Vec<&'static str> = Vec::from_iter(PublicDirectory::FIELDS);
        cbor_order.sort_unstable_by(|&a, &b| match a.len().cmp(&b.len()) {
            Ordering::Greater => Ordering::Greater,
            Ordering::Less => Ordering::Less,
            Ordering::Equal => a.cmp(b),
        });

        // Iterate over the fields.
        for field in cbor_order.iter() {
            // Encode field name.
            field.encode(DagCborCodec, &mut bytes)?;
            // Encode field value.
            match *field {
                "metadata" => {
                    self.metadata.encode(DagCborCodec, &mut bytes)?;
                }
                "userland" => {
                    let new_userland = {
                        let mut tmp = BTreeMap::new();
                        for (k, link) in self.userland.iter() {
                            let cid = link.seal(store).await?;
                            tmp.insert(k.clone(), cid);
                        }
                        tmp
                    };

                    new_userland.encode(DagCborCodec, &mut bytes)?;
                }
                "previous" => {
                    self.previous.encode(DagCborCodec, &mut bytes)?;
                }
                _ => unreachable!(),
            }
        }

        Ok(bytes)
    }

    /// Stores a directory as block(s) in provided block store.
    ///
    /// This function can be recursive if the directory contains other directories.
    #[async_recursion(?Send)]
    pub async fn store<B: BlockStore>(&self, store: &mut B) -> Result<Cid> {
        let bytes = self.encode(store).await?;
        store.put_block(bytes, IpldCodec::DagCbor).await
    }

    /// Reads a file from the directory.
    pub async fn read<B: BlockStore>(
        &self,
        path_segments: &[String],
        store: &mut B,
    ) -> Result<Cid> {
        let node = self.get_node(path_segments, store).await?;
        match node {
            Some(node_rc) => match &*node_rc.borrow() {
                PublicNode::File(file) => Ok(file.userland),
                _ => error(FsError::NotAFile),
            },
            _ => error(FsError::NotFound),
        }
    }

    pub async fn upsert() -> Result<Self> {
        // TODO(appcypher): Implement this.
        todo!()
    }

    /// Writes a file to the directory.
    ///
    /// Rather than mutate the directory directly, we create a new directory and return it.
    pub async fn write<B: BlockStore>(
        &self,
        path_segments: &[String],
        time: DateTime<Utc>,
        content_cid: Cid,
        store: &B,
    ) -> Result<Self> {
        // Get the path segments for the file's parent directory.
        let parent_path = match path_segments.split_last() {
            Some((_, rest)) => rest,
            None => return error(FsError::InvalidPath),
        };

        let mut directory = self.mkdir(parent_path, time, store).await?;

        todo!()
    }

    /// Returns the children links of a directory.
    pub async fn ls<B: BlockStore>(&self) -> Result<Vec<Link>> {
        // TODO(appcypher): Implement this.
        todo!()
    }

    /// Creates a new directory with the given path.
    pub async fn mkdir<B: BlockStore>(
        &self,
        path_segments: &[String],
        time: DateTime<Utc>,
        store: &B,
    ) -> Result<Self> {
        if path_segments.is_empty() {
            return error(FsError::InvalidPath);
        }

        // Clone the directory and create a new root.
        let new_root = self.clone();
        let mut working_node = Rc::new(RefCell::new(PublicNode::Dir(new_root)));

        // Iterate over the path segments until the last segment.
        for (index, segment) in path_segments.iter().enumerate() {
            let mut _next_node = None;

            // This block helps us reduce the lifetime scope of the mutable borrow of working_node.
            {
                // Cast working node to directory.
                let mut node_mut = working_node.borrow_mut();
                let dir = node_mut.as_mut_dir();

                // Fetch node representing path segment in working directory.
                if let Some(found_node) = dir.lookup_node(segment, store).await? {
                    match *found_node.borrow() {
                        // If the node is a directory, set it as the next working node.
                        PublicNode::Dir(_) => {
                            _next_node = Some(Rc::clone(&found_node));
                        }
                        // If the node is a file, we return an error.
                        PublicNode::File(_) => {
                            return if index == path_segments.len() - 1 {
                                error(FsError::FileAlreadyExists)
                            } else {
                                error(FsError::InvalidPath)
                            }
                        }
                    }
                } else {
                    // If the node is not found, we create it.
                    let new_node_rc =
                        Rc::new(RefCell::new(PublicNode::Dir(PublicDirectory::new(time))));

                    // Insert the new node into the working directory.
                    dir.userland
                        .insert(segment.clone(), Link::Node(Rc::clone(&new_node_rc)));

                    // And set that as the new working node.
                    _next_node = Some(new_node_rc);
                }
            }

            working_node = _next_node.unwrap();
        }

        // Get the PublicNode behind the working_node `Rc`.
        let node = mem::replace(
            &mut *working_node.borrow_mut(),
            PublicNode::Dir(PublicDirectory::new(time)),
        );

        Ok(node.into_dir())
    }
}

// Decoding CBOR-encoded PublicDirectory from bytes.
impl Decode<DagCborCodec> for PublicDirectory {
    fn decode<R: Read + Seek>(c: DagCborCodec, r: &mut R) -> Result<Self> {
        // Ensure the major kind is a map.
        let major = decode::read_major(r)?;
        if major.kind() != MajorKind::Map {
            return error(FsError::UndecodableCborData("Unsupported major".into()));
        }

        // Decode the length of the map.
        let _ = decode::read_uint(r, major)?;

        // Ordering the fields by name based on RFC-7049 which is also what libipld uses.
        let mut cbor_order: Vec<&'static str> = Vec::from_iter(PublicDirectory::FIELDS);
        cbor_order.sort_unstable_by(|&a, &b| match a.len().cmp(&b.len()) {
            Ordering::Greater => Ordering::Greater,
            Ordering::Less => Ordering::Less,
            Ordering::Equal => a.cmp(b),
        });

        // Iterate over the fields.
        let mut metadata = None;
        let mut userland = BTreeMap::new();
        let mut previous = None;

        // Iterate over the fields.
        for field in cbor_order.iter() {
            // Decode field name.
            String::decode(c, r)?;

            // Decode field value.
            match *field {
                "metadata" => {
                    metadata = Some(Metadata::decode(c, r)?);
                }
                "userland" => {
                    userland = BTreeMap::<_, Cid>::decode(c, r)?
                        .into_iter()
                        .map(|(k, cid)| (k, Link::Cid(cid)))
                        .collect();
                }
                "previous" => {
                    previous = <Option<Cid>>::decode(c, r)?;
                }
                _ => unreachable!(),
            }
        }

        Ok(Self {
            metadata: metadata
                .ok_or_else(|| FsError::UndecodableCborData("Missing unix_fs".into()))?,
            userland,
            previous,
        })
    }
}

#[cfg(test)]
mod public_directory_tests {
    use std::io::Cursor;

    use super::*;
    use crate::{public::PublicFile, BlockStoreLookup, MemoryBlockStore};
    use chrono::Utc;

    #[async_std::test]
    async fn files_added_to_directory_looked_up_unsuccessful() {
        let root = PublicDirectory::new(Utc::now());

        let mut store = MemoryBlockStore::default();

        let content_cid = Cid::default();

        let time = Utc::now();

        // let root = root
        //     .write(&[String::from("text.txt")], time, content_cid, &mut store)
        //     .await
        //     .unwrap();

        // let node = root.lookup_node("text.txt", &store).await;

        // assert!(node.is_ok());

        // assert_eq!(
        //     node.unwrap(),
        //     Some(Rc::new(PublicNode::File(PublicFile::new(
        //         time,
        //         content_cid
        //     ))))
        // );
    }

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

        let mut cursor = Cursor::new(bytes.as_ref());

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
