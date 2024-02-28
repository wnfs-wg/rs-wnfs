//! The bindgen API for PrivateFile.
use super::Name;
use crate::{
    fs::{
        metadata::JsMetadata,
        utils::{self, error},
        BlockStore, ForeignBlockStore, JsResult, PrivateForest, PrivateNode, Rng,
    },
    value,
};
use chrono::{DateTime, Utc};
use js_sys::{Date, Number, Promise, Uint8Array};
use std::rc::Rc;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
use wasm_bindgen_futures::future_to_promise;
use wnfs::{
    private::{PrivateFile as WnfsPrivateFile, PrivateNode as WnfsPrivateNode},
    traits::Id,
};
//--------------------------------------------------------------------------------------------------
// Type Definitions
//--------------------------------------------------------------------------------------------------

/// A file in a WNFS public file system.
#[wasm_bindgen]
pub struct PrivateFile(pub(crate) Rc<WnfsPrivateFile>);

//--------------------------------------------------------------------------------------------------
// Implementations
//--------------------------------------------------------------------------------------------------

#[wasm_bindgen]
impl PrivateFile {
    /// Creates an empty private file.
    #[wasm_bindgen(constructor)]
    pub fn new(parent_bare_name: Name, time: &Date, mut rng: Rng) -> JsResult<PrivateFile> {
        let time = DateTime::<Utc>::from(time);

        Ok(Self(Rc::new(WnfsPrivateFile::new(
            &parent_bare_name.0,
            time,
            &mut rng,
        ))))
    }

    /// Creates a file with provided content.
    #[wasm_bindgen(js_name = "withContent")]
    pub fn with_content(
        parent_bare_name: Name,
        time: &Date,
        content: Vec<u8>,
        forest: &PrivateForest,
        store: BlockStore,
        mut rng: Rng,
    ) -> JsResult<Promise> {
        let mut store = ForeignBlockStore(store);
        let time = DateTime::<Utc>::from(time);
        let mut forest = Rc::clone(&forest.0);

        Ok(future_to_promise(async move {
            let file = WnfsPrivateFile::with_content(
                &parent_bare_name.0,
                time,
                content,
                &mut forest,
                &mut store,
                &mut rng,
            )
            .await
            .map_err(error("Cannot create a file with provided content"))?;

            Ok(utils::create_private_forest_result(
                PrivateFile(Rc::new(file)).into(),
                forest,
            )?)
        }))
    }

    /// Persists the current state of this file in the BlockStore and PrivateForest.
    /// This will also force a history entry to be created, if there were changes.
    pub fn store(&self, forest: &PrivateForest, store: BlockStore, rng: Rng) -> JsResult<Promise> {
        let node = PrivateNode(WnfsPrivateNode::File(Rc::clone(&self.0)));
        node.store(forest, store, rng)
    }

    /// Gets the entire content of a file.
    #[wasm_bindgen(js_name = "getContent")]
    pub fn get_content(&self, forest: &PrivateForest, store: BlockStore) -> JsResult<Promise> {
        self.read_at(value!(0).into(), None, forest, store)
    }

    /// Gets the exact content size without fetching all content blocks.
    #[wasm_bindgen(js_name = "getSize")]
    pub fn get_size(&self, forest: &PrivateForest, store: BlockStore) -> JsResult<Promise> {
        let file = Rc::clone(&self.0);
        let store = ForeignBlockStore(store);
        let forest = Rc::clone(&forest.0);

        Ok(future_to_promise(async move {
            let size = file
                .get_size(&forest, &store)
                .await
                .map_err(error("Cannot determine file size"))?;

            Ok(value!(size))
        }))
    }

    /// Gets the metadata of this file.
    pub fn metadata(&self) -> JsResult<JsValue> {
        JsMetadata(self.0.get_metadata()).try_into()
    }

    /// Gets the content of the file at given offset & with an optional byte limit.
    #[wasm_bindgen(js_name = "readAt")]
    pub fn read_at(
        &self,
        byte_offset: Number,
        limit: Option<Number>,
        forest: &PrivateForest,
        store: BlockStore,
    ) -> JsResult<Promise> {
        let file = Rc::clone(&self.0);
        let store = ForeignBlockStore(store);
        let forest = Rc::clone(&forest.0);

        let byte_offset = f64::from(byte_offset) as u64;
        let limit = limit.map(|lim| f64::from(lim) as usize);

        Ok(future_to_promise(async move {
            let result = file
                .read_at(byte_offset, limit, &forest, &store)
                .await
                .map_err(error("Cannot read file"))?;

            let uint8array = Uint8Array::from(result.as_ref());

            Ok(value!(uint8array))
        }))
    }

    /// Gets a unique id for node.
    #[wasm_bindgen(js_name = "getId")]
    pub fn get_id(&self) -> String {
        self.0.get_id()
    }

    /// Converts this file to a node.
    #[wasm_bindgen(js_name = "asNode")]
    pub fn as_node(&self) -> PrivateNode {
        PrivateNode(WnfsPrivateNode::File(Rc::clone(&self.0)))
    }
}
