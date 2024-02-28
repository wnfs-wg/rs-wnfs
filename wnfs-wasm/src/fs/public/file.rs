//! The bindgen API for PublicFile.

use crate::{
    fs::{metadata::JsMetadata, utils::error, BlockStore, ForeignBlockStore, JsResult, PublicNode},
    value,
};
use chrono::{DateTime, Utc};
use js_sys::{Error, Number, Promise, Uint8Array};
use libipld_core::cid::Cid;
use std::rc::Rc;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
use wasm_bindgen_futures::future_to_promise;
use wnfs::{
    common::Storable,
    public::{PublicFile as WnfsPublicFile, PublicNode as WnfsPublicNode},
    traits::Id,
};

//--------------------------------------------------------------------------------------------------
// Type Definitions
//--------------------------------------------------------------------------------------------------

/// A file in a WNFS public file system.
#[wasm_bindgen]
pub struct PublicFile(pub(crate) Rc<WnfsPublicFile>);

//--------------------------------------------------------------------------------------------------
// Implementations
//--------------------------------------------------------------------------------------------------

#[wasm_bindgen]
impl PublicFile {
    /// Creates a new file in a WNFS public file system.
    #[wasm_bindgen(constructor)]
    pub fn new(time: &js_sys::Date) -> PublicFile {
        let time = DateTime::<Utc>::from(time);
        Self(WnfsPublicFile::new_rc(time))
    }

    /// Gets a unique id for node.
    #[wasm_bindgen(js_name = "getId")]
    pub fn get_id(&self) -> String {
        self.0.get_id()
    }

    /// Stores a file in provided block store.
    pub fn store(&self, store: BlockStore) -> JsResult<Promise> {
        let file = Rc::clone(&self.0);
        let store = ForeignBlockStore(store);

        Ok(future_to_promise(async move {
            let cid = file
                .store(&store)
                .await
                .map_err(|e| Error::new(&format!("Cannot add to store: {e}")))?;

            let cid_u8array = Uint8Array::from(&cid.to_bytes()[..]);

            Ok(value!(cid_u8array))
        }))
    }

    /// Loads a file given its CID from the block store.
    pub fn load(cid: Vec<u8>, store: BlockStore) -> JsResult<Promise> {
        let store = ForeignBlockStore(store);
        let cid = Cid::try_from(cid).map_err(|e| Error::new(&format!("Cannot parse cid: {e}")))?;

        Ok(future_to_promise(async move {
            let file = WnfsPublicFile::load(&cid, &store)
                .await
                .map_err(|e| Error::new(&format!("Couldn't deserialize directory: {e}")))?;

            Ok(value!(PublicFile(Rc::new(file))))
        }))
    }

    /// Gets the previous CID(s) of the file.
    /// This will usually be an array of a single CID, but may be
    /// - an empty array, if this is the first revision of a file
    /// - an array with multiple elements if this is the merge node of
    ///   multiple concurrent changes to the file.
    #[wasm_bindgen(js_name = "previousCids")]
    pub fn previous_cids(&self) -> Vec<Uint8Array> {
        let cids = self.0.get_previous();
        let arr: Vec<Uint8Array> = cids
            .iter()
            .map(|cid| Uint8Array::from(&cid.to_bytes()[..]))
            .collect();
        arr
    }

    /// Gets the entire content of a file.
    #[wasm_bindgen(js_name = "getContent")]
    pub fn get_content(&self, store: BlockStore) -> JsResult<Promise> {
        self.read_at(value!(0).into(), None, store)
    }

    /// Gets the exact content size without fetching all content blocks.
    #[wasm_bindgen(js_name = "getSize")]
    pub fn get_size(&self, store: BlockStore) -> JsResult<Promise> {
        let file = Rc::clone(&self.0);
        let store = ForeignBlockStore(store);

        Ok(future_to_promise(async move {
            let size = file
                .get_size(&store)
                .await
                .map_err(error("Cannot determine file size"))?;

            Ok(value!(size))
        }))
    }

    /// Gets the metadata of this file.
    pub fn metadata(&self) -> JsResult<JsValue> {
        JsMetadata(self.0.get_metadata()).try_into()
    }

    /// Gets the content cid of the file.
    #[wasm_bindgen(js_name = "getRawContentCid")]
    pub fn get_raw_content_cid(&self, store: BlockStore) -> JsResult<Promise> {
        let file = Rc::clone(&self.0);
        let store = ForeignBlockStore(store);

        Ok(future_to_promise(async move {
            let content_cid: Cid = file.get_raw_content_cid(&store).await;
            Ok(value!(Uint8Array::from(&content_cid.to_bytes()[..])))
        }))
    }

    /// Gets the content of the file at given offset & with an optional byte limit.
    #[wasm_bindgen(js_name = "readAt")]
    pub fn read_at(
        &self,
        byte_offset: Number,
        limit: Option<Number>,
        store: BlockStore,
    ) -> JsResult<Promise> {
        let file = Rc::clone(&self.0);
        let store = ForeignBlockStore(store);
        let byte_offset = f64::from(byte_offset) as u64;
        let limit = limit.map(|lim| f64::from(lim) as usize);

        Ok(future_to_promise(async move {
            let result = file
                .read_at(byte_offset, limit, &store)
                .await
                .map_err(error("Cannot read file"))?;

            let uint8array = Uint8Array::from(result.as_ref());

            Ok(value!(uint8array))
        }))
    }

    /// Sets the content of a file to a byte array.
    #[wasm_bindgen(js_name = "setContent")]
    pub fn set_content(
        &self,
        time: &js_sys::Date,
        content: Vec<u8>,
        store: BlockStore,
    ) -> JsResult<Promise> {
        let mut file = Rc::clone(&self.0);
        let store = ForeignBlockStore(store);
        let time = DateTime::<Utc>::from(time);

        Ok(future_to_promise(async move {
            file.set_content(time, content, &store)
                .await
                .map_err(error("Cannot set file content"))?;

            Ok(value!(PublicFile(file)))
        }))
    }

    /// Converts this directory to a node.
    #[wasm_bindgen(js_name = "asNode")]
    pub fn as_node(&self) -> PublicNode {
        PublicNode(WnfsPublicNode::File(Rc::clone(&self.0)))
    }
}
