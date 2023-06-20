use chrono::Utc;
use futures::StreamExt;
use rand::thread_rng;
use sha3::Sha3_256;
use std::rc::Rc;
use wnfs::private::{
    forest::{hamt::HamtForest, traits::PrivateForest},
    AesKey, PrivateDirectory, RevisionRef,
};
use wnfs_common::{dagcbor, utils, MemoryBlockStore};
use wnfs_hamt::Hasher;
use wnfs_nameaccumulator::{AccumulatorSetup, NameSegment};

#[tokio::main(flavor = "multi_thread")]
async fn main() -> anyhow::Result<()> {
    // ----------- Prerequisites -----------

    let store = &mut MemoryBlockStore::default();
    let rng = &mut thread_rng();
    let setup = &AccumulatorSetup::trusted(rng);
    let forest = &mut Rc::new(HamtForest::new(setup.clone()));

    // ----------- Create a private directory -----------

    // TODO(matheus23) perhaps rework this into something HKDF-based instead & also derive the inumber

    // Some existing user key.
    let some_key = AesKey::new(utils::get_random_bytes::<32>(rng));

    // Creating ratchet_seed from our user key. And intializing the inumber and namefilter.
    let ratchet_seed = Sha3_256::hash(&some_key.as_bytes());
    let inumber = NameSegment::new(rng);

    // Create a root directory from the ratchet_seed, inumber and namefilter. Directory gets saved in forest.
    let root_dir = &mut PrivateDirectory::new_with_seed_and_store(
        &forest.empty_name(),
        Utc::now(),
        ratchet_seed,
        inumber.clone(),
        forest,
        store,
        rng,
    )
    .await
    .unwrap();

    // ----------- Create a subdirectory -----------

    // Add a /movies/anime to the directory.
    root_dir
        .mkdir(
            &["movies".into(), "anime".into()],
            true,
            Utc::now(),
            forest,
            store,
            rng,
        )
        .await?;

    // --------- Method 1: Exchange serialized revision ref -----------

    // serialize the root_dir's revision_ref.
    let cbor = dagcbor::encode(&root_dir.header.derive_revision_ref(setup))?;

    // We can deserialize the revision_ref on the other end.
    let revision_ref = dagcbor::decode(&cbor)?;

    // Now we can fetch the directory from the forest using the revision_ref.
    let fetched_node = forest
        .get_multivalue(&revision_ref, store, &forest.empty_name())
        .next()
        .await
        .unwrap()?;

    println!("{fetched_node:#?}");

    // --------- Method 2: Generate a revision ref from a shared secret -----------

    // We can also create a revision ref from scratch if we remember the parameters.
    let revision_ref = RevisionRef::with_seed(&forest.empty_name(), ratchet_seed, inumber, setup);

    // And we can fetch the directory again using the generated revision_ref.
    let fetched_node = forest
        .get_multivalue(&revision_ref, store, &forest.empty_name())
        .next()
        .await
        .unwrap()?;

    println!("{fetched_node:#?}");

    // The private_ref might point to some old revision of the root_dir.
    // We can do the following to get the latest revision.
    let fetched_dir = fetched_node.search_latest(forest, store).await?.as_dir()?;

    println!("{fetched_dir:#?}");

    Ok(())
}
