use super::{PrivateForest, PrivateRef, Rng};
use crate::{
    fs::{
        utils::{self, error},
        BlockStore, ForeignBlockStore, JsResult, PrivateDirectory, PrivateFile,
    },
    value,
};
use js_sys::{Error, Promise};
use std::rc::Rc;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen_futures::future_to_promise;
use wnfs::{private::PrivateNode as WnfsPrivateNode, Id};

//--------------------------------------------------------------------------------------------------
// Type Definitions
//--------------------------------------------------------------------------------------------------

/// Wraps `wnfs::PrivateNode`.
#[wasm_bindgen]
pub struct PrivateNode(pub(crate) WnfsPrivateNode);

//--------------------------------------------------------------------------------------------------
// Implementations
//--------------------------------------------------------------------------------------------------

#[wasm_bindgen]
impl PrivateNode {
    pub fn store(
        &self,
        forest: &PrivateForest,
        store: BlockStore,
        mut rng: Rng,
    ) -> JsResult<Promise> {
        let node = self.0.clone(); // cheap clone
        let mut store = ForeignBlockStore(store);
        let mut forest = Rc::clone(&forest.0);

        Ok(future_to_promise(async move {
            let private_ref = node
                .store(&mut forest, &mut store, &mut rng)
                .await
                .map_err(error("Cannot store node"))?;

            Ok(utils::create_private_forest_result(
                value!(PrivateRef::from(private_ref)),
                forest,
            )?)
        }))
    }

    pub fn load(
        private_ref: PrivateRef,
        forest: &PrivateForest,
        store: BlockStore,
    ) -> JsResult<Promise> {
        let store = ForeignBlockStore(store);
        let forest = Rc::clone(&forest.0);
        let private_ref = private_ref.try_into()?;

        Ok(future_to_promise(async move {
            let node = WnfsPrivateNode::load(&private_ref, &forest, &store)
                .await
                .map_err(error("Cannot load node"))?;

            Ok(value!(PrivateNode(node)))
        }))
    }

    #[wasm_bindgen(js_name = "asDir")]
    pub fn as_dir(&self) -> JsResult<PrivateDirectory> {
        let dir = self
            .0
            .as_dir()
            .map_err(|e| Error::new(&format!("Cannot cast to a directory: {e}")))?;

        Ok(PrivateDirectory(dir))
    }

    #[wasm_bindgen(js_name = "asFile")]
    pub fn as_file(&self) -> JsResult<PrivateFile> {
        let file = self
            .0
            .as_file()
            .map_err(|e| Error::new(&format!("Cannot cast to a file: {e}")))?;

        Ok(PrivateFile(file))
    }

    #[wasm_bindgen(js_name = "isDir")]
    pub fn is_dir(&self) -> bool {
        self.0.is_dir()
    }

    #[wasm_bindgen(js_name = "isFile")]
    pub fn is_file(&self) -> bool {
        self.0.is_file()
    }

    #[wasm_bindgen(js_name = "getId")]
    pub fn get_id(&self) -> String {
        self.0.get_id()
    }
}
