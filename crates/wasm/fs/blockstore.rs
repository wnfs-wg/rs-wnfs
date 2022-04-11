//! The bindgen API of WNFS block store implemenation.

use std::{borrow::Cow, str::FromStr};

use async_trait::async_trait;
use wasm_bindgen::prelude::wasm_bindgen;
use wnfs::{
    BlockStore as WnfsBlockStore, BlockStoreCidLoad as WnfsBlockStoreCidLoad,
    BlockStoreLookup as WnfsBlockStoreLookup, Cid, Codec, Decode, IpldCodec,
    MemoryBlockStore as WnfsMemoryBlockStore,
};

use super::JsResult;

#[wasm_bindgen]
#[derive(Default)]
pub struct MemoryBlockStore(WnfsMemoryBlockStore);

#[wasm_bindgen]
impl MemoryBlockStore {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self::default()
    }

    #[wasm_bindgen(js_name = "putBlock")]
    pub async fn put_block(&mut self, bytes: Vec<u8>, codec: u64) -> JsResult<String> {
        let codec = IpldCodec::try_from(codec).map_err(|_| js_sys::Error::new("Invalid codec"))?;

        let cid = self
            .0
            .put_block(bytes, codec)
            .await
            .map_err(|_| js_sys::Error::new("Failed to put block"))?;

        Ok(cid.to_string())
    }

    #[wasm_bindgen(js_name = "getBlock")]
    pub async fn get_block(&self, cid: &str) -> JsResult<js_sys::Uint8Array> {
        let cid = Cid::from_str(cid).map_err(|_| js_sys::Error::new("Invalid CID"))?;

        let bytes = self
            .0
            .get_block(&cid)
            .await
            .map_err(|_| js_sys::Error::new("Failed to get block"))?;

        Ok(js_sys::Uint8Array::from(&bytes[..]))
    }
}

#[async_trait]
impl WnfsBlockStore for MemoryBlockStore {
    async fn put_block(
        &mut self,
        bytes: Vec<u8>,
        codec: wnfs::IpldCodec,
    ) -> Result<wnfs::Cid, anyhow::Error> {
        self.0.put_block(bytes, codec).await
    }
}

#[async_trait]
impl WnfsBlockStoreLookup for MemoryBlockStore {
    async fn get_block<'a>(&'a self, cid: &wnfs::Cid) -> Result<Cow<'a, [u8]>, anyhow::Error> {
        self.0.get_block(cid).await
    }
}

#[async_trait]
impl WnfsBlockStoreCidLoad for MemoryBlockStore {
    async fn load<T: Decode<C>, C: Codec>(
        &self,
        cid: &Cid,
        decoder: C,
    ) -> Result<T, anyhow::Error> {
        self.0.load(cid, decoder).await
    }
}
