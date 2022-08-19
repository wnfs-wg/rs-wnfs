//! The bindgen API for PrivateDirectory.

use std::rc::Rc;

use chrono::{DateTime, Utc};
use js_sys::{Array, Date, Promise, Uint8Array};
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::future_to_promise;
use wnfs::{
    private::{
        INumber, PrivateDirectory as WnfsPrivateDirectory, PrivateOpResult as WnfsPrivateOpResult,
    },
    HashOutput, Id,
};

use crate::{
    fs::{
        utils::{self, error},
        BlockStore, ForeignBlockStore, JsResult, Namefilter, PrivateForest, PrivateNode, Rng,
    },
    value,
};

//--------------------------------------------------------------------------------------------------
// Type Definitions
//--------------------------------------------------------------------------------------------------

/// A directory in a WNFS public file system.
#[wasm_bindgen]
pub struct PrivateDirectory(pub(crate) Rc<WnfsPrivateDirectory>);

//--------------------------------------------------------------------------------------------------
// Implementations
//--------------------------------------------------------------------------------------------------

#[wasm_bindgen]
impl PrivateDirectory {
    /// Creates a new directory using the given metadata.
    #[wasm_bindgen(constructor)]
    pub fn new(
        parent_bare_name: Namefilter,
        inumber: Vec<u8>,      // [u8; 32]
        ratchet_seed: Vec<u8>, // [u8; 32]
        time: &Date,
    ) -> JsResult<PrivateDirectory> {
        let inumber: INumber = inumber
            .try_into()
            .map_err(error("Cannot convert inumber"))?;

        let ratchet_seed: HashOutput = ratchet_seed
            .try_into()
            .map_err(error("Cannot convert ratchet seed"))?;

        let time = DateTime::<Utc>::from(time);

        Ok(Self(Rc::new(WnfsPrivateDirectory::new(
            parent_bare_name.0,
            inumber,
            ratchet_seed,
            time,
        ))))
    }

    /// Follows a path and fetches the node at the end of the path.
    #[wasm_bindgen(js_name = "getNode")]
    pub fn get_node(
        &self,
        path_segments: &Array,
        hamt: PrivateForest,
        store: BlockStore,
    ) -> JsResult<Promise> {
        let directory = Rc::clone(&self.0);
        let store = ForeignBlockStore(store);
        let path_segments = utils::convert_path_segments(path_segments)?;

        Ok(future_to_promise(async move {
            let WnfsPrivateOpResult {
                root_dir,
                hamt,
                result,
            } = directory
                .get_node(&path_segments, hamt.0, &store)
                .await
                .map_err(error("Cannot get node"))?;

            Ok(utils::create_private_op_result(
                root_dir,
                hamt,
                result.map(PrivateNode),
            )?)
        }))
    }

    /// Looks up a node by its path name in the current directory.
    #[wasm_bindgen(js_name = "lookupNode")]
    pub fn lookup_node(
        &self,
        path_segment: &str,
        hamt: PrivateForest,
        store: BlockStore,
    ) -> JsResult<Promise> {
        let directory = Rc::clone(&self.0);
        let store = ForeignBlockStore(store);
        let path_segment = path_segment.to_string();

        Ok(future_to_promise(async move {
            let found_node = directory
                .lookup_node(&path_segment, &hamt.0, &store)
                .await
                .map_err(error("Cannot lookup node"))?;

            Ok(value!(found_node.map(PrivateNode)))
        }))
    }

    /// Reads specified file content from the directory.
    pub fn read(
        &self,
        path_segments: &Array,
        hamt: PrivateForest,
        store: BlockStore,
    ) -> JsResult<Promise> {
        let directory = Rc::clone(&self.0);
        let store = ForeignBlockStore(store);
        let path_segments = utils::convert_path_segments(path_segments)?;

        Ok(future_to_promise(async move {
            let WnfsPrivateOpResult {
                root_dir,
                hamt,
                result,
            } = directory
                .read(&path_segments, hamt.0, &store)
                .await
                .map_err(error("Cannot read from directory"))?;

            Ok(utils::create_private_op_result(
                root_dir,
                hamt,
                Uint8Array::from(&result[..]),
            )?)
        }))
    }

    /// Returns names and metadata of the direct children of a directory.
    pub fn ls(
        &self,
        path_segments: &Array,
        hamt: PrivateForest,
        store: BlockStore,
    ) -> JsResult<Promise> {
        let directory = Rc::clone(&self.0);
        let store = ForeignBlockStore(store);
        let path_segments = utils::convert_path_segments(path_segments)?;

        Ok(future_to_promise(async move {
            let WnfsPrivateOpResult {
                root_dir,
                hamt,
                result,
            } = directory
                .ls(&path_segments, hamt.0, &store)
                .await
                .map_err(error("Cannot list directory children"))?;

            let result = result
                .iter()
                .flat_map(|(name, metadata)| utils::create_ls_entry(name, metadata))
                .collect::<Array>();

            Ok(utils::create_private_op_result(root_dir, hamt, result)?)
        }))
    }

    /// Removes a file or directory from the directory.
    pub fn rm(
        &self,
        path_segments: &Array,
        hamt: PrivateForest,
        store: BlockStore,
        rng: Rng,
    ) -> JsResult<Promise> {
        let directory = Rc::clone(&self.0);
        let mut store = ForeignBlockStore(store);
        let path_segments = utils::convert_path_segments(path_segments)?;

        Ok(future_to_promise(async move {
            let WnfsPrivateOpResult {
                root_dir,
                hamt,
                result: node,
            } = directory
                .rm(&path_segments, hamt.0, &mut store, &rng)
                .await
                .map_err(error("Cannot remove from directory"))?;

            Ok(utils::create_private_op_result(
                root_dir,
                hamt,
                PrivateNode(node),
            )?)
        }))
    }

    /// Writes a file to the directory.
    pub fn write(
        &self,
        path_segments: &Array,
        content: Vec<u8>,
        time: &Date,
        hamt: PrivateForest,
        store: BlockStore,
        rng: Rng,
    ) -> JsResult<Promise> {
        let directory = Rc::clone(&self.0);
        let mut store = ForeignBlockStore(store);
        let time = DateTime::<Utc>::from(time);
        let path_segments = utils::convert_path_segments(path_segments)?;

        Ok(future_to_promise(async move {
            let WnfsPrivateOpResult { root_dir, hamt, .. } = directory
                .write(&path_segments, time, content, hamt.0, &mut store, &rng)
                .await
                .map_err(error("Cannot write to directory"))?;

            Ok(utils::create_private_op_result(
                root_dir,
                hamt,
                JsValue::NULL,
            )?)
        }))
    }

    /// Creates a new directory at the specified path.
    ///
    /// This method acts like `mkdir -p` in Unix because it creates intermediate directories if they do not exist.
    pub fn mkdir(
        &self,
        path_segments: &Array,
        time: &Date,
        hamt: PrivateForest,
        store: BlockStore,
        rng: Rng,
    ) -> JsResult<Promise> {
        let directory = Rc::clone(&self.0);
        let mut store = ForeignBlockStore(store);
        let time = DateTime::<Utc>::from(time);
        let path_segments = utils::convert_path_segments(path_segments)?;

        Ok(future_to_promise(async move {
            let WnfsPrivateOpResult { root_dir, hamt, .. } = directory
                .mkdir(&path_segments, time, hamt.0, &mut store, &rng)
                .await
                .map_err(error("Cannot create directory: {e}"))?;

            Ok(utils::create_private_op_result(
                root_dir,
                hamt,
                JsValue::NULL,
            )?)
        }))
    }

    /// Gets a unique id for node.
    #[wasm_bindgen(js_name = "getId")]
    pub fn get_id(&self) -> String {
        self.0.get_id()
    }
}
