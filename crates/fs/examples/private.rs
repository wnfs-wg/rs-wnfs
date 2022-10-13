//! This example shows how to add a directory to a private forest (also HAMT) which encrypts it.
//! It also shows how to retrieve encrypted nodes from the forest using access keys in `PrivateRef`s.

use libipld::Cid;
use wnfs::{
    dagcbor,
    private::{PrivateForest, PrivateRef},
    BlockStore, MemoryBlockStore, Namefilter, PrivateDirectory, PrivateOpResult,
};

use chrono::Utc;
use rand::{thread_rng, RngCore};

use std::rc::Rc;

#[async_std::main]
async fn main() {
    // Create a memory-based blockstore.
    let store = &mut MemoryBlockStore::default();

    // A random number generator the private filesystem can use.
    let rng = &mut thread_rng();

    // Create a new private forest and get the cid to it.
    let (hamt_cid, private_ref) = get_hamt_cid_and_private_ref(store, rng).await;

    // Fetch cbor_bytes from the blockstore.
    let cbor_bytes = store.get_deserializable::<Vec<u8>>(&hamt_cid).await.unwrap();

    // Decode CBOR.
    let forest = dagcbor::decode::<PrivateForest>(cbor_bytes.as_ref()).unwrap();

    // Fetch and decrypt a directory from the HAMT using provided private ref.
    let dir = forest.get(&private_ref, store).await.unwrap();

    // Print the directory.
    println!("{:?}", dir);
}

async fn get_hamt_cid_and_private_ref<B, R>(store: &mut B, rng: &mut R) -> (Cid, PrivateRef)
where
    B: BlockStore,
    R: RngCore,
{
    // Create HAMT intermediate data structure.
    let hamt = Rc::new(PrivateForest::new());

    // Create a new private directory.
    let dir = Rc::new(PrivateDirectory::new(
        Namefilter::default(),
        Utc::now(),
        rng,
    ));

    // Add a /pictures/cats subdirectory.
    let PrivateOpResult { hamt, root_dir, .. } = dir
        .mkdir(
            &["pictures".into(), "cats".into()],
            true,
            Utc::now(),
            hamt,
            store,
            rng,
        )
        .await
        .unwrap();

    // Serialize the HAMT to IPLD DAG CBOR.
    let cbor_bytes = dagcbor::async_encode(&hamt, store).await.unwrap();

    // Persist encoded HAMT to the blockstore.
    let hamt_cid = store.put_serializable(&cbor_bytes).await.unwrap();

    // Private ref contains namefilter, access keys for the directory.
    let private_ref = root_dir.header.get_private_ref().unwrap();

    (hamt_cid, private_ref)
}
