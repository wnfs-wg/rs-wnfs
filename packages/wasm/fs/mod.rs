use chrono::Utc;
use wasm_bindgen::prelude::*;
use wnfs::{public::PublicDirectory, BlockStore, BlockStoreLookup, IpldCodec, MemoryBlockStore};

#[wasm_bindgen]
pub async fn lookup_node() -> String {
    let root = PublicDirectory::new(Utc::now());

    let store = MemoryBlockStore::default();

    let node = root.lookup_node("Test", &store).await.unwrap();

    format!("Node lookup done!: {:?}", node)
}

// #[wasm_bindgen]
// pub async fn lookup_node() -> String {
//     let mut store = MemoryBlockStore::default();

//     let cid = store
//         .put_block(vec![0, 255, 1], IpldCodec::DagCbor)
//         .await
//         .unwrap();

//     let bytes = store.get_block(&cid).await.unwrap();

//     format!("Node lookup done!: CID = {:?} | bytes = {:?}", cid, bytes)
// }
