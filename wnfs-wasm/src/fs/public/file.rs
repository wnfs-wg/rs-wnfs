//! The bindgen API for PublicFile.

use crate::{
    fs::{metadata::JsMetadata, utils::error, BlockStore, ForeignBlockStore, JsResult, PublicNode},
    value,
};
use chrono::{DateTime, Utc};
use js_sys::{Error, Promise, Uint8Array};
use libipld_core::cid::Cid;
use std::sync::Arc;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
use wasm_bindgen_futures::future_to_promise;
use wnfs::{
    common::BlockStore as WnfsBlockStore,
    public::{PublicFile as WnfsPublicFile, PublicNode as WnfsPublicNode},
    traits::Id,
};

//--------------------------------------------------------------------------------------------------
// Type Definitions
//--------------------------------------------------------------------------------------------------

/// A file in a WNFS public file system.
#[wasm_bindgen]
pub struct PublicFile(pub(crate) Arc<WnfsPublicFile>);

//--------------------------------------------------------------------------------------------------
// Implementations
//--------------------------------------------------------------------------------------------------

#[wasm_bindgen]
impl PublicFile {
    /// Creates a new file in a WNFS public file system.
    #[wasm_bindgen(constructor)]
    pub fn new(time: &js_sys::Date, cid: Vec<u8>) -> JsResult<PublicFile> {
        let time = DateTime::<Utc>::from(time);
        let cid = Cid::try_from(&cid[..]).map_err(error("Invalid CID"))?;

        Ok(PublicFile(Arc::new(WnfsPublicFile::new(time, cid))))
    }

    /// Gets a unique id for node.
    #[wasm_bindgen(js_name = "getId")]
    pub fn get_id(&self) -> String {
        self.0.get_id()
    }

    /// Stores a file in provided block store.
    pub fn store(&self, store: BlockStore) -> JsResult<Promise> {
        let file = Arc::clone(&self.0);
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
        let cid = Cid::try_from(cid).map_err(|e| Error::new(&format!("Cannot parse cid: {e}")))?;
        Ok(future_to_promise(async move {
            let file: WnfsPublicFile = store
                .get_deserializable(&cid)
                .await
                .map_err(|e| Error::new(&format!("Couldn't deserialize directory: {e}")))?;

            Ok(value!(PublicFile(Arc::new(file))))
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

    /// Gets the metadata of this file.
    pub fn metadata(&self) -> JsResult<JsValue> {
        JsMetadata(self.0.get_metadata()).try_into()
    }

    /// Gets the content cid of the file.
    #[wasm_bindgen(js_name = "contentCid")]
    pub fn content_cid(&self) -> Vec<u8> {
        self.0.get_content_cid().to_bytes()
    }

    /// Converts this directory to a node.
    #[wasm_bindgen(js_name = "asNode")]
    pub fn as_node(&self) -> PublicNode {
        PublicNode(WnfsPublicNode::File(Arc::clone(&self.0)))
    }
}
