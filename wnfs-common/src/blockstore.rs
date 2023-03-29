//! Block store traits.

use crate::{dagcbor, AsyncSerialize, BlockStoreError, MAX_BLOCK_SIZE};
use anyhow::{bail, Result};
use async_trait::async_trait;
use std::path::PathBuf;
use libipld::{
    cid::Version,
    multihash::{Code, MultihashDigest},
    serde as ipld_serde, Cid, IpldCodec,
};
use serde::{de::DeserializeOwned, Serialize};
use std::{
    borrow::Cow,
    cell::RefCell,
    collections::HashMap,
    sync::{Arc, RwLock},
};

//--------------------------------------------------------------------------------------------------
// Type Definitions
//--------------------------------------------------------------------------------------------------

/// For types that implement block store operations like adding, getting content from the store.
#[async_trait(?Send)]
pub trait BlockStore: Clone {
    async fn get_block(&self, cid: &Cid) -> Result<Cow<Vec<u8>>>;
    async fn put_block(&self, bytes: Vec<u8>, codec: IpldCodec) -> Result<Cid>;

    async fn get_deserializable<V: DeserializeOwned>(&self, cid: &Cid) -> Result<V> {
        let bytes = self.get_block(cid).await?;
        let ipld = dagcbor::decode(bytes.as_ref())?;
        Ok(ipld_serde::from_ipld::<V>(ipld)?)
    }

    async fn put_serializable<V: Serialize>(&self, value: &V) -> Result<Cid> {
        let bytes = dagcbor::encode(&ipld_serde::to_ipld(value)?)?;
        self.put_block(bytes, IpldCodec::DagCbor).await
    }

    async fn put_async_serializable<V: AsyncSerialize>(&self, value: &V) -> Result<Cid> {
        let ipld = value.async_serialize_ipld(self).await?;
        let bytes = dagcbor::encode(&ipld)?;
        self.put_block(bytes, IpldCodec::DagCbor).await
    }

    // This should be the same in all implementations of BlockStore
    fn create_cid(&self, bytes: Vec<u8>, codec: IpldCodec) -> Result<Cid> {
        // If there are too many bytes, abandon this task
        if bytes.len() > MAX_BLOCK_SIZE {
            bail!(BlockStoreError::MaximumBlockSizeExceeded(bytes.len()))
        }
        // Compute the SHA256 hash of the bytes
        let hash = Code::Sha2_256.digest(&bytes);
        // Represent the hash as a V1 CID
        let cid = Cid::new(Version::V1, codec.into(), hash)?;
        // Return Ok with the CID
        Ok(cid)
    }

}

/// An in-memory block store to simulate IPFS.
///
/// IPFS is basically a glorified HashMap.
#[derive(Debug, Default, Clone)]
pub struct MemoryBlockStore(RefCell<HashMap<String, Vec<u8>>>);

//--------------------------------------------------------------------------------------------------
// Implementations
//--------------------------------------------------------------------------------------------------

impl MemoryBlockStore {
    /// Creates a new in-memory block store.
    pub fn new() -> Self {
        Self::default()
    }
}

#[async_trait(?Send)]
impl BlockStore for MemoryBlockStore {
    /// Retrieves an array of bytes from the block store with given CID.
    async fn get_block(&self, cid: &Cid) -> Result<Cow<Vec<u8>>> {
        Ok(Cow::Owned(
            self.0
                .borrow()
                .get(&cid.to_string())
                .ok_or(BlockStoreError::CIDNotFound(*cid))?
                .clone(),
        ))
    }

    /// Stores an array of bytes in the block store.
    async fn put_block(&self, bytes: Vec<u8>, codec: IpldCodec) -> Result<Cid> {
        // Try to build the CID from the bytes and codec
        let cid = self.create_cid(bytes.clone(), codec)?;
        // Insert the bytes into the HashMap using the CID as the key
        self.0.borrow_mut().insert(cid.to_string(), bytes);
        // Return Ok status with the generated CID
        Ok(cid)
    }
}

/// A disk-based blockstore that you can mutate.
pub struct DiskBlockStore(PathBuf);

// -------------------------------------------------------------------------------------------------
// Implementations
// -------------------------------------------------------------------------------------------------

impl DiskBlockStore {
    /// Creates a new disk block store.
    pub fn new(path: PathBuf) -> Self {
        // Ensure the directory is empty, if it exists
        if path.exists() {
            // Remove the directory and its contents
            std::fs::remove_dir_all(&path).unwrap();
        }

        // Create the directories required to store the blocks
        std::fs::create_dir_all(&path).unwrap();
        // Return the new DiskBlockStore
        Self(path)
    }

    pub fn erase(&self) -> Result<()> {
        // Remove the directory
        std::fs::remove_dir_all(&self.0)?;
        // Return Ok status
        Ok(())
    }
}

impl Clone for DiskBlockStore {
    fn clone(&self) -> Self {
        Self::new(self.0.clone())
    }
}

#[async_trait(?Send)]
impl BlockStore for DiskBlockStore {
    /// Stores an array of bytes in the block store.
    async fn put_block(&self, bytes: Vec<u8>, codec: IpldCodec) -> Result<Cid> {
        // Try to build the CID from the bytes and codec
        let cid = self.create_cid(bytes.clone(), codec)?;
        // Create the file at the specified path
        let mut file = std::fs::File::create(self.0.join(cid.to_string()))?;
        // Write the bytes to disk at the File location
        std::io::Write::write_all(&mut file, &bytes)?;
        // Return Ok status with the generated CID
        Ok(cid)
    }

    /// Retrieves an array of bytes from the block store with given CID.
    async fn get_block(&self, cid: &Cid) -> Result<Cow<Vec<u8>>> {
        // Get the bytes from disk, using the given CID as the filename
        let mut file = std::fs::File::open(self.0.join(cid.to_string()))?;
        // Create a mutable vector of bytes
        let mut bytes: Vec<u8> = Vec::new();
        // Read the bytes into that
        std::io::Read::read_to_end(&mut file, &mut bytes)?;
        // Return Ok status with the bytes
        return Ok(Cow::Owned(bytes));
    }
}

/// A concurrency-friendly blockstore that you can mutate.
#[derive(Debug, Default)]
pub struct ThreadSafeMemoryBlockStore(Arc<RwLock<HashMap<String, Vec<u8>>>>);

impl ThreadSafeMemoryBlockStore {
    /// Creates a new in-memory block store.
    pub fn new() -> Self {
        Self::default()
    }
}

impl Clone for ThreadSafeMemoryBlockStore {
    fn clone(&self) -> Self {
        Self::new()
        //Self(RwLock::new(self.0.read().unwrap().clone()))
    }
}

#[async_trait(?Send)]
impl BlockStore for ThreadSafeMemoryBlockStore {
    /// Retrieves an array of bytes from the block store with given CID.
    async fn get_block(&self, cid: &Cid) -> Result<Cow<Vec<u8>>> {
        let bytes = self
            .0
            .read()
            .map_err(|_| BlockStoreError::LockPoisoned)?
            .get(&cid.to_string())
            .ok_or(BlockStoreError::CIDNotFound(*cid))?
            .clone();
        Ok(Cow::Owned(bytes))
    }

    async fn put_block(&self, bytes: Vec<u8>, codec: IpldCodec) -> Result<Cid> {
        // Try to build the CID from the bytes and codec
        let cid = self.create_cid(bytes.clone(), codec)?;
        // Write the bytes to the HashMap using the CID as the key
        self.0
            .write()
            .map_err(|_| BlockStoreError::LockPoisoned)?
            .insert(cid.to_string(), bytes);
        // Return Ok status with the generated CID
        Ok(cid)
    }
}

//--------------------------------------------------------------------------------------------------
// Functions
//--------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use libipld::{cbor::DagCborCodec, codec::Encode};

    // Generic function used to test any type that conforms to the BlockStore trait
    async fn test_block_store<T: BlockStore + Clone + Send + 'static>(store: &mut T) -> Result<()> {
        let first_bytes = {
            let mut tmp = vec![];
            vec![1, 2, 3, 4, 5]
                .to_vec()
                .encode(DagCborCodec, &mut tmp)
                .unwrap();
            tmp
        };

        let second_bytes = {
            let mut tmp = vec![];
            b"hello world"
                .to_vec()
                .encode(DagCborCodec, &mut tmp)
                .unwrap();
            tmp
        };

        let first_cid = &store
            .put_block(first_bytes, IpldCodec::DagCbor)
            .await
            .unwrap();

        let second_cid = &store
            .put_block(second_bytes, IpldCodec::DagCbor)
            .await
            .unwrap();

        let first_loaded: Vec<u8> = store.get_deserializable(first_cid).await.unwrap();
        let second_loaded: Vec<u8> = store.get_deserializable(second_cid).await.unwrap();

        assert_eq!(first_loaded, vec![1, 2, 3, 4, 5]);
        assert_eq!(second_loaded, b"hello world".to_vec());

        Ok(())
    }

    #[async_std::test]
    async fn memory_blockstore() {
        let store = &mut MemoryBlockStore::new();
        test_block_store(store).await.unwrap();
    }

    #[async_std::test]
    async fn disk_blockstore() {
        let store = &mut DiskBlockStore::new(PathBuf::from("test_disk_blockstore"));
        test_block_store(store).await.unwrap();
        store.erase().unwrap();
    }
}
