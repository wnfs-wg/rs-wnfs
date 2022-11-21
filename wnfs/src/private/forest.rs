use std::{collections::BTreeSet, rc::Rc};

use anyhow::Result;
use libipld::Cid;
use log::debug;
use rand_core::RngCore;

use crate::{BlockStore, HashOutput};

use super::{hamt::Hamt, namefilter::Namefilter, Key, PrivateNode, PrivateRef};

//--------------------------------------------------------------------------------------------------
// Type Definitions
//--------------------------------------------------------------------------------------------------

/// PrivateForest is a HAMT that stores CIDs of encrypted private nodes keyed by saturated namefilters.
///
/// On insert, nodes are serialized to DAG CBOR and encrypted with their private refs and then stored in
/// an accompanying block store. And on lookup, the nodes are decrypted and deserialized with the same private
/// refs.
///
/// It is called a forest because it is a collection of file trees.
///
/// # Examples
///
/// ```
/// use wnfs::private::PrivateForest;
///
/// let forest = PrivateForest::new();
///
/// println!("{:?}", forest);
/// ```
// TODO(appcypher): Change Cid to PrivateLink<PrivateNode>.
pub type PrivateForest = Hamt<Namefilter, BTreeSet<Cid>>;

//--------------------------------------------------------------------------------------------------
// Implementations
//--------------------------------------------------------------------------------------------------

impl PrivateForest {
    /// Encrypts supplied bytes with a random nonce and AES key.
    pub(crate) fn encrypt<R: RngCore>(key: &Key, data: &[u8], rng: &mut R) -> Result<Vec<u8>> {
        key.encrypt(&Key::generate_nonce(rng), data)
    }

    /// Puts a new value at the given key.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::rc::Rc;
    ///
    /// use chrono::Utc;
    /// use rand::thread_rng;
    ///
    /// use wnfs::{
    ///     private::{PrivateForest, PrivateRef}, PrivateNode,
    ///     BlockStore, MemoryBlockStore, Namefilter, PrivateDirectory, PrivateOpResult,
    /// };
    ///
    /// #[async_std::main]
    /// async fn main() {
    ///     let store = &mut MemoryBlockStore::default();
    ///     let rng = &mut thread_rng();
    ///     let forest = Rc::new(PrivateForest::new());
    ///     let dir = Rc::new(PrivateDirectory::new(
    ///         Namefilter::default(),
    ///         Utc::now(),
    ///         rng,
    ///     ));
    ///
    ///     let private_ref = &dir.header.get_private_ref().unwrap();
    ///     let name = dir.header.get_saturated_name();
    ///     let node = PrivateNode::Dir(dir);
    ///
    ///     let forest = forest.put(name, private_ref, &node, store, rng).await.unwrap();
    ///     assert_eq!(forest.get(private_ref, PrivateForest::resolve_lowest, store).await.unwrap(), Some(node));
    /// }
    /// ```
    pub async fn put<B: BlockStore, R: RngCore>(
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
        self.put_encrypted(saturated_name, content_cid, store).await
    }

    /// Gets the value at the given key.
    ///
    /// The `resolve_bias` argument helps to pick a CID in case
    /// there are multiple CIDs at this time-step.
    ///
    /// Reasonable values for `resolve_bias` include
    /// - `PrivateForest::resolve_lowest`
    /// - `PrivateForest::resolve_single`
    /// - Using external information to pick the 'best' CID, e.g. `PrivateForest::resolve_one_of(expected_set)`
    ///
    /// When `resolve_bias` returns `None`, then this function returns `Ok(None)` as well,
    /// if it returns `Ok`.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::rc::Rc;
    ///
    /// use chrono::Utc;
    /// use rand::thread_rng;
    ///
    /// use wnfs::{
    ///     private::{PrivateForest, PrivateRef}, PrivateNode,
    ///     BlockStore, MemoryBlockStore, Namefilter, PrivateDirectory, PrivateOpResult,
    /// };
    ///
    /// #[async_std::main]
    /// async fn main() {
    ///     let store = &mut MemoryBlockStore::default();
    ///     let rng = &mut thread_rng();
    ///     let forest = Rc::new(PrivateForest::new());
    ///     let dir = Rc::new(PrivateDirectory::new(
    ///         Namefilter::default(),
    ///         Utc::now(),
    ///         rng,
    ///     ));
    ///
    ///     let private_ref = &dir.header.get_private_ref().unwrap();
    ///     let name = dir.header.get_saturated_name();
    ///     let node = PrivateNode::Dir(dir);
    ///
    ///     let forest = forest.put(name, private_ref, &node, store, rng).await.unwrap();
    ///     assert_eq!(forest.get(private_ref, PrivateForest::resolve_lowest, store).await.unwrap(), Some(node));
    /// }
    /// ```
    pub async fn get<B: BlockStore>(
        &self,
        private_ref: &PrivateRef,
        resolve_bias: impl FnOnce(&BTreeSet<Cid>) -> Option<&Cid>,
        store: &B,
    ) -> Result<Option<PrivateNode>> {
        debug!("Private Forest Get: PrivateRef: {:?}", private_ref);

        // Fetch Cid from root node.
        let cids = match self
            .get_encrypted(&private_ref.saturated_name_hash, store)
            .await?
        {
            Some(cids) => cids,
            None => return Ok(None),
        };

        let cid = match resolve_bias(cids) {
            Some(cid) => cid,
            None => return Ok(None),
        };

        // Fetch encrypted bytes from blockstore.
        let enc_bytes = store.get_block(cid).await?;

        // Decrypt bytes
        let cbor_bytes = private_ref.content_key.0.decrypt(&enc_bytes)?;

        // Deserialize bytes.
        Ok(Some(PrivateNode::deserialize_from_cbor(
            &cbor_bytes,
            &private_ref.revision_key,
        )?))
    }

    /// Checks that a value with the given saturated name hash key exists.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::rc::Rc;
    ///
    /// use chrono::Utc;
    /// use rand::thread_rng;
    /// use sha3::Sha3_256;
    ///
    /// use wnfs::{
    ///     private::{PrivateForest, PrivateRef}, PrivateNode,
    ///     BlockStore, MemoryBlockStore, Namefilter, PrivateDirectory, PrivateOpResult, Hasher
    /// };
    ///
    /// #[async_std::main]
    /// async fn main() {
    ///     let store = &mut MemoryBlockStore::default();
    ///     let rng = &mut thread_rng();
    ///     let forest = Rc::new(PrivateForest::new());
    ///     let dir = Rc::new(PrivateDirectory::new(
    ///         Namefilter::default(),
    ///         Utc::now(),
    ///         rng,
    ///     ));
    ///
    ///     let private_ref = &dir.header.get_private_ref().unwrap();
    ///     let name = dir.header.get_saturated_name();
    ///     let node = PrivateNode::Dir(dir);
    ///     let forest = forest.put(name.clone(), private_ref, &node, store, rng).await.unwrap();
    ///
    ///     let name_hash = &Sha3_256::hash(&name.as_bytes());
    ///
    ///     assert!(forest.has(name_hash, store).await.unwrap());
    /// }
    /// ```
    pub async fn has<B: BlockStore>(
        &self,
        saturated_name_hash: &HashOutput,
        store: &B,
    ) -> Result<bool> {
        Ok(self
            .root
            .get_by_hash(saturated_name_hash, store)
            .await?
            .is_some())
    }

    /// Sets a new encrypted value at the given key.
    pub async fn put_encrypted<B: BlockStore>(
        self: Rc<Self>,
        name: Namefilter,
        value: Cid,
        store: &mut B,
    ) -> Result<Rc<Self>> {
        // TODO(matheus23): This iterates the path in the HAMT twice.
        // We could consider implementing something like upsert instead.
        let mut values = self
            .root
            .get(&name, store)
            .await?
            .cloned()
            .unwrap_or_default();
        values.insert(value);

        let mut hamt = Rc::try_unwrap(self).unwrap_or_else(|rc| (*rc).clone());
        hamt.root = hamt.root.set(name, values, store).await?;
        Ok(Rc::new(hamt))
    }

    /// Gets the encrypted value at the given key.
    #[inline]
    pub async fn get_encrypted<'b, B: BlockStore>(
        &'b self,
        name_hash: &HashOutput,
        store: &B,
    ) -> Result<Option<&'b BTreeSet<Cid>>> {
        self.root.get_by_hash(name_hash, store).await
    }

    /// Removes the encrypted value at the given key.
    pub async fn remove_encrypted<B: BlockStore>(
        self: Rc<Self>,
        name_hash: &HashOutput,
        store: &mut B,
    ) -> Result<(Rc<Self>, Option<BTreeSet<Cid>>)> {
        let mut cloned = (*self).clone();
        let (root, pair) = cloned.root.remove_by_hash(name_hash, store).await?;
        cloned.root = root;
        Ok((Rc::new(cloned), pair.map(|p| p.value)))
    }

    /// Convenience function for usage within `PrivateForest.get`.
    /// Will return the first element in given BTreeSet.
    pub fn resolve_lowest(set: &BTreeSet<Cid>) -> Option<&Cid> {
        set.iter().next()
    }

    /// Convenience function for usage within `PrivateForest.get`.
    /// Will return a CID in given set, if the set has exactly one element.
    pub fn resolve_single(set: &BTreeSet<Cid>) -> Option<&Cid> {
        match &*set.iter().collect::<Vec<&Cid>>() {
            &[cid] => Some(cid),
            _ => None,
        }
    }

    /// Convenience function builder for `PrivateForest.get`.
    /// The returned function will return the first CID in `set`
    /// that also appears in `one_of`.
    pub fn resolve_one_of<F>(
        one_of: &BTreeSet<Cid>,
    ) -> impl Fn(&BTreeSet<Cid>) -> Option<&Cid> + '_ {
        |set: &BTreeSet<Cid>| set.iter().find(|cid| one_of.contains(cid))
    }
}

// //--------------------------------------------------------------------------------------------------
// // Tests
// //--------------------------------------------------------------------------------------------------

#[cfg(test)]
mod hamt_store_tests {
    use proptest::test_runner::{RngAlgorithm, TestRng};
    use std::rc::Rc;
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
            Utc::now(),
            rng,
        ));

        let private_ref = dir.header.get_private_ref().unwrap();
        let saturated_name = dir.header.get_saturated_name();
        let private_node = PrivateNode::Dir(dir.clone());

        let hamt = hamt
            .put(saturated_name, &private_ref, &private_node, store, rng)
            .await
            .unwrap();

        let retrieved = hamt
            .get(&private_ref, PrivateForest::resolve_lowest, store)
            .await
            .unwrap()
            .unwrap();

        assert_eq!(retrieved, private_node);
    }

    #[test(async_std::test)]
    async fn inserted_multivalue_items_can_be_fetched_with_bias() {
        let store = &mut MemoryBlockStore::new();
        let hamt = Rc::new(PrivateForest::new());
        let rng = &mut TestRng::deterministic_rng(RngAlgorithm::ChaCha);

        let dir = Rc::new(PrivateDirectory::new(
            Namefilter::default(),
            Utc::now(),
            rng,
        ));

        let dir_conflict = {
            let mut dir = (*dir).clone();
            dir.metadata.upsert_mtime(Utc::now());
            Rc::new(dir)
        };

        let private_ref = dir.header.get_private_ref().unwrap();
        let private_ref_conflict = dir_conflict.header.get_private_ref().unwrap();
        let saturated_name = dir.header.get_saturated_name();
        let saturated_name_conflict = dir_conflict.header.get_saturated_name();
        let private_node = PrivateNode::Dir(dir.clone());
        let private_node_conflict = PrivateNode::Dir(dir_conflict.clone());

        assert_eq!(saturated_name_conflict, saturated_name);

        // Put the original node in the HAMT
        let hamt = hamt
            .put(saturated_name, &private_ref, &private_node, store, rng)
            .await
            .unwrap();

        // Put the conflicting node in the HAMT at the same key
        let hamt = hamt
            .put(
                saturated_name_conflict,
                &private_ref_conflict,
                &private_node_conflict,
                store,
                rng,
            )
            .await
            .unwrap();

        let ciphertext_cids = hamt
            .get_encrypted(&private_ref.saturated_name_hash, store)
            .await
            .unwrap()
            .unwrap();

        // We expect there to be a conflict, a multivalue
        assert_eq!(ciphertext_cids.len(), 2);

        let conflict_cid = ciphertext_cids.iter().last().unwrap();

        let retrieved = hamt
            .get(
                &private_ref,
                PrivateForest::resolve_one_of::<fn(&BTreeSet<Cid>) -> Option<&Cid>>(
                    &BTreeSet::from([*conflict_cid]),
                ),
                store,
            )
            .await
            .unwrap()
            .unwrap();

        assert_eq!(retrieved, private_node_conflict);
    }
}
