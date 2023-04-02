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
mod carblockstore;
pub use diskblockstore::DiskBlockStore;
pub use memoryblockstore::MemoryBlockStore;
pub use threadsafememoryblockstore::ThreadSafeMemoryBlockStore;
pub use carblockstore::CarBlockStore;

//--------------------------------------------------------------------------------------------------
// Functions
//--------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

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
