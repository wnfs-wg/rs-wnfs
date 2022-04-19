//! The bindgen API for WNFS block store.

use std::str::FromStr;
use std::{borrow::Cow, rc::Rc};

use async_trait::async_trait;
use js_sys::{Error, Promise, Uint8Array};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
use wasm_bindgen_futures::future_to_promise;
use wnfs::{
    BlockStore as WnfsBlockStore, BlockStoreCidLoad as WnfsBlockStoreCidLoad,
    BlockStoreLookup as WnfsBlockStoreLookup, Cid, Codec, Decode, IpldCodec,
    MemoryBlockStore as WnfsMemoryBlockStore, Shared,
};

//--------------------------------------------------------------------------------------------------
// Externs
//--------------------------------------------------------------------------------------------------

#[wasm_bindgen]
extern "C" {
    pub type ExternBlockStore;

    #[wasm_bindgen(js_name = "getBlock")]
    fn get_block(this: ExternBlockStore, cid: String) -> Promise;

    #[wasm_bindgen(js_name = "putBlock")]
    fn put_block(this: ExternBlockStore, cid: String) -> Promise;

    #[wasm_bindgen(js_name = "putBlock")]
    fn load(this: ExternBlockStore, cid: String) -> Promise;
}

//--------------------------------------------------------------------------------------------------
// Type Definitions
//--------------------------------------------------------------------------------------------------

/// An in-memory block store to simulate IPFS.
#[wasm_bindgen]
#[derive(Default)]
pub struct MemoryBlockStore(pub(crate) Shared<WnfsMemoryBlockStore>);

/// A block store provided by the host (JavaScript) for csutom implementation like connection to the IPFS network.
#[wasm_bindgen]
pub struct ForeignBlockStore(ExternBlockStore);

//--------------------------------------------------------------------------------------------------
// Implementations
//--------------------------------------------------------------------------------------------------

#[wasm_bindgen]
impl MemoryBlockStore {
    /// Creates a new in-memory block store.
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self::default()
    }

    /// Stores an array of bytes in the block store.
    #[wasm_bindgen(js_name = "putBlock")]
    pub fn put_block(&self, bytes: Vec<u8>, codec: u64) -> Promise {
        let store = Rc::clone(&self.0);

        future_to_promise(async move {
            let codec = IpldCodec::try_from(codec)
                .map_err(|e| Error::new(&format!("Invalid codec: {e}")))?;

            let cid = store
                .borrow_mut()
                .put_block(bytes, codec)
                .await
                .map_err(|_| Error::new("Failed to put block"))?;

            let value = JsValue::from(cid.to_string());

            Ok(value)
        })
    }

    /// Gets a block of bytes from the store with provided CID.
    #[wasm_bindgen(js_name = "getBlock")]
    pub fn get_block(&self, cid: String) -> Promise {
        let store = Rc::clone(&self.0);

        future_to_promise(async move {
            let cid = Cid::from_str(&cid).map_err(|e| Error::new(&format!("Invalid CID: {e}")))?;

            let store_ref = store.borrow();

            let bytes = store_ref
                .get_block(&cid)
                .await
                .map_err(|e| Error::new(&format!("Failed to get block: {e}")))?;

            let value = JsValue::from(Uint8Array::from(&bytes[..]));

            Ok(value)
        })
    }
}

#[async_trait(?Send)]
impl WnfsBlockStore for MemoryBlockStore {
    async fn put_block(
        &mut self,
        bytes: Vec<u8>,
        codec: wnfs::IpldCodec,
    ) -> Result<wnfs::Cid, anyhow::Error> {
        let mut store = self.0.borrow_mut();
        store.put_block(bytes, codec).await
    }
}

#[async_trait(?Send)]
impl WnfsBlockStoreLookup for MemoryBlockStore {
    async fn get_block<'a>(&'a self, cid: &wnfs::Cid) -> Result<Cow<'a, Vec<u8>>, anyhow::Error> {
        let store = self.0.borrow();
        store.get_block(cid).await.map(|x| Cow::Owned(x.to_vec()))
    }
}

#[async_trait(?Send)]
impl WnfsBlockStoreCidLoad for MemoryBlockStore {
    async fn load<T: Decode<C>, C: Codec>(
        &self,
        cid: &Cid,
        decoder: C,
    ) -> Result<T, anyhow::Error> {
        let store = self.0.borrow();
        store.load(cid, decoder).await
    }
}

#[cfg(test)]
mod blockstore_tests {
    use super::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn it_can_create_memory_block_store() {
        MemoryBlockStore::default();
    }
}
