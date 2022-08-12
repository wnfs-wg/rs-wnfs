//! The bindgen API for PublicDirectory.

use std::rc::Rc;

use chrono::{DateTime, Utc};
use js_sys::{Date, Array, Promise};
use wasm_bindgen::prelude::wasm_bindgen;
use wnfs::private::{PrivateDirectory as WnfsPrivateDirectory, PrivateNode as WnfsPrivateNode};

use crate::fs::JsResult;

//--------------------------------------------------------------------------------------------------
// Type Definitions
//--------------------------------------------------------------------------------------------------

/// A directory in a WNFS public file system.
#[wasm_bindgen]
pub struct PrivateDirectory(pub(crate) Rc<WnfsPrivateDirectory>);

//--------------------------------------------------------------------------------------------------
// Implementations
//--------------------------------------------------------------------------------------------------

#[wasm_bindgen]
impl PrivateDirectory {
        /// Creates a new directory using the given metadata.
        #[wasm_bindgen(constructor)]
        pub fn new(time: &Date) -> Self {
            let time = DateTime::<Utc>::from(time);
            Self(Rc::new(WnfsPrivateDirectory::new(time)))
        }

        /// Follows a path and fetches the node at the end of the path.
        #[wasm_bindgen(js_name = "getNode")]
        pub fn get_node(&self, path_segments: &Array, hamt: &HamtStore) -> JsResult<Promise> {
            // let directory = Rc::clone(&self.0);
            // let store = ForeignBlockStore(store);
            // let path_segments = utils::convert_path_segments(path_segments)?;

            // Ok(future_to_promise(async move {
            //     let WnfsOpResult { root_dir, result } = directory
            //         .get_node(&path_segments, &store)
            //         .await
            //         .map_err(|e| Error::new(&format!("Cannot get node: {e}")))?;

            //     Ok(utils::create_op_result(root_dir, result.map(PublicNode))?)
            // }))
            todo!()
        }
}
