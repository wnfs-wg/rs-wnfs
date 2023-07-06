use anyhow::{anyhow, Result};
use async_trait::async_trait;
use chrono::Utc;
use libipld_core::cid::Cid;
use rand::thread_rng;
use rand_chacha::ChaCha12Rng;
use rand_core::SeedableRng;
use rsa::{traits::PublicKeyParts, BigUint, Oaep, RsaPrivateKey, RsaPublicKey};
use sha2::Sha256;
use std::rc::Rc;
use wnfs::{
    private::{
        share::{recipient, sharer},
        AccessKey, ExchangeKey, PrivateDirectory, PrivateForest, PrivateKey, PrivateNode,
        PUBLIC_KEY_EXPONENT,
    },
    public::{PublicDirectory, PublicLink, PublicNode},
};
use wnfs_common::{utils::get_random_bytes, BlockStore, MemoryBlockStore, CODEC_RAW};

//--------------------------------------------------------------------------------------------------
// Example Code
//--------------------------------------------------------------------------------------------------

#[async_std::main]
async fn main() -> Result<()> {
    // We use a single in-memory block store for this example.
    // In practice, there would actually be network transfer involved.
    let store = &MemoryBlockStore::new();

    // We create a directory, write something to it and get the private forest
    // and the directory's access key:
    let (mut forest, access_key) = root_dir_setup(store).await?;

    // We write a private share into the private forest for giving access to a
    // seed-derived keypair:
    let seed = setup_seeded_keypair_access(&mut forest, access_key, store).await?;

    println!("seed: {:x?}", seed);

    // And regain access to our directory:
    let node = regain_access_from_seed(&forest, seed, store).await?;
    let dir = node.as_dir()?;
    let content_bytes = dir
        .read(&["hello".into(), "world".into()], true, &forest, store)
        .await?;
    let content = String::from_utf8_lossy(&content_bytes);

    println!("Contents were: {content}");

    assert_eq!(content, "Hello, World!");

    Ok(())
}

async fn root_dir_setup(store: &impl BlockStore) -> Result<(Rc<PrivateForest>, AccessKey)> {
    // We generate a new simple example file system:
    let rng = &mut rand::thread_rng();
    let forest = &mut Rc::new(PrivateForest::new());
    let root_dir =
        &mut PrivateDirectory::new_and_store(Default::default(), Utc::now(), forest, store, rng)
            .await?;

    // And write something to it:
    root_dir
        .write(
            &["hello".into(), "world".into()],
            true,
            Utc::now(),
            b"Hello, World!".to_vec(),
            forest,
            store,
            rng,
        )
        .await?;

    // And finally we return the forest and the root directory's access key
    let access_key = root_dir.as_node().store(forest, store, rng).await?;
    Ok((Rc::clone(forest), access_key))
}

async fn setup_seeded_keypair_access(
    forest: &mut Rc<PrivateForest>,
    access_key: AccessKey,
    store: &impl BlockStore,
) -> Result<[u8; 32]> {
    let seed = get_random_bytes(&mut thread_rng());
    let exchange_keypair = SeededExchangeKey::from_seed(seed.clone())?;

    // Store the public key inside some public WNFS.
    // Building from scratch in this case. Would actually be stored next to the private forest usually.
    let public_key_cid = exchange_keypair.store_public_key(store).await?;
    let mut exchange_root = Rc::new(PublicDirectory::new(Utc::now()));
    exchange_root
        .write(
            &["main".into(), "v1.exchange_key".into()],
            public_key_cid,
            Utc::now(),
            store,
        )
        .await?;
    let exchange_root = PublicLink::new(PublicNode::Dir(exchange_root));

    // The user identity's root DID. In practice this would be e.g. an ed25519 key used
    // for e.g. UCANs or key usually used for authenticating writes.
    let root_did = "did:key:zExample".into();

    let counter = recipient::find_latest_share_counter(
        0,
        1000,
        &exchange_keypair.encode_public_key(),
        root_did,
        forest,
        store,
    )
    .await?
    .map(|x| x + 1)
    .unwrap_or_default();

    // Write the encrypted AccessKey into the forest
    sharer::share::<PublicExchangeKey>(
        &access_key,
        counter,
        root_did,
        forest,
        exchange_root,
        store,
    )
    .await?;

    Ok(seed)
}

async fn regain_access_from_seed(
    forest: &PrivateForest,
    seed: [u8; 32],
    store: &impl BlockStore,
) -> Result<PrivateNode> {
    // Re-derive keypair
    let exchange_keypair = SeededExchangeKey::from_seed(seed)?;
    let root_did = "did:key:zExample".into();

    // Re-load private node from forest
    let counter = recipient::find_latest_share_counter(
        0,
        1000,
        &exchange_keypair.encode_public_key(),
        root_did,
        forest,
        store,
    )
    .await?
    .unwrap_or_default();
    let label =
        sharer::create_share_label(counter, root_did, &exchange_keypair.encode_public_key());
    let node = recipient::receive_share(label, &exchange_keypair, forest, store).await?;
    node.search_latest(forest, store).await
}

//--------------------------------------------------------------------------------------------------
// Structs & Implementations
//--------------------------------------------------------------------------------------------------

struct SeededExchangeKey(RsaPrivateKey);

struct PublicExchangeKey(RsaPublicKey);

impl SeededExchangeKey {
    pub fn from_seed(seed: [u8; 32]) -> Result<Self> {
        let rng = &mut ChaCha12Rng::from_seed(seed);
        let private_key = RsaPrivateKey::new(rng, 2048)?;
        Ok(Self(private_key))
    }

    pub async fn store_public_key(&self, store: &impl BlockStore) -> Result<Cid> {
        store.put_block(self.encode_public_key(), CODEC_RAW).await
    }

    pub fn encode_public_key(&self) -> Vec<u8> {
        self.0.n().to_bytes_be()
    }
}

#[async_trait(?Send)]
impl PrivateKey for SeededExchangeKey {
    async fn decrypt(&self, ciphertext: &[u8]) -> Result<Vec<u8>> {
        let padding = Oaep::new::<Sha256>();
        self.0.decrypt(padding, ciphertext).map_err(|e| anyhow!(e))
    }
}

#[async_trait(?Send)]
impl ExchangeKey for PublicExchangeKey {
    async fn encrypt(&self, data: &[u8]) -> Result<Vec<u8>> {
        let padding = Oaep::new::<Sha256>();
        self.0
            .encrypt(&mut rand::thread_rng(), padding, data)
            .map_err(|e| anyhow!(e))
    }

    async fn from_modulus(modulus: &[u8]) -> Result<Self> {
        let n = BigUint::from_bytes_be(modulus);
        let e = BigUint::from(PUBLIC_KEY_EXPONENT);

        Ok(Self(rsa::RsaPublicKey::new(n, e).map_err(|e| anyhow!(e))?))
    }
}
