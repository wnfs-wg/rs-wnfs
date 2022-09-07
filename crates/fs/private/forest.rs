use std::rc::Rc;

use anyhow::Result;
use libipld::Cid;
use log::debug;

use crate::{BlockStore, HashOutput};

use super::{hamt::Hamt, namefilter::Namefilter, Key, PrivateNode, PrivateRef, Rng};

//--------------------------------------------------------------------------------------------------
// Type Definitions
//--------------------------------------------------------------------------------------------------

// TODO(appcypher): Change Cid to PrivateLink<PrivateNode>.
// TODO(appcypher): And eventually to BTreeSet<PrivateLink<PrivateNode>>.
pub type PrivateForest = Hamt<Namefilter, Cid>;

//--------------------------------------------------------------------------------------------------
// Implementations
//--------------------------------------------------------------------------------------------------

impl PrivateForest {
    /// Encrypts supplied bytes with a random nonce and AES key.
    pub(crate) fn encrypt<R: Rng>(key: &Key, data: &[u8], rng: &mut R) -> Result<Vec<u8>> {
        key.encrypt(&Key::generate_nonce(rng), data)
    }

    /// Sets a new value at the given key.
    #[inline]
    pub async fn set<B: BlockStore, R: Rng>(
        self: Rc<Self>,
        saturated_name: Namefilter,
        private_ref: &PrivateRef,
        value: &PrivateNode,
        store: &mut B,
        rng: &mut R,
    ) -> Result<Rc<Self>> {
        debug!("Private Forest Set: PrivateRef: {:?}", private_ref);

        // Serialize node to cbor.
        let cbor_bytes = value.serialize_to_cbor(rng)?;

        // Encrypt bytes with content key.
        let enc_bytes = Self::encrypt(&private_ref.content_key.0, &cbor_bytes, rng)?;

        // Store content section in blockstore and get Cid.
        let content_cid = store.put_block(enc_bytes, libipld::IpldCodec::Raw).await?;

        // Store header and Cid in root node.
        self.set_encrypted(saturated_name, content_cid, store).await
    }

    /// Gets the value at the given key.
    #[inline]
    pub async fn get<B: BlockStore>(
        &self,
        private_ref: &PrivateRef,
        store: &B,
    ) -> Result<Option<PrivateNode>> {
        debug!("Private Forest Get: PrivateRef: {:?}", private_ref);

        // Fetch Cid from root node.
        let cid = match self
            .get_encrypted(&private_ref.saturated_name_hash, store)
            .await?
        {
            Some(value) => value,
            None => return Ok(None),
        };

        // Fetch encrypted bytes from blockstore.
        let enc_bytes = store.get_block(cid).await?;

        // Decrypt bytes
        let cbor_bytes = private_ref.content_key.0.decrypt(&enc_bytes)?;

        // Deserialize bytes.
        Ok(Some(PrivateNode::deserialize_from_cbor(
            &cbor_bytes,
            &private_ref.ratchet_key,
        )?))
    }

    pub async fn has<B: BlockStore>(&self, private_ref: &PrivateRef, store: &B) -> Result<bool> {
        Ok(self
            .root
            .get_by_hash(&private_ref.saturated_name_hash, store)
            .await?
            .is_some())
    }

    /// Sets a new encrypted value at the given key.
    #[inline]
    pub async fn set_encrypted<B: BlockStore>(
        self: Rc<Self>,
        name: Namefilter,
        value: Cid,
        store: &mut B,
    ) -> Result<Rc<Self>> {
        let mut cloned = (*self).clone();
        cloned.root = self.root.set(name, value, store).await?;
        Ok(Rc::new(cloned))
    }

    /// Gets the encrypted value at the given key.
    #[inline]
    pub async fn get_encrypted<'b, B: BlockStore>(
        &'b self,
        name_hash: &HashOutput,
        store: &B,
    ) -> Result<Option<&'b Cid>> {
        self.root.get_by_hash(name_hash, store).await
    }

    /// Removes the encrypted value at the given key.
    pub async fn remove_encrypted<B: BlockStore>(
        self: Rc<Self>,
        name_hash: &HashOutput,
        store: &mut B,
    ) -> Result<(Rc<Self>, Option<Cid>)> {
        let mut cloned = (*self).clone();
        let (root, value) = self.root.remove_by_hash(name_hash, store).await?;
        cloned.root = root;
        Ok((Rc::new(cloned), value))
    }
}

// //--------------------------------------------------------------------------------------------------
// // Tests
// //--------------------------------------------------------------------------------------------------

#[cfg(test)]
mod hamt_store_tests {
    use std::rc::Rc;
    use proptest::test_runner::{TestRng, RngAlgorithm};
    use test_log::test;

    use chrono::Utc;

    use super::*;
    use crate::{private::PrivateDirectory, MemoryBlockStore};

    #[test(async_std::test)]
    async fn inserted_items_can_be_fetched() {
        let store = &mut MemoryBlockStore::new();
        let hamt = Rc::new(PrivateForest::new());
        let rng = &mut TestRng::deterministic_rng(RngAlgorithm::ChaCha);

        let dir = Rc::new(PrivateDirectory::new(
            Namefilter::default(),
            rng.random_bytes::<32>(),
            rng.random_bytes::<32>(),
            Utc::now(),
        ));

        let private_ref = dir.header.get_private_ref().unwrap();
        let saturated_name = dir.header.get_saturated_name();
        let private_node = PrivateNode::Dir(dir.clone());

        let hamt = hamt
            .set(saturated_name, &private_ref, &private_node, store, rng)
            .await
            .unwrap();

        let retrieved = hamt.get(&private_ref, store).await.unwrap().unwrap();

        assert_eq!(retrieved, private_node);
    }
}
