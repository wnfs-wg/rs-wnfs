use crate::{
    decode, encode,
    utils::{Arc, CondSend, CondSync},
    BlockStoreError, MAX_BLOCK_SIZE,
};
use anyhow::{bail, Result};
use bytes::Bytes;
use futures::Future;
use libipld::{
    cbor::DagCborCodec,
    cid::Version,
    multihash::{Code, MultihashDigest},
    serde as ipld_serde, Cid,
};
use parking_lot::Mutex;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::{collections::HashMap, ops::Deref};

//--------------------------------------------------------------------------------------------------
// Constants
//--------------------------------------------------------------------------------------------------

/// The value representing the DAG-JSON codec.
///
/// - <https://ipld.io/docs/codecs/#known-codecs>
/// - <https://github.com/multiformats/multicodec/blob/master/table.csv>
pub const CODEC_DAG_JSON: u64 = 0x0129;

/// The value representing the DAG-CBOR codec.
///
/// - <https://ipld.io/docs/codecs/#known-codecs>
/// - <https://github.com/multiformats/multicodec/blob/master/table.csv>
pub const CODEC_DAG_CBOR: u64 = 0x71;

/// The value representing the DAG-Protobuf codec.
///
/// - <https://ipld.io/docs/codecs/#known-codecs>
/// - <https://github.com/multiformats/multicodec/blob/master/table.csv>
pub const CODEC_DAG_PB: u64 = 0x70;

/// The value representing the raw codec.
///
/// - <https://ipld.io/docs/codecs/#known-codecs>
/// - <https://github.com/multiformats/multicodec/blob/master/table.csv>
pub const CODEC_RAW: u64 = 0x55;

//--------------------------------------------------------------------------------------------------
// Type Definitions
//--------------------------------------------------------------------------------------------------

/// For types that implement block store operations like adding, getting content from the store.
pub trait BlockStore: CondSync {
    fn get_block(&self, cid: &Cid) -> impl Future<Output = Result<Bytes>> + CondSend;

    fn put_block(
        &self,
        bytes: impl Into<Bytes> + CondSend,
        codec: u64,
    ) -> impl Future<Output = Result<Cid>> + CondSend;

    fn get_deserializable<V: DeserializeOwned>(
        &self,
        cid: &Cid,
    ) -> impl Future<Output = Result<V>> + CondSend {
        async {
            let bytes = self.get_block(cid).await?;
            let ipld = decode(bytes.as_ref(), DagCborCodec)?;
            Ok(ipld_serde::from_ipld::<V>(ipld)?)
        }
    }

    fn put_serializable<V: Serialize + CondSync>(
        &self,
        value: &V,
    ) -> impl Future<Output = Result<Cid>> + CondSend {
        async move {
            let bytes = encode(&ipld_serde::to_ipld(value)?, DagCborCodec)?;
            self.put_block(bytes, CODEC_DAG_CBOR).await
        }
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

impl<B: BlockStore, T: Deref<Target = B> + CondSync> BlockStore for T {
    async fn get_block(&self, cid: &Cid) -> Result<Bytes> {
        self.deref().get_block(cid).await
    }

    async fn put_block(&self, bytes: impl Into<Bytes> + CondSend, codec: u64) -> Result<Cid> {
        self.deref().put_block(bytes, codec).await
    }

    async fn get_deserializable<V: DeserializeOwned>(&self, cid: &Cid) -> Result<V> {
        self.deref().get_deserializable(cid).await
    }

    async fn put_serializable<V: Serialize + CondSync>(&self, value: &V) -> Result<Cid> {
        self.deref().put_serializable(value).await
    }

    fn create_cid(&self, bytes: &[u8], codec: u64) -> Result<Cid> {
        self.deref().create_cid(bytes, codec)
    }
}

//--------------------------------------------------------------------------------------------------
// Implementations
//--------------------------------------------------------------------------------------------------

/// An in-memory block store to simulate IPFS.
///
/// IPFS is basically a glorified HashMap.

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct MemoryBlockStore(
    #[serde(serialize_with = "crate::utils::serialize_cid_map")]
    #[serde(deserialize_with = "crate::utils::deserialize_cid_map")]
    pub(crate) Arc<Mutex<HashMap<Cid, Bytes>>>,
);

impl MemoryBlockStore {
    /// Creates a new in-memory block store.
    pub fn new() -> Self {
        Self::default()
    }
}

impl BlockStore for MemoryBlockStore {
    /// Retrieves an array of bytes from the block store with given CID.
    async fn get_block(&self, cid: &Cid) -> Result<Bytes> {
        let bytes = self
            .0
            .lock()
            .get(cid)
            .ok_or(BlockStoreError::CIDNotFound(*cid))?
            .clone();

        Ok(bytes)
    }

    /// Stores an array of bytes in the block store.
    async fn put_block(&self, bytes: impl Into<Bytes> + CondSend, codec: u64) -> Result<Cid> {
        // Convert the bytes into a Bytes object
        let bytes: Bytes = bytes.into();

        // Try to build the CID from the bytes and codec
        let cid = self.create_cid(&bytes, codec)?;

        // Insert the bytes into the HashMap using the CID as the key
        self.0.lock().insert(cid, bytes);

        Ok(cid)
    }
}

//--------------------------------------------------------------------------------------------------
// Tests
//--------------------------------------------------------------------------------------------------

/// Tests the retrieval property of a BlockStore-conforming type.
pub async fn bs_retrieval_test<T>(store: &T) -> Result<()>
where
    T: BlockStore + 'static,
{
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

    Ok(())
}

/// Tests the duplication of a BlockStore-conforming type.
pub async fn bs_duplication_test<T>(store: &T) -> Result<()>
where
    T: BlockStore + 'static,
{
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

    Ok(())
}

/// Tests the serialization of a BlockStore-conforming type.
pub async fn bs_serialization_test<T>(store: &T) -> Result<()>
where
    T: BlockStore + Serialize + 'static + for<'de> Deserialize<'de>,
{
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
