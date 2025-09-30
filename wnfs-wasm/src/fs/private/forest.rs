use super::{Name, Rng};
use crate::{
    fs::{BlockStore, ForeignBlockStore, ForestChange, JsResult, utils, utils::error},
    value,
};
use js_sys::{Array, Promise, Uint8Array};
use libipld_core::cid::Cid;
use std::rc::Rc;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen_futures::future_to_promise;
use wnfs::{
    common::Storable,
    private::forest::{
        hamt::HamtForest as WnfsHamtForest, traits::PrivateForest as WnfsPrivateForest,
    },
};
use wnfs_nameaccumulator::AccumulatorSetup;

//--------------------------------------------------------------------------------------------------
// Type Definitions
//--------------------------------------------------------------------------------------------------

/// A reference to a private forest. Used for the private file system.
#[wasm_bindgen]
pub struct PrivateForest(pub(crate) Rc<WnfsHamtForest>);

//--------------------------------------------------------------------------------------------------
// Implementations
//--------------------------------------------------------------------------------------------------

#[wasm_bindgen]
impl PrivateForest {
    /// Creates a new private forest.
    #[wasm_bindgen(constructor)]
    pub fn new(mut rng: Rng, rsa_modulus_big_endian: Option<Vec<u8>>) -> JsResult<PrivateForest> {
        match rsa_modulus_big_endian {
            Some(rsa_modulus_big_endian) => {
                let modulus_big_endian = utils::expect_bytes::<256>(rsa_modulus_big_endian)?;
                let setup = AccumulatorSetup::with_modulus(&modulus_big_endian, &mut rng);
                Ok(Self(Rc::new(WnfsHamtForest::new(setup))))
            }
            None => Ok(Self(Rc::new(WnfsHamtForest::new_rsa_2048(&mut rng)))),
        }
    }

    /// Loads an existing private forest from a given CID
    /// You need to have previously `.store()`ed it to get its CID.
    pub fn load(cid: Vec<u8>, store: BlockStore) -> JsResult<Promise> {
        let store = ForeignBlockStore(store);
        let cid = Cid::read_bytes(&cid[..]).map_err(error("Cannot parse cid"))?;

        Ok(future_to_promise(async move {
            let forest = WnfsHamtForest::load(&cid, &store)
                .await
                .map_err(error("Couldn't deserialize forest"))?;

            Ok(value!(PrivateForest(Rc::new(forest))))
        }))
    }

    /// Stores this private forest in provided block store.
    /// Returns the CID from which it can be `.load()`ed again.
    pub fn store(&self, store: BlockStore) -> JsResult<Promise> {
        let forest = Rc::clone(&self.0);
        let store = ForeignBlockStore(store);

        Ok(future_to_promise(async move {
            let cid = forest
                .store(&store)
                .await
                .map_err(error("Cannot add to store"))?;

            let cid_u8array = Uint8Array::from(&cid.to_bytes()[..]);

            Ok(value!(cid_u8array))
        }))
    }

    #[wasm_bindgen]
    pub fn merge(&self, other: &PrivateForest, store: BlockStore) -> JsResult<Promise> {
        let mut store = ForeignBlockStore(store);
        let main = Rc::clone(&self.0);
        let other = Rc::clone(&other.0);

        Ok(future_to_promise(async move {
            let merged = main
                .merge(&other, &mut store)
                .await
                .map_err(error("Error in private forest 'merge'"))?;

            Ok(value!(PrivateForest(merged.into())))
        }))
    }

    #[wasm_bindgen]
    pub fn diff(&self, other: &PrivateForest, store: BlockStore) -> JsResult<Promise> {
        let mut store = ForeignBlockStore(store);
        let main = Rc::clone(&self.0);
        let other = Rc::clone(&other.0);

        Ok(future_to_promise(async move {
            let diff = main
                .diff(&other, &mut store)
                .await
                .map_err(error("Error in private forest 'merge'"))?;

            Ok(value!(
                diff.into_iter()
                    .map(|c| value!(ForestChange(c.map(&|c| c.0))))
                    .collect::<Array>()
            ))
        }))
    }

    #[wasm_bindgen(js_name = "emptyName")]
    pub fn empty_name(&self) -> Name {
        Name(self.0.empty_name())
    }
}
