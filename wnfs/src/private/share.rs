//! The WNFS Shared Private Data Extension Specification is a protocol that allows users to exchange private data
//! asynchronously when the other party may be offline using store-and-forward networks.
//! The protocol uses asymmetric encryption, with RSA public keys, and is versioned to support multiple versions on the same file system.
//! Public keys are widely distributed in a "exchange keys partition" and are grouped by device for easy selection by the sender.
//! Asymmetrically encrypted access keys containing pointers to the private data are stored in the "Private Forest" and are labeled with a name filter that includes the sender's and recipient's information,
//! as well as a counter.

use self::sharer::share;
use super::{forest::traits::PrivateForest, AccessKey, ExchangeKey};
use crate::{error::ShareError, public::PublicLink};
use anyhow::{bail, Result};
use std::marker::PhantomData;
use wnfs_common::BlockStore;

//--------------------------------------------------------------------------------------------------
// Constants
//--------------------------------------------------------------------------------------------------

const EXCHANGE_KEY_NAME: &str = "v1.exchange_key";

//--------------------------------------------------------------------------------------------------
// Type Definitions
//--------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub struct Share<'a, K: ExchangeKey, S: BlockStore, F: PrivateForest> {
    access_key: &'a AccessKey,
    count: u64,
    sharer: Option<Sharer<'a, F>>,
    recipients: Vec<Recipient>,
    store: &'a S,
    phantom: PhantomData<K>,
}

#[derive(Debug)]
pub struct Sharer<'a, F: PrivateForest> {
    pub root_did: String,
    pub forest: &'a mut F,
}

#[derive(Debug)]
pub struct Recipient {
    pub exchange_root: PublicLink,
}

//--------------------------------------------------------------------------------------------------
// Implementations
//--------------------------------------------------------------------------------------------------

impl<'a, K: ExchangeKey, S: BlockStore, F: PrivateForest> Share<'a, K, S, F> {
    ///  Creates a new instance of the Share with the given access key and count, and initializes the other fields as "None".
    pub fn new(store: &'a S, access_key: &'a AccessKey, count: u64) -> Self {
        Self {
            access_key,
            count,
            sharer: None,
            recipients: Vec::new(),
            store,
            phantom: PhantomData,
        }
    }

    /// Sets the sharer field.
    pub fn by(&mut self, sharer: Sharer<'a, F>) -> &mut Self {
        self.sharer = Some(sharer);
        self
    }

    /// Takes a vector of recipients as an argument, and adds it to the existing recipients of the Share struct.
    pub fn with(&mut self, recipients: Vec<Recipient>) -> &mut Self {
        self.recipients.extend(recipients);
        self
    }

    /// Takes a recipient as an argument and adds it to the existing recipients of the Share struct.
    pub fn to(&mut self, recipient: Recipient) -> &mut Self {
        self.recipients.push(recipient);
        self
    }

    /// Performs the sharing operation with the previously set sharer and recipients.
    /// It takes the access key, sharer, and recipients, and performs the share operation,
    /// encrypts the access key and stores it in the sharer's private forest.
    pub async fn finish(&mut self) -> Result<()> {
        if self.sharer.is_none() || self.recipients.is_empty() {
            bail!(ShareError::NoSharerOrRecipients);
        }

        let sharer = self.sharer.take().unwrap();
        let recipients = std::mem::take(&mut self.recipients);

        for recipient in recipients {
            share::<K>(
                self.access_key,
                self.count,
                &sharer.root_did,
                sharer.forest,
                recipient.exchange_root,
                self.store,
            )
            .await?;
        }

        Ok(())
    }
}

//--------------------------------------------------------------------------------------------------
// Functions
//--------------------------------------------------------------------------------------------------

pub mod sharer {
    use super::EXCHANGE_KEY_NAME;
    use crate::{
        private::{forest::traits::PrivateForest, AccessKey, ExchangeKey, PublicKeyModulus},
        public::PublicLink,
    };
    use anyhow::Result;
    use async_stream::try_stream;
    use futures::{Stream, StreamExt};
    use wnfs_common::{BlockStore, CODEC_RAW};
    use wnfs_nameaccumulator::{Name, NameSegment};

    /// Encrypts and shares a access key with multiple recipients using their
    /// exchange keys and stores the shares in the sharer's private forest.
    #[allow(clippy::too_many_arguments)]
    pub async fn share<K: ExchangeKey>(
        access_key: &AccessKey,
        share_count: u64,
        sharer_root_did: &str,
        sharer_forest: &mut impl PrivateForest,
        recipient_exchange_root: PublicLink,
        store: &impl BlockStore,
    ) -> Result<()> {
        let mut exchange_keys = fetch_exchange_keys(recipient_exchange_root, store).await;
        let encoded_key = &serde_ipld_dagcbor::to_vec(access_key)?;

        while let Some(result) = exchange_keys.next().await {
            let public_key_modulus = result?;
            let exchange_key = K::from_modulus(&public_key_modulus).await?;
            let encrypted_key = exchange_key.encrypt(encoded_key).await?;
            let share_label = create_share_name(
                share_count,
                sharer_root_did,
                &public_key_modulus,
                sharer_forest,
            );

            let access_key_cid = store.put_block(encrypted_key, CODEC_RAW).await?;

            sharer_forest
                .put_encrypted(&share_label, Some(access_key_cid), store)
                .await?;
        }

        Ok(())
    }

    /// Fetches the exchange keys of recipients using their exchange root, resolve the root_dir,
    /// search for the exchange key, and read the exchange key's cid in the recipient's store and
    /// yield the exchange key's value.
    pub async fn fetch_exchange_keys(
        recipient_exchange_root: PublicLink,
        store: &impl BlockStore,
    ) -> impl Stream<Item = Result<PublicKeyModulus>> + '_ {
        Box::pin(try_stream! {
            let root_dir = recipient_exchange_root
                .resolve_value(store)
                .await?
                .as_dir()?;

            let devices = root_dir.ls(&[], store).await?;
            for (device, _) in devices {
                let value = root_dir.ls(&[device.clone()], store).await?;
                for (name, _) in value {
                    if name == EXCHANGE_KEY_NAME {
                        let cid = root_dir.read(&[device, name], store).await?;
                        yield store.get_block(&cid).await?.to_vec();
                        break
                    }
                }
            }
        })
    }

    /// Generates the name for a share for given recipient,
    /// at given count and from given sharer.
    pub fn create_share_name(
        share_count: u64,
        sharer_root_did: &str,
        recipient_exchange_key: &[u8],
        sharer_forest: &impl PrivateForest,
    ) -> Name {
        sharer_forest.empty_name().with_segments_added([
            NameSegment::from_seed(sharer_root_did.as_bytes()),
            NameSegment::from_seed(recipient_exchange_key),
            NameSegment::from_seed(share_count.to_le_bytes()),
        ])
    }
}

pub mod recipient {
    use super::sharer;
    use crate::{
        error::ShareError,
        private::{forest::traits::PrivateForest, AccessKey, PrivateKey, PrivateNode},
    };
    use anyhow::Result;
    use sha3::Sha3_256;
    use wnfs_common::BlockStore;
    use wnfs_hamt::Hasher;
    use wnfs_nameaccumulator::Name;

    /// Seeks to the latest share counter that is populated.
    pub async fn find_latest_share_counter(
        share_count_start: u64,
        limit: u64,
        recipient_exchange_key: &[u8],
        sharer_root_did: &str,
        sharer_forest: &impl PrivateForest,
        store: &impl BlockStore,
    ) -> Result<Option<u64>> {
        for share_count in share_count_start..share_count_start + limit {
            let share_label = sharer::create_share_name(
                share_count,
                sharer_root_did,
                recipient_exchange_key,
                sharer_forest,
            );

            if !sharer_forest.has(&share_label, store).await? {
                if share_count == share_count_start {
                    // There don't seem to be any shares
                    return Ok(None);
                } else {
                    // We've hit the first unpopulated label,
                    // so the last one was the last valid share counter.
                    return Ok(Some(share_count - 1));
                }
            }
        }

        // We've exhausted the limit, this seems to be the last valid share count under the limit
        Ok(Some(share_count_start + limit - 1))
    }

    /// Lets a recipient receive a share from a sharer using the sharer's forest and store.
    /// The recipient's private forest and store are used to store the share.
    pub async fn receive_share(
        share_label: &Name,
        recipient_key: &impl PrivateKey,
        sharer_forest: &impl PrivateForest,
        store: &impl BlockStore,
    ) -> Result<PrivateNode> {
        let setup = sharer_forest.get_accumulator_setup();
        // Get cid to encrypted payload from sharer's forest using share_label
        let access_key_cid = sharer_forest
            .get_encrypted_by_hash(&Sha3_256::hash(&share_label.as_accumulator(setup)), store)
            .await?
            .ok_or(ShareError::AccessKeyNotFound)?
            .first()
            .ok_or(ShareError::AccessKeyNotFound)?;

        // Get encrypted access key from store using cid
        let encrypted_access_key = store.get_block(access_key_cid).await?.to_vec();

        // Decrypt access key using recipient's private key and decode it.
        let access_key: AccessKey =
            serde_ipld_dagcbor::from_slice(&recipient_key.decrypt(&encrypted_access_key).await?)?;

        // Use decrypted key to get cid to encrypted node in sharer's forest.
        PrivateNode::from_private_ref(
            &access_key.derive_private_ref()?,
            sharer_forest,
            store,
            None,
        )
        .await
    }
}

//--------------------------------------------------------------------------------------------------
// Tests
//--------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::{
        recipient::{self, find_latest_share_counter},
        sharer, Recipient, Sharer, EXCHANGE_KEY_NAME,
    };
    use crate::{
        private::{
            forest::{hamt::HamtForest, traits::PrivateForest},
            share::Share,
            AccessKey, PrivateDirectory, RsaPublicKey,
        },
        public::{PublicLink, PublicNode},
    };
    use chrono::Utc;
    use rand_chacha::ChaCha12Rng;
    use rand_core::SeedableRng;
    use std::rc::Rc;
    use wnfs_common::{BlockStore, MemoryBlockStore};

    mod helper {
        use crate::{
            private::{
                forest::traits::PrivateForest, share::EXCHANGE_KEY_NAME, PrivateDirectory,
                RsaPrivateKey,
            },
            public::PublicDirectory,
        };
        use anyhow::Result;
        use chrono::Utc;
        use rand_core::CryptoRngCore;
        use std::rc::Rc;
        use wnfs_common::{BlockStore, CODEC_RAW};

        pub(super) async fn create_sharer_dir(
            forest: &mut impl PrivateForest,
            store: &impl BlockStore,
            rng: &mut impl CryptoRngCore,
        ) -> Result<Rc<PrivateDirectory>> {
            let mut dir = PrivateDirectory::new_and_store(
                &forest.empty_name(),
                Utc::now(),
                forest,
                store,
                rng,
            )
            .await?;

            dir.write(
                &["text.txt".into()],
                true,
                Utc::now(),
                b"Hello World!".to_vec(),
                forest,
                store,
                rng,
            )
            .await?;

            Ok(dir)
        }

        pub(super) async fn create_recipient_exchange_root(
            store: &impl BlockStore,
        ) -> Result<(RsaPrivateKey, Rc<PublicDirectory>)> {
            let key = RsaPrivateKey::new()?;
            let exchange_key = key.get_public_key().get_public_key_modulus()?;
            let exchange_key_cid = store.put_block(exchange_key, CODEC_RAW).await?;

            let mut root_dir = Rc::new(PublicDirectory::new(Utc::now()));
            root_dir
                .write(
                    &["device1".into(), EXCHANGE_KEY_NAME.into()],
                    exchange_key_cid,
                    Utc::now(),
                    store,
                )
                .await?;

            Ok((key, root_dir))
        }
    }

    #[async_std::test]
    async fn can_share_and_recieve_share() {
        let rng = &mut ChaCha12Rng::seed_from_u64(0);
        let store = &MemoryBlockStore::new();
        let sharer_forest = &mut Rc::new(HamtForest::new_rsa_2048(rng));

        let sharer_root_did = "did:key:z6MkhaXgBZDvotDkL5257faiztiGiC2QtKLGpbnnEGta2doK";

        // Create directory to share.
        let sharer_dir = helper::create_sharer_dir(sharer_forest, store, rng)
            .await
            .unwrap();

        // Establish recipient exchange root.
        let (recipient_key, recipient_exchange_root) =
            helper::create_recipient_exchange_root(store).await.unwrap();

        // Construct access key from sharer's directory.
        let access_key = sharer_dir
            .as_node()
            .store(sharer_forest, store, rng)
            .await
            .unwrap();

        // Share access key with recipient.
        Share::<RsaPublicKey, _, _>::new(store, &access_key, 0)
            .by(Sharer {
                root_did: sharer_root_did.into(),
                forest: sharer_forest,
            })
            .to(Recipient {
                exchange_root: PublicLink::new(PublicNode::Dir(recipient_exchange_root)),
            })
            .finish()
            .await
            .unwrap();

        // Create share label.
        let share_label = sharer::create_share_name(
            0,
            sharer_root_did,
            &recipient_key
                .get_public_key()
                .get_public_key_modulus()
                .unwrap(),
            sharer_forest,
        );

        // Grab node using share label.
        let node = recipient::receive_share(&share_label, &recipient_key, sharer_forest, store)
            .await
            .unwrap();

        // Assert node is the same as the original.
        assert_eq!(node.as_dir().unwrap(), sharer_dir);
    }

    #[async_std::test]
    async fn serialized_share_payload_can_be_deserialized() {
        let rng = &mut ChaCha12Rng::seed_from_u64(0);
        let store = &MemoryBlockStore::new();
        let forest = &mut Rc::new(HamtForest::new_rsa_2048(rng));
        let dir =
            PrivateDirectory::new_and_store(&forest.empty_name(), Utc::now(), forest, store, rng)
                .await
                .unwrap();

        let access_key = dir.as_node().store(forest, store, rng).await.unwrap();

        let serialized = serde_ipld_dagcbor::to_vec(&access_key).unwrap();

        // Must be smaller than 190 bytes to fit within RSAES-OAEP limits
        assert!(serialized.len() <= 190);

        let deserialized: AccessKey = serde_ipld_dagcbor::from_slice(&serialized).unwrap();

        assert_eq!(access_key, deserialized);
    }

    #[async_std::test]
    async fn find_latest_share_counter_finds_highest_count() {
        let rng = &mut ChaCha12Rng::seed_from_u64(0);
        let store = &MemoryBlockStore::new();
        let forest = &mut Rc::new(HamtForest::new_rsa_2048(rng));

        let sharer_root_did = "did:key:z6MkhaXgBZDvotDkL5257faiztiGiC2QtKLGpbnnEGta2doK";

        // Establish recipient exchange root.
        let (_, recipient_exchange_root) =
            helper::create_recipient_exchange_root(store).await.unwrap();

        // Get exchange public key
        let recipient_exchange_key_cid = recipient_exchange_root
            .read(&["device1".into(), EXCHANGE_KEY_NAME.into()], store)
            .await
            .unwrap();

        let recipient_exchange_key = store.get_block(&recipient_exchange_key_cid).await.unwrap();

        // Test finding latest share before having shared
        let max_share_count_before = find_latest_share_counter(
            0,
            100,
            recipient_exchange_key.as_ref(),
            sharer_root_did,
            forest,
            store,
        )
        .await
        .unwrap();

        // We expect no shares to be found at all without sharing
        assert_eq!(max_share_count_before, None);

        // Create something to share access to.
        let dir =
            PrivateDirectory::new_and_store(&forest.empty_name(), Utc::now(), forest, store, rng)
                .await
                .unwrap();

        // Create the share
        let access_key = dir.as_node().store(forest, store, rng).await.unwrap();

        let expected_max_share_count = 5;

        for i in 0..=expected_max_share_count {
            sharer::share::<RsaPublicKey>(
                &access_key,
                i,
                sharer_root_did,
                forest,
                PublicLink::with_rc_dir(Rc::clone(&recipient_exchange_root)),
                store,
            )
            .await
            .unwrap();
        }

        let max_share_count = find_latest_share_counter(
            0,
            100,
            recipient_exchange_key.as_ref(),
            sharer_root_did,
            forest,
            store,
        )
        .await
        .unwrap();

        // We expect the count to be the latest share
        assert_eq!(max_share_count, Some(expected_max_share_count));
    }
}
