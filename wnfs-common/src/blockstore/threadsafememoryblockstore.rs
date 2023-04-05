use crate::{BlockStore, BlockStoreError};
use anyhow::Result;
use async_trait::async_trait;
use libipld::{Cid, IpldCodec};
use std::{
    borrow::Cow,
    collections::HashMap,
    sync::{Arc, RwLock},
};

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
        let cid = self.create_cid(&bytes, codec)?;
        // Write the bytes to the HashMap using the CID as the key
        self.0
            .write()
            .map_err(|_| BlockStoreError::LockPoisoned)?
            .insert(cid.to_string(), bytes);
        // Return Ok status with the generated CID
        Ok(cid)
    }
}
