use crate::{decode, encode, AsyncSerialize, BlockStoreError, MAX_BLOCK_SIZE};
use anyhow::{bail, Result};
use async_trait::async_trait;
use bytes::Bytes;
use libipld::{
    cbor::DagCborCodec,
    cid::Version,
    multihash::{Code, MultihashDigest},
    serde as ipld_serde, Cid,
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::{cell::RefCell, collections::HashMap};

//--------------------------------------------------------------------------------------------------
// Constants
//--------------------------------------------------------------------------------------------------

/// The value representing the DAG-JSON codec.
///
/// - https://ipld.io/docs/codecs/#known-codecs
/// - https://github.com/multiformats/multicodec/blob/master/table.csv
pub const CODEC_DAG_JSON: u64 = 0x0129;

/// The value representing the DAG-CBOR codec.
///
/// - https://ipld.io/docs/codecs/#known-codecs
/// - https://github.com/multiformats/multicodec/blob/master/table.csv
pub const CODEC_DAG_CBOR: u64 = 0x71;

/// The value representing the DAG-Protobuf codec.
///
/// - https://ipld.io/docs/codecs/#known-codecs
/// - https://github.com/multiformats/multicodec/blob/master/table.csv
pub const CODEC_DAG_PB: u64 = 0x70;

/// The value representing the raw codec.
///
/// - https://ipld.io/docs/codecs/#known-codecs
/// - https://github.com/multiformats/multicodec/blob/master/table.csv
pub const CODEC_RAW: u64 = 0x55;

//--------------------------------------------------------------------------------------------------
// Type Definitions
//--------------------------------------------------------------------------------------------------

/// For types that implement block store operations like adding, getting content from the store.
#[async_trait(?Send)]
pub trait BlockStore: Sized {
    async fn get_block(&self, cid: &Cid) -> Result<Bytes>;
    async fn put_block(&self, bytes: impl Into<Bytes>, codec: u64) -> Result<Cid>;

    async fn get_deserializable<V: DeserializeOwned>(&self, cid: &Cid) -> Result<V> {
        let bytes = self.get_block(cid).await?;
        let ipld = decode(bytes.as_ref(), DagCborCodec)?;
        Ok(ipld_serde::from_ipld::<V>(ipld)?)
    }

    async fn put_serializable<V: Serialize>(&self, value: &V) -> Result<Cid> {
        let bytes = encode(&ipld_serde::to_ipld(value)?, DagCborCodec)?;
        self.put_block(bytes, CODEC_DAG_CBOR).await
    }

    async fn put_async_serializable<V: AsyncSerialize>(&self, value: &V) -> Result<Cid> {
        let ipld = value.async_serialize_ipld(self).await?;
        let bytes = encode(&ipld, DagCborCodec)?;
        self.put_block(bytes, CODEC_DAG_CBOR).await
    }

    // This should be the same in all implementations of BlockStore
    fn create_cid(&self, bytes: &[u8], codec: u64) -> Result<Cid> {
        // If there are too many bytes, abandon this task
        if bytes.len() > MAX_BLOCK_SIZE {
            bail!(BlockStoreError::MaximumBlockSizeExceeded(bytes.len()))
        }

        // Compute the Blake3 hash of the bytes
        let hash = Code::Blake3_256.digest(bytes);

        // Represent the hash as a V1 CID
        let cid = Cid::new(Version::V1, codec, hash)?;

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
pub struct MemoryBlockStore(RefCell<HashMap<Cid, Bytes>>);

impl MemoryBlockStore {
    /// Creates a new in-memory block store.
    pub fn new() -> Self {
        Self::default()
    }
}

#[async_trait(?Send)]
impl BlockStore for MemoryBlockStore {
    /// Retrieves an array of bytes from the block store with given CID.
    async fn get_block(&self, cid: &Cid) -> Result<Bytes> {
        let bytes = self
            .0
            .borrow()
            .get(cid)
            .ok_or(BlockStoreError::CIDNotFound(*cid))?
            .clone();

        Ok(bytes)
    }

    /// Stores an array of bytes in the block store.
    async fn put_block(&self, bytes: impl Into<Bytes>, codec: u64) -> Result<Cid> {
        // Convert the bytes into a Bytes object
        let bytes: Bytes = bytes.into();

        // Try to build the CID from the bytes and codec
        let cid = self.create_cid(&bytes, codec)?;

        // Insert the bytes into the HashMap using the CID as the key
        self.0.borrow_mut().insert(cid, bytes);

        // Return Ok status with the generated CID
        Ok(cid)
    }
}

//--------------------------------------------------------------------------------------------------
// Tests
//--------------------------------------------------------------------------------------------------

/// The following methods are generic functions that can be used to test any type that conforms to the BlockStore trait.
/// In utilizing this structure, externally defined types can still test for retrieval, duplication, and serialization compatibility.
pub async fn bs_retrieval_test<T: BlockStore + Send + 'static>(store: &T) -> Result<()> {
    // Example objects to insert and remove from the blockstore
    let first_bytes = vec![1, 2, 3, 4, 5];
    let second_bytes = b"hello world".to_vec();

    // Insert the objects into the blockstore
    let first_cid = store.put_serializable(&first_bytes).await?;
    let second_cid = store.put_serializable(&second_bytes).await?;

    // Retrieve the objects from the blockstore
    let first_loaded: Vec<u8> = store.get_deserializable(&first_cid).await?;
    let second_loaded: Vec<u8> = store.get_deserializable(&second_cid).await?;

    // Assert that the objects are the same as the ones we inserted
    assert_eq!(first_loaded, first_bytes);
    assert_eq!(second_loaded, second_bytes);

    // Return Ok
    Ok(())
}

// Generic function used to test any type that conforms to the BlockStore trait
pub async fn bs_duplication_test<T: BlockStore + Send + 'static>(store: &T) -> Result<()> {
    // Example objects to insert and remove from the blockstore
    let first_bytes = vec![1, 2, 3, 4, 5];
    let second_bytes = first_bytes.clone();

    // Insert the objects into the blockstore
    let first_cid = store.put_serializable(&first_bytes).await?;
    let second_cid = store.put_serializable(&second_bytes).await?;

    // Assert that the two vecs produced the same CID
    assert_eq!(first_cid, second_cid);

    // Retrieve the objects from the blockstore
    let first_loaded: Vec<u8> = store.get_deserializable(&first_cid).await?;
    let second_loaded: Vec<u8> = store.get_deserializable(&second_cid).await?;

    // Assert that the objects are the same as the ones we inserted
    assert_eq!(first_loaded, first_bytes);
    assert_eq!(second_loaded, second_bytes);
    // Assert that the objects we loaded are the same
    assert_eq!(first_loaded, second_loaded);
    // Return Ok
    Ok(())
}

pub async fn bs_serialization_test<
    T: BlockStore + Send + Serialize + 'static + for<'de> Deserialize<'de>,
>(
    store: &T,
) -> Result<()> {
    // Example objects to insert and remove from the blockstore
    let bytes = vec![1, 2, 3, 4, 5];
    // Insert the object into the blockstore
    let cid = store.put_serializable(&bytes).await?;
    // Serialize the BlockStore
    let serial_store: Vec<u8> = encode(&store, DagCborCodec)?;
    // Construct a new BlockStore from the Serialized object
    let deserial_store: T = decode(&serial_store, DagCborCodec)?;
    // Retrieve the object from the blockstore
    let loaded: Vec<u8> = deserial_store.get_deserializable(&cid).await?;
    // Assert that the objects are the same as the ones we inserted
    assert_eq!(loaded, bytes);
    // Return Ok
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[async_std::test]
    async fn memory_blockstore() -> Result<()> {
        let store = &MemoryBlockStore::new();
        bs_retrieval_test(store).await?;
        bs_duplication_test(store).await?;
        bs_serialization_test(store).await?;
        Ok(())
    }
}
