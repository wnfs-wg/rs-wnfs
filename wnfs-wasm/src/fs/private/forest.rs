use crate::{
    fs::{utils::error, BlockStore, ForeignBlockStore, JsResult},
    value,
};
use js_sys::{Promise, Uint8Array};
use std::rc::Rc;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen_futures::future_to_promise;
use wnfs::{ipld::Cid, private::PrivateForest as WnfsPrivateForest, BlockStore as WnfsBlockStore};

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

    /// Loads an existing private forest from a given CID
    /// You need to have previously `.store()`ed it to get its CID.
    pub fn load(cid: Vec<u8>, store: BlockStore) -> JsResult<Promise> {
        let store = ForeignBlockStore(store);
        let cid = Cid::read_bytes(&cid[..]).map_err(error("Cannot parse cid"))?;

        Ok(future_to_promise(async move {
            let forest: WnfsPrivateForest = store
                .get_deserializable(&cid)
                .await
                .map_err(error("Couldn't deserialize forest"))?;

            Ok(value!(PrivateForest(Rc::new(forest))))
        }))
    }

    /// Stores this private forest in provided block store.
    /// Returns the CID from which it can be `.load()`ed again.
    pub fn store(&self, store: BlockStore) -> JsResult<Promise> {
        let forest = Rc::clone(&self.0);
        let mut store = ForeignBlockStore(store);

        Ok(future_to_promise(async move {
            let cid = store
                .put_async_serializable(&forest)
                .await
                .map_err(error("Cannot add to store"))?;

            let cid_u8array = Uint8Array::from(&cid.to_bytes()[..]);

            Ok(value!(cid_u8array))
        }))
    }
}
