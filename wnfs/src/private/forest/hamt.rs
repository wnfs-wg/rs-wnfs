use super::traits::PrivateForest;
use crate::error::FsError;
use anyhow::Result;
use async_trait::async_trait;
use libipld_core::{cid::Cid, ipld::Ipld};
use quick_cache::sync::Cache;
use rand_core::CryptoRngCore;
use serde::{
    de::Error as DeError, ser::Error as SerError, Deserialize, Deserializer, Serialize, Serializer,
};
use std::{collections::BTreeSet, rc::Rc};
use wnfs_common::{AsyncSerialize, BlockStore, HashOutput, Link};
use wnfs_hamt::{merge, Hamt, Hasher, KeyValueChange, Pair};
use wnfs_nameaccumulator::{AccumulatorSetup, ElementsProof, Name, NameAccumulator};

const APPROX_CACHE_ENTRY_SIZE: usize =
    std::mem::size_of::<(Name, NameAccumulator, ElementsProof)>();
/// This gives us a *very rough* 2 MB limit on the cache.
/// It's sligthly more, since the `NameSegment`s inside the `Name`s aren't accounted for.
const NAME_CACHE_CAPACITY: usize = 2_000_000 / APPROX_CACHE_ENTRY_SIZE;

//--------------------------------------------------------------------------------------------------
// Type Definitions
//--------------------------------------------------------------------------------------------------

/// HamtForest is a HAMT that stores CIDs of encrypted private nodes keyed by name accumulators.
///
/// Inserted nodes should be encrypted ciphertexts of private wnfs nodes.
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
pub struct HamtForest {
    hamt: Hamt<NameAccumulator, BTreeSet<Cid>, blake3::Hasher>,
    accumulator: AccumulatorSetup,
    name_cache: Rc<Cache<Name, (NameAccumulator, ElementsProof)>>,
}

//--------------------------------------------------------------------------------------------------
// Implementations
//--------------------------------------------------------------------------------------------------

impl HamtForest {
    /// Create a new, empty hamt forest with given pre-run accumulator setup
    pub fn new(setup: AccumulatorSetup) -> Self {
        Self {
            hamt: Hamt::new(),
            accumulator: setup,
            name_cache: Rc::new(Cache::new(NAME_CACHE_CAPACITY)),
        }
    }

    /// Create a new, empty hamt forest with given pre-run accumulator setup wrapped in an `Rc`.
    pub fn rc(setup: AccumulatorSetup) -> Rc<Self> {
        Rc::new(Self::new(setup))
    }

    /// Create a new, empty hamt forest with an accumulator setup with its
    /// security based on the factors of the RSA-2048 factoring challenge
    /// modulus being unknown.
    ///
    /// This runs much faster than `new_trusted`, but relies on the RSA-2048
    /// factoring challenge not being broken. Great for tests.
    pub fn new_rsa_2048(rng: &mut impl CryptoRngCore) -> Self {
        Self::new(AccumulatorSetup::from_rsa_2048(rng))
    }

    /// Creates an `Rc` of a new, empty hamt forest with an accumulator setup
    /// based on the factors of the RSA-2048 factoring challenge modulus.
    pub fn rc_rsa_2048(rng: &mut impl CryptoRngCore) -> Rc<Self> {
        Rc::new(Self::new_rsa_2048(rng))
    }

    /// Create a new, empty hamt forest with and run a trusted accumulator
    /// steup. During this setup process there is a brief point in time
    /// at which some memory is written which, if observed, will break
    /// the security of the cryptographic accumulators.
    ///
    /// If the possibility of observing memory is part of your threat model,
    /// avoid this and try to generate the accumulator setup in advance via
    /// - a hardware security module (by generating an RSA modulus),
    /// - secure multiparty computation,
    /// - or by re-using the RSA-2048 factoring challenge.
    ///
    /// This function is fairly slow, as it's not using the most efficient
    /// methods for generating an RSA modulus.
    pub fn new_trusted(rng: &mut impl CryptoRngCore) -> Self {
        Self::new(AccumulatorSetup::trusted(rng))
    }

    /// Creates an `Rc` of a new, empty hamt forest with a trusted accumulator
    /// setup.
    pub fn rc_trusted(rng: &mut impl CryptoRngCore) -> Rc<Self> {
        Rc::new(Self::new_trusted(rng))
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

    fn get_proven_name(&self, name: &Name) -> (NameAccumulator, ElementsProof) {
        match self
            .name_cache
            .get_or_insert_with(name, || Ok(name.into_proven_accumulator(&self.accumulator)))
        {
            // Neat trick to avoid .unwrap():
            Ok(r) => r,
            Err(r) => r,
        }
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
            &blake3::Hasher::hash(&self.get_accumulated_name(name)),
            store,
        )
        .await
    }

    async fn put_encrypted(
        &mut self,
        name: &Name,
        values: impl IntoIterator<Item = Cid>,
        store: &impl BlockStore,
    ) -> Result<NameAccumulator> {
        let accumulator = self.get_accumulated_name(name);

        match self.hamt.root.get_mut(&accumulator, store).await? {
            Some(cids) => cids.extend(values),
            None => {
                self.hamt
                    .root
                    .set(accumulator.clone(), values.into_iter().collect(), store)
                    .await?;
            }
        }

        Ok(accumulator)
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
        let name_hash = &blake3::Hasher::hash(&self.get_accumulated_name(name));
        self.get_encrypted_by_hash(name_hash, store).await
    }

    async fn remove_encrypted(
        &mut self,
        name: &Name,
        store: &impl BlockStore,
    ) -> Result<Option<Pair<NameAccumulator, BTreeSet<Cid>>>> {
        let name_hash = &blake3::Hasher::hash(&self.get_accumulated_name(name));
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

    fn get_proven_name(&self, name: &Name) -> (NameAccumulator, ElementsProof) {
        (**self).get_proven_name(name)
    }

    async fn has_by_hash(&self, name_hash: &HashOutput, store: &impl BlockStore) -> Result<bool> {
        (**self).has_by_hash(name_hash, store).await
    }

    async fn has(&self, name: &Name, store: &impl BlockStore) -> Result<bool> {
        (**self).has(name, store).await
    }

    async fn put_encrypted(
        &mut self,
        name: &Name,
        values: impl IntoIterator<Item = Cid>,
        store: &impl BlockStore,
    ) -> Result<NameAccumulator> {
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
        name: &Name,
        store: &impl BlockStore,
    ) -> Result<Option<Pair<NameAccumulator, BTreeSet<Cid>>>> {
        Rc::make_mut(self).remove_encrypted(name, store).await
    }
}

impl HamtForest {
    /// Merges a private forest with another. If there is a conflict with the values,they are union
    /// combined into a single value in the final merge node
    ///
    /// # Examples
    ///
    /// ```
    /// use std::rc::Rc;
    /// use anyhow::Result;
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
    ///     let forest = &mut HamtForest::rc_rsa_2048(rng);
    ///     let root_dir = &mut PrivateDirectory::new_and_store(
    ///         &forest.empty_name(),
    ///         Utc::now(),
    ///         forest,
    ///         store,
    ///         rng
    ///     ).await?;
    ///     root_dir.as_node().store(forest, store, rng).await?;
    ///
    ///     // Make two conflicting writes
    ///     let forest_one = &mut Rc::clone(forest);
    ///     let dir_one = &mut Rc::clone(root_dir);
    ///     dir_one.mkdir(&["DirOne".into()], true, Utc::now(), forest_one, store, rng).await?;
    ///     dir_one.as_node().store(forest_one, store, rng).await?;
    ///
    ///     let forest_two = &mut Rc::clone(forest);
    ///     let dir_two = &mut Rc::clone(root_dir);
    ///     dir_two.mkdir(&["DirTwo".into()], true, Utc::now(), forest_two, store, rng).await?;
    ///     let access_key = dir_two.as_node().store(forest_two, store, rng).await?;
    ///     let label = access_key.get_label();
    ///     let key = access_key.get_temporal_key()?;
    ///
    ///     // Merge the forests together
    ///     let forest_merged = forest_one.merge(forest_two, store).await?;
    ///
    ///     let multivalue: Vec<_> = forest_merged
    ///         .get_multivalue_by_hash(label, key, store, None)
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

        // TODO(matheus23) Should we find some way to sensibly merge caches?
        let name_cache = self.name_cache.clone();

        Ok(Self {
            hamt: Hamt {
                version: self.hamt.version.clone(),
                root: merged_root,
            },
            accumulator: self.accumulator.clone(),
            name_cache,
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
            .async_serialize(libipld_core::serde::Serializer, store)
            .await
            .map_err(serde::ser::Error::custom)?;

        let accumulator_ipld = self
            .accumulator
            .serialize(libipld_core::serde::Serializer)
            .map_err(serde::ser::Error::custom)?;

        let Ipld::Map(mut ipld_map) = hamt_ipld else {
            let msg =
                format!("Expected HAMT root to serialize to an IPLD map, but got {hamt_ipld:#?}");
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

        Ok(Self {
            hamt,
            accumulator,
            name_cache: Rc::new(Cache::new(NAME_CACHE_CAPACITY)),
        })
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
    use rand_chacha::ChaCha12Rng;
    use rand_core::SeedableRng;
    use std::rc::Rc;
    use wnfs_common::MemoryBlockStore;
    use wnfs_nameaccumulator::NameSegment;

    #[async_std::test]
    async fn test_put_get() {
        let store = &mut MemoryBlockStore::new();
        let rng = &mut ChaCha12Rng::seed_from_u64(0);
        let forest = &mut HamtForest::rc_rsa_2048(rng);

        let cid = Cid::default();
        let name = forest.empty_name().with_segments_added([
            NameSegment::new_hashed("Testing", b"one"),
            NameSegment::new_hashed("Testing", b"two"),
        ]);

        forest.put_encrypted(&name, [cid], store).await.unwrap();
        let result = forest.get_encrypted(&name, store).await.unwrap();

        assert_eq!(result, Some(&BTreeSet::from([cid])));
    }

    #[async_std::test]
    async fn inserted_items_can_be_fetched() {
        let store = &mut MemoryBlockStore::new();
        let rng = &mut ChaCha12Rng::seed_from_u64(0);
        let forest = &mut HamtForest::rc_rsa_2048(rng);
        let dir = PrivateDirectory::rc(&forest.empty_name(), Utc::now(), rng);

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
        let rng = &mut ChaCha12Rng::seed_from_u64(0);
        let forest = &mut HamtForest::rc_rsa_2048(rng);
        let dir = PrivateDirectory::rc(&forest.empty_name(), Utc::now(), rng);

        let dir_conflict = {
            let mut dir = (*dir).clone();
            dir.content.metadata.upsert_mtime(Utc::now());
            Rc::new(dir)
        };

        let private_node = PrivateNode::Dir(dir.clone());
        let private_node_conflict = PrivateNode::Dir(dir_conflict.clone());

        // Put the original node in the private forest
        let access_key = private_node.store(forest, store, rng).await.unwrap();

        let private_ref = access_key.derive_private_ref().unwrap();

        // Put the conflicting node in the private forest at the same key
        let access_key_conflict = private_node_conflict
            .store(forest, store, rng)
            .await
            .unwrap();

        let private_ref_conflict = access_key_conflict.derive_private_ref().unwrap();

        assert_eq!(private_ref.label, private_ref_conflict.label);

        let ciphertext_entries = forest
            .get_encrypted_by_hash(&private_ref.label, store)
            .await
            .unwrap()
            .unwrap();

        // We expect there to be a conflict, a multivalue
        // Two of these entries should be content blocks, one entry should be the header block they share.
        assert_eq!(ciphertext_entries.len(), 3);

        let retrieved = PrivateNode::load(&access_key, forest, store, Some(forest.empty_name()))
            .await
            .unwrap();

        let retrieved_conflict = PrivateNode::load(
            &access_key_conflict,
            forest,
            store,
            Some(forest.empty_name()),
        )
        .await
        .unwrap();

        assert_eq!(retrieved, private_node);
        assert_eq!(retrieved_conflict, private_node_conflict);
    }
}

#[cfg(test)]
mod snapshot_tests {
    use super::*;
    use rand_chacha::ChaCha12Rng;
    use rand_core::SeedableRng;
    use wnfs_common::utils::SnapshotBlockStore;
    use wnfs_nameaccumulator::NameSegment;

    #[async_std::test]
    async fn test_hamt() {
        let rng = &mut ChaCha12Rng::seed_from_u64(0);
        let store = &SnapshotBlockStore::default();
        let forest = &mut HamtForest::rc_rsa_2048(rng);
        let base_name = forest.empty_name();
        let name_segments = [
            vec![NameSegment::new(rng)],
            vec![NameSegment::new(rng), NameSegment::new(rng)],
            vec![
                NameSegment::new(rng),
                NameSegment::new(rng),
                NameSegment::new(rng),
            ],
        ];

        for segments in name_segments {
            forest
                .put_encrypted(
                    &base_name.with_segments_added(segments),
                    [Cid::default()],
                    store,
                )
                .await
                .unwrap();
        }

        let cid = forest.store(store).await.unwrap();
        let store = store.get_block_snapshot(&cid).await.unwrap();

        insta::assert_json_snapshot!(store);
    }
}
