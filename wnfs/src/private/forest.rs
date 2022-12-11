use super::{hamt::Hamt, namefilter::Namefilter, ChangeType, Key, PrivateNode, PrivateRef};
use crate::{BlockStore, HashOutput, Hasher};
use anyhow::Result;
use libipld::Cid;
use log::debug;
use rand_core::RngCore;
use std::{collections::BTreeSet, fmt, rc::Rc};

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
    /// use chrono::Utc;
    /// use rand::thread_rng;
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
    ///     let private_ref = &dir.header.get_private_ref();
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
    /// use chrono::Utc;
    /// use rand::thread_rng;
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
    ///     let private_ref = &dir.header.get_private_ref();
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
    /// use chrono::Utc;
    /// use rand::thread_rng;
    /// use sha3::Sha3_256;
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
    ///     let private_ref = &dir.header.get_private_ref();
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

        let mut forest = Rc::try_unwrap(self).unwrap_or_else(|rc| (*rc).clone());
        forest.root = forest.root.set(name, values, store).await?;
        Ok(Rc::new(forest))
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

impl<H> Hamt<Namefilter, BTreeSet<Cid>, H>
where
    H: Hasher + fmt::Debug + Clone + 'static,
{
    /// Merges a private forest with another. If there is a conflict with values, it combines the two values in the final merge node
    ///
    /// # Examples
    ///
    /// ```
    /// use std::rc::Rc;
    /// use chrono::{Utc, Days};
    /// use rand::{thread_rng, Rng};
    /// use wnfs::{
    ///     private::{PrivateForest, PrivateRef}, PrivateNode,
    ///     BlockStore, MemoryBlockStore, Namefilter, PrivateDirectory, PrivateOpResult,
    /// };
    ///
    /// #[async_std::main]
    /// async fn main() {
    ///     let store = &mut MemoryBlockStore::default();
    ///     let rng = &mut thread_rng();
    ///
    ///     let ratchet_seed = rng.gen::<[u8; 32]>();
    ///     let inumber = rng.gen::<[u8; 32]>();
    ///
    ///     let main_forest = Rc::new(PrivateForest::new());
    ///     let root_dir = Rc::new(PrivateDirectory::with_seed(
    ///         Namefilter::default(),
    ///         Utc::now(),
    ///         ratchet_seed,
    ///         inumber
    ///     ));
    ///     let main_forest = main_forest
    ///         .put(
    ///             root_dir.header.get_saturated_name(),
    ///             &root_dir.header.get_private_ref(),
    ///             &PrivateNode::Dir(Rc::clone(&root_dir)),
    ///             store,
    ///             rng
    ///         )
    ///         .await
    ///         .unwrap();
    ///
    ///     let other_forest = Rc::new(PrivateForest::new());
    ///     let root_dir = Rc::new(PrivateDirectory::with_seed(
    ///         Namefilter::default(),
    ///         Utc::now().checked_add_days(Days::new(1)).unwrap(),
    ///         ratchet_seed,
    ///         inumber
    ///     ));
    ///     let other_forest = other_forest
    ///         .put(
    ///             root_dir.header.get_saturated_name(),
    ///             &root_dir.header.get_private_ref(),
    ///             &PrivateNode::Dir(Rc::clone(&root_dir)),
    ///             store,
    ///             rng
    ///         )
    ///         .await
    ///         .unwrap();
    ///
    ///     let merge_forest = main_forest.merge(&other_forest, store).await.unwrap();
    ///
    ///     assert_eq!(
    ///         2,
    ///         merge_forest
    ///             .root
    ///             .get(&root_dir.header.get_saturated_name(), store)
    ///             .await
    ///             .unwrap()
    ///             .unwrap()
    ///             .len()
    ///     );
    /// }
    /// ```
    pub async fn merge<B: BlockStore>(&self, other: &Self, store: &mut B) -> Result<Self> {
        let kv_changes = self.kv_diff(other, None, store).await?;

        let mut merge_node = Rc::clone(&self.root);
        for change in kv_changes {
            match change.r#type {
                ChangeType::Remove => {
                    merge_node = merge_node
                        .set(change.key, change.other_value.unwrap(), store)
                        .await?;
                }
                ChangeType::Modify => {
                    let mut merge_values = self
                        .root
                        .get(&change.key, store)
                        .await?
                        .cloned()
                        .unwrap_or_default();

                    merge_values.extend(
                        other
                            .root
                            .get(&change.key, store)
                            .await?
                            .cloned()
                            .unwrap_or_default(),
                    );

                    merge_node = merge_node.set(change.key, merge_values, store).await?;
                }
                _ => (),
            }
        }

        Ok(Self::with_root(merge_node))
    }
}

// //--------------------------------------------------------------------------------------------------
// // Tests
// //--------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        private::{HashNibbles, Node, PrivateDirectory},
        utils::test_setup,
        MemoryBlockStore,
    };
    use chrono::Utc;
    use helper::*;
    use proptest::test_runner::{RngAlgorithm, TestRng};
    use std::rc::Rc;

    mod helper {
        use crate::{utils, HashOutput, Hasher, Namefilter};
        use lazy_static::lazy_static;
        use libipld::{Cid, Multihash};
        use rand::{thread_rng, RngCore};

        lazy_static! {
            pub(super) static ref HASH_KV_PAIRS: Vec<(HashOutput, Namefilter, Cid)> = vec![
                (
                    utils::make_digest(&[0xA0]),
                    generate_saturated_name_hash(&mut thread_rng()),
                    generate_cid(&mut thread_rng())
                ),
                (
                    utils::make_digest(&[0xA3]),
                    generate_saturated_name_hash(&mut thread_rng()),
                    generate_cid(&mut thread_rng())
                ),
                (
                    utils::make_digest(&[0xA7]),
                    generate_saturated_name_hash(&mut thread_rng()),
                    generate_cid(&mut thread_rng())
                ),
                (
                    utils::make_digest(&[0xAC]),
                    generate_saturated_name_hash(&mut thread_rng()),
                    generate_cid(&mut thread_rng())
                ),
                (
                    utils::make_digest(&[0xAE]),
                    generate_saturated_name_hash(&mut thread_rng()),
                    generate_cid(&mut thread_rng())
                ),
            ];
        }

        #[derive(Debug, Clone)]
        pub(super) struct MockHasher;
        impl Hasher for MockHasher {
            fn hash<K: AsRef<[u8]>>(key: &K) -> HashOutput {
                HASH_KV_PAIRS
                    .iter()
                    .find(|(_, v, _)| key.as_ref() == v.as_ref())
                    .unwrap()
                    .0
            }
        }

        pub(super) fn generate_saturated_name_hash(rng: &mut impl RngCore) -> Namefilter {
            let mut namefilter = Namefilter::default();
            namefilter.add(&utils::get_random_bytes::<32>(rng));
            namefilter.saturate();
            namefilter
        }

        pub(super) fn generate_cid(rng: &mut impl RngCore) -> Cid {
            let bytes = {
                let mut tmp = [0u8; 10];
                let (a, b) = tmp.split_at_mut(2);
                a.copy_from_slice(&[0x55, 0x08]);
                b.copy_from_slice(&utils::get_random_bytes::<8>(rng));
                tmp
            };

            Cid::new_v1(0x55, Multihash::from_bytes(&bytes).unwrap())
        }
    }

    #[async_std::test]
    async fn inserted_items_can_be_fetched() {
        let store = &mut MemoryBlockStore::new();
        let forest = Rc::new(PrivateForest::new());
        let rng = &mut TestRng::deterministic_rng(RngAlgorithm::ChaCha);

        let dir = Rc::new(PrivateDirectory::new(
            Namefilter::default(),
            Utc::now(),
            rng,
        ));

        let private_ref = dir.header.get_private_ref();
        let saturated_name = dir.header.get_saturated_name();
        let private_node = PrivateNode::Dir(dir.clone());

        let forest = forest
            .put(saturated_name, &private_ref, &private_node, store, rng)
            .await
            .unwrap();

        let retrieved = forest
            .get(&private_ref, PrivateForest::resolve_lowest, store)
            .await
            .unwrap()
            .unwrap();

        assert_eq!(retrieved, private_node);
    }

    #[async_std::test]
    async fn inserted_multivalue_items_can_be_fetched_with_bias() {
        let store = &mut MemoryBlockStore::new();
        let forest = Rc::new(PrivateForest::new());
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

        let private_ref = dir.header.get_private_ref();
        let private_ref_conflict = dir_conflict.header.get_private_ref();
        let saturated_name = dir.header.get_saturated_name();
        let saturated_name_conflict = dir_conflict.header.get_saturated_name();
        let private_node = PrivateNode::Dir(dir.clone());
        let private_node_conflict = PrivateNode::Dir(dir_conflict.clone());

        assert_eq!(saturated_name_conflict, saturated_name);

        // Put the original node in the private forest
        let forest = forest
            .put(saturated_name, &private_ref, &private_node, store, rng)
            .await
            .unwrap();

        // Put the conflicting node in the private forest at the same key
        let forest = forest
            .put(
                saturated_name_conflict,
                &private_ref_conflict,
                &private_node_conflict,
                store,
                rng,
            )
            .await
            .unwrap();

        let ciphertext_cids = forest
            .get_encrypted(&private_ref.saturated_name_hash, store)
            .await
            .unwrap()
            .unwrap();

        // We expect there to be a conflict, a multivalue
        assert_eq!(ciphertext_cids.len(), 2);

        let conflict_cid = ciphertext_cids.iter().last().unwrap();

        let retrieved = forest
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

    #[async_std::test]
    async fn can_merge_nodes_with_different_structure_and_modified_changes() {
        let (store, rng) = test_setup::init!(mut store, mut rng);

        // A node that adds the first 3 pairs of HASH_KV_PAIRS.
        let mut other_node = Rc::new(Node::<_, _, MockHasher>::default());
        for (digest, k, v) in HASH_KV_PAIRS.iter().take(3) {
            other_node = other_node
                .set_value(
                    &mut HashNibbles::new(digest),
                    k.clone(),
                    BTreeSet::from([*v]),
                    store,
                )
                .await
                .unwrap();
        }

        // Another node that keeps the first pair, modify the second pair, removes the third pair, and adds the fourth and fifth pair.
        let mut main_node = Rc::new(Node::<_, _, MockHasher>::default());
        main_node = main_node
            .set_value(
                &mut HashNibbles::new(&HASH_KV_PAIRS[0].0),
                HASH_KV_PAIRS[0].1.clone(),
                BTreeSet::from([HASH_KV_PAIRS[0].2]),
                store,
            )
            .await
            .unwrap();

        let new_cid = generate_cid(rng);
        main_node = main_node
            .set_value(
                &mut HashNibbles::new(&HASH_KV_PAIRS[1].0),
                HASH_KV_PAIRS[1].1.clone(),
                BTreeSet::from([new_cid]),
                store,
            )
            .await
            .unwrap();

        for (digest, k, v) in HASH_KV_PAIRS.iter().skip(3).take(2) {
            main_node = main_node
                .set_value(
                    &mut HashNibbles::new(digest),
                    k.clone(),
                    BTreeSet::from([*v]),
                    store,
                )
                .await
                .unwrap();
        }

        let main_forest = Hamt::<Namefilter, BTreeSet<Cid>, _>::with_root(main_node);
        let other_forest = Hamt::<Namefilter, BTreeSet<Cid>, _>::with_root(other_node);

        let merge_forest = main_forest.merge(&other_forest, store).await.unwrap();

        for (i, (digest, _, v)) in HASH_KV_PAIRS.iter().take(5).enumerate() {
            let retrieved = merge_forest.root.get_by_hash(digest, store).await.unwrap();
            if i != 1 {
                assert_eq!(retrieved.unwrap(), &BTreeSet::from([*v]));
            } else {
                // The second pair should contain two merged Cids.
                assert!(retrieved.unwrap().contains(&new_cid));
                assert!(retrieved.unwrap().contains(&HASH_KV_PAIRS[1].2));
            }
        }
    }
}

#[cfg(test)]
mod proptests {
    use test_strategy::proptest;

    #[proptest]
    fn merge_associativity() {}

    #[proptest]
    fn merge_commutativity() {}

    #[proptest]
    fn merge_idempotency() {}
}
