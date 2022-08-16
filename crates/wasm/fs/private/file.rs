//! The bindgen API for PrivateFile.

use chrono::{DateTime, Utc};
use js_sys::Date;
use wasm_bindgen::prelude::wasm_bindgen;
use wnfs::{
    private::{namefilter::Namefilter, INumber, PrivateFile as WnfsPrivateFile},
    HashOutput, Id,
};

use crate::fs::{utils::error, JsResult};

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
    /// Creates a new file in a WNFS public file system.
    #[wasm_bindgen(constructor)]
    pub fn new(
        parent_bare_name: Vec<u8>, // [u8; 256]
        inumber: Vec<u8>,          // [u8; 32]
        ratchet_seed: Vec<u8>,     // [u8; 32]
        time: &Date,
        content: Vec<u8>,
    ) -> JsResult<PrivateFile> {
        let parent_bare_name = Namefilter::try_from(parent_bare_name)
            .map_err(error("Cannot convert parent bare name"))?;
        let inumber: INumber = inumber
            .try_into()
            .map_err(error("Cannot convert inumber"))?;
        let ratchet_seed: HashOutput = ratchet_seed
            .try_into()
            .map_err(error("Cannot convert ratchet seed"))?;
        let time = DateTime::<Utc>::from(time);

        Ok(PrivateFile(WnfsPrivateFile::new(
            parent_bare_name,
            inumber,
            ratchet_seed,
            time,
            content,
        )))
    }

    /// Gets a unique id for node.
    #[wasm_bindgen(js_name = "getId")]
    pub fn get_id(&self) -> String {
        self.0.get_id()
    }
}
