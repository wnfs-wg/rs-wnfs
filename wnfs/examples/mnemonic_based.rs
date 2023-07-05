use anyhow::{anyhow, Result};
use async_trait::async_trait;
use bip39::{Language, Mnemonic, MnemonicType, Seed};
use chrono::Utc;
use libipld_core::cid::Cid;
use rand_chacha::ChaCha12Rng;
use rand_core::SeedableRng;
use rsa::{traits::PublicKeyParts, BigUint, Oaep, RsaPrivateKey, RsaPublicKey};
use sha2::Sha256;
use std::rc::Rc;
use wnfs::{
    private::{
        share::{recipient, sharer},
        ExchangeKey, PrivateDirectory, PrivateForest, PrivateKey, PUBLIC_KEY_EXPONENT,
    },
    public::{PublicDirectory, PublicLink, PublicNode},
};
use wnfs_common::{BlockStore, MemoryBlockStore, CODEC_RAW};

#[async_std::main]
async fn main() -> Result<()> {
    let mnemonic = Mnemonic::new(MnemonicType::Words12, Language::English);

    let phrase = mnemonic.phrase();
    println!("phrase: {}", phrase);

    // BIP 39 based seed derivation from a mnemonic
    let seed = Seed::new(&mnemonic, /* optional password */ "");

    let root_did = "did:key:TODO(matheus23)".into();

    // Consider using BIP 32 for hierarchical keys, if needed/useful,
    // e.g. for additionally managing keypairs for write access via UCANs
    let exchange_keypair = SeededExchangeKey::from_seed(seed)?;
    let store = &MemoryBlockStore::new();
    let public_key_cid = exchange_keypair.store_public_key(store).await?;

    // Building from scratch in this case. Would actually be stored next
    // to the private forest usually:
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

    let rng = &mut rand::thread_rng();
    let forest = &mut Rc::new(PrivateForest::new());
    let root_dir =
        PrivateDirectory::new_and_store(Default::default(), Utc::now(), forest, store, rng).await?;

    let access_key = root_dir.as_node().store(forest, store, rng).await?;

    sharer::share::<PublicExchangeKey>(&access_key, 0, root_did, forest, exchange_root, store)
        .await?;

    // Share was written to the private forest now.

    let label = sharer::create_share_label(0, root_did, &exchange_keypair.encode_public_key());
    let _node = recipient::receive_share(label, &exchange_keypair, forest, store).await?;

    Ok(())
}

struct SeededExchangeKey(RsaPrivateKey);

struct PublicExchangeKey(RsaPublicKey);

impl SeededExchangeKey {
    pub fn from_seed(seed: Seed) -> Result<Self> {
        let seed_bytes: &[u8; 32] = seed.as_bytes()[..32].try_into()?;
        let rng = &mut ChaCha12Rng::from_seed(seed_bytes.clone());
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
