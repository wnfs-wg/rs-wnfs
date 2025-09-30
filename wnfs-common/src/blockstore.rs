use crate::{
    BlockStoreError, MAX_BLOCK_SIZE,
    utils::{Arc, CondSend, CondSync},
};
use bytes::Bytes;
use cid::Cid;
use futures::Future;
use ipld_core::cid::Version;
use multihash::Multihash;
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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

/// The multihash marker for blake3 hashes.
///
/// <https://github.com/multiformats/multicodec/blob/master/table.csv#L21>
pub const MULTIHASH_BLAKE3: u64 = 0x1e;

//--------------------------------------------------------------------------------------------------
// Traits
//--------------------------------------------------------------------------------------------------

/// For types that implement block store operations like adding, getting content from the store.
pub trait BlockStore: CondSync {
    /// Retrieve a block from this store via its hash (`Cid`).
    ///
    /// If this store can't find the block, it may raise an error like `BlockNotFound`.
    fn get_block(
        &self,
        cid: &Cid,
    ) -> impl Future<Output = Result<Bytes, BlockStoreError>> + CondSend;

    /// Put some bytes into the blockstore. These bytes should be encoded with the given codec.
    ///
    /// E.g. `CODEC_RAW` for raw bytes blocks, `CODEC_DAG_CBOR` for dag-cbor, etc.
    ///
    /// This codec will determine the codec encoded in the final `Cid` that's returned.
    ///
    /// If the codec is incorrect, this function won't fail, but any tools that depend on the
    /// correctness of the codec may fail. (E.g. tools that follow the links of blocks).
    ///
    /// This funciton allows the blockstore to choose the hashing function itself.
    /// The hashing function that was chosen will be readable from the `Cid` metadata.
    ///
    /// If you need control over the concrete hashing function that's used, see `put_block_keyed`.
    fn put_block(
        &self,
        bytes: impl Into<Bytes> + CondSend,
        codec: u64,
    ) -> impl Future<Output = Result<Cid, BlockStoreError>> + CondSend {
        let bytes = bytes.into();
        async move {
            let cid = self.create_cid(&bytes, codec)?;
            self.put_block_keyed(cid, bytes).await?;
            Ok(cid)
        }
    }

    /// Put a block of data into this blockstore. The block's CID needs to match the CID given.
    ///
    /// It's up to the blockstore whether to check this fact or assume it when this function is called.
    ///
    /// The default implementation of `put_block` will use this function under the hood and use
    /// the correct CID provided by the `create_cid` function.
    ///
    /// This is useful to be able to add blocks that were generated from other
    /// clients with differently configured hashing functions to this blockstore.
    fn put_block_keyed(
        &self,
        cid: Cid,
        bytes: impl Into<Bytes> + CondSend,
    ) -> impl Future<Output = Result<(), BlockStoreError>> + CondSend;

    /// Find out whether a call to `get_block` would return with a result or not.
    ///
    /// This is useful for data exchange protocols to find out what needs to be fetched
    /// externally and what doesn't.
    fn has_block(
        &self,
        cid: &Cid,
    ) -> impl Future<Output = Result<bool, BlockStoreError>> + CondSend;

    // This should be the same in all implementations of BlockStore
    fn create_cid(&self, bytes: &[u8], codec: u64) -> Result<Cid, BlockStoreError> {
        // If there are too many bytes, abandon this task
        if bytes.len() > MAX_BLOCK_SIZE {
            return Err(BlockStoreError::MaximumBlockSizeExceeded(bytes.len()));
        }

        // Compute the Blake3 hash of the bytes
        let hash = Multihash::wrap(MULTIHASH_BLAKE3, blake3::hash(bytes).as_bytes()).unwrap();

        // Represent the hash as a V1 CID
        let cid = Cid::new(Version::V1, codec, hash)?;

        Ok(cid)
    }
}

//--------------------------------------------------------------------------------------------------
// Implementations
//--------------------------------------------------------------------------------------------------

impl<B: BlockStore> BlockStore for &B {
    async fn get_block(&self, cid: &Cid) -> Result<Bytes, BlockStoreError> {
        (**self).get_block(cid).await
    }

    async fn put_block(
        &self,
        bytes: impl Into<Bytes> + CondSend,
        codec: u64,
    ) -> Result<Cid, BlockStoreError> {
        (**self).put_block(bytes, codec).await
    }

    async fn put_block_keyed(
        &self,
        cid: Cid,
        bytes: impl Into<Bytes> + CondSend,
    ) -> Result<(), BlockStoreError> {
        (**self).put_block_keyed(cid, bytes).await
    }

    async fn has_block(&self, cid: &Cid) -> Result<bool, BlockStoreError> {
        (**self).has_block(cid).await
    }

    fn create_cid(&self, bytes: &[u8], codec: u64) -> Result<Cid, BlockStoreError> {
        (**self).create_cid(bytes, codec)
    }
}

impl<B: BlockStore> BlockStore for Box<B> {
    async fn get_block(&self, cid: &Cid) -> Result<Bytes, BlockStoreError> {
        (**self).get_block(cid).await
    }

    async fn put_block(
        &self,
        bytes: impl Into<Bytes> + CondSend,
        codec: u64,
    ) -> Result<Cid, BlockStoreError> {
        (**self).put_block(bytes, codec).await
    }

    async fn put_block_keyed(
        &self,
        cid: Cid,
        bytes: impl Into<Bytes> + CondSend,
    ) -> Result<(), BlockStoreError> {
        (**self).put_block_keyed(cid, bytes).await
    }

    async fn has_block(&self, cid: &Cid) -> Result<bool, BlockStoreError> {
        (**self).has_block(cid).await
    }

    fn create_cid(&self, bytes: &[u8], codec: u64) -> Result<Cid, BlockStoreError> {
        (**self).create_cid(bytes, codec)
    }
}

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
    async fn get_block(&self, cid: &Cid) -> Result<Bytes, BlockStoreError> {
        let bytes = self
            .0
            .lock()
            .get(cid)
            .ok_or(BlockStoreError::CIDNotFound(*cid))?
            .clone();

        Ok(bytes)
    }

    async fn put_block_keyed(
        &self,
        cid: Cid,
        bytes: impl Into<Bytes> + CondSend,
    ) -> Result<(), BlockStoreError> {
        self.0.lock().insert(cid, bytes.into());

        Ok(())
    }

    async fn has_block(&self, cid: &Cid) -> Result<bool, BlockStoreError> {
        Ok(self.0.lock().contains_key(cid))
    }
}
