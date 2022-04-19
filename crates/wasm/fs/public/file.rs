//! The bindgen API for PublicFile.

use std::str::FromStr;

use chrono::{DateTime, Utc};
use js_sys::Error;
use wasm_bindgen::prelude::wasm_bindgen;
use wnfs::{public::{PublicFile as WnfsPublicFile, Id}, Cid};

use crate::fs::JsResult;

/// A file in a WNFS public file system.
#[wasm_bindgen]
pub struct PublicFile(WnfsPublicFile);

#[wasm_bindgen]
impl PublicFile {
    /// Creates a new file in a WNFS public file system.
    #[wasm_bindgen(constructor)]
    pub fn new(time: &js_sys::Date, cid: &str) -> JsResult<PublicFile> {
        let time = DateTime::<Utc>::from(time);
        let cid = Cid::from_str(cid).map_err(|_| Error::new("Invalid CID"))?;
        Ok(PublicFile(WnfsPublicFile::new(time, cid)))
    }

    /// Gets a unique id for node.
    #[wasm_bindgen(js_name = "getId")]
    pub fn get_id(&self) -> String {
        self.0.get_id()
    }
}
