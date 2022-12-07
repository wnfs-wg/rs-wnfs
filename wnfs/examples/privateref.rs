use chrono::Utc;
use libipld::{serde::Serializer, Ipld};
use rand::thread_rng;
use sha3::Sha3_256;
use skip_ratchet::Ratchet;
use std::{io::Cursor, rc::Rc};
use wnfs::{
    ipld::{DagCborCodec, Decode, Encode},
    private::{self, Key, PrivateForest, PrivateRef, RevisionKey},
    utils, Hasher, MemoryBlockStore, Namefilter, PrivateDirectory, PrivateNode, PrivateOpResult,
};

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    // Prerequisites:
    let store = &mut MemoryBlockStore::default();
    let rng = &mut thread_rng();
    let forest = Rc::new(PrivateForest::new());

    // Some existing user key.
    let some_key = Key::new(utils::get_random_bytes::<32>(rng));

    // Creating ratchet_seed from our key. And intializing the inumber and namefilter.
    let ratchet_seed = Sha3_256::hash(&some_key.as_bytes());
    let inumber = utils::get_random_bytes::<32>(rng); // Needs to be random

    // Create the directory from the ratchet_seed, inumber and namefilter.
    let root_dir = Rc::new(PrivateDirectory::with_seed(
        Namefilter::default(),
        Utc::now(),
        ratchet_seed,
        inumber,
    ));

    // Get the privateref from the root_dir.
    let private_ref = root_dir.header.get_private_ref();
    let name = root_dir.header.get_saturated_name();

    // Store the directory in the forest.
    let forest = forest
        .put(
            name,
            &private_ref,
            &PrivateNode::Dir(Rc::clone(&root_dir)),
            store,
            rng,
        )
        .await?;

    // Add a /movies/anime to the directory.
    let PrivateOpResult {
        forest, root_dir, ..
    } = root_dir
        .mkdir(
            &["movies".into(), "anime".into()],
            true,
            Utc::now(),
            forest,
            store,
            rng,
        )
        .await?;

    // We can create a revision_key from the ratchet_seed.
    let ratchet = Ratchet::zero(ratchet_seed);
    let revision_key = RevisionKey::from(Key::new(ratchet.derive_key()));

    // Now let's serialize the root_dir's private_ref.
    let cbor = encode_ipld(root_dir.header.get_private_ref().serialize(
        Serializer,
        &revision_key,
        rng,
    )?)?;

    // We can deserialize the private_ref using the information we have.
    let private_ref = decode_ipld(cbor, &revision_key)?;

    // Now we can fetch the directory from the forest.
    let fetched_dir = forest
        .get(&private_ref, PrivateForest::resolve_lowest, store)
        .await?;

    println!("{:#?}", fetched_dir);

    // We can also create one from scratch.
    let private_ref = PrivateRef::with_seed(Namefilter::default(), ratchet_seed, inumber);
    println!("Private ref: {:?}", private_ref);

    // Again we can fetch the directory from the forest.
    let fetched_dir = forest
        .get(&private_ref, PrivateForest::resolve_lowest, store)
        .await?;

    println!("{:#?}", fetched_dir);

    if let PrivateNode::Dir(fetched_dir) = fetched_dir.unwrap() {
        let PrivateOpResult { result, .. } = fetched_dir.get_node(&[], true, forest, store).await?;
        println!("{:#?}", result);
    }

    Ok(())
}

fn encode_ipld(ipld: Ipld) -> anyhow::Result<Vec<u8>> {
    let mut bytes = Vec::new();
    ipld.encode(DagCborCodec, &mut bytes)?;
    Ok(bytes)
}

fn decode_ipld(bytes: Vec<u8>, revision_key: &RevisionKey) -> anyhow::Result<PrivateRef> {
    let ipld = Ipld::decode(DagCborCodec, &mut Cursor::new(bytes))?;
    PrivateRef::deserialize(ipld, revision_key).map_err(Into::into)
}
