use super::{
    hamt::{self, Hamt},
    namefilter::Namefilter,
    PrivateNode, RevisionRef,
};
use crate::{AesError, BlockStore, HashOutput, Hasher, Link};
use anyhow::Result;
use async_stream::stream;
use futures::Stream;
use libipld::Cid;
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
/// It is called a forest because it can store a collection of file trees.
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
    ///     let forest = &mut Rc::new(PrivateForest::new());
    ///     let dir = Rc::new(PrivateDirectory::new(
    ///         Namefilter::default(),
    ///         Utc::now(),
    ///         rng,
    ///     ));
    ///
    ///     let node = PrivateNode::Dir(dir);
    ///     let private_ref = node.store(forest, store, rng).await.unwrap();
    ///
    ///     assert!(forest.has(&private_ref.saturated_name_hash, store).await.unwrap());
    /// }
    /// ```
    pub async fn has(
        &self,
        saturated_name_hash: &HashOutput,
        store: &impl BlockStore,
    ) -> Result<bool> {
        Ok(self
            .root
            .get_by_hash(saturated_name_hash, store)
            .await?
            .is_some())
    }

    /// Adds new encrypted values at the given key.
    pub async fn put_encrypted(
        self: &mut Rc<Self>,
        name: Namefilter,
        values: impl IntoIterator<Item = Cid>,
        store: &mut impl BlockStore,
    ) -> Result<()> {
        // TODO(matheus23): This iterates the path in the HAMT twice.
        // We could consider implementing something like upsert instead.
        // Or some kind of "cursor".
        let mut cids = self
            .root
            .get(&name, store)
            .await?
            .cloned()
            .unwrap_or_default();

        cids.extend(values);

        Rc::make_mut(self).root.set(name, cids, store).await?;
        Ok(())
    }

    /// Gets the encrypted values at the given key.
    #[inline]
    pub async fn get_encrypted<'b>(
        &'b self,
        name_hash: &HashOutput,
        store: &impl BlockStore,
    ) -> Result<Option<&'b BTreeSet<Cid>>> {
        self.root.get_by_hash(name_hash, store).await
    }

    /// Removes the encrypted value at the given key.
    pub async fn remove_encrypted(
        self: &mut Rc<Self>,
        name_hash: &HashOutput,
        store: &mut impl BlockStore,
    ) -> Result<Option<BTreeSet<Cid>>> {
        let pair = Rc::make_mut(self)
            .root
            .remove_by_hash(name_hash, store)
            .await?;
        Ok(pair.map(|p| p.value))
    }

    /// Returns a stream of all private nodes that could be decrypted at given revision.
    ///
    /// The stream of results is ordered by CID.
    ///
    /// Each item in the resulting stream represents an instance of a concurrent write.
    pub fn get_multivalue<'a>(
        &'a self,
        revision: &'a RevisionRef,
        store: &'a impl BlockStore,
    ) -> impl Stream<Item = Result<PrivateNode>> + 'a {
        Box::pin(stream! {
            match self
                .get_encrypted(&revision.saturated_name_hash, store)
                .await
            {
                Ok(Some(cids)) => {
                    for cid in cids {
                        match PrivateNode::from_cid(*cid, &revision.temporal_key, store).await {
                            Ok(node) => yield Ok(node),
                            Err(e) if matches!(e.downcast_ref::<AesError>(), Some(_)) => {
                                // we likely matched a PrivateNodeHeader instead of a PrivateNode.
                                // we skip it
                            }
                            // If something else goes wrong, we tell the user about it
                            Err(e) => yield Err(e)
                        }
                    }
                }
                Ok(None) => {},
                Err(e) => yield Err(e),
            }
        })
    }
}

impl<H> Hamt<Namefilter, BTreeSet<Cid>, H>
where
    H: Hasher + fmt::Debug + Clone + 'static,
{
    /// Merges a private forest with another. If there is a conflict with the values,they are union
    /// combined into a single value in the final merge node
    ///
    /// # Examples
    ///
    /// ```
    /// use std::rc::Rc;
    /// use chrono::{Utc, Days};
    /// use rand::{thread_rng, Rng};
    /// use futures::StreamExt;
    /// use wnfs::{
    ///     private::{PrivateForest, RevisionRef}, PrivateNode,
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
    ///     let main_forest = &mut Rc::new(PrivateForest::new());
    ///     let root_dir = Rc::new(PrivateDirectory::with_seed(
    ///         Namefilter::default(),
    ///         Utc::now(),
    ///         ratchet_seed,
    ///         inumber
    ///     ));
    ///     root_dir.store(main_forest, store, rng).await.unwrap();
    ///
    ///     let other_forest = &mut Rc::new(PrivateForest::new());
    ///     let root_dir = Rc::new(PrivateDirectory::with_seed(
    ///         Namefilter::default(),
    ///         Utc::now().checked_add_days(Days::new(1)).unwrap(),
    ///         ratchet_seed,
    ///         inumber
    ///     ));
    ///     root_dir.store(other_forest, store, rng).await.unwrap();
    ///
    ///     let merge_forest = main_forest.merge(other_forest, store).await.unwrap();
    ///
    ///     let revision_ref = RevisionRef::with_seed(
    ///         Namefilter::default(),
    ///         ratchet_seed,
    ///         inumber
    ///     );
    ///
    ///     assert_eq!(
    ///         2,
    ///         merge_forest
    ///             .get_multivalue(&revision_ref, store)
    ///             .collect::<Vec<anyhow::Result<PrivateNode>>>()
    ///             .await
    ///             .into_iter()
    ///             .filter_map(|result| result.ok())
    ///             .collect::<Vec<PrivateNode>>()
    ///             .len()
    ///     );
    /// }
    /// ```
    pub async fn merge(&self, other: &Self, store: &mut impl BlockStore) -> Result<Self> {
        let merge_node = hamt::merge(
            Link::from(Rc::clone(&self.root)),
            Link::from(Rc::clone(&other.root)),
            |a, b| Ok(a.union(b).cloned().collect()),
            store,
        )
        .await?;

        Ok(Self {
            version: self.version.clone(),
            root: merge_node,
        })
    }
}

//--------------------------------------------------------------------------------------------------
// Tests
//--------------------------------------------------------------------------------------------------

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
        use libipld::{Cid, Multihash};
        use once_cell::sync::Lazy;
        use rand::{thread_rng, RngCore};

        pub(super) static HASH_KV_PAIRS: Lazy<Vec<(HashOutput, Namefilter, Cid)>> =
            Lazy::new(|| {
                vec![
                    (
                        utils::make_digest(&[0xA0]),
                        generate_saturated_name_hash(&mut thread_rng()),
                        generate_cid(&mut thread_rng()),
                    ),
                    (
                        utils::make_digest(&[0xA3]),
                        generate_saturated_name_hash(&mut thread_rng()),
                        generate_cid(&mut thread_rng()),
                    ),
                    (
                        utils::make_digest(&[0xA7]),
                        generate_saturated_name_hash(&mut thread_rng()),
                        generate_cid(&mut thread_rng()),
                    ),
                    (
                        utils::make_digest(&[0xAC]),
                        generate_saturated_name_hash(&mut thread_rng()),
                        generate_cid(&mut thread_rng()),
                    ),
                    (
                        utils::make_digest(&[0xAE]),
                        generate_saturated_name_hash(&mut thread_rng()),
                        generate_cid(&mut thread_rng()),
                    ),
                ]
            });

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
        let forest = &mut Rc::new(PrivateForest::new());
        let rng = &mut TestRng::deterministic_rng(RngAlgorithm::ChaCha);

        let dir = Rc::new(PrivateDirectory::new(
            Namefilter::default(),
            Utc::now(),
            rng,
        ));

        let private_node = PrivateNode::Dir(dir.clone());
        let private_ref = private_node.store(forest, store, rng).await.unwrap();
        let retrieved = PrivateNode::load(&private_ref, forest, store)
            .await
            .unwrap();

        assert_eq!(retrieved, private_node);
    }

    #[async_std::test]
    async fn multivalue_conflict_can_be_fetched_individually() {
        let store = &mut MemoryBlockStore::new();
        let forest = &mut Rc::new(PrivateForest::new());
        let rng = &mut TestRng::deterministic_rng(RngAlgorithm::ChaCha);

        let dir = Rc::new(PrivateDirectory::new(
            Namefilter::default(),
            Utc::now(),
            rng,
        ));

        let dir_conflict = {
            let mut dir = (*dir).clone();
            dir.content.metadata.upsert_mtime(Utc::now());
            Rc::new(dir)
        };

        let private_node = PrivateNode::Dir(dir.clone());
        let private_node_conflict = PrivateNode::Dir(dir_conflict.clone());

        // Put the original node in the private forest
        let private_ref = private_node.store(forest, store, rng).await.unwrap();

        // Put the conflicting node in the private forest at the same key
        let private_ref_conflict = private_node_conflict
            .store(forest, store, rng)
            .await
            .unwrap();

        assert_eq!(
            private_ref.saturated_name_hash,
            private_ref_conflict.saturated_name_hash
        );

        let ciphertext_cids = forest
            .get_encrypted(&private_ref.saturated_name_hash, store)
            .await
            .unwrap()
            .unwrap();

        // We expect there to be a conflict, a multivalue
        // Two of these CIDs should be content blocks, one CID should be the header block they share.
        assert_eq!(ciphertext_cids.len(), 3);

        let retrieved = PrivateNode::load(&private_ref, forest, store)
            .await
            .unwrap();
        let retrieved_conflict = PrivateNode::load(&private_ref_conflict, forest, store)
            .await
            .unwrap();

        assert_eq!(retrieved, private_node);
        assert_eq!(retrieved_conflict, private_node_conflict);
    }

    #[async_std::test]
    async fn can_merge_nodes_with_different_structure_and_modified_changes() {
        let (store, rng) = test_setup::init!(mut store, mut rng);

        // A node that adds the first 3 pairs of HASH_KV_PAIRS.
        let other_node = &mut Rc::new(Node::<_, _, MockHasher>::default());
        for (digest, k, v) in HASH_KV_PAIRS.iter().take(3) {
            other_node
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
        let main_node = &mut Rc::new(Node::<_, _, MockHasher>::default());
        main_node
            .set_value(
                &mut HashNibbles::new(&HASH_KV_PAIRS[0].0),
                HASH_KV_PAIRS[0].1.clone(),
                BTreeSet::from([HASH_KV_PAIRS[0].2]),
                store,
            )
            .await
            .unwrap();

        let new_cid = generate_cid(rng);
        main_node
            .set_value(
                &mut HashNibbles::new(&HASH_KV_PAIRS[1].0),
                HASH_KV_PAIRS[1].1.clone(),
                BTreeSet::from([new_cid]),
                store,
            )
            .await
            .unwrap();

        for (digest, k, v) in HASH_KV_PAIRS.iter().skip(3).take(2) {
            main_node
                .set_value(
                    &mut HashNibbles::new(digest),
                    k.clone(),
                    BTreeSet::from([*v]),
                    store,
                )
                .await
                .unwrap();
        }

        let main_forest = Hamt::<Namefilter, BTreeSet<Cid>, _>::with_root(Rc::clone(main_node));
        let other_forest = Hamt::<Namefilter, BTreeSet<Cid>, _>::with_root(Rc::clone(other_node));

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
