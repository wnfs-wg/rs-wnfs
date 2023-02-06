use std::rc::Rc;

use js_sys::Promise;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
use wasm_bindgen_futures::future_to_promise;
use wnfs::{
    private::{AesKey, PrivateForest as WnfsPrivateForest, PrivateRef, TemporalKey, KEY_BYTE_SIZE},
    HASH_BYTE_SIZE,
};

use crate::{
    fs::{
        utils::{self, error},
        BlockStore, ForeignBlockStore, JsResult,
    },
    value,
};

use super::PrivateNode;

//--------------------------------------------------------------------------------------------------
// Type Definitions
//--------------------------------------------------------------------------------------------------

/// A reference to a private forest. Used for the private file system.
#[wasm_bindgen]
pub struct PrivateForest(pub(crate) Rc<WnfsPrivateForest>);

//--------------------------------------------------------------------------------------------------
// Implementations
//--------------------------------------------------------------------------------------------------

#[wasm_bindgen]
impl PrivateForest {
    /// Creates a new private forest.
    #[wasm_bindgen(constructor)]
    #[allow(clippy::new_without_default)]
    pub fn new() -> PrivateForest {
        Self(Rc::new(WnfsPrivateForest::default()))
    }

    #[wasm_bindgen]
    pub fn get(
        &self,
        saturated_namefilter_hash: Vec<u8>,
        temporal_key: Vec<u8>,
        store: BlockStore,
    ) -> JsResult<Promise> {
        let store = ForeignBlockStore(store);
        let forest = self.0.clone();

        let saturated_name_hash = utils::expect_bytes::<HASH_BYTE_SIZE>(saturated_namefilter_hash)?;

        let key_bytes = utils::expect_bytes::<KEY_BYTE_SIZE>(temporal_key)?;
        let key = AesKey::new(key_bytes);
        let temporal_key = TemporalKey(key);

        let private_ref = PrivateRef::with_temporal_key(saturated_name_hash, temporal_key);

        Ok(future_to_promise(async move {
            let node_option = forest
                .get(&private_ref, WnfsPrivateForest::resolve_lowest, &store)
                .await
                .map_err(error("Cannot 'get' in forest"))?;

            Ok(match node_option {
                Some(node) => value!(PrivateNode(node)),
                None => JsValue::NULL,
            })
        }))
    }
}
