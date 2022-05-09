//! Block store traits.

use std::{borrow::Cow, io::Cursor};

use anyhow::Result;
use async_trait::async_trait;
use hashbrown::HashMap;
use libipld::{cbor::DagCborCodec, cid::Version, codec::Decode, Cid, IpldCodec};
use multihash::{Code, MultihashDigest};

use super::FsError;

//--------------------------------------------------------------------------------------------------
// Type Definitions
//--------------------------------------------------------------------------------------------------

/// For types that implement getting a block from a CID.
#[async_trait(?Send)]
pub trait BlockStoreLookup {
    async fn get_block<'a>(&'a self, cid: &Cid) -> Result<Cow<'a, Vec<u8>>>;
}

// /// For types that implement loading decodable object from a blockstore using a CID.
// #[async_trait(?Send)]
// pub trait BlockStoreCidLoad {
//     /// Loads a decodable object from the store with provided CID.
//     async fn load<T: Decode<C>, C: Codec>(&self, cid: &Cid, decoder: C) -> Result<T>;
// }

/// For types that implement block store operations like adding, getting content from the store.
#[async_trait(?Send)]
pub trait BlockStore: BlockStoreLookup {
    async fn put_block(&mut self, bytes: Vec<u8>, codec: IpldCodec) -> Result<Cid>;
}

/// An in-memory block store to simulate IPFS.
///
/// IPFS is basically an glorified HashMap.
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

#[async_trait(?Send)]
impl BlockStore for MemoryBlockStore {
    /// Stores an array of bytes in the block store.
    async fn put_block(&mut self, bytes: Vec<u8>, codec: IpldCodec) -> Result<Cid> {
        let hash = Code::Sha2_256.digest(&bytes);
        let cid = Cid::new(Version::V1, codec.into(), hash)?;

        self.0.insert((&cid).to_string(), bytes);

        Ok(cid)
    }
}

#[async_trait(?Send)]
impl BlockStoreLookup for MemoryBlockStore {
    /// Retrieves an array of bytes from the block store with given CID.
    async fn get_block<'a>(&'a self, cid: &Cid) -> Result<Cow<'a, Vec<u8>>> {
        let bytes = self
            .0
            .get(&cid.to_string())
            .ok_or(FsError::CIDNotFoundInBlockstore)?;

        Ok(Cow::Borrowed(bytes))
    }
}

//--------------------------------------------------------------------------------------------------
// Functions
//--------------------------------------------------------------------------------------------------

/// Loads a CBOR-encoded data from the store with provided CID.
pub async fn load<B: BlockStore, T: Decode<DagCborCodec>>(store: &B, cid: &Cid) -> Result<T> {
    let bytes = store.get_block(cid).await?;
    let decoded = T::decode(DagCborCodec, &mut Cursor::new(bytes.as_ref()))?;
    Ok(decoded)
}

#[cfg(test)]
mod blockstore_tests {
    use libipld::{cbor::DagCborCodec, codec::Encode};

    use super::*;

    #[async_std::test]
    async fn inserted_items_can_be_fetched() {
        let mut store = MemoryBlockStore::new();

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

        let first_loaded: Vec<u8> = super::load(&store, first_cid).await.unwrap();
        let second_loaded: Vec<u8> = super::load(&store, second_cid).await.unwrap();

        assert_eq!(first_loaded, vec![1, 2, 3, 4, 5]);
        assert_eq!(second_loaded, b"hello world".to_vec());
    }
}
