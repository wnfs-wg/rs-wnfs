//! Block store traits.

use crate::{dagcbor, AsyncSerialize, BlockStoreError, MAX_BLOCK_SIZE};
use anyhow::{bail, Result};
use async_trait::async_trait;
use libipld::{
    cid::Version,
    multihash::{Code, MultihashDigest},
    serde as ipld_serde, Cid, IpldCodec,
};
use serde::{de::DeserializeOwned, Serialize};
use std::borrow::Cow;

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

mod diskblockstore;
mod memoryblockstore;
mod threadsafememoryblockstore;

pub use diskblockstore::DiskBlockStore;
pub use memoryblockstore::MemoryBlockStore;
pub use threadsafememoryblockstore::ThreadSafeMemoryBlockStore;

//--------------------------------------------------------------------------------------------------
// Functions
//--------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    // Generic function used to test any type that conforms to the BlockStore trait
    async fn bs_retrieval<T: BlockStore + Clone + Send + 'static>(store: &mut T) -> Result<()> {
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
    async fn bs_duplication<T: BlockStore + Clone + Send + 'static>(store: &mut T) -> Result<()> {
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

    #[async_std::test]
    async fn memory_blockstore() {
        let store = &mut MemoryBlockStore::new();
        bs_retrieval(store).await.unwrap();
    }

    #[async_std::test]
    async fn disk_blockstore() {
        let store = &mut DiskBlockStore {
            path: PathBuf::from("test_disk_blockstore"),
        };
        bs_retrieval(store).await.unwrap();
        store.erase().unwrap();
    }

    #[async_std::test]
    async fn dedup_blockstore() {
        // Test the deduplication of the MemoryBlockStore
        let store = &mut MemoryBlockStore::new();
        bs_duplication(store).await.unwrap();

        // Test the deduplication of the DiskBlockStore
        let store = &mut DiskBlockStore {
            path: PathBuf::from("test_dedup_blockstore"),
        };
        bs_duplication(store).await.unwrap();
        store.erase().unwrap();
    }
}
