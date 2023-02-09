//! The WNFS Shared Private Data Extension Specification is a protocol that allows users to exchange private data
//! asynchronously when the other party may be offline using store-and-forward networks.
//! The protocol uses asymmetric encryption, with RSA public keys, and is versioned to support multiple versions on the same file system.
//! Public keys are widely distributed in a "exchange keys partition" and are grouped by device for easy selection by the sender.
//! Payloads containing pointers to the private data are stored in the "Private Forest" and are labeled with a name filter that includes the sender's and recipient's information,
//! as well as a counter.

use self::sharer::share;

use super::{ContentKey, ExchangeKey, PrivateRef, RevisionKey};
use crate::{
    private::PrivateForest, public::PublicLink, BlockStore, HashOutput, Hasher, NodeType,
    PrivateNode, ShareError,
};
use anyhow::{bail, Result};
use rand_core::RngCore;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use sha3::Sha3_256;
use std::{marker::PhantomData, rc::Rc};

//--------------------------------------------------------------------------------------------------
// Constants
//--------------------------------------------------------------------------------------------------

const EXCHANGE_KEY_NAME: &str = "v1.exchange_key";

//--------------------------------------------------------------------------------------------------
// Type Definitions
//--------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub struct Share<'a, K: ExchangeKey, S: BlockStore> {
    payload: &'a SharePayload,
    count: u64,
    sharer: Option<Sharer<'a, S>>,
    recipients: Vec<Recipient<'a, S>>,
    phantom: PhantomData<K>,
}

#[derive(Debug)]
pub struct Sharer<'a, S: BlockStore> {
    pub root_did: String,
    pub forest: Rc<PrivateForest>,
    pub store: &'a mut S,
}

#[derive(Debug)]
pub struct Recipient<'a, S: BlockStore> {
    pub exchange_root: PublicLink,
    pub store: &'a S,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SharePayload {
    Temporal(TemporalSharePointer),
    Snapshot(SnapshotSharePointer),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TemporalSharePointer {
    pub label: HashOutput,
    pub revision_key: RevisionKey,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SnapshotSharePointer {
    pub label: HashOutput,
    pub content_key: ContentKey,
}

#[derive(Debug, Serialize, Deserialize)]
struct TemporalSharePointerSerializable {
    r#type: NodeType,
    label: HashOutput,
    revision_key: RevisionKey,
}

#[derive(Debug, Serialize, Deserialize)]
struct SnapshotSharePointerSerializable {
    r#type: NodeType,
    label: HashOutput,
    content_key: ContentKey,
}

//--------------------------------------------------------------------------------------------------
// Implementations
//--------------------------------------------------------------------------------------------------

impl<'a, K: ExchangeKey, S: BlockStore> Share<'a, K, S> {
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
    pub fn by(&mut self, sharer: Sharer<'a, S>) -> &mut Self {
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
    pub async fn finish(&mut self) -> Result<Rc<PrivateForest>> {
        if matches!(&self.sharer, None) || matches!(self.recipients.len(), 0) {
            bail!(ShareError::NoSharerOrRecipients);
        }

        let sharer = self.sharer.take().unwrap();
        let recipients = std::mem::take(&mut self.recipients);

        let mut forest = sharer.forest;
        for recipient in recipients {
            forest = share::<K>(
                self.payload,
                self.count,
                &sharer.root_did,
                forest,
                sharer.store,
                recipient.exchange_root,
                recipient.store,
            )
            .await?;
        }

        Ok(forest)
    }
}

impl SharePayload {
    /// Create a share payload from a private fs node.
    pub async fn from_node(
        node: &PrivateNode,
        temporal: bool,
        forest: Rc<PrivateForest>,
        store: &mut impl BlockStore,
        rng: &mut impl RngCore,
    ) -> Result<(Self, Rc<PrivateForest>)> {
        let (payload, forest) = if temporal {
            let (ptr, forest) = TemporalSharePointer::from_node(node, forest, store, rng).await?;
            (Self::Temporal(ptr), forest)
        } else {
            let (ptr, forest) = SnapshotSharePointer::from_node(node, forest, store, rng).await?;
            (Self::Snapshot(ptr), forest)
        };

        Ok((payload, forest))
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
        forest: Rc<PrivateForest>,
        store: &mut impl BlockStore,
        rng: &mut impl RngCore,
    ) -> Result<(Self, Rc<PrivateForest>)> {
        let header = node.get_header();
        let revision_key = header.derive_revision_key();
        let saturated_name = header.get_saturated_name_with_key(&revision_key);
        let private_ref =
            PrivateRef::with_revision_key(Sha3_256::hash(&saturated_name), revision_key.clone());

        let forest = forest
            .put(saturated_name.clone(), &private_ref, node, store, rng)
            .await?;

        let payload = TemporalSharePointer {
            label: private_ref.saturated_name_hash,
            revision_key,
        };

        Ok((payload, forest))
    }
}

impl SnapshotSharePointer {
    /// Create a snapshot share pointer from a private fs node.
    pub async fn from_node(
        node: &PrivateNode,
        forest: Rc<PrivateForest>,
        store: &mut impl BlockStore,
        rng: &mut impl RngCore,
    ) -> Result<(Self, Rc<PrivateForest>)> {
        let header = node.get_header();
        let revision_key = header.derive_revision_key();
        let content_key = revision_key.derive_content_key();
        let saturated_name = header.get_saturated_name_with_key(&revision_key);
        let private_ref =
            PrivateRef::with_revision_key(Sha3_256::hash(&saturated_name), revision_key);

        let forest = forest
            .put(saturated_name, &private_ref, node, store, rng)
            .await?;

        let payload = SnapshotSharePointer {
            label: private_ref.saturated_name_hash,
            content_key,
        };

        Ok((payload, forest))
    }
}

impl Serialize for TemporalSharePointer {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        TemporalSharePointerSerializable {
            r#type: NodeType::TemporalSharePointer,
            label: self.label,
            revision_key: self.revision_key.clone(),
        }
        .serialize(serializer)
    }
}

impl Serialize for SnapshotSharePointer {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        SnapshotSharePointerSerializable {
            r#type: NodeType::SnapshotSharePointer,
            label: self.label,
            content_key: self.content_key.clone(),
        }
        .serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for SnapshotSharePointer {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let SnapshotSharePointerSerializable {
            label, content_key, ..
        } = SnapshotSharePointerSerializable::deserialize(deserializer)?;

        Ok(Self { label, content_key })
    }
}

impl<'de> Deserialize<'de> for TemporalSharePointer {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let TemporalSharePointerSerializable {
            label,
            revision_key,
            ..
        } = TemporalSharePointerSerializable::deserialize(deserializer)?;

        Ok(Self {
            label,
            revision_key,
        })
    }
}

//--------------------------------------------------------------------------------------------------
// Functions
//--------------------------------------------------------------------------------------------------

pub mod sharer {
    use super::{SharePayload, EXCHANGE_KEY_NAME};
    use crate::{
        dagcbor,
        private::{ExchangeKey, PrivateForest, PublicKeyModulus},
        public::PublicLink,
        BlockStore, Namefilter, PublicOpResult,
    };
    use anyhow::Result;
    use async_stream::try_stream;
    use futures::{Stream, StreamExt};
    use libipld::IpldCodec;
    use std::rc::Rc;

    /// Encrypts and shares a payload with multiple recipients using their
    /// exchange keys and stores the shares in the sharer's private forest.
    #[allow(clippy::too_many_arguments)]
    pub async fn share<K: ExchangeKey>(
        share_payload: &SharePayload,
        share_count: u64,
        sharer_root_did: &str,
        mut sharer_forest: Rc<PrivateForest>,
        sharer_store: &mut impl BlockStore,
        recipient_exchange_root: PublicLink,
        recipient_store: &impl BlockStore,
    ) -> Result<Rc<PrivateForest>> {
        let mut exchange_keys = fetch_exchange_keys(recipient_exchange_root, recipient_store).await;
        let encoded_payload = &dagcbor::encode(share_payload)?;

        while let Some(result) = exchange_keys.next().await {
            let public_key_modulus = result?;
            let exchange_key = K::from_exchange_key(&public_key_modulus)?;
            let encrypted_payload = exchange_key.encrypt(encoded_payload)?;
            let share_label = create_share_label(share_count, sharer_root_did, &public_key_modulus);

            let payload_cid = sharer_store
                .put_block(encrypted_payload, IpldCodec::Raw)
                .await?;

            sharer_forest = sharer_forest
                .put_encrypted(share_label, payload_cid, sharer_store)
                .await?;
        }

        Ok(sharer_forest)
    }

    /// Fetches the exchange keys of recipients using their exchange root, resolve the root_dir,
    /// search for the exchange key, and read the exchange key's cid in the recipient's store and
    /// yield the exchange key's value.
    pub async fn fetch_exchange_keys(
        recipient_exchange_root: PublicLink,
        recipient_store: &impl BlockStore,
    ) -> impl Stream<Item = Result<PublicKeyModulus>> + '_ {
        Box::pin(try_stream! {
            let root_dir = recipient_exchange_root
                .resolve_value(recipient_store)
                .await?
                .as_dir()?;

            let PublicOpResult { result: devices, mut root_dir } = root_dir.ls(&[], recipient_store).await?;
            for (device, _) in devices {
                let value = root_dir.ls(&[device.clone()], recipient_store).await?;
                root_dir = value.root_dir;

                for (name, _) in value.result {
                    if name == EXCHANGE_KEY_NAME {
                        let value = root_dir.read(&[device, name], recipient_store).await?;
                        root_dir = value.root_dir;
                        yield recipient_store.get_block(&value.result).await?.to_vec();
                        break
                    }
                }
            }
        })
    }

    /// Creates a unique label for a share by concatenating the sharer's root DID, the recipient's exchange key,
    /// and the share count, then applies a hash function to it and returns the resulting label.
    pub fn create_share_label(
        share_count: u64,
        sharer_root_did: &str,
        recipient_exchange_key: &[u8],
    ) -> Namefilter {
        let mut label = Namefilter::default();
        label.add(&sharer_root_did.as_bytes());
        label.add(&recipient_exchange_key);
        label.add(&share_count.to_le_bytes());
        label.saturate();
        label
    }
}

pub mod recipient {
    use crate::{
        dagcbor,
        private::{PrivateForest, PrivateKey, PrivateRef},
        BlockStore, Hasher, Namefilter, PrivateNode, ShareError,
    };
    use anyhow::{bail, Result};
    use sha3::Sha3_256;
    use std::rc::Rc;

    use super::{sharer, SharePayload, TemporalSharePointer};

    /// Checks if a share count is available.
    pub async fn find_share(
        share_count: u64,
        limit: u64,
        recipient_exchange_key: &[u8],
        sharer_root_did: &str,
        sharer_forest: &PrivateForest,
        sharer_store: &impl BlockStore,
    ) -> Result<Option<u64>> {
        for i in 0..limit {
            let share_label = sharer::create_share_label(
                share_count + i,
                sharer_root_did,
                recipient_exchange_key,
            );

            if sharer_forest
                .has(&Sha3_256::hash(&share_label), sharer_store)
                .await?
            {
                return Ok(Some(share_count + i));
            }
        }

        Ok(None)
    }

    /// Lets a recipient receive a share from a sharer using the sharer's forest and store.
    /// The recipient's private forest and store are used to store the share.
    pub async fn receive_share(
        share_label: Namefilter,
        recipient_key: &impl PrivateKey,
        sharer_forest: Rc<PrivateForest>,
        sharer_store: &mut impl BlockStore,
    ) -> Result<Option<PrivateNode>> {
        // Get cid to encrypted payload from sharer's forest using share_label
        let payload_cid = sharer_forest
            .get_encrypted(&Sha3_256::hash(&share_label), sharer_store)
            .await?
            .ok_or(ShareError::SharePayloadNotFound)?
            .first()
            .ok_or(ShareError::SharePayloadNotFound)?;

        // Get encrypted payload from sharer's store using cid
        let encrypted_payload = sharer_store.get_block(payload_cid).await?.to_vec();

        // Decrypt payload using recipient's private key and decode it.
        let payload: SharePayload = dagcbor::decode(&recipient_key.decrypt(&encrypted_payload)?)?;

        let SharePayload::Temporal(TemporalSharePointer {
            label,
            revision_key,
        }) =  payload else {
            // TODO(appcypher): We currently need both RevisionKey and ContentKey to decrypt a node.
            bail!(ShareError::UnsupportedSnapshotShareReceipt);
        };

        // Use decrypted payload to get cid to encrypted node in sharer's forest.
        let private_ref = PrivateRef::with_revision_key(label, revision_key);
        sharer_forest
            .get(&private_ref, PrivateForest::resolve_lowest, sharer_store)
            .await
    }
}

//--------------------------------------------------------------------------------------------------
// Tests
//--------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use chrono::Utc;

    use super::{recipient, sharer, Recipient, Share, SharePayload, Sharer};
    use crate::{
        dagcbor, private::RsaPublicKey, public::PublicLink, utils::test_setup, PrivateDirectory,
        PrivateOpResult, PublicNode,
    };

    mod helper {
        use crate::{
            private::{share::EXCHANGE_KEY_NAME, PrivateForest, RsaPrivateKey},
            BlockStore, Namefilter, PrivateDirectory, PrivateOpResult, PublicDirectory,
            PublicOpResult,
        };
        use anyhow::Result;
        use chrono::Utc;
        use libipld::IpldCodec;
        use rand_core::RngCore;
        use std::rc::Rc;

        pub(super) async fn create_sharer_dir(
            forest: Rc<PrivateForest>,
            store: &mut impl BlockStore,
            rng: &mut impl RngCore,
        ) -> Result<PrivateOpResult<()>> {
            let PrivateOpResult {
                root_dir, forest, ..
            } = PrivateDirectory::new_and_store(
                Namefilter::default(),
                Utc::now(),
                forest,
                store,
                rng,
            )
            .await?;

            root_dir
                .write(
                    &["text.txt".into()],
                    true,
                    Utc::now(),
                    b"Hello World!".to_vec(),
                    forest,
                    store,
                    rng,
                )
                .await
        }

        pub(super) async fn create_recipient_exchange_root(
            store: &mut impl BlockStore,
        ) -> Result<(RsaPrivateKey, Rc<PublicDirectory>)> {
            let key = RsaPrivateKey::new()?;
            let exchange_key = key.get_public_key().get_public_key_modulus()?;
            let exchange_key_cid = store.put_block(exchange_key, IpldCodec::Raw).await?;

            let PublicOpResult { root_dir, .. } = Rc::new(PublicDirectory::new(Utc::now()))
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
        let recipient_store = test_setup::init!(mut store);
        let (sharer_store, sharer_forest, rng) = test_setup::init!(mut store, forest, mut rng);
        let sharer_root_did = "did:key:z6MkhaXgBZDvotDkL5257faiztiGiC2QtKLGpbnnEGta2doK";

        // Create directory to share.
        let PrivateOpResult {
            root_dir: sharer_dir,
            forest: sharer_forest,
            ..
        } = helper::create_sharer_dir(sharer_forest, sharer_store, rng)
            .await
            .unwrap();

        // Establish recipient exchange root.
        let (recipient_key, recipient_exchange_root) =
            helper::create_recipient_exchange_root(recipient_store)
                .await
                .unwrap();

        // Construct share payload from sharer's directory.
        let (sharer_payload, sharer_forest) = SharePayload::from_node(
            &sharer_dir.as_node(),
            true,
            sharer_forest,
            sharer_store,
            rng,
        )
        .await
        .unwrap();

        // Share payload with recipient.
        let sharer_forest = Share::<RsaPublicKey, _>::new(&sharer_payload, 0)
            .by(Sharer {
                root_did: sharer_root_did.into(),
                store: sharer_store,
                forest: sharer_forest,
            })
            .to(Recipient {
                exchange_root: PublicLink::from(PublicNode::Dir(recipient_exchange_root)),
                store: recipient_store,
            })
            .finish()
            .await
            .unwrap();

        // Create share label.
        let share_label = sharer::create_share_label(
            0,
            sharer_root_did,
            &recipient_key
                .get_public_key()
                .get_public_key_modulus()
                .unwrap(),
        );

        // Grab node using share label.
        let node =
            recipient::receive_share(share_label, &recipient_key, sharer_forest, sharer_store)
                .await
                .unwrap()
                .unwrap();

        // Assert payload is the same as the original.
        assert_eq!(node.as_dir().unwrap(), sharer_dir);
    }

    #[async_std::test]
    async fn serialized_share_payload_can_be_deserialized() {
        async_std::task::block_on(async {
            let (forest, store, rng) = test_setup::init!(forest, mut store, mut rng);

            let PrivateOpResult {
                root_dir, forest, ..
            } = PrivateDirectory::new_and_store(Default::default(), Utc::now(), forest, store, rng)
                .await
                .unwrap();

            let (payload, _) =
                SharePayload::from_node(&root_dir.as_node(), true, forest, store, rng)
                    .await
                    .unwrap();

            let serialized = dagcbor::encode(&payload).unwrap();
            let deserialized: SharePayload = dagcbor::decode(&serialized).unwrap();

            assert_eq!(payload, deserialized);
        })
    }
}
