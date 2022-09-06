//! The bindgen API for PrivateFile.

use chrono::{DateTime, Utc};
use js_sys::Date;
use wasm_bindgen::prelude::wasm_bindgen;
use wnfs::{private::PrivateFile as WnfsPrivateFile, Id};

use crate::fs::{JsResult, Namefilter, Rng};

//--------------------------------------------------------------------------------------------------
// Type Definitions
//--------------------------------------------------------------------------------------------------

/// A file in a WNFS public file system.
#[wasm_bindgen]
pub struct PrivateFile(WnfsPrivateFile);

//--------------------------------------------------------------------------------------------------
// Implementations
//--------------------------------------------------------------------------------------------------

#[wasm_bindgen]
impl PrivateFile {
    /// Creates a new private file.
    #[wasm_bindgen(constructor)]
    pub fn new(
        parent_bare_name: Namefilter,
        time: &Date,
        content: Vec<u8>,
        mut rng: Rng,
    ) -> JsResult<PrivateFile> {
        let time = DateTime::<Utc>::from(time);

        Ok(PrivateFile(WnfsPrivateFile::new(
            parent_bare_name.0,
            time,
            content,
            &mut rng,
        )))
    }

    /// Gets a unique id for node.
    #[wasm_bindgen(js_name = "getId")]
    pub fn get_id(&self) -> String {
        self.0.get_id()
    }
}
