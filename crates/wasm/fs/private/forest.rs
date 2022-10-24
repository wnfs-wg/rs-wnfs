use std::rc::Rc;

use js_sys::{Error, Promise, Uint8Array};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
use wasm_bindgen_futures::future_to_promise;
use wnfs::{
    private::{Key, PrivateForest as WnfsPrivateForest, PrivateRef, RatchetKey, KEY_BYTE_SIZE},
    HASH_BYTE_SIZE,
};

use crate::{
    fs::{utils::error, BlockStore, ForeignBlockStore, JsResult},
    value,
};

use super::PrivateNode;

//--------------------------------------------------------------------------------------------------
// Type Definitions
//--------------------------------------------------------------------------------------------------

/// A HAMT forest in a WNFS public file system.
#[wasm_bindgen]
pub struct PrivateForest(pub(crate) Rc<WnfsPrivateForest>);

//--------------------------------------------------------------------------------------------------
// Implementations
//--------------------------------------------------------------------------------------------------

#[wasm_bindgen]
impl PrivateForest {
    /// Creates a new HAMT forest.
    #[wasm_bindgen(constructor)]
    #[allow(clippy::new_without_default)]
    pub fn new() -> PrivateForest {
        Self(Rc::new(WnfsPrivateForest::default()))
    }

    #[wasm_bindgen]
    pub fn get(
        &self,
        saturated_namefilter_hash: Vec<u8>,
        revision_key: Vec<u8>,
        store: BlockStore,
    ) -> JsResult<Promise> {
        let store = ForeignBlockStore(store);
        let forest = self.0.clone();

        let saturated_name_hash = expect_bytes::<HASH_BYTE_SIZE>(saturated_namefilter_hash)?;

        let key_bytes = expect_bytes::<KEY_BYTE_SIZE>(revision_key)?;
        let key = Key::new(key_bytes);
        let ratchet_key = RatchetKey(key);

        let private_ref = PrivateRef::from_ratchet_key(saturated_name_hash, ratchet_key);

        Ok(future_to_promise(async move {
            let node_option = forest
                .get(&private_ref, &store)
                .await
                .map_err(error("Cannot 'get' in forest"))?;

            Ok(match node_option {
                Some(node) => value!(PrivateNode(node)),
                None => JsValue::NULL,
            })
        }))
    }
}

#[inline]
fn expect_bytes<const N: usize>(bytes: Vec<u8>) -> JsResult<[u8; N]> {
    bytes.try_into().map_err(|v: Vec<u8>| {
        Error::new(&format!(
            "Unexpected number of bytes received. Expected {N}, but got {}",
            v.len()
        ))
    })
}
