//! The WNFS Shared Private Data Extension Specification is a protocol that allows users to exchange private data
//! asynchronously when the other party may be offline using store-and-forward networks.
//! The protocol uses asymmetric encryption, with RSA public keys, and is versioned to support multiple versions on the same file system.
//! Public keys are widely distributed in a "exchange keys partition" and are grouped by device for easy selection by the sender.
//! Payloads containing pointers to the private data are stored in the "Private Forest" and are labeled with a name filter that includes the sender's and recipient's information,
//! as well as a counter.

use self::sharer::share;

use super::{ContentKey, PrivateRef, RevisionKey, RsaKeyPair};
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
pub struct Share<'a, R: RsaKeyPair, S: BlockStore> {
    payload: &'a SharePayload,
    count: usize,
    sharer: Option<Sharer<'a, S>>,
    recipients: Vec<Recipient<'a, S>>,
    phantom: PhantomData<R>,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SharePayload {
    Temporal(TemporalSharePointer),
    Snapshot(SnapshotSharePointer),
}

#[derive(Debug, Clone)]
pub struct TemporalSharePointer {
    pub label: HashOutput,
    pub revision_key: RevisionKey,
}

#[derive(Debug, Clone)]
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

impl<'a, R: RsaKeyPair, S: BlockStore> Share<'a, R, S> {
    ///  Creates a new instance of the Share with the given payload and count, and initializes the other fields as "None".
    pub fn new(payload: &'a SharePayload, count: usize) -> Self {
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
        if matches!((&self.sharer, self.recipients.len()), (None, 0)) {
            bail!(ShareError::NoSharerOrRecipients);
        }

        let sharer = self.sharer.take().unwrap();
        let recipients = std::mem::take(&mut self.recipients);

        let mut forest = sharer.forest;
        for recipient in recipients {
            forest = share::<R>(
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
        let header = node.get_header();
        let revision_key = header.derive_revision_key();
        let saturated_name = header.get_saturated_name_with_key(&revision_key);
        let private_ref =
            PrivateRef::with_revision_key(Sha3_256::hash(&saturated_name), revision_key.clone());

        let forest = forest
            .put(saturated_name.clone(), &private_ref, node, store, rng)
            .await?;

        let payload = if temporal {
            Self::Temporal(TemporalSharePointer {
                label: private_ref.saturated_name_hash,
                revision_key,
            })
        } else {
            Self::Snapshot(SnapshotSharePointer {
                label: private_ref.saturated_name_hash,
                content_key: revision_key.derive_content_key(),
            })
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
        private::{PrivateForest, PublicKeyModulus, RsaKeyPair},
        public::PublicLink,
        BlockStore, FsError, Namefilter, PublicOpResult,
    };
    use anyhow::Result;
    use async_stream::try_stream;
    use futures::{Stream, StreamExt};
    use libipld::IpldCodec;
    use std::rc::Rc;

    /// Encrypts and shares a payload with multiple recipients using their
    /// exchange keys and stores the shares in the sharer's private forest.
    #[allow(clippy::too_many_arguments)]
    pub async fn share<R: RsaKeyPair>(
        share_payload: &SharePayload,
        share_count: usize,
        sharer_root_did: &str,
        mut sharer_forest: Rc<PrivateForest>,
        sharer_store: &mut impl BlockStore,
        recipient_exchange_root: PublicLink,
        recipient_store: &impl BlockStore,
    ) -> Result<Rc<PrivateForest>> {
        let mut exchange_keys = fetch_exchange_keys(recipient_exchange_root, recipient_store).await;
        let encoded_payload = &dagcbor::encode(share_payload)?;

        while let Some(result) = exchange_keys.next().await {
            let exchange_key_modulus = result?;
            let exchange_key = R::from_public_key_modulus(&exchange_key_modulus)?;
            let encrypted_payload = exchange_key.encrypt(encoded_payload)?;
            let share_label =
                create_share_label(share_count, sharer_root_did, &exchange_key_modulus);

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

            let PublicOpResult { result: devices, mut root_dir  } = root_dir.ls(&[], recipient_store).await?;
            for _ in devices {
                let PublicOpResult { result: entries, root_dir: root } = root_dir.ls(&[], recipient_store).await?;
                root_dir = root;
                for (name, _) in entries {
                    if name == EXCHANGE_KEY_NAME {
                        root_dir
                            .lookup_node(&name, recipient_store)
                            .await?
                            .ok_or(FsError::NotFound)?
                            .as_file()?;

                        let PublicOpResult { result: cid, root_dir: root } = root_dir.read(&[name], recipient_store).await?;
                        root_dir = root;
                        yield recipient_store.get_block(&cid).await?.to_vec();
                        break
                    }
                }
            }
        })
    }

    /// Creates a unique label for a share by concatenating the sharer's root DID, the recipient's exchange key,
    /// and the share count, then applies a hash function to it and returns the resulting label.
    fn create_share_label(
        share_count: usize,
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
        private::{PrivateForest, PrivateRef, RsaKeyPair},
        BlockStore, Hasher, Namefilter, PrivateNode, ShareError,
    };
    use anyhow::{bail, Result};
    use sha3::Sha3_256;
    use std::rc::Rc;

    use super::{SharePayload, TemporalSharePointer};

    /// Lets a recipient receive a share from a sharer using the sharer's forest and store.
    /// The recipient's private forest and store are used to store the share.
    pub async fn receive_share(
        share_label: Namefilter,
        recipient_key: &impl RsaKeyPair,
        recipient_store: &mut impl BlockStore,
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
        let encrypted_payload = recipient_store.get_block(payload_cid).await?.to_vec();
        let payload: SharePayload = dagcbor::decode(&recipient_key.decrypt(&encrypted_payload)?)?;

        let (label, revision_key) = match payload {
            SharePayload::Temporal(TemporalSharePointer {
                label,
                revision_key,
            }) => (label, revision_key),
            // TODO(appcypher): We currently need both RevisionKey and ContentKey to decrypt a node.
            _ => bail!(ShareError::UnsupportedSnapshotShareReceipt),
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
    mod utils {
        // ...
    }

    #[async_std::test]
    async fn test1() {
        // let forest = Rc::new(PrivateForest::new());
        // let rng = &mut TestRng::deterministic_rng(RngAlgorithm::ChaCha);
        // let file = Rc::new(PrivateFile::new(Namefilter::default(), Utc::now(), rng)); // TODO(appcypher)
        // let exchange_root = Rc::new(PublicDirectory::new(Utc::now())); // TODO(appcypher)

        // let (payload, forest) = SharePayload::from_node(
        //     &PrivateNode::File(file),
        //     false,
        //     forest,
        //     &mut MemoryBlockStore::default(),
        //     rng,
        // )
        // .await
        // .unwrap();

        // let _ = Share::<RsaKeys, _>::new(&payload, 0)
        //     .by(Sharer {
        //         root_did: String::from("did:key:"),
        //         store: &mut MemoryBlockStore::default(),
        //         forest,
        //     })
        //     .to(Recipient {
        //         exchange_root: PublicLink::from(PublicNode::Dir(exchange_root)),
        //         store: &MemoryBlockStore::default(),
        //     })
        //     .finish()
        //     .await
        //     .unwrap();
    }

    #[async_std::test]
    async fn serialized_public_file_can_be_deserialized() {
        // let original_file = PublicFile::new(Utc::now(), Cid::default());

        // let serialized_file = dagcbor::encode(&original_file).unwrap();
        // let deserialized_file: PublicFile = dagcbor::decode(serialized_file.as_ref()).unwrap();

        // assert_eq!(deserialized_file, original_file);
    }
}
