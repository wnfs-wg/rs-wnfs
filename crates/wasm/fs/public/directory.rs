//! The bindgen API for PublicDirectory.

use std::{rc::Rc, str::FromStr};

use chrono::{DateTime, Utc};
use js_sys::{Array, Error, Object, Promise, Reflect};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
use wasm_bindgen_futures::future_to_promise;
use wnfs::{
    public::{
        OpResult as WnfsOpResult, PublicDirectory as WnfsPublicDirectory,
        PublicNode as WnfsPublicNode,
    },
    shared, Cid, Shared,
};

use crate::fs::{JsResult, MemoryBlockStore};
use crate::value;

/// A directory in a WNFS public file system.
#[wasm_bindgen]
pub struct PublicDirectory(Shared<WnfsPublicDirectory>);

/// Wraps a shared<PublicNode>
#[wasm_bindgen]
pub struct SharedNode(Shared<WnfsPublicNode>);

//--------------------------------------------------------------------------------------------------
// Implementations
//--------------------------------------------------------------------------------------------------

#[wasm_bindgen]
impl PublicDirectory {
    /// Creates a new directory using the given metadata.
    #[wasm_bindgen(constructor)]
    pub fn new(time: &js_sys::Date) -> PublicDirectory {
        let time = DateTime::<Utc>::from(time);
        PublicDirectory(shared(WnfsPublicDirectory::new(time)))
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
        diverge: bool,
    ) -> JsResult<Promise> {
        let directory = Rc::clone(&self.0);
        let store = Rc::clone(&store.0);
        let path_segments = utils::convert_path_segments(path_segments)?;

        Ok(future_to_promise(async move {
            let directory = directory.borrow();
            let store = store.borrow();

            let WnfsOpResult {
                root_node, result, ..
            } = directory
                .get_node(&path_segments, &*store, diverge)
                .await
                .map_err(|e| Error::new(&format!("Cannot create directory: {e}")))?;

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
        let directory = Rc::clone(&self.0);
        let store = Rc::clone(&store.0);
        let path_segment = path_segment.to_string();

        Ok(future_to_promise(async move {
            let directory = directory.borrow();
            let store = store.borrow();

            let found_node = directory
                .lookup_node(&path_segment, &*store)
                .await
                .map_err(|e| Error::new(&format!("Cannot create directory: {e}")))?;

            Ok(value!(found_node.map(SharedNode)))
        }))
    }

    /// Stores a directory as block(s) in provided block store.
    pub fn store(&self, store: &mut MemoryBlockStore) -> JsResult<Promise> {
        let directory = Rc::clone(&self.0);
        let store = Rc::clone(&store.0);

        Ok(future_to_promise(async move {
            let directory = directory.borrow();
            let mut store = store.borrow_mut();

            let cid = directory
                .store(&mut *store)
                .await
                .map_err(|e| Error::new(&format!("Cannot create directory: {e}")))?;

            Ok(value!(cid.to_string()))
        }))
    }

    /// Reads specified file content from the directory.
    pub fn read(&self, path_segments: &Array, store: &MemoryBlockStore) -> JsResult<Promise> {
        let directory = Rc::clone(&self.0);
        let store = Rc::clone(&store.0);
        let path_segments = utils::convert_path_segments(path_segments)?;

        Ok(future_to_promise(async move {
            let directory = directory.borrow();
            let mut store = store.borrow_mut();

            let WnfsOpResult {
                root_node, result, ..
            } = directory
                .read(&path_segments, &mut *store)
                .await
                .map_err(|e| Error::new(&format!("Cannot create directory: {e}")))?;

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
        let directory = Rc::clone(&self.0);
        let store = Rc::clone(&store.0);
        let path_segments = utils::convert_path_segments(path_segments)?;

        Ok(future_to_promise(async move {
            let directory = directory.borrow();
            let store = store.borrow();

            let WnfsOpResult {
                root_node, result, ..
            } = directory
                .ls(&path_segments, &*store)
                .await
                .map_err(|e| Error::new(&format!("Cannot create directory: {e}")))?;

            let result = result
                .iter()
                .map(|(name, _)| value!(name))
                .collect::<Array>(); //  result.iter().map(|x| x);

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
        let directory = Rc::clone(&self.0);
        let store = Rc::clone(&store.0);
        let path_segments = utils::convert_path_segments(path_segments)?;

        Ok(future_to_promise(async move {
            let directory = directory.borrow();
            let store = store.borrow();

            let WnfsOpResult {
                root_node, result, ..
            } = directory
                .rm(&path_segments, &*store)
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
        let directory = Rc::clone(&self.0);
        let store = Rc::clone(&store.0);
        let cid = Cid::from_str(content_cid).map_err(|_| Error::new("Invalid CID"))?;
        let time = DateTime::<Utc>::from(time);
        let path_segments = utils::convert_path_segments(path_segments)?;

        Ok(future_to_promise(async move {
            let directory = directory.borrow();
            let store = store.borrow();

            let WnfsOpResult {
                root_node, result, ..
            } = directory
                .write(&path_segments, cid, time, &*store)
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
        let directory = Rc::clone(&self.0);
        let store = Rc::clone(&store.0);
        let time = DateTime::<Utc>::from(time);
        let path_segments = utils::convert_path_segments(path_segments)?;

        Ok(future_to_promise(async move {
            let directory = directory.borrow();
            let store = store.borrow();

            let WnfsOpResult {
                root_node, result, ..
            } = directory
                .mkdir(&path_segments, time, &*store)
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

    /// Converts directory to a shared node.
    /// TODO(appcypher): This will be unnecessary once we support `Shared<PublicDirectory>`.
    #[wasm_bindgen(js_name = "toNode")]
    pub fn to_node(&self) -> SharedNode {
        let dir = &*self.0.borrow();
        SharedNode(shared(WnfsPublicNode::Dir(dir.clone())))
    }
}

#[wasm_bindgen]
impl SharedNode {
    #[wasm_bindgen(js_name = "asDir")]
    pub fn as_dir(&self) -> PublicDirectory {
        let node = self.0.borrow();
        let dir = node.as_dir();
        // TODO(appcypher): The clone here is not ideal but it means WnfsPublicNode::Dir might have to hold `Shared<PublicDirectory>` instead which will complicate the structure.
        PublicDirectory(shared(dir.clone()))
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

//--------------------------------------------------------------------------------------------------
// Tests
//--------------------------------------------------------------------------------------------------

#[cfg(test)]
mod public_directory_tests {
    use super::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn it_can_create_directory() {
        let time = &js_sys::Date::new_0();

        PublicDirectory::new(time);
    }
}
