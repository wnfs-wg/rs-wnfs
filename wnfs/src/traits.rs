use crate::{
    error::AesError,
    private::{PrivateNode, RevisionRef},
};
use anyhow::Result;
use async_stream::stream;
use async_trait::async_trait;
use futures::stream::LocalBoxStream;
use libipld::Cid;
use std::{collections::BTreeSet, rc::Rc};
use wnfs_common::{BlockStore, HashOutput};
use wnfs_hamt::Pair;
use wnfs_nameaccumulator::{AccumulatorSetup, Name, NameAccumulator};

//--------------------------------------------------------------------------------------------------
// Traits
//--------------------------------------------------------------------------------------------------

/// Implements getting a unique identifier for a node.
pub trait Id {
    /// Gets an identifier for the node.
    fn get_id(&self) -> String;
}

#[async_trait(?Send)]
pub trait PrivateForest {
    fn empty_name(&self) -> Name;

    fn get_accumulator_setup(&self) -> &AccumulatorSetup;

    async fn has_by_hash(&self, name_hash: &HashOutput, store: &impl BlockStore) -> Result<bool>;

    async fn has(&self, name: &Name, store: &impl BlockStore) -> Result<bool>;

    async fn put_encrypted<'a>(
        self: &mut Rc<Self>,
        name: &'a Name,
        values: impl IntoIterator<Item = Cid>,
        store: &mut impl BlockStore,
    ) -> Result<&'a NameAccumulator>;

    async fn get_encrypted_by_hash<'b>(
        &'b self,
        name_hash: &HashOutput,
        store: &impl BlockStore,
    ) -> Result<Option<&'b BTreeSet<Cid>>>;

    async fn get_encrypted(
        &self,
        name: &Name,
        store: &impl BlockStore,
    ) -> Result<Option<&BTreeSet<Cid>>>;

    async fn remove_encrypted(
        self: &mut Rc<Self>,
        name_hash: &HashOutput,
        store: &mut impl BlockStore,
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
        mounted_relative_to: &'a Name,
    ) -> LocalBoxStream<'a, Result<PrivateNode>> {
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
}
