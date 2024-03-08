//! The bindgen API for WNFS block store.

use anyhow::Result;
use bytes::Bytes;
use js_sys::{Promise, Reflect, Uint8Array};
use libipld_core::cid::Cid;
use std::str::FromStr;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
use wasm_bindgen_futures::JsFuture;
use wnfs::common::{BlockStore as WnfsBlockStore, BlockStoreError};

//--------------------------------------------------------------------------------------------------
// Externs
//--------------------------------------------------------------------------------------------------

#[wasm_bindgen(typescript_custom_section)]
const TS_BLOCKSTORE: &'static str = r#"
export interface BlockStore {
    putBlockKeyed(cid: Uint8Array, bytes: Uint8Array): Promise<void>;
    getBlock(cid: Uint8Array): Promise<Uint8Array | undefined>;
    hasBlock(cid: Uint8Array): Promise<boolean>;
    putBlock?(bytes: Uint8Array, codec: number): Promise<Uint8Array>;
}
"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "BlockStore")]
    pub type BlockStore;

    #[wasm_bindgen(method, js_name = "putBlockKeyed")]
    pub(crate) fn put_block_keyed(store: &BlockStore, cid: Vec<u8>, bytes: Vec<u8>) -> Promise;

    #[wasm_bindgen(method, js_name = "putBlock")]
    pub(crate) fn put_block(store: &BlockStore, bytes: Vec<u8>, codec: u32) -> Promise;

    #[wasm_bindgen(method, js_name = "getBlock")]
    pub(crate) fn get_block(store: &BlockStore, cid: Vec<u8>) -> Promise;

    #[wasm_bindgen(method, js_name = "hasBlock")]
    pub(crate) fn has_block(store: &BlockStore, cid: Vec<u8>) -> Promise;
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
    async fn put_block_keyed(
        &self,
        cid: Cid,
        bytes: impl Into<Bytes>,
    ) -> Result<(), BlockStoreError> {
        let bytes: Bytes = bytes.into();

        JsFuture::from(self.0.put_block_keyed(cid.to_bytes(), bytes.into()))
            .await
            .map_err(handle_blockstore_err)?;

        Ok(())
    }

    async fn get_block(&self, cid: &Cid) -> Result<Bytes, BlockStoreError> {
        let value = JsFuture::from(self.0.get_block(cid.to_bytes()))
            .await
            .map_err(handle_blockstore_err)?;

        if value.is_undefined() {
            return Err(BlockStoreError::CIDNotFound(*cid));
        }

        // Convert the value to a vector of bytes.
        let bytes = Uint8Array::new(&value).to_vec();
        Ok(Bytes::from(bytes))
    }

    async fn has_block(&self, cid: &Cid) -> Result<bool, BlockStoreError> {
        let has_block = JsFuture::from(self.0.has_block(cid.to_bytes()))
            .await
            .map_err(handle_blockstore_err)?;

        Ok(js_sys::Boolean::from(has_block).value_of())
    }

    async fn put_block(&self, bytes: impl Into<Bytes>, codec: u64) -> Result<Cid, BlockStoreError> {
        let bytes: Bytes = bytes.into();

        if Reflect::has(&self.0, &"putBlock".into()).map_err(reflection_err)? {
            let codec = codec.try_into().map_err(|e| {
                anyhow::anyhow!("Can't convert 64-bit codec to 32-bit codec for javascript: {e:?}")
            })?;
            let cid = JsFuture::from(self.0.put_block(bytes.into(), codec))
                .await
                .map_err(handle_blockstore_err)?;

            // Convert the value to a vector of bytes.
            let bytes = Uint8Array::new(&cid).to_vec();

            // Construct CID from the bytes.
            Ok(Cid::try_from(&bytes[..])?)
        } else {
            let cid = self.create_cid(&bytes, codec)?;
            self.put_block_keyed(cid, bytes).await?;
            Ok(cid)
        }
    }
}

fn handle_blockstore_err(js_err: JsValue) -> BlockStoreError {
    match into_blockstore_err(js_err) {
        Ok(err) => err,
        Err(err) => err,
    }
}

fn into_blockstore_err(js_err: JsValue) -> Result<BlockStoreError, BlockStoreError> {
    let code = Reflect::get(&js_err, &"code".into()).map_err(reflection_err)?;

    if let Some(code) = code.as_string() {
        Ok(match code.as_ref() {
            "MAXIMUM_BLOCK_SIZE_EXCEEDED" => BlockStoreError::MaximumBlockSizeExceeded(
                Reflect::get(&js_err, &"size".into())
                    .map_err(reflection_err)?
                    .as_f64()
                    .ok_or_else(|| reflection_err("'size' field on error not a number"))?
                    as usize,
            ),
            "CID_NOT_FOUND" => BlockStoreError::CIDNotFound(Cid::from_str(
                &Reflect::get(&js_err, &"cid".into())
                    .map_err(reflection_err)?
                    .as_string()
                    .ok_or_else(|| reflection_err("'cid' field on error not a string"))?,
            )?),
            "CID_ERROR" => BlockStoreError::CIDError(libipld_core::cid::Error::ParsingError),
            _ => {
                // It may just be another error type
                BlockStoreError::Custom(anyhow::anyhow!("Blockstore operation failed: {js_err:?}"))
            }
        })
    } else {
        // 'code' may not be a string, e.g. undefined or integer, due to other errors on the js side.
        Ok(BlockStoreError::Custom(anyhow::anyhow!(
            "Blockstore operation failed: {js_err:?}"
        )))
    }
}

fn reflection_err(err: impl core::fmt::Debug) -> BlockStoreError {
    BlockStoreError::Custom(anyhow::anyhow!(
        "Fatal error while collecting JS error in blockstore operation: {err:?}"
    ))
}
