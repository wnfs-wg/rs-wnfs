use chrono::Utc;
use futures::StreamExt;
use rand::thread_rng;
use serde::{Deserialize, Serialize};
use sha3::Sha3_256;
use std::{io::Cursor, rc::Rc};
use wnfs::{
    ipld::{DagCborCodec, Decode, Encode, Ipld, Serializer},
    private::{
        hamt::Hasher, AesKey, Namefilter, PrivateDirectory, PrivateForest, PrivateOpResult,
        RevisionRef,
    },
    utils, MemoryBlockStore,
};

#[tokio::main(flavor = "multi_thread")]
async fn main() -> anyhow::Result<()> {
    // ----------- Prerequisites -----------

    let store = &mut MemoryBlockStore::default();
    let rng = &mut thread_rng();
    let forest = &mut Rc::new(PrivateForest::new());

    // ----------- Create a private directory -----------

    // TODO(matheus23) perhaps rework this into something HKDF-based instead & also derive the inumber

    // Some existing user key.
    let some_key = AesKey::new(utils::get_random_bytes::<32>(rng));

    // Creating ratchet_seed from our user key. And intializing the inumber and namefilter.
    let ratchet_seed = Sha3_256::hash(&some_key.as_bytes());
    let inumber = utils::get_random_bytes::<32>(rng); // Needs to be random

    // Create a root directory from the ratchet_seed, inumber and namefilter. Directory gets saved in forest.
    let PrivateOpResult { root_dir, .. } = PrivateDirectory::new_with_seed_and_store(
        Namefilter::default(),
        Utc::now(),
        ratchet_seed,
        inumber,
        forest,
        store,
        rng,
    )
    .await
    .unwrap();

    // ----------- Create a subdirectory -----------

    // Add a /movies/anime to the directory.
    let PrivateOpResult { root_dir, .. } = root_dir
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
    let cbor = encode(&root_dir.header.derive_revision_ref())?;

    // We can deserialize the revision_ref on the other end.
    let revision_ref = decode(cbor)?;

    // Now we can fetch the directory from the forest using the revision_ref.
    let fetched_node = forest
        .get_multivalue(&revision_ref, store)
        .next()
        .await
        .unwrap()?;

    println!("{:#?}", fetched_node);

    // --------- Method 2: Generate a revision ref from a shared secret -----------

    // We can also create a revision ref from scratch if we remember the parameters.
    let revision_ref = RevisionRef::with_seed(Namefilter::default(), ratchet_seed, inumber);

    // And we can fetch the directory again using the generated revision_ref.
    let fetched_node = forest
        .get_multivalue(&revision_ref, store)
        .next()
        .await
        .unwrap()?;

    println!("{:#?}", fetched_node);

    // The private_ref might point to some old revision of the root_dir.
    // We can do the following to get the latest revision.
    let fetched_dir = fetched_node.search_latest(&forest, store).await?.as_dir()?;

    println!("{:#?}", fetched_dir);

    Ok(())
}

fn encode(revision_ref: &RevisionRef) -> anyhow::Result<Vec<u8>> {
    let mut bytes = Vec::new();
    let ipld = revision_ref.serialize(Serializer)?;
    ipld.encode(DagCborCodec, &mut bytes)?;
    Ok(bytes)
}

fn decode(bytes: Vec<u8>) -> anyhow::Result<RevisionRef> {
    let ipld = Ipld::decode(DagCborCodec, &mut Cursor::new(bytes))?;
    RevisionRef::deserialize(ipld).map_err(Into::into)
}
