use anyhow::Result;
use chrono::Utc;
use libipld_core::cid::Cid;
use rand::thread_rng;
use std::{collections::BTreeSet, rc::Rc};
use wnfs::private::{
    forest::{
        hamt::HamtForest,
        proofs::{ForestProofs, ProvingHamtForest},
        traits::PrivateForest,
    },
    AccessKey, PrivateDirectory, PrivateNode,
};
use wnfs_common::{BlockStore, MemoryBlockStore};
use wnfs_nameaccumulator::NameAccumulator;

#[async_std::main]
async fn main() -> Result<()> {
    // In between operations, Alice, Bob, and the persistence service would
    // exchange blocks via bitswap, car mirror or some other protocol.
    // Here we're simplifying by sharing a 'global' block store.
    let store = &MemoryBlockStore::new();

    // Alice creates a private file system with some data.
    // She shares read access with bob by securely transferring the access_key.
    // She also publicly announces bob has access to a certain directory at allowed_write_name.
    let (old_forest_cid, access_key, allowed_write_name) = alice_actions(store).await?;

    // Bob can take the access_key and forest and create writes.
    // The output will be a new state of the forest as well as a set of proofs, proving
    // he didn't touch anything in the file system except what he was allowed to.
    let (proofs, new_forest_cid) = bob_actions(old_forest_cid, access_key, store).await?;

    // A persistence service can check Bob's changes between the forests via his proofs.
    // The service does *not* need read access (it doesn't get to know the access_key)
    // and it only gains limited information from the proofs from Bob.
    // The idea is that in practice the persistence service can accept updates from anyone
    // that were indirectly given access by Alice out-of-bounds, and it will store the updated
    // file system.
    persistence_service_actions(
        old_forest_cid,
        new_forest_cid,
        proofs,
        allowed_write_name,
        store,
    )
    .await
}

/// Alice creates a directory and gives access to it out to someone else.
/// The returned AccessKey gives read access and the NameAccumulator is
/// supposed to be publicly signed for verifyable write access.
async fn alice_actions(store: &impl BlockStore) -> Result<(Cid, AccessKey, NameAccumulator)> {
    let rng = &mut thread_rng();
    let forest = &mut Rc::new(HamtForest::new_rsa_2048(rng));
    let root_dir =
        &mut PrivateDirectory::new_and_store(&forest.empty_name(), Utc::now(), forest, store, rng)
            .await?;

    let access_key = root_dir.as_node().store(forest, store, rng).await?;
    let cid = forest.store(store).await?;
    let setup = forest.get_accumulator_setup();
    let allowed_name = root_dir.header.get_name().as_accumulator(setup).clone();

    Ok((cid, access_key, allowed_name))
}

/// Bob can take the forest, read data using the private ref
/// and prove writes.
async fn bob_actions(
    old_forest_cid: Cid,
    root_dir_access: AccessKey,
    store: &impl BlockStore,
) -> Result<(ForestProofs, Cid)> {
    let hamt_forest = HamtForest::load(&old_forest_cid, store).await?;
    let mut forest = ProvingHamtForest::new(Rc::new(hamt_forest));
    let rng = &mut thread_rng();

    let mut root_node = PrivateNode::load(&root_dir_access, &forest, store, None).await?;
    let root_dir = root_node.as_dir_mut()?;

    // Do arbitrary writes in any paths you have access to
    root_dir
        .write(
            &["Some".into(), "file.txt".into()],
            true,
            Utc::now(),
            b"Hello, Alice!".to_vec(),
            &mut forest,
            store,
            rng,
        )
        .await?;

    root_dir.as_node().store(&mut forest, store, rng).await?;

    let ProvingHamtForest { forest, proofs } = forest;

    let new_forest_cid = forest.store(store).await?;

    Ok((proofs, new_forest_cid))
}

/// A persistence service can verify write proofs relative to a signed
/// accumulator without read access.
async fn persistence_service_actions(
    old_forest_cid: Cid,
    new_forest_cid: Cid,
    proofs: ForestProofs,
    allowed_access: NameAccumulator,
    store: &impl BlockStore,
) -> Result<()> {
    let old_forest = HamtForest::load(&old_forest_cid, store).await?;
    let new_forest = HamtForest::load(&new_forest_cid, store).await?;

    let forest = ProvingHamtForest::from_proofs(proofs, Rc::new(new_forest));

    forest
        .verify_against_previous_state(&old_forest, &BTreeSet::from([allowed_access]), store)
        .await
}
