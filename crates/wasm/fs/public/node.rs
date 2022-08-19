use js_sys::Error;
use wasm_bindgen::prelude::wasm_bindgen;
use wnfs::{public::PublicNode as WnfsPublicNode, Id};

use crate::fs::{JsResult, PublicDirectory, PublicFile};

//--------------------------------------------------------------------------------------------------
// Type Definitions
//--------------------------------------------------------------------------------------------------

/// Wraps a wnfs PublicNode.
#[wasm_bindgen]
pub struct PublicNode(pub(crate) WnfsPublicNode);

//--------------------------------------------------------------------------------------------------
// Implementations
//--------------------------------------------------------------------------------------------------

#[wasm_bindgen]
impl PublicNode {
    #[wasm_bindgen(js_name = "asDir")]
    pub fn as_dir(&self) -> JsResult<PublicDirectory> {
        let dir = self
            .0
            .as_dir()
            .map_err(|e| Error::new(&format!("Cannot cast to a directory: {e}")))?;

        Ok(PublicDirectory(dir))
    }

    #[wasm_bindgen(js_name = "asFile")]
    pub fn as_file(&self) -> JsResult<PublicFile> {
        let file = self
            .0
            .as_file()
            .map_err(|e| Error::new(&format!("Cannot cast to a file: {e}")))?;

        Ok(PublicFile(file))
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
