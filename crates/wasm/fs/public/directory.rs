//! The bindgen API for PublicDirectory.

use std::rc::Rc;

use chrono::{DateTime, Utc};
use js_sys::{Array, Date, Promise, Uint8Array};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
use wasm_bindgen_futures::future_to_promise;
use wnfs::{
    ipld::Cid,
    public::{
        PublicDirectory as WnfsPublicDirectory, PublicNode as WnfsPublicNode,
        PublicOpResult as WnfsPublicOpResult,
    },
    BlockStore as WnfsBlockStore, Id,
};

use crate::fs::{
    metadata::JsMetadata,
    utils::{self, error},
    BlockStore, ForeignBlockStore, JsResult, PublicNode,
};
use crate::value;

//--------------------------------------------------------------------------------------------------
// Type Definitions
//--------------------------------------------------------------------------------------------------

/// A directory in a WNFS public file system.
#[wasm_bindgen]
pub struct PublicDirectory(pub(crate) Rc<WnfsPublicDirectory>);

//--------------------------------------------------------------------------------------------------
// Implementations
//--------------------------------------------------------------------------------------------------

#[wasm_bindgen]
impl PublicDirectory {
    /// Creates a new directory using the given metadata.
    #[wasm_bindgen(constructor)]
    pub fn new(time: &Date) -> Self {
        let time = DateTime::<Utc>::from(time);
        Self(Rc::new(WnfsPublicDirectory::new(time)))
    }

    /// Follows a path and fetches the node at the end of the path.
    #[wasm_bindgen(js_name = "getNode")]
    pub fn get_node(&self, path_segments: &Array, store: BlockStore) -> JsResult<Promise> {
        let directory = Rc::clone(&self.0);
        let store = ForeignBlockStore(store);
        let path_segments = utils::convert_path_segments(path_segments)?;

        Ok(future_to_promise(async move {
            let WnfsPublicOpResult { root_dir, result } = directory
                .get_node(&path_segments, &store)
                .await
                .map_err(error("Cannot get node"))?;

            Ok(utils::create_public_op_result(
                root_dir,
                result.map(PublicNode),
            )?)
        }))
    }

    /// Looks up a node by its path name in the current directory.
    #[wasm_bindgen(js_name = "lookupNode")]
    pub fn lookup_node(&self, path_segment: &str, store: BlockStore) -> JsResult<Promise> {
        let directory = Rc::clone(&self.0);
        let store = ForeignBlockStore(store);
        let path_segment = path_segment.to_string();

        Ok(future_to_promise(async move {
            let found_node = directory
                .lookup_node(&path_segment, &store)
                .await
                .map_err(error("Cannot lookup node"))?;

            Ok(value!(found_node.map(PublicNode)))
        }))
    }

    /// Stores directory in provided block store.
    pub fn store(&self, store: BlockStore) -> JsResult<Promise> {
        let directory = Rc::clone(&self.0);
        let mut store = ForeignBlockStore(store);

        Ok(future_to_promise(async move {
            let cid = directory
                .store(&mut store)
                .await
                .map_err(error("Cannot add to store"))?;

            let cid_u8array = Uint8Array::from(&cid.to_bytes()[..]);

            Ok(value!(cid_u8array))
        }))
    }

    /// Loads a directory given its CID from the block store.
    pub fn load(cid: Vec<u8>, store: BlockStore) -> JsResult<Promise> {
        let store = ForeignBlockStore(store);
        let cid = Cid::read_bytes(&cid[..]).map_err(error("Cannot parse cid"))?;

        Ok(future_to_promise(async move {
            let directory: WnfsPublicDirectory = store
                .get_deserializable(&cid)
                .await
                .map_err(error("Couldn't deserialize directory"))?;

            Ok(value!(PublicDirectory(Rc::new(directory))))
        }))
    }

    /// Reads specified file content from the directory.
    pub fn read(&self, path_segments: &Array, store: BlockStore) -> JsResult<Promise> {
        let directory = Rc::clone(&self.0);
        let mut store = ForeignBlockStore(store);
        let path_segments = utils::convert_path_segments(path_segments)?;

        Ok(future_to_promise(async move {
            let WnfsPublicOpResult { root_dir, result } = directory
                .read(&path_segments, &mut store)
                .await
                .map_err(error("Cannot read from directory"))?;

            let result = Uint8Array::from(&result.to_bytes()[..]);

            Ok(utils::create_public_op_result(root_dir, result)?)
        }))
    }

    /// Returns names and metadata of the direct children of a directory.
    pub fn ls(&self, path_segments: &Array, store: BlockStore) -> JsResult<Promise> {
        let directory = Rc::clone(&self.0);
        let store = ForeignBlockStore(store);
        let path_segments = utils::convert_path_segments(path_segments)?;

        Ok(future_to_promise(async move {
            let WnfsPublicOpResult { root_dir, result } = directory
                .ls(&path_segments, &store)
                .await
                .map_err(error("Cannot list directory content"))?;

            let result = result
                .iter()
                .flat_map(|(name, metadata)| utils::create_ls_entry(name, metadata))
                .collect::<Array>();

            Ok(utils::create_public_op_result(root_dir, result)?)
        }))
    }

    /// Removes a file or directory from the directory.
    pub fn rm(&self, path_segments: &Array, store: BlockStore) -> JsResult<Promise> {
        let directory = Rc::clone(&self.0);
        let store = ForeignBlockStore(store);
        let path_segments = utils::convert_path_segments(path_segments)?;

        Ok(future_to_promise(async move {
            let WnfsPublicOpResult {
                root_dir,
                result: node,
            } = directory
                .rm(&path_segments, &store)
                .await
                .map_err(error("Cannot remove from directory"))?;

            Ok(utils::create_public_op_result(root_dir, PublicNode(node))?)
        }))
    }

    /// Writes a file to the directory.
    pub fn write(
        &self,
        path_segments: &Array,
        content_cid: Vec<u8>,
        time: &Date,
        store: BlockStore,
    ) -> JsResult<Promise> {
        let directory = Rc::clone(&self.0);
        let store = ForeignBlockStore(store);

        let cid = Cid::try_from(content_cid).map_err(error("Invalid CID"))?;
        let time = DateTime::<Utc>::from(time);
        let path_segments = utils::convert_path_segments(path_segments)?;

        Ok(future_to_promise(async move {
            let WnfsPublicOpResult { root_dir, .. } = directory
                .write(&path_segments, cid, time, &store)
                .await
                .map_err(error("Cannot write to directory"))?;

            Ok(utils::create_public_op_result(root_dir, JsValue::NULL)?)
        }))
    }

    /// Moves a specified path to a new location.
    #[wasm_bindgen(js_name = "basicMv")]
    pub fn basic_mv(
        &self,
        path_segments_from: &Array,
        path_segments_to: &Array,
        time: &Date,
        store: BlockStore,
    ) -> JsResult<Promise> {
        let directory = Rc::clone(&self.0);
        let store = ForeignBlockStore(store);
        let time = DateTime::<Utc>::from(time);
        let path_segments_from = utils::convert_path_segments(path_segments_from)?;
        let path_segments_to = utils::convert_path_segments(path_segments_to)?;

        Ok(future_to_promise(async move {
            let WnfsPublicOpResult { root_dir, .. } = directory
                .basic_mv(&path_segments_from, &path_segments_to, time, &store)
                .await
                .map_err(error("Cannot move content between directories"))?;

            Ok(utils::create_public_op_result(root_dir, JsValue::NULL)?)
        }))
    }

    /// Creates a new directory at the specified path.
    ///
    /// This method acts like `mkdir -p` in Unix because it creates intermediate directories if they do not exist.
    pub fn mkdir(
        &self,
        path_segments: &Array,
        time: &Date,
        store: BlockStore,
    ) -> JsResult<Promise> {
        let directory = Rc::clone(&self.0);
        let store = ForeignBlockStore(store);
        let time = DateTime::<Utc>::from(time);
        let path_segments = utils::convert_path_segments(path_segments)?;

        Ok(future_to_promise(async move {
            let WnfsPublicOpResult { root_dir, .. } = directory
                .mkdir(&path_segments, time, &store)
                .await
                .map_err(error("Cannot create directory"))?;

            Ok(utils::create_public_op_result(root_dir, JsValue::NULL)?)
        }))
    }

    #[wasm_bindgen(js_name = "baseHistoryOn")]
    pub fn base_history_on(&self, base: &PublicDirectory, store: BlockStore) -> JsResult<Promise> {
        let directory = Rc::clone(&self.0);
        let base = base.0.clone();
        let mut store = ForeignBlockStore(store);

        Ok(future_to_promise(async move {
            let WnfsPublicOpResult { root_dir, .. } = directory
                .base_history_on(base, &mut store)
                .await
                .map_err(error("Cannot do history rebase (base_history_on)"))?;

            Ok(utils::create_public_op_result(root_dir, JsValue::NULL)?)
        }))
    }

    /// Gets the previous cid of the directory or null if it doesn't have a previous cid.
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

    /// Gets the metadata of the directory
    pub fn metadata(&self) -> JsResult<JsValue> {
        JsMetadata(self.0.get_metadata()).try_into()
    }

    /// Converts directory to a node.
    #[wasm_bindgen(js_name = "asNode")]
    pub fn as_node(&self) -> PublicNode {
        PublicNode(WnfsPublicNode::Dir(Rc::clone(&self.0)))
    }

    /// Gets a unique id for node.
    #[wasm_bindgen(js_name = "getId")]
    pub fn get_id(&self) -> String {
        self.0.get_id()
    }
}
