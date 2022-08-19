//! The bindgen API for PublicFile.

use crate::{fs::metadata::JsMetadata, value};
use chrono::{DateTime, Utc};
use js_sys::{Error, Promise, Uint8Array};
use std::rc::Rc;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
use wasm_bindgen_futures::future_to_promise;
use wnfs::{ipld::Cid, public::PublicFile as WnfsPublicFile, BlockStore as WnfsBlockStore, Id};

use crate::fs::{BlockStore, ForeignBlockStore, JsResult};

/// A file in a WNFS public file system.
#[wasm_bindgen]
pub struct PublicFile(pub(crate) Rc<WnfsPublicFile>);

#[wasm_bindgen]
impl PublicFile {
    /// Creates a new file in a WNFS public file system.
    #[wasm_bindgen(constructor)]
    pub fn new(time: &js_sys::Date, cid: Uint8Array) -> JsResult<PublicFile> {
        let time = DateTime::<Utc>::from(time);

        let cid = Cid::try_from(&cid.to_vec()[..])
            .map_err(|e| Error::new(&format!("Invalid CID: {e}")))?;

        Ok(PublicFile(Rc::new(WnfsPublicFile::new(time, cid))))
    }

    /// Gets a unique id for node.
    #[wasm_bindgen(js_name = "getId")]
    pub fn get_id(&self) -> String {
        self.0.get_id()
    }

    /// Stores a file in provided block store.
    pub fn store(&self, store: BlockStore) -> JsResult<Promise> {
        let file = Rc::clone(&self.0);
        let mut store = ForeignBlockStore(store);

        Ok(future_to_promise(async move {
            let cid = file
                .store(&mut store)
                .await
                .map_err(|e| Error::new(&format!("Cannot add to store: {e}")))?;

            let cid_u8array = Uint8Array::from(&cid.to_bytes()[..]);

            Ok(value!(cid_u8array))
        }))
    }

    /// Loads a file given its CID from the block store.
    pub fn load(cid: Vec<u8>, store: BlockStore) -> JsResult<Promise> {
        let store = ForeignBlockStore(store);
        let cid = Cid::try_from(cid)
            .map_err(|e| Error::new(&format!("Cannot parse cid: {e}")))?;
        Ok(future_to_promise(async move {
            let file: WnfsPublicFile = store
                .get_deserializable(&cid)
                .await
                .map_err(|e| Error::new(&format!("Couldn't deserialize directory: {e}")))?;

            Ok(value!(PublicFile(Rc::new(file))))
        }))
    }

    /// Gets the previous cid of the file or null if the file doesn't have a previous cid.
    #[wasm_bindgen(js_name = "previousCid")]
    pub fn previous_cid(&self) -> JsValue {
        match self.0.get_previous() {
            Some(cid) => {
                let cid_u8array = Uint8Array::from(&cid.to_bytes()[..]);
                value!(cid_u8array)
            }
            None => JsValue::NULL,
        }
    }

    /// Gets the metadata of this file.
    pub fn metadata(&self) -> JsResult<JsValue> {
        JsMetadata(self.0.get_metadata()).try_into()
    }

    /// Gets the content cid of the file.
    #[wasm_bindgen(js_name = "contentCid")]
    pub fn content_cid(&self) -> Vec<u8> {
        self.0.get_content_cid().to_bytes()
    }
}
