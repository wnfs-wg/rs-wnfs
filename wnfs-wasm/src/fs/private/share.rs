use super::{AccessKey, ForeignExchangeKey, ForeignPrivateKey, PrivateForest, PrivateNode};
use crate::{
    fs::{utils::error, BlockStore, ForeignBlockStore, JsResult, PrivateKey},
    value,
};
use js_sys::Promise;
use libipld_core::cid::Cid;
use std::rc::Rc;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen_futures::future_to_promise;
use wnfs::{
    private::share::{recipient, sharer},
    public::PublicLink,
};

//--------------------------------------------------------------------------------------------------
// Functions
//--------------------------------------------------------------------------------------------------

#[wasm_bindgen]
pub fn share(
    access_key: AccessKey,
    share_count: u32,
    sharer_root_did: String,
    sharer_forest: &PrivateForest,
    recipient_exchange_root: Vec<u8>,
    store: BlockStore,
) -> JsResult<Promise> {
    let mut sharer_forest = Rc::clone(&sharer_forest.0);
    let cid = Cid::try_from(&recipient_exchange_root[..]).map_err(error("Invalid CID"))?;
    let store = ForeignBlockStore(store);

    Ok(future_to_promise(async move {
        sharer::share::<ForeignExchangeKey>(
            &access_key.0,
            share_count.into(),
            &sharer_root_did,
            &mut sharer_forest,
            PublicLink::from_cid(cid),
            &store,
        )
        .await
        .map_err(error("Cannot share item"))?;

        Ok(value!(PrivateForest(sharer_forest)))
    }))
}

#[wasm_bindgen(js_name = "createShareName")]
pub fn create_share_name(
    share_count: u32,
    sharer_root_did: &str,
    recipient_exchange_key: &[u8],
    sharer_forest: &PrivateForest,
) -> Name {
    let sharer_forest = Rc::clone(&sharer_forest.0);
    Name(sharer::create_share_name(
        share_count.into(),
        sharer_root_did,
        recipient_exchange_key,
        &sharer_forest,
    ))
}

#[wasm_bindgen(js_name = "findLatestShareCounter")]
pub fn find_latest_share_counter(
    share_count: u32,
    limit: u32,
    recipient_exchange_key: Vec<u8>,
    sharer_root_did: String,
    sharer_forest: &PrivateForest,
    store: BlockStore,
) -> JsResult<Promise> {
    let store = ForeignBlockStore(store);
    let sharer_forest = Rc::clone(&sharer_forest.0);

    Ok(future_to_promise(async move {
        let count = recipient::find_latest_share_counter(
            share_count.into(),
            limit.into(),
            &recipient_exchange_key,
            &sharer_root_did,
            &sharer_forest,
            &store,
        )
        .await
        .map_err(error("Cannot find share"))?;

        Ok(value!(count))
    }))
}

#[wasm_bindgen(js_name = "receiveShare")]
pub fn receive_share(
    share_name: Name,
    recipient_key: PrivateKey,
    sharer_forest: &PrivateForest,
    store: BlockStore,
) -> JsResult<Promise> {
    let store = ForeignBlockStore(store);
    let recipient_key = ForeignPrivateKey(recipient_key);
    let sharer_forest = Rc::clone(&sharer_forest.0);

    Ok(future_to_promise(async move {
        let node = recipient::receive_share(&share_label.0, &recipient_key, &sharer_forest, &store)
            .await
            .map_err(error("Cannot receive share"))?;

        Ok(value!(PrivateNode(node)))
    }))
}
