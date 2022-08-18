//! The bindgen API for PublicFile.

use chrono::{DateTime, Utc};
use js_sys::Uint8Array;
use wasm_bindgen::prelude::wasm_bindgen;
use wnfs::{ipld::Cid, public::PublicFile as WnfsPublicFile, Id};

use crate::fs::{utils::error, JsResult};

//--------------------------------------------------------------------------------------------------
// Type Definitions
//--------------------------------------------------------------------------------------------------

/// A file in a WNFS public file system.
#[wasm_bindgen]
pub struct PublicFile(WnfsPublicFile);

//--------------------------------------------------------------------------------------------------
// Implementations
//--------------------------------------------------------------------------------------------------

#[wasm_bindgen]
impl PublicFile {
    /// Creates a new file in a WNFS public file system.
    #[wasm_bindgen(constructor)]
    pub fn new(time: &js_sys::Date, cid: Uint8Array) -> JsResult<PublicFile> {
        let time = DateTime::<Utc>::from(time);
        let cid = Cid::try_from(&cid.to_vec()[..]).map_err(error("Invalid CID"))?;

        Ok(PublicFile(WnfsPublicFile::new(time, cid)))
    }

    /// Gets a unique id for node.
    #[wasm_bindgen(js_name = "getId")]
    pub fn get_id(&self) -> String {
        self.0.get_id()
    }
}
