use std::marker::PhantomData;

use anyhow::Result;
use libipld::Cid;

use crate::{BlockStore, HashOutput};

use super::{Hamt, Key, Namefilter, PrivateNode, PrivateRef, NONCE_SIZE};

//--------------------------------------------------------------------------------------------------
// Type Definitions
//--------------------------------------------------------------------------------------------------

pub type EncryptedPrivateNode = (Option<Vec<u8>>, Cid); // TODO(appcypher): Use Link<Vec<u8>> instead of Cid?
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

    /// Decrypts supplied bytes with a random nonce and AES key.
    pub(crate) fn decrypt(key: &Key, cipher_text: &[u8]) -> Result<Vec<u8>> {
        key.decrypt(cipher_text)
    }

    /// Sets a new value at the given key.
    #[inline]
    pub async fn set(
        &mut self,
        name: Namefilter,
        private_ref: &PrivateRef,
        value: &PrivateNode,
    ) -> Result<()> {
        // Serialize header and content section as dag-cbor bytes.
        let (header_bytes, content_bytes) = value.serialize_as_cbor()?;

        // Encrypt header and content section.
        let enc_content_bytes = Self::encrypt(&private_ref.content_key, &content_bytes)?;
        let enc_header_bytes = if let Some(key) = &private_ref.ratchet_key.bare {
            Some(Self::encrypt(key, &header_bytes)?)
        } else {
            None
        };

        // Store content section in blockstore and get Cid.
        let content_cid = self
            .store
            .put_block(enc_content_bytes, libipld::IpldCodec::Raw)
            .await?;

        // Store header and Cid in root node.
        self.set_encrypted(name, (enc_header_bytes, content_cid))
            .await
    }

    /// Gets the value at the given key.
    #[inline]
    pub async fn get(&self, private_ref: &PrivateRef) -> Result<Option<PrivateNode>> {
        // Fetch encrypted header and Cid from root node.
        let (enc_header_bytes, content_cid) =
            match self.get_encrypted(&private_ref.saturated_name_hash).await? {
                Some(value) => value,
                None => return Ok(None),
            };

        // Fetch encrypted content section from blockstore.
        let enc_content_bytes = self.store.get_block(content_cid).await?;

        // Decrypt header and content section.
        let content_bytes = Self::decrypt(&private_ref.content_key, &enc_content_bytes)?;
        let header_bytes = if enc_header_bytes.is_some() & private_ref.ratchet_key.bare.is_some() {
            Some(Self::decrypt(
                private_ref.ratchet_key.bare.as_ref().unwrap(),
                enc_header_bytes.as_ref().unwrap(),
            )?)
        } else {
            None
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
