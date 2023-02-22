//! This example shows how to add a directory to a private forest (a HAMT) where encrypted ciphertexts are stored.
//! It also shows how to retrieve encrypted nodes from the forest using `PrivateRef`s.

use chrono::Utc;
use libipld::Cid;
use rand::{thread_rng, RngCore};
use std::rc::Rc;
use wnfs::{
    private::{PrivateForest, PrivateRef},
    BlockStore, MemoryBlockStore, Namefilter, PrivateDirectory, PrivateNode, PrivateOpResult,
};

#[async_std::main]
async fn main() {
    // Create an in-memory block store.
    let store = &mut MemoryBlockStore::default();

    // Create a random number generator the private filesystem can use.
    let rng = &mut thread_rng();

    // Create a new private forest and get the cid to it.
    let (forest_cid, private_ref) = get_forest_cid_and_private_ref(store, rng).await;

    // Fetch CBOR bytes of private forest from the blockstore.
    let forest = store
        .get_deserializable::<PrivateForest>(&forest_cid)
        .await
        .unwrap();

    // Fetch and decrypt a directory from the private forest using provided private ref.
    let dir = PrivateNode::load(&private_ref, &forest, store)
        .await
        .unwrap();

    // Print the directory.
    println!("{:#?}", dir);
}

async fn get_forest_cid_and_private_ref(
    store: &mut impl BlockStore,
    rng: &mut impl RngCore,
) -> (Cid, PrivateRef) {
    // Create the private forest (a HAMT), a map-like structure where file and directory ciphertexts are stored.
    let forest = &mut Rc::new(PrivateForest::new());

    // Create a new directory.
    let dir = Rc::new(PrivateDirectory::new(
        Namefilter::default(),
        Utc::now(),
        rng,
    ));

    // Add a /pictures/cats subdirectory.
    let PrivateOpResult { root_dir, .. } = dir
        .mkdir(
            &["pictures".into(), "cats".into()],
            true,
            Utc::now(),
            forest,
            store,
            rng,
        )
        .await
        .unwrap();

    // Private ref contains data and keys for fetching and decrypting the directory node in the private forest.
    let private_ref = root_dir.store(forest, store, rng).await.unwrap();

    // Persist encoded private forest to the block store.
    let forest_cid = store.put_async_serializable(forest).await.unwrap();

    (forest_cid, private_ref)
}
