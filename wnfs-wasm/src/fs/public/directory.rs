//! The bindgen API for PublicDirectory.

use crate::{
    fs::{
        BlockStore, ForeignBlockStore, JsResult, PublicNode,
        metadata::JsMetadata,
        utils::{self, error},
    },
    value,
};
use chrono::{DateTime, Utc};
use js_sys::{Array, Date, Promise, Uint8Array};
use libipld_core::cid::Cid;
use std::rc::Rc;
use wasm_bindgen::{JsValue, prelude::wasm_bindgen};
use wasm_bindgen_futures::future_to_promise;
use wnfs::{
    common::Storable,
    public::{PublicDirectory as WnfsPublicDirectory, PublicNode as WnfsPublicNode},
    traits::Id,
};

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
            let found_node = directory
                .get_node(&path_segments, &store)
                .await
                .map_err(error("Cannot get node"))?;

            Ok(value!(found_node.cloned().map(PublicNode)))
        }))
    }

    /// Looks up a node by its path name in the current directory.
    #[wasm_bindgen(js_name = "lookupNode")]
    pub fn lookup_node(&self, path_segment: String, store: BlockStore) -> JsResult<Promise> {
        let directory = Rc::clone(&self.0);
        let store = ForeignBlockStore(store);

        Ok(future_to_promise(async move {
            let found_node = directory
                .lookup_node(&path_segment, &store)
                .await
                .map_err(error("Cannot lookup node"))?;

            Ok(value!(found_node.cloned().map(PublicNode)))
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

            let cid_u8array = Uint8Array::from(cid.to_bytes().as_ref());

            Ok(value!(cid_u8array))
        }))
    }

    /// Loads a directory given its CID from the block store.
    pub fn load(cid: Vec<u8>, store: BlockStore) -> JsResult<Promise> {
        let store = ForeignBlockStore(store);
        let cid = Cid::read_bytes(&cid[..]).map_err(error("Cannot parse cid"))?;

        Ok(future_to_promise(async move {
            let directory = WnfsPublicDirectory::load(&cid, &store)
                .await
                .map_err(error("Couldn't deserialize directory"))?;

            Ok(value!(PublicDirectory(Rc::new(directory))))
        }))
    }

    /// Reads specified file content from the directory.
    pub fn read(&self, path_segments: &Array, store: BlockStore) -> JsResult<Promise> {
        let directory = Rc::clone(&self.0);
        let store = ForeignBlockStore(store);
        let path_segments = utils::convert_path_segments(path_segments)?;

        Ok(future_to_promise(async move {
            let result = directory
                .read(&path_segments, &store)
                .await
                .map_err(error("Cannot read from directory"))?;

            let u8array = Uint8Array::from(result.as_ref());

            Ok(value!(u8array))
        }))
    }

    /// Returns names and metadata of the direct children of a directory.
    pub fn ls(&self, path_segments: &Array, store: BlockStore) -> JsResult<Promise> {
        let directory = Rc::clone(&self.0);
        let store = ForeignBlockStore(store);
        let path_segments = utils::convert_path_segments(path_segments)?;

        Ok(future_to_promise(async move {
            let result = directory
                .ls(&path_segments, &store)
                .await
                .map_err(error("Cannot list directory content"))?;

            let result = result
                .iter()
                .flat_map(|(name, metadata)| utils::create_ls_entry(name, metadata))
                .collect::<Array>();

            Ok(value!(result))
        }))
    }

    /// Removes a file or directory from the directory.
    pub fn rm(&self, path_segments: &Array, store: BlockStore) -> JsResult<Promise> {
        let mut directory = Rc::clone(&self.0);
        let store = ForeignBlockStore(store);
        let path_segments = utils::convert_path_segments(path_segments)?;

        Ok(future_to_promise(async move {
            let node = (&mut directory)
                .rm(&path_segments, &store)
                .await
                .map_err(error("Cannot remove from directory"))?;

            Ok(utils::create_public_op_result(directory, PublicNode(node))?)
        }))
    }

    /// Writes a file to the directory.
    pub fn write(
        &self,
        path_segments: &Array,
        content: Vec<u8>,
        time: &Date,
        store: BlockStore,
    ) -> JsResult<Promise> {
        let mut directory = Rc::clone(&self.0);
        let store = ForeignBlockStore(store);

        let time = DateTime::<Utc>::from(time);
        let path_segments = utils::convert_path_segments(path_segments)?;

        Ok(future_to_promise(async move {
            (&mut directory)
                .write(&path_segments, content, time, &store)
                .await
                .map_err(error("Cannot write to directory"))?;

            Ok(utils::create_public_op_result(directory, JsValue::NULL)?)
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
        let mut directory = Rc::clone(&self.0);
        let store = ForeignBlockStore(store);
        let time = DateTime::<Utc>::from(time);
        let path_segments_from = utils::convert_path_segments(path_segments_from)?;
        let path_segments_to = utils::convert_path_segments(path_segments_to)?;

        Ok(future_to_promise(async move {
            (&mut directory)
                .basic_mv(&path_segments_from, &path_segments_to, time, &store)
                .await
                .map_err(error("Cannot move content between directories"))?;

            Ok(utils::create_public_op_result(directory, JsValue::NULL)?)
        }))
    }

    /// Copies a specific node to another location.
    pub fn cp(
        &self,
        path_segments_from: &Array,
        path_segments_to: &Array,
        time: &Date,
        store: BlockStore,
    ) -> JsResult<Promise> {
        let mut directory = Rc::clone(&self.0);
        let store = ForeignBlockStore(store);
        let time = DateTime::<Utc>::from(time);
        let path_segments_from = utils::convert_path_segments(path_segments_from)?;
        let path_segments_to = utils::convert_path_segments(path_segments_to)?;

        Ok(future_to_promise(async move {
            (&mut directory)
                .cp(&path_segments_from, &path_segments_to, time, &store)
                .await
                .map_err(error("Cannot copy content between directories"))?;

            Ok(utils::create_public_op_result(directory, JsValue::NULL)?)
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
        let mut directory = Rc::clone(&self.0);
        let store = ForeignBlockStore(store);
        let time = DateTime::<Utc>::from(time);
        let path_segments = utils::convert_path_segments(path_segments)?;

        Ok(future_to_promise(async move {
            (&mut directory)
                .mkdir(&path_segments, time, &store)
                .await
                .map_err(error("Cannot create directory"))?;

            Ok(utils::create_public_op_result(directory, JsValue::NULL)?)
        }))
    }

    /// Gets the previous CID(s) of the directory.
    /// This will usually be an array of a single CID, but may be
    /// - an empty array, if this is the first revision of a directory
    /// - an array with multiple elements if this is the merge node of
    ///   multiple concurrent changes to the directory.
    #[wasm_bindgen(js_name = "previousCids")]
    pub fn previous_cids(&self) -> Vec<Uint8Array> {
        let cids = self.0.get_previous();
        let arr: Vec<Uint8Array> = cids
            .iter()
            .map(|cid| Uint8Array::from(&cid.to_bytes()[..]))
            .collect();
        arr
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
