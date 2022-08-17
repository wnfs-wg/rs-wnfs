use std::marker::PhantomData;

use anyhow::Result;
use libipld::Cid;
use log::debug;

use crate::{BlockStore, HashOutput};

use super::{hamt::Hamt, namefilter::Namefilter, Key, PrivateNode, PrivateRef, NONCE_SIZE};

//--------------------------------------------------------------------------------------------------
// Type Definitions
//--------------------------------------------------------------------------------------------------

pub type EncryptedPrivateNode = (Option<Vec<u8>>, Cid); // TODO(appcypher): Change to PrivateLink<PrivateNode>.
pub type PrivateRoot = Hamt<Namefilter, EncryptedPrivateNode>;

#[derive(Debug)]
pub struct HamtStore<'a, B: BlockStore, R: Rng> {
    pub root: PrivateRoot,
    pub store: &'a mut B,
    pub rng: PhantomData<R>,
}

pub trait Rng {
    fn random_bytes<const N: usize>() -> [u8; N];
}

//--------------------------------------------------------------------------------------------------
// Implementations
//--------------------------------------------------------------------------------------------------

impl<'a, B: BlockStore, R: Rng> HamtStore<'a, B, R> {
    /// Creates a new HamtStore.
    pub fn new(store: &'a mut B) -> Self {
        Self {
            root: Hamt::default(),
            rng: PhantomData,
            store,
        }
    }

    /// Encrypts supplied bytes with a random nonce and AES key.
    pub(crate) fn encrypt(key: &Key, data: &[u8]) -> Result<Vec<u8>> {
        key.encrypt(&R::random_bytes::<NONCE_SIZE>(), data)
    }

    /// Sets a new value at the given key.
    #[inline]
    pub async fn set(
        &mut self,
        saturated_name: Namefilter,
        private_ref: &PrivateRef,
        value: &PrivateNode,
    ) -> Result<()> {
        debug!("hamt store set: PrivateRef: {:?}", private_ref);
        // Serialize header and content section as dag-cbor bytes.
        let (header_bytes, content_bytes) = value.serialize_as_cbor()?;

        // Encrypt header and content section.
        let enc_content_bytes = Self::encrypt(&private_ref.content_key.0, &content_bytes)?;
        let enc_header_bytes = Some(Self::encrypt(&private_ref.ratchet_key.0, &header_bytes)?);

        // Store content section in blockstore and get Cid.
        let content_cid = self
            .store
            .put_block(enc_content_bytes, libipld::IpldCodec::Raw)
            .await?;

        // Store header and Cid in root node.
        self.set_encrypted(saturated_name, (enc_header_bytes, content_cid))
            .await
    }

    /// Gets the value at the given key.
    #[inline]
    pub async fn get(&self, private_ref: &PrivateRef) -> Result<Option<PrivateNode>> {
        debug!("hamt store get: PrivateRef: {:?}", private_ref);

        // Fetch encrypted header and Cid from root node.
        let (enc_header_bytes, content_cid) =
            match self.get_encrypted(&private_ref.saturated_name_hash).await? {
                Some(value) => value,
                None => return Ok(None),
            };

        // Fetch encrypted content section from blockstore.
        let enc_content_bytes = self.store.get_block(content_cid).await?;

        // Decrypt header and content section.
        let content_bytes = private_ref.content_key.0.decrypt(&enc_content_bytes)?;
        let header_bytes = match enc_header_bytes {
            Some(enc_header_bytes) => Some(private_ref.ratchet_key.0.decrypt(enc_header_bytes)?),
            _ => None,
        };

        // Deserialize header and content section.
        Ok(Some(PrivateNode::deserialize_from_cbor(
            &header_bytes,
            &content_bytes,
        )?))
    }

    /// Sets a new encrypted value at the given key.
    #[inline]
    pub async fn set_encrypted(
        &mut self,
        name: Namefilter,
        value: EncryptedPrivateNode,
    ) -> Result<()> {
        let root = self.root.root.set(name, value, self.store).await?;
        self.root.root = root;
        Ok(())
    }

    /// Gets the encrypted value at the given key.
    #[inline]
    pub async fn get_encrypted<'b>(
        &'b self,
        name_hash: &HashOutput,
    ) -> Result<Option<&'b EncryptedPrivateNode>> {
        self.root.root.get_by_hash(name_hash, self.store).await
    }

    /// Removes the encrypted value at the given key.
    pub async fn remove_encrypted(
        &mut self,
        name_hash: &HashOutput,
    ) -> Result<Option<EncryptedPrivateNode>> {
        let (root, value) = self.root.root.remove_by_hash(name_hash, self.store).await?;
        self.root.root = root;
        Ok(value)
    }
}

//--------------------------------------------------------------------------------------------------
// Tests
//--------------------------------------------------------------------------------------------------

#[cfg(test)]
mod hamt_store_tests {
    use std::rc::Rc;
    use test_log::test;

    use chrono::Utc;

    use super::*;
    use crate::{private::PrivateDirectory, utils::Rand, MemoryBlockStore};

    #[test(async_std::test)]
    async fn inserted_items_can_be_fetched() {
        let store = &mut MemoryBlockStore::new();
        let hamt = &mut HamtStore::<_, Rand>::new(store);

        let dir = Rc::new(PrivateDirectory::new(
            Namefilter::default(),
            Rand::random_bytes::<32>(),
            Rand::random_bytes::<32>(),
            Utc::now(),
        ));

        let private_ref = dir.header.get_private_ref().unwrap();
        let saturated_name = dir.header.get_saturated_name();
        let private_node = PrivateNode::Dir(dir.clone());

        hamt.set(saturated_name, &private_ref, &private_node)
            .await
            .unwrap();
        let retrieved = hamt.get(&private_ref).await.unwrap().unwrap();

        assert_eq!(retrieved, private_node);
    }
}