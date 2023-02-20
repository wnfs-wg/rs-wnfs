use std::rc::Rc;

use js_sys::Promise;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
use wasm_bindgen_futures::future_to_promise;
use wnfs::{
    ipld::Cid,
    private::{
        AesKey, PrivateForest as WnfsPrivateForest, PrivateRef as WnfsPrivateRef, TemporalKey,
        KEY_BYTE_SIZE,
    },
    HASH_BYTE_SIZE,
};

use crate::{
    fs::{
        utils::{self, error},
        BlockStore, ForeignBlockStore, JsResult, Rng,
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

#[wasm_bindgen]
pub struct PrivateRef {
    pub(crate) label: Vec<u8>,
    pub(crate) temporal_key: Vec<u8>,
    pub(crate) content_cid: Vec<u8>,
}

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
    pub fn get(self, private_ref: PrivateRef, store: BlockStore) -> JsResult<Promise> {
        let store = ForeignBlockStore(store);
        let forest = Rc::clone(&self.0);

        Ok(future_to_promise(async move {
            let private_ref = private_ref.try_into()?;

            let node = forest
                .get(&private_ref, &store)
                .await
                .map_err(error("Error in private forest 'get'"))?;

            Ok(value!(PrivateNode(node)))
        }))
    }

    #[wasm_bindgen]
    pub fn put(self, node: PrivateNode, store: BlockStore, mut rng: Rng) -> JsResult<Promise> {
        let mut store = ForeignBlockStore(store);
        let mut forest = self.0;
        let node = node.0;

        Ok(future_to_promise(async move {
            let private_ref = forest
                .put(&node, &mut store, &mut rng)
                .await
                .map_err(error("Error in private forest 'put'"))?;

            let private_ref: PrivateRef = private_ref.into();

            Ok(utils::create_private_forest_result(
                private_ref.into(),
                forest,
            )?)
        }))
    }
}

#[wasm_bindgen]
impl PrivateRef {
    #[wasm_bindgen(constructor)]
    pub fn new(label: Vec<u8>, temporal_key: Vec<u8>, content_cid: Vec<u8>) -> Self {
        Self {
            label,
            temporal_key,
            content_cid,
        }
    }

    #[wasm_bindgen(js_name = "getLabel")]
    pub fn get_label(&self) -> Vec<u8> {
        self.label.clone()
    }

    #[wasm_bindgen(js_name = "getTemporalKey")]
    pub fn get_temporal_key(&self) -> Vec<u8> {
        self.temporal_key.clone()
    }

    #[wasm_bindgen(js_name = "getContentCid")]
    pub fn get_content_cid(&self) -> Vec<u8> {
        self.content_cid.clone()
    }
}

impl TryInto<WnfsPrivateRef> for PrivateRef {
    type Error = js_sys::Error;

    fn try_into(self) -> Result<WnfsPrivateRef, Self::Error> {
        let PrivateRef {
            label,
            temporal_key,
            content_cid,
        } = self;
        let saturated_name_hash = utils::expect_bytes::<HASH_BYTE_SIZE>(label)?;

        let key_bytes = utils::expect_bytes::<KEY_BYTE_SIZE>(temporal_key)?;
        let key = AesKey::new(key_bytes);
        let temporal_key = TemporalKey(key);

        let content_cid = Cid::try_from(content_cid).map_err(error("Error parsing CID"))?;
        Ok(WnfsPrivateRef {
            saturated_name_hash,
            temporal_key,
            content_cid,
        })
    }
}

impl From<WnfsPrivateRef> for PrivateRef {
    fn from(private_ref: WnfsPrivateRef) -> Self {
        let WnfsPrivateRef {
            saturated_name_hash,
            temporal_key,
            content_cid,
        } = private_ref;
        PrivateRef {
            label: Vec::from(saturated_name_hash),
            temporal_key: Vec::from(temporal_key.0.bytes()),
            content_cid: content_cid.to_bytes(),
        }
    }
}
