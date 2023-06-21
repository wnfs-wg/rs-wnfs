//! This example shows how to add a directory to a private forest (a HAMT) where encrypted ciphertexts are stored.
//! It also shows how to retrieve encrypted nodes from the forest using `PrivateRef`s.

use anyhow::Result;
use chrono::Utc;
use libipld_core::cid::Cid;
use rand::{thread_rng, RngCore};
use std::rc::Rc;
use wnfs::private::{PrivateDirectory, PrivateForest, PrivateNode, PrivateRef};
use wnfs_common::{BlockStore, MemoryBlockStore};
use wnfs_namefilter::Namefilter;

#[async_std::main]
async fn main() -> Result<()> {
    // Create an in-memory block store.
    let store = &MemoryBlockStore::default();

    // Create a random number generator the private filesystem can use.
    let rng = &mut thread_rng();

    // Create a new private forest and get the cid to it.
    let (forest_cid, private_ref) = create_forest_and_add_directory(store, rng).await?;

    // Deserialize private forest from the blockstore.
    let forest = store
        .get_deserializable::<PrivateForest>(&forest_cid)
        .await?;

    // Fetch and decrypt a directory from the private forest using provided private ref.
    let dir = PrivateNode::load(&private_ref, &forest, store).await?;

    // Print the directory.
    println!("{:#?}", dir);

    Ok(())
}

async fn create_forest_and_add_directory(
    store: &impl BlockStore,
    rng: &mut impl RngCore,
) -> Result<(Cid, PrivateRef)> {
    // Create the private forest (a HAMT), a map-like structure where file and directory ciphertexts are stored.
    let forest = &mut Rc::new(PrivateForest::new());

    // Create a new directory.
    let dir = &mut Rc::new(PrivateDirectory::new(
        Namefilter::default(),
        Utc::now(),
        rng,
    ));

    // Add a /pictures/cats subdirectory.
    dir.mkdir(
        &["pictures".into(), "cats".into()],
        true,
        Utc::now(),
        forest,
        store,
        rng,
    )
    .await?;

    // Private ref contains data and keys for fetching and decrypting the directory node in the private forest.
    let private_ref = dir.store(forest, store, rng).await?;

    // Persist encoded private forest to the block store.
    let forest_cid = store.put_async_serializable(forest).await?;

    Ok((forest_cid, private_ref))
}
