use super::{Name, NameAccumulator};
use crate::{
    fs::{
        private::{PrivateDirectory, PrivateFile, PrivateForest, PrivateRef},
        utils::{self, error},
        BlockStore, ForeignBlockStore, JsResult, Rng,
    },
    value,
};
use js_sys::{Error, Promise, Uint8Array};
use std::{collections::BTreeSet, rc::Rc};
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen_futures::future_to_promise;
use wnfs::{
    hamt::{ChangeType, KeyValueChange},
    libipld::Cid,
    private::PrivateNode as WnfsPrivateNode,
    traits::Id,
};
use wnfs_nameaccumulator::NameAccumulator as WnfsNameAccumulator;

//--------------------------------------------------------------------------------------------------
// Type Definitions
//--------------------------------------------------------------------------------------------------

/// Wraps `wnfs::PrivateNode`.
#[wasm_bindgen]
pub struct PrivateNode(pub(crate) WnfsPrivateNode);

#[wasm_bindgen]
pub struct ForestChange(pub(crate) KeyValueChange<WnfsNameAccumulator, BTreeSet<Cid>>);

//--------------------------------------------------------------------------------------------------
// Implementations
//--------------------------------------------------------------------------------------------------

#[wasm_bindgen]
impl PrivateNode {
    /// Persists the current state of this node in the BlockStore and PrivateForest.
    /// This will also force a history entry to be created, if there were changes.
    pub fn store(
        &self,
        forest: &PrivateForest,
        store: BlockStore,
        mut rng: Rng,
    ) -> JsResult<Promise> {
        let node = self.0.clone(); // cheap clone
        let mut store = ForeignBlockStore(store);
        let mut forest = Rc::clone(&forest.0);

        Ok(future_to_promise(async move {
            let private_ref = node
                .store(&mut forest, &mut store, &mut rng)
                .await
                .map_err(error("Cannot store node"))?;

            Ok(utils::create_private_forest_result(
                value!(PrivateRef::from(private_ref)),
                forest,
            )?)
        }))
    }

    /// Loads a node from the PrivateForest using the PrivateRef.
    pub fn load(
        private_ref: PrivateRef,
        forest: &PrivateForest,
        store: BlockStore,
        parent_name: Option<Name>,
    ) -> JsResult<Promise> {
        let store = ForeignBlockStore(store);
        let forest = Rc::clone(&forest.0);
        let private_ref = private_ref.try_into()?;
        let parent_name = parent_name.map(|name| name.0.clone());

        Ok(future_to_promise(async move {
            let node = WnfsPrivateNode::load(&private_ref, &forest, &store, parent_name)
                .await
                .map_err(error("Cannot load node"))?;

            Ok(value!(PrivateNode(node)))
        }))
    }

    #[wasm_bindgen(js_name = "asDir")]
    pub fn as_dir(&self) -> JsResult<PrivateDirectory> {
        let dir = self
            .0
            .as_dir()
            .map_err(|e| Error::new(&format!("Cannot cast to a directory: {e}")))?;

        Ok(PrivateDirectory(dir))
    }

    #[wasm_bindgen(js_name = "asFile")]
    pub fn as_file(&self) -> JsResult<PrivateFile> {
        let file = self
            .0
            .as_file()
            .map_err(|e| Error::new(&format!("Cannot cast to a file: {e}")))?;

        Ok(PrivateFile(file))
    }

    #[wasm_bindgen(js_name = "isDir")]
    pub fn is_dir(&self) -> bool {
        self.0.is_dir()
    }

    #[wasm_bindgen(js_name = "isFile")]
    pub fn is_file(&self) -> bool {
        self.0.is_file()
    }

    #[wasm_bindgen(js_name = "getId")]
    pub fn get_id(&self) -> String {
        self.0.get_id()
    }
}

#[wasm_bindgen]
impl ForestChange {
    #[wasm_bindgen(js_name = "getChangeType")]
    pub fn get_change_type(&self) -> String {
        match self.0.r#type {
            ChangeType::Add => "add",
            ChangeType::Remove => "remove",
            ChangeType::Modify => "modify",
        }
        .into()
    }

    #[wasm_bindgen(js_name = "getKey")]
    pub fn get_key(&self) -> NameAccumulator {
        NameAccumulator(self.0.key.clone())
    }

    #[wasm_bindgen(js_name = "getValue1")]
    pub fn get_value1(&self) -> Vec<Uint8Array> {
        self.0
            .value1
            .as_ref()
            .map_or_else(Vec::<Uint8Array>::new, |b| {
                b.iter()
                    .map(|cid| Uint8Array::from(&cid.to_bytes()[..]))
                    .collect()
            })
    }

    #[wasm_bindgen(js_name = "getValue2")]
    pub fn get_value2(&self) -> Vec<Uint8Array> {
        self.0
            .value2
            .as_ref()
            .map_or_else(Vec::<Uint8Array>::new, |b| {
                b.iter()
                    .map(|cid| Uint8Array::from(&cid.to_bytes()[..]))
                    .collect()
            })
    }
}
