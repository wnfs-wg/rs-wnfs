use crate::{
    error::AesError,
    private::{PrivateNode, RevisionRef},
};
use anyhow::Result;
use async_stream::stream;
use async_trait::async_trait;
use futures::stream::LocalBoxStream;
use libipld::Cid;
use std::collections::BTreeSet;
use wnfs_common::{BlockStore, HashOutput};
use wnfs_hamt::Pair;
use wnfs_nameaccumulator::{AccumulatorSetup, Name, NameAccumulator};

#[async_trait(?Send)]
pub trait PrivateForest {
    /// TODO(matheus23) docs
    fn empty_name(&self) -> Name;

    /// TODO(matheus23) docs
    fn get_accumulator_setup(&self) -> &AccumulatorSetup;

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
    ///     private::{
    ///         PrivateRef, PrivateDirectory, PrivateNode,
    ///         forest::{hamt::HamtForest, traits::PrivateForest},
    ///     },
    ///     common::MemoryBlockStore,
    ///     hamt::Hasher,
    /// };
    ///
    /// #[async_std::main]
    /// async fn main() {
    ///     let store = &mut MemoryBlockStore::default();
    ///     let rng = &mut thread_rng();
    ///     let forest = &mut Rc::new(HamtForest::new_rsa_2048(rng));
    ///     let dir = Rc::new(PrivateDirectory::new(
    ///         &forest.empty_name(),
    ///         Utc::now(),
    ///         rng,
    ///     ));
    ///
    ///     let node = PrivateNode::Dir(dir);
    ///     let private_ref = node.store(forest, store, rng).await.unwrap();
    ///
    ///     assert!(forest.has_by_hash(&private_ref.saturated_name_hash, store).await.unwrap());
    /// }
    /// ```
    async fn has_by_hash(&self, name_hash: &HashOutput, store: &impl BlockStore) -> Result<bool>;

    /// TODO(matheus23) docs
    async fn has(&self, name: &Name, store: &impl BlockStore) -> Result<bool>;

    /// Adds new encrypted values at the given key.
    async fn put_encrypted<'a>(
        &mut self,
        name: &'a Name,
        values: impl IntoIterator<Item = Cid>,
        store: &impl BlockStore,
    ) -> Result<&'a NameAccumulator>;

    async fn get_encrypted_by_hash<'b>(
        &'b self,
        name_hash: &HashOutput,
        store: &impl BlockStore,
    ) -> Result<Option<&'b BTreeSet<Cid>>>;

    /// Gets the encrypted values at the given key.
    async fn get_encrypted(
        &self,
        name: &Name,
        store: &impl BlockStore,
    ) -> Result<Option<&BTreeSet<Cid>>>;

    /// Removes the encrypted values at the given key.
    async fn remove_encrypted(
        &mut self,
        name: &Name,
        store: &impl BlockStore,
    ) -> Result<Option<Pair<NameAccumulator, BTreeSet<Cid>>>>;

    /// Returns a stream of all private nodes that could be decrypted at given revision.
    ///
    /// The stream of results is ordered by CID.
    ///
    /// Each item in the resulting stream represents an instance of a concurrent write.
    fn get_multivalue<'a>(
        &'a self,
        revision: &'a RevisionRef,
        store: &'a impl BlockStore,
        mounted_relative_to: Option<Name>,
    ) -> LocalBoxStream<'a, Result<PrivateNode>> {
        Box::pin(stream! {
            match self
                .get_encrypted_by_hash(&revision.saturated_name_hash, store)
                .await
            {
                Ok(Some(cids)) => {
                    let setup = self.get_accumulator_setup();

                    for cid in cids {
                        match PrivateNode::from_cid(*cid, &revision.temporal_key, store, mounted_relative_to.clone(), setup).await {
                            Ok(node) => yield Ok(node),
                            Err(e) if e.downcast_ref::<AesError>().is_some() => {
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
