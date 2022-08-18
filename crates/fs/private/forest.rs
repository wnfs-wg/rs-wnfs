use anyhow::Result;
use libipld::Cid;
use log::debug;

use crate::{BlockStore, HashOutput};

use super::{hamt::Hamt, namefilter::Namefilter, Key, PrivateNode, PrivateRef, NONCE_SIZE};

//--------------------------------------------------------------------------------------------------
// Type Definitions
//--------------------------------------------------------------------------------------------------

pub type EncryptedPrivateNode = (Option<Vec<u8>>, Cid); // TODO(appcypher): Change to PrivateLink<PrivateNode>.
pub type PrivateForest = Hamt<Namefilter, EncryptedPrivateNode>;

pub trait Rng {
    fn random_bytes<const N: usize>(&self) -> [u8; N];
}

//--------------------------------------------------------------------------------------------------
// Implementations
//--------------------------------------------------------------------------------------------------

impl PrivateForest {
    /// Encrypts supplied bytes with a random nonce and AES key.
    pub(crate) fn encrypt<R: Rng>(key: &Key, data: &[u8], rng: &R) -> Result<Vec<u8>> {
        key.encrypt(&rng.random_bytes::<NONCE_SIZE>(), data)
    }

    /// Sets a new value at the given key.
    #[inline]
    pub async fn set<B: BlockStore, R: Rng>(
        &mut self,
        saturated_name: Namefilter,
        private_ref: &PrivateRef,
        value: &PrivateNode,
        store: &mut B,
        rng: &R,
    ) -> Result<()> {
        debug!("hamt store set: PrivateRef: {:?}", private_ref);

        // Serialize header and content section as dag-cbor bytes.
        let (header_bytes, content_bytes) = value.serialize_as_cbor()?;

        // Encrypt header and content section.
        let enc_content_bytes = Self::encrypt(&private_ref.content_key.0, &content_bytes, rng)?;
        let enc_header_bytes = Some(Self::encrypt(
            &private_ref.ratchet_key.0,
            &header_bytes,
            rng,
        )?);

        // Store content section in blockstore and get Cid.
        let content_cid = store
            .put_block(enc_content_bytes, libipld::IpldCodec::Raw)
            .await?;

        // Store header and Cid in root node.
        self.set_encrypted(saturated_name, (enc_header_bytes, content_cid), store)
            .await
    }

    /// Gets the value at the given key.
    #[inline]
    pub async fn get<B: BlockStore>(
        &self,
        private_ref: &PrivateRef,
        store: &B,
    ) -> Result<Option<PrivateNode>> {
        debug!("hamt store get: PrivateRef: {:?}", private_ref);

        // Fetch encrypted header and Cid from root node.
        let (enc_header_bytes, content_cid) = match self
            .get_encrypted(&private_ref.saturated_name_hash, store)
            .await?
        {
            Some(value) => value,
            None => return Ok(None),
        };

        // Fetch encrypted content section from blockstore.
        let enc_content_bytes = store.get_block(content_cid).await?;

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
    pub async fn set_encrypted<B: BlockStore>(
        &mut self,
        name: Namefilter,
        value: EncryptedPrivateNode,
        store: &mut B,
    ) -> Result<()> {
        let root = self.root.set(name, value, store).await?;
        self.root = root;
        Ok(())
    }

    /// Gets the encrypted value at the given key.
    #[inline]
    pub async fn get_encrypted<'b, B: BlockStore>(
        &'b self,
        name_hash: &HashOutput,
        store: &B,
    ) -> Result<Option<&'b EncryptedPrivateNode>> {
        self.root.get_by_hash(name_hash, store).await
    }

    /// Removes the encrypted value at the given key.
    pub async fn remove_encrypted<B: BlockStore>(
        &mut self,
        name_hash: &HashOutput,
        store: &mut B,
    ) -> Result<Option<EncryptedPrivateNode>> {
        let (root, value) = self.root.remove_by_hash(name_hash, store).await?;
        self.root = root;
        Ok(value)
    }
}

// //--------------------------------------------------------------------------------------------------
// // Tests
// //--------------------------------------------------------------------------------------------------

#[cfg(test)]
mod hamt_store_tests {
    use std::rc::Rc;
    use test_log::test;

    use chrono::Utc;

    use super::*;
    use crate::{private::PrivateDirectory, utils::TestRng, MemoryBlockStore};

    #[test(async_std::test)]
    async fn inserted_items_can_be_fetched() {
        let store = &mut MemoryBlockStore::new();
        let hamt = &mut PrivateForest::new();
        let rng = &TestRng();

        let dir = Rc::new(PrivateDirectory::new(
            Namefilter::default(),
            rng.random_bytes::<32>(),
            rng.random_bytes::<32>(),
            Utc::now(),
        ));

        let private_ref = dir.header.get_private_ref().unwrap();
        let saturated_name = dir.header.get_saturated_name();
        let private_node = PrivateNode::Dir(dir.clone());

        hamt.set(saturated_name, &private_ref, &private_node, store, rng)
            .await
            .unwrap();

        let retrieved = hamt.get(&private_ref, store).await.unwrap().unwrap();

        assert_eq!(retrieved, private_node);
    }
}
