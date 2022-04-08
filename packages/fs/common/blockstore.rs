//! Block store traits.

use super::FsError;
use anyhow::Result;
use async_trait::async_trait;
use hashbrown::HashMap;
use libipld::{
    cbor::DagCborCodec,
    cid::Version,
    codec::{Codec, Decode},
    Cid, IpldCodec,
};
use multihash::{Code, MultihashDigest};
use std::borrow::Cow;

//--------------------------------------------------------------------------------------------------
// Type Definitions
//--------------------------------------------------------------------------------------------------

/// For types that implement getting a block from a CID.
#[async_trait]
pub trait BlockStoreLookup {
    async fn get_block<'a>(&'a self, cid: &Cid) -> Result<Cow<'a, [u8]>>;
}

/// For types that implement loading a cbor model from a blockstore using a CID.
#[async_trait]
pub trait BlockStoreCidLoad {
    /// Loads a cbor model from the store with provided CID.
    async fn load<T: Decode<DagCborCodec>>(&self, cid: &Cid) -> Result<T>;
}

/// For types that implement block store operations.
#[async_trait]
pub trait BlockStore: BlockStoreLookup + BlockStoreCidLoad {
    async fn put_block(&mut self, bytes: Vec<u8>, codec: IpldCodec) -> Result<Cid>;
}

/// An in-memory block store to simulate IPFS. IPFS is basically an glorified HashMap.
#[derive(Debug, Default)]
pub struct MemoryBlockStore(HashMap<String, Vec<u8>>);

//--------------------------------------------------------------------------------------------------
// Implementations
//--------------------------------------------------------------------------------------------------

impl MemoryBlockStore {
    /// Creates a new in-memory block store.
    pub fn new() -> Self {
        Self::default()
    }
}

#[async_trait]
impl BlockStore for MemoryBlockStore {
    /// Stores an array of bytes in the block store.
    async fn put_block(&mut self, bytes: Vec<u8>, codec: IpldCodec) -> Result<Cid> {
        let hash = Code::Sha2_256.digest(&bytes);
        let cid = Cid::new(Version::V1, codec.into(), hash)?;

        self.0.insert((&cid).to_string(), bytes);

        Ok(cid)
    }
}

#[async_trait]
impl BlockStoreLookup for MemoryBlockStore {
    /// Retrieves an array of bytes from the block store with given CID.
    async fn get_block<'a>(&'a self, cid: &Cid) -> Result<Cow<'a, [u8]>> {
        let bytes = self
            .0
            .get(&cid.to_string())
            .ok_or(FsError::CIDNotFoundInBlockstore)?;

        Ok(Cow::Borrowed(bytes))
    }
}

#[async_trait]
impl BlockStoreCidLoad for MemoryBlockStore {
    /// Loads a cbor-encoded data from the store with provided CID.
    async fn load<T: Decode<DagCborCodec>>(&self, cid: &Cid) -> Result<T> {
        let bytes = self.get_block(cid).await?;
        let decoded = DagCborCodec.decode(bytes.as_ref())?;
        Ok(decoded)
    }
}

#[cfg(test)]
mod blockstore_tests {}
