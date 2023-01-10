use js_sys::Error;
use wasm_bindgen::prelude::wasm_bindgen;
use wnfs::{private::PrivateNode as WnfsPrivateNode, Id};

use crate::fs::{JsResult, PrivateDirectory, PrivateFile};

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
