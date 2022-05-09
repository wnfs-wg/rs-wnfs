//! The bindgen API for PublicDirectory.

use std::{rc::Rc, str::FromStr};

use chrono::{DateTime, Utc};
use js_sys::{Array, Error, Object, Promise, Reflect};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
use wasm_bindgen_futures::future_to_promise;
use wnfs::{
    public::{
        Id, OpResult as WnfsOpResult, PublicDirectory as WnfsPublicDirectory,
        PublicNode as WnfsPublicNode,
    },
    shared, Cid,
};

use crate::fs::{JsResult, MemoryBlockStore, SharedNode};
use crate::value;

/// A directory in a WNFS public file system.
#[wasm_bindgen]
pub struct PublicDirectory(pub(super) WnfsPublicDirectory);

//--------------------------------------------------------------------------------------------------
// Implementations
//--------------------------------------------------------------------------------------------------

#[wasm_bindgen]
impl PublicDirectory {
    /// Creates a new directory using the given metadata.
    #[wasm_bindgen(constructor)]
    pub fn new(time: &js_sys::Date) -> PublicDirectory {
        let time = DateTime::<Utc>::from(time);
        PublicDirectory(WnfsPublicDirectory::new(time))
    }

    /// Follows a path and fetches the node at the end of the path.
    ///
    /// If path is empty, this returns a cloned directory based on `self`.
    ///
    /// If `diverge` is true, this will clone the spine of the path.
    #[wasm_bindgen(js_name = "getNode")]
    pub fn get_node(
        &mut self,
        path_segments: &Array,
        store: &MemoryBlockStore,
    ) -> JsResult<Promise> {
        let directory = self.0.clone();
        let store = Rc::clone(&store.0);
        let path_segments = utils::convert_path_segments(path_segments)?;

        Ok(future_to_promise(async move {
            let store = store.borrow();

            let WnfsOpResult {
                root_node, result, ..
            } = directory
                .get_node(&path_segments, &*store, false)
                .await
                .map_err(|e| Error::new(&format!("Cannot get node: {e}")))?;

            let op_result = Object::new();
            Reflect::set(
                &op_result,
                &value!("rootNode"),
                &value!(SharedNode(root_node)),
            )?;
            Reflect::set(
                &op_result,
                &value!("result"),
                &value!(result.map(SharedNode)),
            )?;

            Ok(value!(op_result))
        }))
    }

    /// Looks up a node by its path name in the current directory.
    #[wasm_bindgen(js_name = "lookupNode")]
    pub fn lookup_node(
        &mut self,
        path_segment: &str,
        store: &MemoryBlockStore,
    ) -> JsResult<Promise> {
        let directory = self.0.clone();
        let store = Rc::clone(&store.0);
        let path_segment = path_segment.to_string();

        Ok(future_to_promise(async move {
            let store = store.borrow();

            let found_node = directory
                .lookup_node(&path_segment, &*store)
                .await
                .map_err(|e| Error::new(&format!("Cannot lookup node: {e}")))?;

            Ok(value!(found_node.map(SharedNode)))
        }))
    }

    /// Stores a directory as block(s) in provided block store.
    pub fn store(&self, store: &mut MemoryBlockStore) -> JsResult<Promise> {
        let directory = self.0.clone();
        let store = Rc::clone(&store.0);

        Ok(future_to_promise(async move {
            let mut store = store.borrow_mut();

            let cid = directory
                .store(&mut *store)
                .await
                .map_err(|e| Error::new(&format!("Cannot add to store: {e}")))?;

            Ok(value!(cid.to_string()))
        }))
    }

    /// Reads specified file content from the directory.
    pub fn read(&self, path_segments: &Array, store: &MemoryBlockStore) -> JsResult<Promise> {
        let directory = self.0.clone();
        let store = Rc::clone(&store.0);
        let path_segments = utils::convert_path_segments(path_segments)?;

        Ok(future_to_promise(async move {
            let mut store = store.borrow_mut();

            let WnfsOpResult {
                root_node, result, ..
            } = directory
                .read(&path_segments, &mut *store)
                .await
                .map_err(|e| Error::new(&format!("Cannot read from directory: {e}")))?;

            let op_result = Object::new();
            Reflect::set(
                &op_result,
                &value!("rootNode"),
                &value!(SharedNode(root_node)),
            )?;
            Reflect::set(&op_result, &value!("result"), &value!(result.to_string()))?;

            Ok(value!(op_result))
        }))
    }

    /// Returns the name and metadata of the direct children of a directory.
    pub fn ls(&self, path_segments: &Array, store: &MemoryBlockStore) -> JsResult<Promise> {
        let directory = self.0.clone();
        let store = Rc::clone(&store.0);
        let path_segments = utils::convert_path_segments(path_segments)?;

        Ok(future_to_promise(async move {
            let store = store.borrow();

            let WnfsOpResult {
                root_node, result, ..
            } = directory
                .ls(&path_segments, &*store)
                .await
                .map_err(|e| Error::new(&format!("Cannot list directory children: {e}")))?;

            let result = result
                .iter()
                .map(|(name, _)| value!(name))
                .collect::<Array>();

            let op_result = Object::new();
            Reflect::set(
                &op_result,
                &value!("rootNode"),
                &value!(SharedNode(root_node)),
            )?;
            Reflect::set(&op_result, &value!("result"), &value!(result))?;

            Ok(value!(op_result))
        }))
    }

    /// Removes a file or directory from the directory.
    ///
    /// Rather than mutate the directory directly, we create a new directory and return it.
    pub fn rm(&self, path_segments: &Array, store: &MemoryBlockStore) -> JsResult<Promise> {
        let directory = self.0.clone();
        let store = Rc::clone(&store.0);
        let path_segments = utils::convert_path_segments(path_segments)?;

        Ok(future_to_promise(async move {
            let store = store.borrow();

            let WnfsOpResult {
                root_node, result, ..
            } = directory
                .rm(&path_segments, &*store)
                .await
                .map_err(|e| Error::new(&format!("Cannot remove from directory: {e}")))?;

            let op_result = Object::new();
            Reflect::set(
                &op_result,
                &value!("rootNode"),
                &value!(SharedNode(root_node)),
            )?;
            Reflect::set(&op_result, &value!("result"), &value!(SharedNode(result)))?;

            Ok(value!(op_result))
        }))
    }

    /// Writes a file to the directory.
    ///
    /// Rather than mutate the directory directly, we create a new directory and return it.
    pub fn write(
        &mut self,
        path_segments: &Array,
        content_cid: &str,
        time: &js_sys::Date,
        store: &MemoryBlockStore,
    ) -> JsResult<Promise> {
        let directory = self.0.clone();
        let store = Rc::clone(&store.0);
        let cid = Cid::from_str(content_cid).map_err(|_| Error::new("Invalid CID"))?;
        let time = DateTime::<Utc>::from(time);
        let path_segments = utils::convert_path_segments(path_segments)?;

        Ok(future_to_promise(async move {
            let store = store.borrow();

            let WnfsOpResult {
                root_node, result, ..
            } = directory
                .write(&path_segments, cid, time, &*store)
                .await
                .map_err(|e| Error::new(&format!("Cannot write to directory: {e}")))?;

            let op_result = Object::new();
            Reflect::set(
                &op_result,
                &value!("rootNode"),
                &value!(SharedNode(root_node)),
            )?;
            Reflect::set(&op_result, &value!("result"), &value!(SharedNode(result)))?;

            Ok(value!(op_result))
        }))
    }

    /// Creates a new directory at the specified path.
    ///
    /// If path is empty, this returns a cloned directory based on `self`.
    ///
    /// This method acts like `mkdir -p` in Unix because it creates intermediate directories if they do not exist.
    pub fn mkdir(
        &mut self,
        path_segments: &Array,
        time: &js_sys::Date,
        store: &MemoryBlockStore,
    ) -> JsResult<Promise> {
        let directory = self.0.clone();
        let store = Rc::clone(&store.0);
        let time = DateTime::<Utc>::from(time);
        let path_segments = utils::convert_path_segments(path_segments)?;

        Ok(future_to_promise(async move {
            let store = store.borrow();

            let WnfsOpResult {
                root_node, result, ..
            } = directory
                .mkdir(&path_segments, time, &*store, false)
                .await
                .map_err(|e| Error::new(&format!("Cannot create directory: {e}")))?;

            let op_result = Object::new();
            Reflect::set(
                &op_result,
                &value!("rootNode"),
                &value!(SharedNode(root_node)),
            )?;
            Reflect::set(&op_result, &value!("result"), &value!(SharedNode(result)))?;

            Ok(value!(op_result))
        }))
    }

    /// Gets a unique id for node.
    #[wasm_bindgen(js_name = "getId")]
    pub fn get_id(&self) -> String {
        self.0.get_id()
    }

    /// Converts directory to a shared node.
    #[wasm_bindgen(js_name = "toNode")]
    pub fn to_node(&self) -> SharedNode {
        let dir = self.0.clone();
        SharedNode(shared(WnfsPublicNode::Dir(dir)))
    }
}

//--------------------------------------------------------------------------------------------------
// Utilities
//--------------------------------------------------------------------------------------------------

mod utils {
    use js_sys::{Array, Error};
    use wasm_bindgen::JsValue;

    use crate::fs::JsResult;

    pub(crate) fn map_to_rust_vec<T, F: FnMut(JsValue) -> JsResult<T>>(
        array: &Array,
        f: F,
    ) -> JsResult<Vec<T>> {
        array
            .to_vec()
            .into_iter()
            .map(f)
            .collect::<JsResult<Vec<_>>>()
    }

    pub(crate) fn convert_path_segments(path_segments: &Array) -> JsResult<Vec<String>> {
        map_to_rust_vec(path_segments, |v| {
            v.as_string()
                .ok_or_else(|| Error::new("Invalid path segments: Expected an array of strings"))
        })
    }
}
