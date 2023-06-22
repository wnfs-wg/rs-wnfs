use super::traits::PrivateForest;
use crate::error::FsError;
use anyhow::Result;
use async_trait::async_trait;
use libipld::{Cid, Ipld};
use rand_core::RngCore;
use serde::{
    de::Error as DeError, ser::Error as SerError, Deserialize, Deserializer, Serialize, Serializer,
};
use sha3::Sha3_256;
use std::{collections::BTreeSet, rc::Rc};
use wnfs_common::{AsyncSerialize, BlockStore, HashOutput, Link};
use wnfs_hamt::{merge, Hamt, Hasher, KeyValueChange, Pair};
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
/// use wnfs::private::forest::hamt::HamtForest;
/// use rand::thread_rng;
///
/// let forest = HamtForest::new_rsa_2048(&mut thread_rng());
///
/// println!("{:?}", forest);
/// ```
#[derive(Debug, Clone)]
pub struct HamtForest<H: Hasher = Sha3_256> {
    hamt: Hamt<NameAccumulator, BTreeSet<Cid>, H>,
    accumulator: AccumulatorSetup,
}

//--------------------------------------------------------------------------------------------------
// Implementations
//--------------------------------------------------------------------------------------------------

impl HamtForest {
    /// Creates a new empty PrivateForest with given accumulator setup
    pub fn new(setup: AccumulatorSetup) -> Self {
        Self {
            hamt: Hamt::new(),
            accumulator: setup,
        }
    }

    pub fn new_rsa_2048(rng: &mut impl RngCore) -> Self {
        Self::new(AccumulatorSetup::from_rsa_2048(rng))
    }

    pub fn new_trusted(rng: &mut impl RngCore) -> Self {
        Self::new(AccumulatorSetup::trusted(rng))
    }

    /// Gets the difference in changes between two forests.
    #[inline]
    pub async fn diff(
        &self,
        other: &Self,
        store: &impl BlockStore,
    ) -> Result<Vec<KeyValueChange<NameAccumulator, BTreeSet<Cid>>>> {
        if self.accumulator != other.accumulator {
            return Err(FsError::IncompatibleAccumulatorSetups.into());
        }

        self.hamt.diff(&other.hamt, store).await
    }

    /// Serializes the forest and stores it in the given block store.
    pub async fn store(&self, store: &impl BlockStore) -> Result<Cid> {
        store.put_async_serializable(self).await
    }

    /// Deserializes a forest from the given block store.
    pub async fn load(cid: &Cid, store: &impl BlockStore) -> Result<Self> {
        store.get_deserializable(cid).await
    }
}

#[async_trait(?Send)]
impl PrivateForest for HamtForest {
    fn empty_name(&self) -> Name {
        Name::empty(&self.accumulator)
    }

    fn get_accumulator_setup(&self) -> &AccumulatorSetup {
        &self.accumulator
    }

    async fn has_by_hash(&self, name_hash: &HashOutput, store: &impl BlockStore) -> Result<bool> {
        Ok(self
            .hamt
            .root
            .get_by_hash(name_hash, store)
            .await?
            .is_some())
    }

    async fn has(&self, name: &Name, store: &impl BlockStore) -> Result<bool> {
        self.has_by_hash(
            &Sha3_256::hash(name.as_accumulator(&self.accumulator)),
            store,
        )
        .await
    }

    async fn put_encrypted<'a>(
        &mut self,
        name: &'a Name,
        values: impl IntoIterator<Item = Cid>,
        store: &impl BlockStore,
    ) -> Result<&'a NameAccumulator> {
        let name = name.as_accumulator(&self.accumulator);

        match self.hamt.root.get_mut(name, store).await? {
            Some(cids) => cids.extend(values),
            None => {
                self.hamt
                    .root
                    .set(name.clone(), values.into_iter().collect(), store)
                    .await?;
            }
        }

        Ok(name)
    }

    #[inline]
    async fn get_encrypted_by_hash<'b>(
        &'b self,
        name_hash: &HashOutput,
        store: &impl BlockStore,
    ) -> Result<Option<&'b BTreeSet<Cid>>> {
        self.hamt.root.get_by_hash(name_hash, store).await
    }

    async fn get_encrypted(
        &self,
        name: &Name,
        store: &impl BlockStore,
    ) -> Result<Option<&BTreeSet<Cid>>> {
        let name_hash = &Sha3_256::hash(&name.as_accumulator(&self.accumulator).as_ref());
        self.get_encrypted_by_hash(name_hash, store).await
    }

    async fn remove_encrypted(
        &mut self,
        name_hash: &HashOutput,
        store: &impl BlockStore,
    ) -> Result<Option<Pair<NameAccumulator, BTreeSet<Cid>>>> {
        self.hamt.root.remove_by_hash(name_hash, store).await
    }
}

#[async_trait(?Send)]
impl PrivateForest for Rc<HamtForest> {
    fn empty_name(&self) -> Name {
        (**self).empty_name()
    }

    fn get_accumulator_setup(&self) -> &AccumulatorSetup {
        (**self).get_accumulator_setup()
    }

    async fn has_by_hash(&self, name_hash: &HashOutput, store: &impl BlockStore) -> Result<bool> {
        (**self).has_by_hash(name_hash, store).await
    }

    async fn has(&self, name: &Name, store: &impl BlockStore) -> Result<bool> {
        (**self).has(name, store).await
    }

    async fn put_encrypted<'a>(
        &mut self,
        name: &'a Name,
        values: impl IntoIterator<Item = Cid>,
        store: &impl BlockStore,
    ) -> Result<&'a NameAccumulator> {
        Rc::make_mut(self).put_encrypted(name, values, store).await
    }

    async fn get_encrypted_by_hash<'b>(
        &'b self,
        name_hash: &HashOutput,
        store: &impl BlockStore,
    ) -> Result<Option<&'b BTreeSet<Cid>>> {
        (**self).get_encrypted_by_hash(name_hash, store).await
    }

    async fn get_encrypted(
        &self,
        name: &Name,
        store: &impl BlockStore,
    ) -> Result<Option<&BTreeSet<Cid>>> {
        (**self).get_encrypted(name, store).await
    }

    async fn remove_encrypted(
        &mut self,
        name_hash: &HashOutput,
        store: &impl BlockStore,
    ) -> Result<Option<Pair<NameAccumulator, BTreeSet<Cid>>>> {
        Rc::make_mut(self).remove_encrypted(name_hash, store).await
    }
}

impl<H> HamtForest<H>
where
    H: Hasher + Clone + 'static,
{
    /// Merges a private forest with another. If there is a conflict with the values,they are union
    /// combined into a single value in the final merge node
    ///
    /// # Examples
    ///
    /// ```
    /// use anyhow::Result;
    /// use std::rc::Rc;
    /// use chrono::Utc;
    /// use rand::thread_rng;
    /// use futures::StreamExt;
    /// use wnfs::{
    ///     common::MemoryBlockStore,
    ///     private::{
    ///         PrivateDirectory,
    ///         forest::{hamt::HamtForest, traits::PrivateForest},
    ///     },
    /// };
    ///
    /// #[async_std::main]
    /// async fn main() -> Result<()> {
    ///     let store = &mut MemoryBlockStore::new();
    ///     let rng = &mut thread_rng();
    ///
    ///     let forest = &mut Rc::new(HamtForest::new_rsa_2048(rng));
    ///     let root_dir = &mut PrivateDirectory::new_and_store(
    ///         &forest.empty_name(),
    ///         Utc::now(),
    ///         forest,
    ///         store,
    ///         rng
    ///     ).await?;
    ///     root_dir.store(forest, store, rng).await?;
    ///
    ///     // Make two conflicting writes
    ///     let forest_one = &mut Rc::clone(forest);
    ///     let dir_one = &mut Rc::clone(root_dir);
    ///     dir_one.mkdir(&["DirOne".into()], true, Utc::now(), forest_one, store, rng).await?;
    ///     dir_one.store(forest_one, store, rng).await?;
    ///
    ///     let forest_two = &mut Rc::clone(forest);
    ///     let dir_two = &mut Rc::clone(root_dir);
    ///     dir_two.mkdir(&["DirTwo".into()], true, Utc::now(), forest_two, store, rng).await?;
    ///     let private_ref = dir_two.store(forest_two, store, rng).await?;
    ///
    ///     // Merge the forests together
    ///     let forest_merged = forest_one.merge(forest_two, store).await?;
    ///
    ///     // Read the revision slot with conflicting writes
    ///     let revision_ref = private_ref.as_revision_ref();
    ///
    ///     let multivalue: Vec<_> = forest_merged
    ///         .get_multivalue(&revision_ref, store, None)
    ///         .collect::<Vec<_>>()
    ///         .await
    ///         .into_iter()
    ///         .filter_map(|result| result.ok())
    ///         .collect::<Vec<_>>();
    ///
    ///     // There's two conflicting values in the slot
    ///     assert_eq!(2, multivalue.len());
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn merge(&self, other: &Self, store: &impl BlockStore) -> Result<Self> {
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
impl AsyncSerialize for HamtForest {
    async fn async_serialize<S, B>(&self, serializer: S, store: &B) -> Result<S::Ok, S::Error>
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
            let msg = format!("Expected HAMT root to serialize to an IPLD map, but got {hamt_ipld:#?}");
            return Err(SerError::custom(FsError::InvalidDeserialization(msg)));
        };

        ipld_map.insert("accumulator".into(), accumulator_ipld);

        Ipld::Map(ipld_map).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HamtForest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let ipld: Ipld = Deserialize::deserialize(deserializer)?;
        let hamt = Hamt::deserialize(ipld.clone()).map_err(serde::de::Error::custom)?;
        let Ipld::Map(ipld_map) = ipld else {
            let msg = format!("Expected IPLD Map representing a private forest, but got {ipld:#?}");
            return Err(DeError::custom(FsError::InvalidDeserialization(msg)));
        };
        let Some(accumulator_ipld) = ipld_map.get("accumulator").cloned() else {
            let msg = "IPLD Map entry for 'accumulator' missing in private forest".to_string();
            return Err(DeError::custom(FsError::InvalidDeserialization(msg)));
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
    use crate::private::{PrivateDirectory, PrivateNode};
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
            name.add(Some(&NameSegment::new(rng)), setup);
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
        let forest = &mut Rc::new(HamtForest::new_rsa_2048(rng));

        let cid = Cid::default();
        let name = forest.empty_name().with_segments_added([
            NameSegment::from_seed(b"one"),
            NameSegment::from_seed(b"two"),
        ]);

        forest.put_encrypted(&name, [cid], store).await.unwrap();
        let result = forest.get_encrypted(&name, store).await.unwrap();

        assert_eq!(result, Some(&BTreeSet::from([cid])));
    }

    #[async_std::test]
    async fn inserted_items_can_be_fetched() {
        let store = &mut MemoryBlockStore::new();
        let rng = &mut TestRng::deterministic_rng(RngAlgorithm::ChaCha);
        let forest = &mut Rc::new(HamtForest::new_rsa_2048(rng));

        let dir = Rc::new(PrivateDirectory::new(&forest.empty_name(), Utc::now(), rng));

        let private_node = PrivateNode::Dir(dir.clone());
        let private_ref = private_node.store(forest, store, rng).await.unwrap();
        let retrieved = PrivateNode::load(&private_ref, forest, store, Some(forest.empty_name()))
            .await
            .unwrap();

        assert_eq!(retrieved, private_node);
    }

    #[async_std::test]
    async fn multivalue_conflict_can_be_fetched_individually() {
        let store = &mut MemoryBlockStore::new();
        let rng = &mut TestRng::deterministic_rng(RngAlgorithm::ChaCha);
        let forest = &mut Rc::new(HamtForest::new_rsa_2048(rng));

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

        let ciphertext_entries = forest
            .get_encrypted_by_hash(&private_ref.saturated_name_hash, store)
            .await
            .unwrap()
            .unwrap();

        // We expect there to be a conflict, a multivalue
        // Two of these entries should be content blocks, one entry should be the header block they share.
        assert_eq!(ciphertext_entries.len(), 3);

        let retrieved = PrivateNode::load(&private_ref, forest, store, Some(forest.empty_name()))
            .await
            .unwrap();

        let retrieved_conflict = PrivateNode::load(
            &private_ref_conflict,
            forest,
            store,
            Some(forest.empty_name()),
        )
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

        let main_forest = HamtForest {
            hamt: Hamt::<NameAccumulator, BTreeSet<Cid>, _>::with_root(Rc::clone(main_node)),
            accumulator: setup.clone(),
        };

        let other_forest = HamtForest {
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
