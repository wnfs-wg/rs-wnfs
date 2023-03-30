use crate::{BlockStore, BlockStoreError};
use anyhow::Result;
use async_trait::async_trait;
use libipld::{Cid, IpldCodec};
use std::{borrow::Cow, cell::RefCell, collections::HashMap};

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
