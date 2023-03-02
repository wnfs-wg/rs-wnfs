use super::{ForeignExchangeKey, ForeignPrivateKey, Namefilter, PrivateForest, PrivateNode};
use crate::{
    fs::{utils::error, BlockStore, ForeignBlockStore, JsResult, PrivateKey, Rng},
    value,
};
use js_sys::{Array, Promise, Reflect};
use std::rc::Rc;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen_futures::future_to_promise;
use wnfs::{
    libipld::Cid,
    private::{recipient, sharer, SharePayload as WnfsSharePayload},
    public::PublicLink,
};

//--------------------------------------------------------------------------------------------------
// Type Definitions
//--------------------------------------------------------------------------------------------------

#[wasm_bindgen]
pub struct SharePayload(pub(crate) WnfsSharePayload);

//--------------------------------------------------------------------------------------------------
// Implementations
//--------------------------------------------------------------------------------------------------

#[wasm_bindgen]
impl SharePayload {
    #[wasm_bindgen(js_name = "fromNode")]
    pub fn from_node(
        node: PrivateNode,
        temporal: bool,
        forest: &PrivateForest,
        store: BlockStore,
        mut rng: Rng,
    ) -> JsResult<Promise> {
        let mut store = ForeignBlockStore(store);
        let mut forest = Rc::clone(&forest.0);

        Ok(future_to_promise(async move {
            let payload =
                WnfsSharePayload::from_node(&node.0, temporal, &mut forest, &mut store, &mut rng)
                    .await
                    .map_err(error("Cannot create share payload"))?;

            let result = Array::new();

            Reflect::set(&result, &value!(0), &SharePayload(payload).into())
                .map_err(error("Failed to set file"))?;

            Reflect::set(&result, &value!(1), &PrivateForest(forest).into())
                .map_err(error("Failed to set forest"))?;

            Ok(value!(result))
        }))
    }

    #[wasm_bindgen(js_name = "getLabel")]
    pub fn get_label(&self) -> Vec<u8> {
        self.0.get_label().to_vec()
    }
}

//--------------------------------------------------------------------------------------------------
// Functions
//--------------------------------------------------------------------------------------------------

#[wasm_bindgen]
pub fn share(
    share_payload: SharePayload,
    share_count: u32,
    sharer_root_did: String,
    sharer_forest: &PrivateForest,
    sharer_store: BlockStore,
    recipient_exchange_root: Vec<u8>,
    recipient_store: BlockStore,
) -> JsResult<Promise> {
    let mut sharer_store = ForeignBlockStore(sharer_store);
    let mut sharer_forest = Rc::clone(&sharer_forest.0);
    let recipient_store = ForeignBlockStore(recipient_store);
    let cid = Cid::try_from(&recipient_exchange_root[..]).map_err(error("Invalid CID"))?;

    Ok(future_to_promise(async move {
        sharer::share::<ForeignExchangeKey>(
            &share_payload.0,
            share_count.into(),
            &sharer_root_did,
            &mut sharer_forest,
            &mut sharer_store,
            PublicLink::from_cid(cid),
            &recipient_store,
        )
        .await
        .map_err(error("Cannot share payload"))?;

        Ok(value!(PrivateForest(sharer_forest)))
    }))
}

#[wasm_bindgen(js_name = "createShareLabel")]
pub fn create_share_label(
    share_count: u32,
    sharer_root_did: &str,
    recipient_exchange_key: &[u8],
) -> Namefilter {
    Namefilter(sharer::create_share_label(
        share_count.into(),
        sharer_root_did,
        recipient_exchange_key,
    ))
}

#[wasm_bindgen(js_name = "findLatestShareCounter")]
pub fn find_latest_share_counter(
    share_count: u32,
    limit: u32,
    recipient_exchange_key: Vec<u8>,
    sharer_root_did: String,
    sharer_forest: &PrivateForest,
    sharer_store: BlockStore,
) -> JsResult<Promise> {
    let sharer_store = ForeignBlockStore(sharer_store);
    let sharer_forest = Rc::clone(&sharer_forest.0);

    Ok(future_to_promise(async move {
        let count = recipient::find_latest_share_counter(
            share_count.into(),
            limit.into(),
            &recipient_exchange_key,
            &sharer_root_did,
            &sharer_forest,
            &sharer_store,
        )
        .await
        .map_err(error("Cannot find share"))?;

        Ok(value!(count))
    }))
}

#[wasm_bindgen(js_name = "receiveShare")]
pub fn receive_share(
    share_label: Namefilter,
    recipient_key: PrivateKey,
    sharer_forest: &PrivateForest,
    sharer_store: BlockStore,
) -> JsResult<Promise> {
    let sharer_store = ForeignBlockStore(sharer_store);
    let recipient_key = ForeignPrivateKey(recipient_key);
    let mut sharer_forest = Rc::clone(&sharer_forest.0);

    Ok(future_to_promise(async move {
        let node = recipient::receive_share(
            share_label.0,
            &recipient_key,
            &mut sharer_forest,
            &sharer_store,
        )
        .await
        .map_err(error("Cannot receive share"))?;

        Ok(value!(PrivateNode(node)))
    }))
}
