use crate::{dagcbor, AsyncSerialize, BlockStoreError, MAX_BLOCK_SIZE};
use anyhow::{bail, Result};
use async_trait::async_trait;
use libipld::{
    cid::Version,
    multihash::{Code, MultihashDigest},
    serde as ipld_serde, Cid, IpldCodec,
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::{borrow::Cow, cell::RefCell, collections::HashMap};

//--------------------------------------------------------------------------------------------------
// Type Definitions
//--------------------------------------------------------------------------------------------------

/// For types that implement block store operations like adding, getting content from the store.
#[async_trait(?Send)]
pub trait BlockStore: Sized {
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
    fn create_cid(&self, bytes: &Vec<u8>, codec: IpldCodec) -> Result<Cid> {
        // If there are too many bytes, abandon this task
        if bytes.len() > MAX_BLOCK_SIZE {
            bail!(BlockStoreError::MaximumBlockSizeExceeded(bytes.len()))
        }
        // Compute the SHA256 hash of the bytes
        let hash = Code::Sha2_256.digest(bytes);
        // Represent the hash as a V1 CID
        let cid = Cid::new(Version::V1, codec.into(), hash)?;
        // Return Ok with the CID
        Ok(cid)
    }
}

//--------------------------------------------------------------------------------------------------
// Implementations
//--------------------------------------------------------------------------------------------------

/// An in-memory block store to simulate IPFS.
///
/// IPFS is basically a glorified HashMap.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct MemoryBlockStore(RefCell<HashMap<String, Vec<u8>>>);

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
        let cid = self.create_cid(&bytes, codec)?;
        // Insert the bytes into the HashMap using the CID as the key
        self.0.borrow_mut().insert(cid.to_string(), bytes);
        // Return Ok status with the generated CID
        Ok(cid)
    }
}

//--------------------------------------------------------------------------------------------------
// Tests
//--------------------------------------------------------------------------------------------------

/// The following methods are generic functions that can be used to test any type that conforms to the BlockStore trait.
/// In utilizing this structure, externally defined types can still test for retrieval, duplication, and serialization compatibility.
pub async fn bs_retrieval<T: BlockStore + Send + 'static>(store: &T) -> Result<()> {
    // Example objects to insert and remove from the blockstore
    let first_bytes = vec![1, 2, 3, 4, 5];
    let second_bytes = b"hello world".to_vec();

    // Insert the objects into the blockstore
    let first_cid = store.put_serializable(&first_bytes).await.unwrap();
    let second_cid = store.put_serializable(&second_bytes).await.unwrap();

    // Retrieve the objects from the blockstore
    let first_loaded: Vec<u8> = store.get_deserializable(&first_cid).await.unwrap();
    let second_loaded: Vec<u8> = store.get_deserializable(&second_cid).await.unwrap();

    // Assert that the objects are the same as the ones we inserted
    assert_eq!(first_loaded, first_bytes);
    assert_eq!(second_loaded, second_bytes);

    // Return Ok
    Ok(())
}

// Generic function used to test any type that conforms to the BlockStore trait
pub async fn bs_duplication<T: BlockStore + Send + 'static>(store: &T) -> Result<()> {
    // Example objects to insert and remove from the blockstore
    let first_bytes = vec![1, 2, 3, 4, 5];
    let second_bytes = first_bytes.clone();

    // Insert the objects into the blockstore
    let first_cid = store.put_serializable(&first_bytes).await.unwrap();
    let second_cid = store.put_serializable(&second_bytes).await.unwrap();

    // Assert that the two vecs produced the same CID
    assert_eq!(first_cid, second_cid);

    // Retrieve the objects from the blockstore
    let first_loaded: Vec<u8> = store.get_deserializable(&first_cid).await.unwrap();
    let second_loaded: Vec<u8> = store.get_deserializable(&second_cid).await.unwrap();

    // Assert that the objects are the same as the ones we inserted
    assert_eq!(first_loaded, first_bytes);
    assert_eq!(second_loaded, second_bytes);
    // Assert that the objects we loaded are the same
    assert_eq!(first_loaded, second_loaded);

    // Return Ok
    Ok(())
}

pub async fn bs_serialization<
    T: BlockStore + Send + Serialize + 'static + for<'de> Deserialize<'de>,
>(
    store: &T,
) -> Result<()> {
    // Example objects to insert and remove from the blockstore
    let bytes = vec![1, 2, 3, 4, 5];
    // Insert the object into the blockstore
    let cid = store.put_serializable(&bytes).await.unwrap();

    // Create a buffer to hold the serialized BlockStore
    let mut writer: Vec<u8> = Vec::new();
    // Serialize the blockstore
    serde_json::to_writer_pretty(&mut writer, &store).unwrap();

    // Construct a new BlockStore from the Serialized object
    let new_store: T = serde_json::from_reader(writer.as_slice()).unwrap();

    // Retrieve the object from the blockstore
    let loaded: Vec<u8> = new_store.get_deserializable(&cid).await.unwrap();

    // Assert that the objects are the same as the ones we inserted
    assert_eq!(loaded, bytes);
    Ok(())
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[async_std::test]
    async fn memory_blockstore() {
        let store = &MemoryBlockStore::new();
        bs_retrieval(store).await.unwrap();
        bs_duplication(store).await.unwrap();
        bs_serialization(store).await.unwrap();
    }
}
