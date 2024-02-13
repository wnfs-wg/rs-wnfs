//! The bindgen API for WNFS block store.

use super::utils::anyhow_error;
use anyhow::Result;
use bytes::Bytes;
use js_sys::{Promise, Uint8Array};
use libipld_core::cid::Cid;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen_futures::JsFuture;
use wnfs::common::BlockStore as WnfsBlockStore;

//--------------------------------------------------------------------------------------------------
// Externs
//--------------------------------------------------------------------------------------------------

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "BlockStore")]
    pub type BlockStore;

    #[wasm_bindgen(method, js_name = "putBlock")]
    pub(crate) fn put_block(store: &BlockStore, bytes: Vec<u8>, codec: u32) -> Promise;

    #[wasm_bindgen(method, js_name = "getBlock")]
    pub(crate) fn get_block(store: &BlockStore, cid: Vec<u8>) -> Promise;
}

//--------------------------------------------------------------------------------------------------
// Type Definitions
//--------------------------------------------------------------------------------------------------

/// A block store provided by the host (JavaScript) for custom implementation like connection to the IPFS network.
#[wasm_bindgen]
pub struct ForeignBlockStore(pub(crate) BlockStore);

//--------------------------------------------------------------------------------------------------
// Implementations
//--------------------------------------------------------------------------------------------------

impl WnfsBlockStore for ForeignBlockStore {
    /// Stores an array of bytes in the block store.
    async fn put_block(&self, bytes: impl Into<Bytes>, codec: u64) -> Result<Cid> {
        let bytes: Bytes = bytes.into();

        let value = JsFuture::from(self.0.put_block(bytes.into(), codec.try_into()?))
            .await
            .map_err(anyhow_error("Cannot get block: {:?}"))?;

        // Convert the value to a vector of bytes.
        let bytes = Uint8Array::new(&value).to_vec();

        // Construct CID from the bytes.
        Ok(Cid::try_from(&bytes[..])?)
    }

    /// Retrieves an array of bytes from the block store with given CID.
    async fn get_block<'a>(&'a self, cid: &Cid) -> Result<Bytes> {
        let value = JsFuture::from(self.0.get_block(cid.to_bytes()))
            .await
            .map_err(anyhow_error("Cannot get block: {:?}"))?;

        // Convert the value to a vector of bytes.
        let bytes = Uint8Array::new(&value).to_vec();
        Ok(Bytes::from(bytes))
    }
}
