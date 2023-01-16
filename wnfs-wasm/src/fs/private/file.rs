//! The bindgen API for PrivateFile.

use crate::{
    fs::{
        metadata::JsMetadata,
        utils::{self, error},
        BlockStore, ForeignBlockStore, JsResult, Namefilter, PrivateForest, Rng,
    },
    value,
};
use chrono::{DateTime, Utc};
use js_sys::{Date, Promise, Uint8Array};
use std::rc::Rc;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
use wasm_bindgen_futures::future_to_promise;
use wnfs::{Id, PrivateFile as WnfsPrivateFile};

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
    pub fn new(parent_bare_name: Namefilter, time: &Date, mut rng: Rng) -> JsResult<PrivateFile> {
        let time = DateTime::<Utc>::from(time);

        Ok(Self(Rc::new(WnfsPrivateFile::new(
            parent_bare_name.0,
            time,
            &mut rng,
        ))))
    }

    /// Creates a file with provided content.
    #[wasm_bindgen(js_name = "withContent")]
    pub fn with_content(
        parent_bare_name: Namefilter,
        time: &Date,
        content: Vec<u8>,
        forest: PrivateForest,
        store: BlockStore,
        mut rng: Rng,
    ) -> JsResult<Promise> {
        let mut store = ForeignBlockStore(store);
        let time = DateTime::<Utc>::from(time);

        Ok(future_to_promise(async move {
            let (file, forest) = WnfsPrivateFile::with_content(
                parent_bare_name.0,
                time,
                content,
                forest.0,
                &mut store,
                &mut rng,
            )
            .await
            .map_err(error("Cannot create a file with provided content"))?;

            Ok(utils::create_private_file_result(
                PrivateFile(Rc::new(file)),
                PrivateForest(forest),
            )?)
        }))
    }

    /// Gets the entire content of a file.
    #[wasm_bindgen(js_name = "getContent")]
    pub fn get_content(&self, forest: PrivateForest, store: BlockStore) -> JsResult<Promise> {
        let file = Rc::clone(&self.0);
        let store = ForeignBlockStore(store);

        Ok(future_to_promise(async move {
            let content = file
                .get_content(&forest.0, &store)
                .await
                .map_err(error("Cannot get content of file"))?;

            Ok(value!(Uint8Array::from(content.as_slice())))
        }))
    }

    /// Gets the metadata of this file.
    pub fn metadata(&self) -> JsResult<JsValue> {
        JsMetadata(self.0.get_metadata()).try_into()
    }

    /// Gets a unique id for node.
    #[wasm_bindgen(js_name = "getId")]
    pub fn get_id(&self) -> String {
        self.0.get_id()
    }
}
