//! Public fs directory node.

use std::{
    cmp::Ordering,
    collections::BTreeMap,
    future::Future,
    io::{Read, Seek},
    rc::Rc,
};

use crate::{error, shared, BlockStore, FsError, Metadata, Shared, UnixFsNodeKind};
use anyhow::Result;
use async_recursion::async_recursion;
use chrono::{DateTime, Utc};
use field_names::FieldNames;
use libipld::{
    cbor::{cbor::MajorKind, decode, encode, DagCborCodec},
    codec::{Decode, Encode},
    Cid, IpldCodec,
};

use super::{Link, PublicFile, PublicNode};

//--------------------------------------------------------------------------------------------------
// Type Definitions
//--------------------------------------------------------------------------------------------------

/// A directory in a WNFS public file system.
#[derive(Debug, Clone, PartialEq, Eq, FieldNames)]
pub struct PublicDirectory {
    pub(crate) metadata: Metadata,
    pub(crate) userland: BTreeMap<String, Link>,
    pub(crate) previous: Option<Cid>,
}

/// A fork of a directory.
pub struct Fork<T> {
    pub forked_dir: Shared<PublicNode>,
    pub working_node: T,
}

//--------------------------------------------------------------------------------------------------
// Implementations
//--------------------------------------------------------------------------------------------------

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
    ///
    /// If path is empty, this returns a cloned directory based on `self`.
    pub async fn get_node<B: BlockStore>(
        &self,
        path_segments: &[String],
        store: &B,
    ) -> Result<Fork<Option<Shared<PublicNode>>>> {
        // TODO(appcypher): This does not need to be deep cloned like `mkdir`.
        // Set working node to current directory.
        let forked_dir = shared(PublicNode::Dir(self.clone()));
        let mut working_node = Some(Rc::clone(&forked_dir));

        // Iterate over the path segments.
        for (index, segment) in path_segments.iter().enumerate() {
            // Cast working node to directory.
            let node_rc = working_node.unwrap();
            let node_ref = node_rc.borrow();
            let dir = node_ref.as_dir();

            // Fetch node representing the path segment in the working directory.
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
            working_node = None;
            break;
        }

        Ok(Fork {
            forked_dir,
            working_node,
        })
    }

    /// Looks up a node by its path name in the current directory.
    ///
    /// TODO(appcypher): What is a valid path segment identifier?
    pub async fn lookup_node<B: BlockStore>(
        &self,
        path_segment: &str,
        store: &B,
    ) -> Result<Option<Shared<PublicNode>>> {
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
        match self.get_node(path_segments, store).await? {
            Fork {
                working_node: Some(node_rc),
                ..
            } => match &*node_rc.borrow() {
                PublicNode::File(file) => Ok(file.userland),
                _ => error(FsError::NotAFile),
            },
            _ => error(FsError::NotFound),
        }
    }

    /// Updates or inserts a new node at the specified path.
    ///
    /// NOTE(appcypher): This is meant for internal use only as it mutates the directory in place for performance.
    /// Ideally, this method should be called with a newly forked directory.
    pub(super) async fn upsert<Fut>(
        &mut self,
        path_segment: &str,
        update_fn: impl FnOnce(Option<Link>) -> Fut,
    ) -> Result<()>
    where
        Fut: Future<Output = Result<Option<Link>>>,
    {
        match update_fn(self.userland.get(path_segment).cloned()).await? {
            // If the link is none, we remove the node from the userland.
            None => {
                self.userland.remove(path_segment);
            }
            // If the link is some, we insert the node into the userland.
            Some(link) => {
                self.userland.insert(path_segment.to_string(), link);
            }
        }

        Ok(())
    }

    /// Writes a file to the directory.
    ///
    /// Rather than mutate the directory directly, we create a new directory and return it.
    pub async fn write<B: BlockStore>(
        &self,
        path_segments: &[String],
        content_cid: Cid,
        time: DateTime<Utc>,
        store: &B,
    ) -> Result<Shared<PublicNode>> {
        // If it does not already exist, create the file's parent directory.
        let (
            Fork {
                forked_dir,
                working_node: parent_directory,
            },
            tail,
        ) = match path_segments.split_last() {
            None => return error(FsError::InvalidPath),
            Some((tail, parent_path_segments)) => {
                (self.mkdir(parent_path_segments, time, store).await?, tail)
            }
        };

        // Insert or create file in parent directory.
        parent_directory
            .borrow_mut()
            .as_mut_dir()
            .upsert(tail, move |link| async move {
                // If a link is provided, it is a cue to update it.
                if let Some(link) = link {
                    let node = link.resolve(store).await?;
                    return match &mut *node.borrow_mut() {
                        PublicNode::File(file) => {
                            file.metadata = Metadata::new(time, UnixFsNodeKind::File);
                            file.userland = content_cid;
                            Ok(Some(link))
                        }
                        _ => error(FsError::DirectoryAlreadyExists),
                    };
                }

                // If nothing is provided, it is a cue to return a new file node.
                let link = Link::with_file(PublicFile::new(time, content_cid));
                Ok(Some(link))
            })
            .await?;

        Ok(forked_dir)
    }

    /// Creates a new directory at the specified path.
    ///
    /// If path is empty, this returns a cloned directory based on `self`.
    ///
    /// This method acts like `mkdir -p` in Unix because it creates intermediate directories if they do not exist.
    pub async fn mkdir<B: BlockStore>(
        &self,
        path_segments: &[String],
        time: DateTime<Utc>,
        store: &B,
    ) -> Result<Fork<Shared<PublicNode>>> {
        // TODO(appcypher): Investigate that deep cloning is actually done.
        // Clone the directory to prevent mutation of the original directory.
        let forked_dir = shared(PublicNode::Dir(self.clone()));
        let mut working_node = Rc::clone(&forked_dir);

        // Iterate over path segments.
        for (index, segment) in path_segments.iter().enumerate() {
            let mut _next_node = None;

            // This block helps us shorten the lifetime scope of the mutable borrow of working_node below.
            {
                // Cast working node to directory.
                let mut node_mut = working_node.borrow_mut();
                let dir = node_mut.as_mut_dir();

                // Fetch node representing the path segment in the working directory.
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
                    let new_node_rc = shared(PublicNode::Dir(PublicDirectory::new(time)));

                    // Insert the new node into the working directory.
                    dir.userland
                        .insert(segment.clone(), Link::Node(Rc::clone(&new_node_rc)));

                    // And set it as the new working node.
                    _next_node = Some(new_node_rc);
                }
            }

            working_node = _next_node.unwrap();
        }

        Ok(Fork {
            forked_dir,
            working_node,
        })
    }

    /// Returns the name and metadata of the direct children of a directory.
    pub async fn ls<B: BlockStore>(
        &self,
        path_segments: &[String],
        store: &B,
    ) -> Result<Vec<(String, Metadata)>> {
        let node = self
            .get_node(path_segments, store)
            .await?
            .working_node
            .ok_or(FsError::NotFound)?;

        let node = node.borrow();
        match &*node {
            PublicNode::Dir(dir) => {
                // Save the directory's children info in a vector.
                let mut result = vec![];
                for (name, link) in dir.userland.iter() {
                    match &*link.resolve(store).await?.borrow() {
                        PublicNode::File(file) => {
                            result.push((name.clone(), file.metadata.clone()));
                        }
                        PublicNode::Dir(dir) => {
                            result.push((name.clone(), dir.metadata.clone()));
                        }
                    }
                }
                Ok(result)
            }
            _ => error(FsError::NotADirectory),
        }
    }

    /// Removes a file or directory from the directory.
    pub async fn rm<B: BlockStore>(
        &self,
        path_segments: &[String],
        store: &mut B,
    ) -> Result<Shared<PublicNode>> {
        // TODO(appcypher): This should do a deep clone after.
        // Get node's parent directory.
        let (
            Fork {
                working_node: parent_node,
                forked_dir,
            },
            tail,
        ) = match path_segments.split_last() {
            None => return error(FsError::InvalidPath),
            Some((tail, parent_path_segments)) => {
                (self.get_node(parent_path_segments, store).await?, tail)
            }
        };

        let parent_node = parent_node.ok_or(FsError::NotFound)?;
        match &mut *parent_node.borrow_mut() {
            PublicNode::Dir(dir) => {
                // Remove the file from the parent directory if present.
                dir.upsert(tail, |link| async move {
                    match link {
                        Some(_) => Ok(None),
                        _ => error(FsError::NotFound),
                    }
                })
                .await?;
            }
            _ => return error(FsError::NotADirectory),
        };

        Ok(forked_dir)
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

        let store = MemoryBlockStore::default();

        let content_cid = Cid::default();

        let time = Utc::now();

        let root = root
            .write(&[String::from("text.txt")], content_cid, time, &store)
            .await
            .unwrap();

        let root = root.borrow();

        let node = root.as_dir().lookup_node("text.txt", &store).await.unwrap();

        assert!(node.is_some());

        assert_eq!(
            node.unwrap(),
            shared(PublicNode::File(PublicFile::new(time, content_cid)))
        );
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
