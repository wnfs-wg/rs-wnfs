//! The bindgen API for PrivateDirectory.

use std::rc::Rc;

use chrono::{DateTime, Utc};
use js_sys::{Array, Date, Promise, Uint8Array};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
use wasm_bindgen_futures::future_to_promise;
use wnfs::{
    Id, PrivateDirectory as WnfsPrivateDirectory, PrivateOpResult as WnfsPrivateOpResult,
    HASH_BYTE_SIZE,
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
    /// Creates a new private directory.
    #[wasm_bindgen(constructor)]
    pub fn new(
        parent_bare_name: Namefilter,
        time: &Date,
        mut rng: Rng,
    ) -> JsResult<PrivateDirectory> {
        let time = DateTime::<Utc>::from(time);

        Ok(Self(Rc::new(WnfsPrivateDirectory::new(
            parent_bare_name.0,
            time,
            &mut rng,
        ))))
    }

    /// Creates a new directory with the ratchet seed and inumber provided.
    #[wasm_bindgen(js_name = "withSeed")]
    pub fn with_seed(
        parent_bare_name: Namefilter,
        time: &Date,
        ratchet_seed: Vec<u8>,
        inumber: Vec<u8>,
    ) -> JsResult<PrivateDirectory> {
        let time = DateTime::<Utc>::from(time);
        let ratchet_seed = utils::expect_bytes::<HASH_BYTE_SIZE>(ratchet_seed)?;
        let inumber = utils::expect_bytes::<HASH_BYTE_SIZE>(inumber)?;

        Ok(Self(Rc::new(WnfsPrivateDirectory::with_seed(
            parent_bare_name.0,
            time,
            ratchet_seed,
            inumber,
        ))))
    }

    /// Follows a path and fetches the node at the end of the path.
    #[wasm_bindgen(js_name = "getNode")]
    pub fn get_node(
        &self,
        path_segments: &Array,
        search_latest: bool,
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
                .get_node(&path_segments, search_latest, hamt.0, &store)
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
        search_latest: bool,
        hamt: PrivateForest,
        store: BlockStore,
    ) -> JsResult<Promise> {
        let directory = Rc::clone(&self.0);
        let store = ForeignBlockStore(store);
        let path_segment = path_segment.to_string();

        Ok(future_to_promise(async move {
            let found_node = directory
                .lookup_node(&path_segment, search_latest, &hamt.0, &store)
                .await
                .map_err(error("Cannot lookup node"))?;

            Ok(value!(found_node.map(PrivateNode)))
        }))
    }

    /// Reads specified file content from the directory.
    pub fn read(
        &self,
        path_segments: &Array,
        search_latest: bool,
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
                .read(&path_segments, search_latest, hamt.0, &store)
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
        search_latest: bool,
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
                .ls(&path_segments, search_latest, hamt.0, &store)
                .await
                .map_err(error("Cannot list directory content"))?;

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
        search_latest: bool,
        hamt: PrivateForest,
        store: BlockStore,
        mut rng: Rng,
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
                .rm(&path_segments, search_latest, hamt.0, &mut store, &mut rng)
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
    #[allow(clippy::too_many_arguments)]
    pub fn write(
        &self,
        path_segments: &Array,
        search_latest: bool,
        content: Vec<u8>,
        time: &Date,
        hamt: PrivateForest,
        store: BlockStore,
        mut rng: Rng,
    ) -> JsResult<Promise> {
        let directory = Rc::clone(&self.0);
        let mut store = ForeignBlockStore(store);
        let time = DateTime::<Utc>::from(time);
        let path_segments = utils::convert_path_segments(path_segments)?;

        Ok(future_to_promise(async move {
            let WnfsPrivateOpResult { root_dir, hamt, .. } = directory
                .write(
                    &path_segments,
                    search_latest,
                    time,
                    content,
                    hamt.0,
                    &mut store,
                    &mut rng,
                )
                .await
                .map_err(error("Cannot write to directory"))?;

            Ok(utils::create_private_op_result(
                root_dir,
                hamt,
                JsValue::NULL,
            )?)
        }))
    }

    /// Moves a specified path to a new location.
    #[wasm_bindgen(js_name = "basicMv")]
    #[allow(clippy::too_many_arguments)]
    pub fn basic_mv(
        &self,
        path_segments_from: &Array,
        path_segments_to: &Array,
        search_latest: bool,
        time: &Date,
        hamt: PrivateForest,
        store: BlockStore,
        mut rng: Rng,
    ) -> JsResult<Promise> {
        let directory = Rc::clone(&self.0);
        let mut store = ForeignBlockStore(store);
        let time = DateTime::<Utc>::from(time);
        let path_segments_from = utils::convert_path_segments(path_segments_from)?;
        let path_segments_to = utils::convert_path_segments(path_segments_to)?;

        Ok(future_to_promise(async move {
            let WnfsPrivateOpResult { root_dir, hamt, .. } = directory
                .basic_mv(
                    &path_segments_from,
                    &path_segments_to,
                    search_latest,
                    time,
                    hamt.0,
                    &mut store,
                    &mut rng,
                )
                .await
                .map_err(error("Cannot move content between directories"))?;

            Ok(utils::create_private_op_result(
                root_dir,
                hamt,
                JsValue::NULL,
            )?)
        }))
    }

    /// Copies a specified path to a new location.
    #[allow(clippy::too_many_arguments)]
    pub fn cp(
        &self,
        path_segments_from: &Array,
        path_segments_to: &Array,
        search_latest: bool,
        time: &Date,
        hamt: PrivateForest,
        store: BlockStore,
        mut rng: Rng,
    ) -> JsResult<Promise> {
        let directory = Rc::clone(&self.0);
        let mut store = ForeignBlockStore(store);
        let time = DateTime::<Utc>::from(time);
        let path_segments_from = utils::convert_path_segments(path_segments_from)?;
        let path_segments_to = utils::convert_path_segments(path_segments_to)?;

        Ok(future_to_promise(async move {
            let WnfsPrivateOpResult { root_dir, hamt, .. } = directory
                .cp(
                    &path_segments_from,
                    &path_segments_to,
                    search_latest,
                    time,
                    hamt.0,
                    &mut store,
                    &mut rng,
                )
                .await
                .map_err(error("Cannot copy content between directories"))?;

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
        search_latest: bool,
        time: &Date,
        hamt: PrivateForest,
        store: BlockStore,
        mut rng: Rng,
    ) -> JsResult<Promise> {
        let directory = Rc::clone(&self.0);
        let mut store = ForeignBlockStore(store);
        let time = DateTime::<Utc>::from(time);
        let path_segments = utils::convert_path_segments(path_segments)?;

        Ok(future_to_promise(async move {
            let WnfsPrivateOpResult { root_dir, hamt, .. } = directory
                .mkdir(
                    &path_segments,
                    search_latest,
                    time,
                    hamt.0,
                    &mut store,
                    &mut rng,
                )
                .await
                .map_err(error("Cannot create directory"))?;

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
