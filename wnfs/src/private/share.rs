//! The WNFS Shared Private Data Extension Specification is a protocol that allows users to exchange private data
//! asynchronously when the other party may be offline using store-and-forward networks.
//! The protocol uses asymmetric encryption, with RSA public keys, and is versioned to support multiple versions on the same file system.
//! Public keys are widely distributed in a "exchange keys partition" and are grouped by device for easy selection by the sender.
//! Payloads containing pointers to the private data are stored in the "Private Forest" and are labeled with a name filter that includes the sender's and recipient's information,
//! as well as a counter.

use self::sharer::share;
use super::{forest::traits::PrivateForest, ExchangeKey, PrivateNode, SnapshotKey, TemporalKey};
use crate::{error::ShareError, public::PublicLink};
use anyhow::{bail, Result};
use libipld::Cid;
use rand_core::CryptoRngCore;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
use wnfs_common::{BlockStore, HashOutput};

//--------------------------------------------------------------------------------------------------
// Constants
//--------------------------------------------------------------------------------------------------

const EXCHANGE_KEY_NAME: &str = "v1.exchange_key";

//--------------------------------------------------------------------------------------------------
// Type Definitions
//--------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub struct Share<'a, K: ExchangeKey, S: BlockStore, F: PrivateForest> {
    payload: &'a SharePayload,
    count: u64,
    sharer: Option<Sharer<'a, S, F>>,
    recipients: Vec<Recipient<'a, S>>,
    phantom: PhantomData<K>,
}

#[derive(Debug)]
pub struct Sharer<'a, S: BlockStore, F: PrivateForest> {
    pub root_did: String,
    pub forest: &'a mut F,
    pub store: &'a mut S,
}

#[derive(Debug)]
pub struct Recipient<'a, S: BlockStore> {
    pub exchange_root: PublicLink,
    pub store: &'a S,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SharePayload {
    #[serde(rename = "wnfs/share/temporal")]
    Temporal(TemporalSharePointer),
    #[serde(rename = "wnfs/share/snapshot")]
    Snapshot(SnapshotSharePointer),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TemporalSharePointer {
    #[serde(serialize_with = "crate::utils::serialize_byte_slice32")]
    #[serde(deserialize_with = "crate::utils::deserialize_byte_slice32")]
    pub label: HashOutput,
    #[serde(rename = "contentCid")]
    pub content_cid: Cid,
    #[serde(rename = "temporalKey")]
    pub temporal_key: TemporalKey,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SnapshotSharePointer {
    #[serde(serialize_with = "crate::utils::serialize_byte_slice32")]
    #[serde(deserialize_with = "crate::utils::deserialize_byte_slice32")]
    pub label: HashOutput,
    #[serde(rename = "contentCid")]
    pub content_cid: Cid,
    #[serde(rename = "snapshotKey")]
    pub snapshot_key: SnapshotKey,
}

//--------------------------------------------------------------------------------------------------
// Implementations
//--------------------------------------------------------------------------------------------------

impl<'a, K: ExchangeKey, S: BlockStore, F: PrivateForest> Share<'a, K, S, F> {
    ///  Creates a new instance of the Share with the given payload and count, and initializes the other fields as "None".
    pub fn new(payload: &'a SharePayload, count: u64) -> Self {
        Self {
            payload,
            count,
            sharer: None,
            recipients: Vec::new(),
            phantom: PhantomData,
        }
    }

    /// Sets the sharer field.
    pub fn by(&mut self, sharer: Sharer<'a, S, F>) -> &mut Self {
        self.sharer = Some(sharer);
        self
    }

    /// Takes a vector of recipients as an argument, and adds it to the existing recipients of the Share struct.
    pub fn with(&mut self, recipients: Vec<Recipient<'a, S>>) -> &mut Self {
        self.recipients.extend(recipients);
        self
    }

    /// Takes a recipient as an argument and adds it to the existing recipients of the Share struct.
    pub fn to(&mut self, recipient: Recipient<'a, S>) -> &mut Self {
        self.recipients.push(recipient);
        self
    }

    /// Performs the sharing operation with the previously set sharer and recipients.
    /// It takes the payload, sharer, and recipients, and performs the share operation,
    /// encrypts the payload and stores it in the sharer's private forest.
    pub async fn finish(&mut self) -> Result<()> {
        if self.sharer.is_none() || self.recipients.is_empty() {
            bail!(ShareError::NoSharerOrRecipients);
        }

        let sharer = self.sharer.take().unwrap();
        let recipients = std::mem::take(&mut self.recipients);

        for recipient in recipients {
            share::<K>(
                self.payload,
                self.count,
                &sharer.root_did,
                sharer.forest,
                sharer.store,
                recipient.exchange_root,
                recipient.store,
            )
            .await?;
        }

        Ok(())
    }
}

impl SharePayload {
    /// Create a share payload from a private fs node.
    pub async fn from_node(
        node: &PrivateNode,
        temporal: bool,
        forest: &mut impl PrivateForest,
        store: &impl BlockStore,
        rng: &mut impl CryptoRngCore,
    ) -> Result<Self> {
        let payload = if temporal {
            let ptr = TemporalSharePointer::from_node(node, forest, store, rng).await?;
            Self::Temporal(ptr)
        } else {
            let ptr = SnapshotSharePointer::from_node(node, forest, store, rng).await?;
            Self::Snapshot(ptr)
        };

        Ok(payload)
    }

    pub fn get_label(&self) -> HashOutput {
        match self {
            Self::Temporal(payload) => payload.label,
            Self::Snapshot(payload) => payload.label,
        }
    }
}

impl TemporalSharePointer {
    /// Create a temporal share pointer from a private fs node.
    pub async fn from_node(
        node: &PrivateNode,
        forest: &mut impl PrivateForest,
        store: &impl BlockStore,
        rng: &mut impl CryptoRngCore,
    ) -> Result<Self> {
        let private_ref = node.store(forest, store, rng).await?;

        let payload = TemporalSharePointer {
            label: private_ref.revision_name_hash,
            content_cid: private_ref.content_cid,
            temporal_key: private_ref.temporal_key,
        };

        Ok(payload)
    }
}

impl SnapshotSharePointer {
    /// Create a snapshot share pointer from a private fs node.
    pub async fn from_node(
        node: &PrivateNode,
        forest: &mut impl PrivateForest,
        store: &impl BlockStore,
        rng: &mut impl CryptoRngCore,
    ) -> Result<Self> {
        let private_ref = node.store(forest, store, rng).await?;

        let payload = Self {
            label: private_ref.revision_name_hash,
            content_cid: private_ref.content_cid,
            snapshot_key: private_ref.temporal_key.derive_snapshot_key(),
        };

        Ok(payload)
    }
}

//--------------------------------------------------------------------------------------------------
// Functions
//--------------------------------------------------------------------------------------------------

pub mod sharer {
    use super::{SharePayload, EXCHANGE_KEY_NAME};
    use crate::{
        private::{forest::traits::PrivateForest, ExchangeKey, PublicKeyModulus},
        public::PublicLink,
    };
    use anyhow::Result;
    use async_stream::try_stream;
    use futures::{Stream, StreamExt};
    use libipld::IpldCodec;
    use wnfs_common::BlockStore;
    use wnfs_nameaccumulator::{Name, NameSegment};

    // TODO(appcypher): When ref mut is eliminated in BlockStore trait, make this into one BlockStore argument.
    /// Encrypts and shares a payload with multiple recipients using their
    /// exchange keys and stores the shares in the sharer's private forest.
    #[allow(clippy::too_many_arguments)]
    pub async fn share<K: ExchangeKey>(
        share_payload: &SharePayload,
        share_count: u64,
        sharer_root_did: &str,
        sharer_forest: &mut impl PrivateForest,
        sharer_store: &impl BlockStore,
        recipient_exchange_root: PublicLink,
        recipient_store: &impl BlockStore,
    ) -> Result<()> {
        let mut exchange_keys = fetch_exchange_keys(recipient_exchange_root, recipient_store).await;
        let encoded_payload = &serde_ipld_dagcbor::to_vec(share_payload)?;

        while let Some(result) = exchange_keys.next().await {
            let public_key_modulus = result?;
            let exchange_key = K::from_modulus(&public_key_modulus).await?;
            let encrypted_payload = exchange_key.encrypt(encoded_payload).await?;
            let share_label = create_share_name(
                share_count,
                sharer_root_did,
                &public_key_modulus,
                sharer_forest,
            );

            let payload_cid = sharer_store
                .put_block(encrypted_payload, IpldCodec::Raw)
                .await?;

            sharer_forest
                .put_encrypted(&share_label, Some(payload_cid), sharer_store)
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
    use super::{sharer, SharePayload, TemporalSharePointer};
    use crate::{
        error::ShareError,
        private::{forest::traits::PrivateForest, PrivateKey, PrivateNode, PrivateRef},
    };
    use anyhow::{bail, Result};
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
        let payload_cid = sharer_forest
            .get_encrypted_by_hash(&Sha3_256::hash(&share_label.as_accumulator(setup)), store)
            .await?
            .ok_or(ShareError::SharePayloadNotFound)?
            .first()
            .ok_or(ShareError::SharePayloadNotFound)?;

        // Get encrypted payload from store using cid
        let encrypted_payload = store.get_block(payload_cid).await?.to_vec();

        // Decrypt payload using recipient's private key and decode it.
        let payload: SharePayload =
            serde_ipld_dagcbor::from_slice(&recipient_key.decrypt(&encrypted_payload).await?)?;

        let SharePayload::Temporal(TemporalSharePointer {
            label,
            content_cid,
            temporal_key,
        }) = payload else {
            // TODO(appcypher): We currently need both TemporalKey to decrypt a node.
            bail!(ShareError::UnsupportedSnapshotShareReceipt);
        };

        // Use decrypted payload to get cid to encrypted node in sharer's forest.
        let private_ref = PrivateRef::with_temporal_key(label, temporal_key, content_cid);
        PrivateNode::load(&private_ref, sharer_forest, store, None).await
    }
}

//--------------------------------------------------------------------------------------------------
// Tests
//--------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::{
        recipient::{self, find_latest_share_counter},
        sharer, Recipient, Share, SharePayload, Sharer, EXCHANGE_KEY_NAME,
    };
    use crate::{
        private::{
            forest::{hamt::HamtForest, traits::PrivateForest},
            PrivateDirectory, RsaPublicKey,
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
        use libipld::IpldCodec;
        use rand_core::CryptoRngCore;
        use std::rc::Rc;
        use wnfs_common::BlockStore;

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
            let exchange_key_cid = store.put_block(exchange_key, IpldCodec::Raw).await?;

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
        let recipient_store = &mut MemoryBlockStore::default();
        let sharer_store = &mut MemoryBlockStore::default();
        let sharer_forest = &mut Rc::new(HamtForest::new_rsa_2048(rng));

        let sharer_root_did = "did:key:z6MkhaXgBZDvotDkL5257faiztiGiC2QtKLGpbnnEGta2doK";

        // Create directory to share.
        let sharer_dir = helper::create_sharer_dir(sharer_forest, sharer_store, rng)
            .await
            .unwrap();

        // Establish recipient exchange root.
        let (recipient_key, recipient_exchange_root) =
            helper::create_recipient_exchange_root(recipient_store)
                .await
                .unwrap();

        // Construct share payload from sharer's directory.
        let sharer_payload = SharePayload::from_node(
            &sharer_dir.as_node(),
            true,
            sharer_forest,
            sharer_store,
            rng,
        )
        .await
        .unwrap();

        // Share payload with recipient.
        Share::<RsaPublicKey, _, _>::new(&sharer_payload, 0)
            .by(Sharer {
                root_did: sharer_root_did.into(),
                store: sharer_store,
                forest: sharer_forest,
            })
            .to(Recipient {
                exchange_root: PublicLink::new(PublicNode::Dir(recipient_exchange_root)),
                store: recipient_store,
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
        let node =
            recipient::receive_share(&share_label, &recipient_key, sharer_forest, sharer_store)
                .await
                .unwrap();

        // Assert payload is the same as the original.
        assert_eq!(node.as_dir().unwrap(), sharer_dir);
    }

    #[async_std::test]
    async fn serialized_share_payload_can_be_deserialized() {
        let rng = &mut ChaCha12Rng::seed_from_u64(0);
        let store = &mut MemoryBlockStore::default();
        let forest = &mut Rc::new(HamtForest::new_rsa_2048(rng));
        let dir =
            PrivateDirectory::new_and_store(&forest.empty_name(), Utc::now(), forest, store, rng)
                .await
                .unwrap();

        let payload = SharePayload::from_node(&dir.as_node(), true, forest, store, rng)
            .await
            .unwrap();

        let serialized = serde_ipld_dagcbor::to_vec(&payload).unwrap();

        // Must be smaller than 190 bytes to fit within RSAES-OAEP limits
        assert!(serialized.len() <= 190);

        let deserialized: SharePayload = serde_ipld_dagcbor::from_slice(&serialized).unwrap();

        assert_eq!(payload, deserialized);
    }

    #[async_std::test]
    async fn find_latest_share_counter_finds_highest_count() {
        let rng = &mut ChaCha12Rng::seed_from_u64(0);
        let sharer_store = &mut MemoryBlockStore::default();
        let recipient_store = &mut MemoryBlockStore::default();
        let forest = &mut Rc::new(HamtForest::new_rsa_2048(rng));

        let sharer_root_did = "did:key:z6MkhaXgBZDvotDkL5257faiztiGiC2QtKLGpbnnEGta2doK";

        // Establish recipient exchange root.
        let (_, recipient_exchange_root) = helper::create_recipient_exchange_root(recipient_store)
            .await
            .unwrap();

        // Get exchange public key
        let recipient_exchange_key_cid = recipient_exchange_root
            .read(
                &["device1".into(), EXCHANGE_KEY_NAME.into()],
                recipient_store,
            )
            .await
            .unwrap();

        let recipient_exchange_key = recipient_store
            .get_block(&recipient_exchange_key_cid)
            .await
            .unwrap();

        // Test finding latest share before having shared
        let max_share_count_before = find_latest_share_counter(
            0,
            100,
            recipient_exchange_key.as_ref(),
            sharer_root_did,
            forest,
            sharer_store,
        )
        .await
        .unwrap();

        // We expect no shares to be found at all without sharing
        assert_eq!(max_share_count_before, None);

        // Create something to share access to.
        let dir = PrivateDirectory::new_and_store(
            &forest.empty_name(),
            Utc::now(),
            forest,
            sharer_store,
            rng,
        )
        .await
        .unwrap();

        // Create the share
        let payload = SharePayload::from_node(&dir.as_node(), true, forest, sharer_store, rng)
            .await
            .unwrap();

        let expected_max_share_count = 5;

        for i in 0..=expected_max_share_count {
            sharer::share::<RsaPublicKey>(
                &payload,
                i,
                sharer_root_did,
                forest,
                sharer_store,
                PublicLink::with_rc_dir(Rc::clone(&recipient_exchange_root)),
                recipient_store,
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
            sharer_store,
        )
        .await
        .unwrap();

        // We expect the count to be the latest share
        assert_eq!(max_share_count, Some(expected_max_share_count));
    }
}
