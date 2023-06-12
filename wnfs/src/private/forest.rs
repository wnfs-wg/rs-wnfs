use super::{PrivateNode, RevisionRef};
use crate::error::{AesError, FsError};
use anyhow::Result;
use async_stream::stream;
use async_trait::async_trait;
use futures::Stream;
use libipld::{Cid, Ipld};
use rand_core::RngCore;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use sha3::Sha3_256;
use std::{collections::BTreeSet, rc::Rc};
use wnfs_common::{AsyncSerialize, BlockStore, HashOutput, Link};
use wnfs_hamt::{merge, Hamt, Hasher, KeyValueChange};
use wnfs_nameaccumulator::{AccumulatorSetup, Name, NameAccumulator};

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
#[derive(Debug, Clone)]
pub struct PrivateForest<H: Hasher = Sha3_256> {
    hamt: Hamt<NameAccumulator, BTreeSet<Cid>, H>,
    accumulator: AccumulatorSetup,
}

//--------------------------------------------------------------------------------------------------
// Implementations
//--------------------------------------------------------------------------------------------------

impl PrivateForest {
    /// Creates a new empty PrivateForest with given accumulator setup
    pub fn new(setup: AccumulatorSetup) -> Self {
        Self {
            hamt: Hamt::new(),
            accumulator: setup,
        }
    }

    pub fn new_rsa_2048(rng: &mut impl RngCore) -> Self {
        Self {
            hamt: Hamt::new(),
            accumulator: AccumulatorSetup::from_rsa_2048(rng),
        }
    }

    pub fn empty_name(&self) -> Name {
        Name::empty(&self.accumulator)
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
    ///     private::{PrivateForest, PrivateRef, PrivateDirectory, PrivateNode},
    ///     common::{BlockStore, MemoryBlockStore},
    ///     hamt::Hasher,
    ///     namefilter::Namefilter,
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
    pub async fn has_by_hash(
        &self,
        name_hash: &HashOutput,
        store: &impl BlockStore,
    ) -> Result<bool> {
        Ok(self
            .hamt
            .root
            .get_by_hash(name_hash, store)
            .await?
            .is_some())
    }

    /// TODO(matheus23) docs
    pub async fn has(&self, name: &Name, store: &impl BlockStore) -> Result<bool> {
        self.has_by_hash(
            &Sha3_256::hash(name.as_accumulator(&self.accumulator)),
            store,
        )
        .await
    }

    /// Adds new encrypted values at the given key.
    pub async fn put_encrypted<'a>(
        self: &mut Rc<Self>,
        name: &'a Name,
        values: impl IntoIterator<Item = Cid>,
        store: &mut impl BlockStore,
    ) -> Result<&'a NameAccumulator> {
        let name = name.as_accumulator(&self.accumulator);
        println!("Put {:02x?}", Sha3_256::hash(name));
        // TODO(matheus23): This iterates the path in the HAMT twice.
        // We could consider implementing something like upsert instead.
        // Or some kind of "cursor".
        let mut cids = self
            .hamt
            .root
            .get(&name, store)
            .await?
            .cloned()
            .unwrap_or_default();

        cids.extend(values);

        Rc::make_mut(self)
            .hamt
            .root
            .set(name.clone(), cids, store)
            .await?;

        Ok(name)
    }

    /// Gets the encrypted values at the given key.
    #[inline]
    pub async fn get_encrypted_by_hash<'b>(
        &'b self,
        name_hash: &HashOutput,
        store: &impl BlockStore,
    ) -> Result<Option<&'b BTreeSet<Cid>>> {
        println!("Get {:02x?}", name_hash);
        self.hamt.root.get_by_hash(name_hash, store).await
    }

    pub async fn get_encrypted(
        &self,
        name: &Name,
        store: &impl BlockStore,
    ) -> Result<Option<&BTreeSet<Cid>>> {
        let name_hash = &Sha3_256::hash(&name.as_accumulator(&self.accumulator).as_ref());
        self.get_encrypted_by_hash(name_hash, store).await
    }

    /// Removes the encrypted value at the given key.
    pub async fn remove_encrypted(
        self: &mut Rc<Self>,
        name_hash: &HashOutput,
        store: &mut impl BlockStore,
    ) -> Result<Option<BTreeSet<Cid>>> {
        let pair = Rc::make_mut(self)
            .hamt
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
        mounted_relative_to: &'a Name,
    ) -> impl Stream<Item = Result<PrivateNode>> + 'a {
        Box::pin(stream! {
            match self
                .get_encrypted_by_hash(&revision.saturated_name_hash, store)
                .await
            {
                Ok(Some(cids)) => {
                    for cid in cids {
                        match PrivateNode::from_cid(*cid, &revision.temporal_key, store, mounted_relative_to).await {
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

    /// Gets the difference in changes between two forests.
    #[inline]
    pub async fn diff(
        &self,
        other: &Self,
        store: &mut impl BlockStore,
    ) -> Result<Vec<KeyValueChange<NameAccumulator, BTreeSet<Cid>>>> {
        self.hamt.diff(&other.hamt, store).await
    }

    pub fn get_accumulator_setup(&self) -> &AccumulatorSetup {
        &self.accumulator
    }
}

impl<H> PrivateForest<H>
where
    H: Hasher + Clone + 'static,
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
    ///     private::{PrivateForest, RevisionRef, PrivateDirectory, PrivateNode},
    ///     common::{BlockStore, MemoryBlockStore},
    ///     namefilter::Namefilter,
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
        if self.accumulator != other.accumulator {
            return Err(FsError::IncompatibleAccumulatorSetups.into());
        }

        let merged_root = merge(
            Link::from(Rc::clone(&self.hamt.root)),
            Link::from(Rc::clone(&other.hamt.root)),
            |a, b| Ok(a.union(b).cloned().collect()),
            store,
        )
        .await?;

        Ok(Self {
            hamt: Hamt {
                version: self.hamt.version.clone(),
                root: merged_root,
            },
            accumulator: self.accumulator.clone(),
        })
    }
}

#[async_trait(?Send)]
impl AsyncSerialize for PrivateForest {
    async fn async_serialize<S, B>(&self, serializer: S, store: &mut B) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        B: BlockStore + ?Sized,
    {
        let hamt_ipld = self
            .hamt
            .async_serialize(libipld::serde::Serializer, store)
            .await
            .map_err(serde::ser::Error::custom)?;

        let accumulator_ipld = self
            .accumulator
            .serialize(libipld::serde::Serializer)
            .map_err(serde::ser::Error::custom)?;

        let Ipld::Map(mut ipld_map) = hamt_ipld else {
            return todo!(); // TODO(matheus23) probably use `Node::AsyncSerialize` instead of `Hamt`
        };

        ipld_map.insert("accumulator".into(), accumulator_ipld);

        Ipld::Map(ipld_map).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for PrivateForest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let ipld: Ipld = Deserialize::deserialize(deserializer)?;
        let hamt = Hamt::deserialize(ipld.clone()).map_err(serde::de::Error::custom)?;
        let Ipld::Map(ipld_map) = ipld else {
            return todo!(); // TODO(matheus23)
        };
        let Some(accumulator_ipld) = ipld_map.get("accumulator").cloned() else {
            return todo!(); // TODO(matheus23)
        };
        let accumulator =
            AccumulatorSetup::deserialize(accumulator_ipld).map_err(serde::de::Error::custom)?;

        Ok(Self { hamt, accumulator })
    }
}

//--------------------------------------------------------------------------------------------------
// Tests
//--------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::private::PrivateDirectory;
    use chrono::Utc;
    use helper::*;
    use proptest::test_runner::{RngAlgorithm, TestRng};
    use std::rc::Rc;
    use wnfs_common::MemoryBlockStore;
    use wnfs_hamt::{HashNibbles, Node};
    use wnfs_nameaccumulator::NameSegment;

    mod helper {
        use libipld::{Cid, Multihash};
        use once_cell::sync::Lazy;
        use rand::{thread_rng, RngCore};
        use wnfs_common::{utils, HashOutput};
        use wnfs_hamt::Hasher;
        use wnfs_nameaccumulator::{AccumulatorSetup, NameAccumulator, NameSegment};

        pub(super) static HASH_KV_PAIRS: Lazy<Vec<(HashOutput, Vec<u8>, Cid)>> = Lazy::new(|| {
            let setup = AccumulatorSetup::from_rsa_2048(&mut thread_rng());
            vec![
                (
                    utils::to_hash_output(&[0xA0]),
                    generate_name_accumulator(&setup, &mut thread_rng()),
                    generate_cid(&mut thread_rng()),
                ),
                (
                    utils::to_hash_output(&[0xA3]),
                    generate_name_accumulator(&setup, &mut thread_rng()),
                    generate_cid(&mut thread_rng()),
                ),
                (
                    utils::to_hash_output(&[0xA7]),
                    generate_name_accumulator(&setup, &mut thread_rng()),
                    generate_cid(&mut thread_rng()),
                ),
                (
                    utils::to_hash_output(&[0xAC]),
                    generate_name_accumulator(&setup, &mut thread_rng()),
                    generate_cid(&mut thread_rng()),
                ),
                (
                    utils::to_hash_output(&[0xAE]),
                    generate_name_accumulator(&setup, &mut thread_rng()),
                    generate_cid(&mut thread_rng()),
                ),
            ]
        });

        #[derive(Debug, Clone)]
        pub(super) struct MockHasher;
        impl Hasher for MockHasher {
            fn hash<K: AsRef<[u8]>>(key: &K) -> HashOutput {
                let key_ref = key.as_ref();
                HASH_KV_PAIRS
                    .iter()
                    .find(|(_, v, _)| key_ref == v)
                    .unwrap()
                    .0
            }
        }

        pub(super) fn generate_name_accumulator(
            setup: &AccumulatorSetup,
            rng: &mut impl RngCore,
        ) -> Vec<u8> {
            let mut name = NameAccumulator::empty(setup);
            name.add(&NameSegment::new(rng), setup);
            name.as_ref().to_vec()
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
    async fn test_put_get() {
        let store = &mut MemoryBlockStore::new();
        let rng = &mut TestRng::deterministic_rng(RngAlgorithm::ChaCha);
        let forest = &mut Rc::new(PrivateForest::new_rsa_2048(rng));

        let cid = Cid::default();
        let name = forest.empty_name().with_segments_added([
            NameSegment::from_seed(b"one"),
            NameSegment::from_seed(b"two"),
        ]);

        forest.put_encrypted(&name, [cid], store).await.unwrap();
        let result = forest
            .get_encrypted_by_hash(
                &Sha3_256::hash(name.as_accumulator(forest.get_accumulator_setup())),
                store,
            )
            .await
            .unwrap();

        assert_eq!(result, Some(&BTreeSet::from([cid])));
    }

    #[async_std::test]
    async fn inserted_items_can_be_fetched() {
        let store = &mut MemoryBlockStore::new();
        let rng = &mut TestRng::deterministic_rng(RngAlgorithm::ChaCha);
        let forest = &mut Rc::new(PrivateForest::new_rsa_2048(rng));

        let dir = Rc::new(PrivateDirectory::new(&forest.empty_name(), Utc::now(), rng));

        let private_node = PrivateNode::Dir(dir.clone());
        let private_ref = private_node.store(forest, store, rng).await.unwrap();
        let retrieved = PrivateNode::load(&private_ref, forest, store, &forest.empty_name())
            .await
            .unwrap();

        assert_eq!(retrieved, private_node);
    }

    #[async_std::test]
    async fn multivalue_conflict_can_be_fetched_individually() {
        let store = &mut MemoryBlockStore::new();
        let rng = &mut TestRng::deterministic_rng(RngAlgorithm::ChaCha);
        let forest = &mut Rc::new(PrivateForest::new_rsa_2048(rng));

        let dir = Rc::new(PrivateDirectory::new(&forest.empty_name(), Utc::now(), rng));

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
            .get_encrypted_by_hash(&private_ref.saturated_name_hash, store)
            .await
            .unwrap()
            .unwrap();

        // We expect there to be a conflict, a multivalue
        // Two of these CIDs should be content blocks, one CID should be the header block they share.
        assert_eq!(ciphertext_cids.len(), 3);

        let retrieved = PrivateNode::load(&private_ref, forest, store, &forest.empty_name())
            .await
            .unwrap();
        let retrieved_conflict =
            PrivateNode::load(&private_ref_conflict, forest, store, &forest.empty_name())
                .await
                .unwrap();

        assert_eq!(retrieved, private_node);
        assert_eq!(retrieved_conflict, private_node_conflict);
    }

    #[async_std::test]
    async fn can_merge_nodes_with_different_structure_and_modified_changes() {
        let store = &mut MemoryBlockStore::new();
        let rng = &mut TestRng::deterministic_rng(RngAlgorithm::ChaCha);
        let setup = &AccumulatorSetup::from_rsa_2048(rng);

        // A node that adds the first 3 pairs of HASH_KV_PAIRS.
        let other_node = &mut Rc::new(Node::<_, _, MockHasher>::default());
        for (digest, k, v) in HASH_KV_PAIRS.iter().take(3) {
            other_node
                .set_value(
                    &mut HashNibbles::new(digest),
                    NameAccumulator::parse_bytes(k).unwrap(),
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
                NameAccumulator::parse_bytes(&HASH_KV_PAIRS[0].1).unwrap(),
                BTreeSet::from([HASH_KV_PAIRS[0].2]),
                store,
            )
            .await
            .unwrap();

        let new_cid = generate_cid(rng);
        main_node
            .set_value(
                &mut HashNibbles::new(&HASH_KV_PAIRS[1].0),
                NameAccumulator::parse_bytes(&HASH_KV_PAIRS[1].1).unwrap(),
                BTreeSet::from([new_cid]),
                store,
            )
            .await
            .unwrap();

        for (digest, k, v) in HASH_KV_PAIRS.iter().skip(3).take(2) {
            main_node
                .set_value(
                    &mut HashNibbles::new(digest),
                    NameAccumulator::parse_bytes(k).unwrap(),
                    BTreeSet::from([*v]),
                    store,
                )
                .await
                .unwrap();
        }

        let main_forest = PrivateForest {
            hamt: Hamt::<NameAccumulator, BTreeSet<Cid>, _>::with_root(Rc::clone(main_node)),
            accumulator: setup.clone(),
        };

        let other_forest = PrivateForest {
            hamt: Hamt::<NameAccumulator, BTreeSet<Cid>, _>::with_root(Rc::clone(other_node)),
            accumulator: setup.clone(),
        };

        let merge_forest = main_forest.merge(&other_forest, store).await.unwrap();

        for (i, (digest, _, v)) in HASH_KV_PAIRS.iter().take(5).enumerate() {
            let retrieved = merge_forest
                .hamt
                .root
                .get_by_hash(digest, store)
                .await
                .unwrap();
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
