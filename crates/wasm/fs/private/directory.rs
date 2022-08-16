//! The bindgen API for PrivateDirectory.

use std::rc::Rc;

use chrono::{DateTime, Utc};
use js_sys::{Array, Date, Promise, Uint8Array};
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::future_to_promise;
use wnfs::{
    private::{
        namefilter::Namefilter, HamtStore, INumber, PrivateDirectory as WnfsPrivateDirectory,
        PrivateOpResult as WnfsPrivateOpResult,
    },
    HashOutput, Id,
};

use crate::{
    fs::{
        utils::{self, error},
        BlockStore, ExternRng, ForeignBlockStore, JsResult, PrivateNode,
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
        parent_bare_name: Vec<u8>, // [u8; 256]
        inumber: Vec<u8>,          // [u8; 32]
        ratchet_seed: Vec<u8>,     // [u8; 32]
        time: &Date,
    ) -> JsResult<PrivateDirectory> {
        let parent_bare_name = Namefilter::try_from(parent_bare_name)
            .map_err(error("Cannot convert parent bare name"))?;
        let inumber: INumber = inumber
            .try_into()
            .map_err(error("Cannot convert inumber"))?;
        let ratchet_seed: HashOutput = ratchet_seed
            .try_into()
            .map_err(error("Cannot convert ratchet seed"))?;

        let time = DateTime::<Utc>::from(time);

        Ok(Self(Rc::new(WnfsPrivateDirectory::new(
            parent_bare_name,
            inumber,
            ratchet_seed,
            time,
        ))))
    }

    /// Follows a path and fetches the node at the end of the path.
    #[wasm_bindgen(js_name = "getNode")]
    pub fn get_node(&self, path_segments: &Array, store: BlockStore) -> JsResult<Promise> {
        let directory = Rc::clone(&self.0);
        let mut store = ForeignBlockStore(store);
        let path_segments = utils::convert_path_segments(path_segments)?;

        Ok(future_to_promise(async move {
            let hamt = HamtStore::<_, ExternRng>::new(&mut store);
            let WnfsPrivateOpResult { root_dir, result } = directory
                .get_node(&path_segments, &hamt)
                .await
                .map_err(error("Cannot get node"))?;

            Ok(utils::create_op_result(
                PrivateDirectory(root_dir),
                result.map(PrivateNode),
            )?)
        }))
    }

    /// Looks up a node by its path name in the current directory.
    #[wasm_bindgen(js_name = "lookupNode")]
    pub fn lookup_node(&self, path_segment: &str, store: BlockStore) -> JsResult<Promise> {
        let directory = Rc::clone(&self.0);
        let mut store = ForeignBlockStore(store);
        let path_segment = path_segment.to_string();

        Ok(future_to_promise(async move {
            let hamt = HamtStore::<_, ExternRng>::new(&mut store);
            let found_node = directory
                .lookup_node(&path_segment, &hamt)
                .await
                .map_err(error("Cannot lookup node"))?;

            Ok(value!(found_node.map(PrivateNode)))
        }))
    }

    /// Reads specified file content from the directory.
    pub fn read(&self, path_segments: &Array, store: BlockStore) -> JsResult<Promise> {
        let directory = Rc::clone(&self.0);
        let mut store = ForeignBlockStore(store);
        let path_segments = utils::convert_path_segments(path_segments)?;

        Ok(future_to_promise(async move {
            let hamt = HamtStore::<_, ExternRng>::new(&mut store);
            let WnfsPrivateOpResult { root_dir, result } = directory
                .read(&path_segments, &hamt)
                .await
                .map_err(error("Cannot read from directory"))?;

            Ok(utils::create_op_result(
                PrivateDirectory(root_dir),
                Uint8Array::from(&result[..]),
            )?)
        }))
    }

    /// Removes a file or directory from the directory.
    pub fn rm(&self, path_segments: &Array, store: BlockStore) -> JsResult<Promise> {
        let directory = Rc::clone(&self.0);
        let mut store = ForeignBlockStore(store);
        let path_segments = utils::convert_path_segments(path_segments)?;

        Ok(future_to_promise(async move {
            let mut hamt = HamtStore::<_, ExternRng>::new(&mut store);
            let WnfsPrivateOpResult {
                root_dir,
                result: node,
            } = directory
                .rm(&path_segments, &mut hamt)
                .await
                .map_err(error("Cannot remove from directory"))?;

            Ok(utils::create_op_result(
                PrivateDirectory(root_dir),
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
        store: BlockStore,
    ) -> JsResult<Promise> {
        let directory = Rc::clone(&self.0);
        let mut store = ForeignBlockStore(store);
        let time = DateTime::<Utc>::from(time);
        let path_segments = utils::convert_path_segments(path_segments)?;

        Ok(future_to_promise(async move {
            let mut hamt = HamtStore::<_, ExternRng>::new(&mut store);
            let WnfsPrivateOpResult { root_dir, .. } = directory
                .write(&path_segments, time, content, &mut hamt)
                .await
                .map_err(error("Cannot write to directory"))?;

            Ok(utils::create_op_result(
                PrivateDirectory(root_dir),
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
        store: BlockStore,
    ) -> JsResult<Promise> {
        let directory = self.0.clone();
        let mut store = ForeignBlockStore(store);
        let time = DateTime::<Utc>::from(time);
        let path_segments = utils::convert_path_segments(path_segments)?;

        Ok(future_to_promise(async move {
            let mut hamt = HamtStore::<_, ExternRng>::new(&mut store);
            let WnfsPrivateOpResult { root_dir, .. } = directory
                .mkdir(&path_segments, time, &mut hamt)
                .await
                .map_err(error("Cannot create directory: {e}"))?;

            Ok(utils::create_op_result(
                PrivateDirectory(root_dir),
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
