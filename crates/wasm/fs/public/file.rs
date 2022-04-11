//! The bindgen API of PublicFile.

use std::str::FromStr;

use chrono::{DateTime, Utc};
use wasm_bindgen::prelude::wasm_bindgen;
use wnfs::{public::PublicFile as WnfsPublicFile, Cid};

use crate::fs::{JsResult, MemoryBlockStore};

#[wasm_bindgen]
pub struct PublicFile(WnfsPublicFile);

#[wasm_bindgen]
impl PublicFile {
    #[wasm_bindgen(constructor)]
    pub fn new(time: &js_sys::Date, cid: &str) -> JsResult<PublicFile> {
        // let time = DateTime::<Utc>::from(time); // TODO(appcypher): Fix this.
        let time = Utc::now();

        let cid = Cid::from_str(cid).map_err(|_| js_sys::Error::new("Invalid CID"))?;

        Ok(PublicFile(WnfsPublicFile::new(time, cid)))
    }
}
