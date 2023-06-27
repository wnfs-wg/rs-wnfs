use super::{PrivateNodeHeader, TemporalKey};
use crate::{
    error::FsError,
    private::{
        encrypted::Encrypted, forest::traits::PrivateForest, link::PrivateLink, PrivateDirectory,
        PrivateFile, PrivateNodeContentSerializable, PrivateRef,
    },
    traits::Id,
};
use anyhow::{bail, Result};
use async_once_cell::OnceCell;
use async_recursion::async_recursion;
use chrono::{DateTime, Utc};
use futures::StreamExt;
use libipld::Cid;
use rand_core::CryptoRngCore;
use skip_ratchet::{JumpSize, RatchetSeeker};
use std::{cmp::Ordering, collections::BTreeSet, fmt::Debug, rc::Rc};
use wnfs_common::BlockStore;
use wnfs_nameaccumulator::{AccumulatorSetup, Name};

//--------------------------------------------------------------------------------------------------
// Type Definitions
//--------------------------------------------------------------------------------------------------

/// Represents a node in the WNFS private file system. This can either be a file or a directory.
///
/// # Examples
///
/// ```
/// use wnfs::private::{
///     PrivateDirectory, PrivateNode,
///     forest::{hamt::HamtForest, traits::PrivateForest},
/// };
/// use chrono::Utc;
/// use std::rc::Rc;
/// use rand::thread_rng;
///
/// let rng = &mut thread_rng();
/// let forest = &mut Rc::new(HamtForest::new_rsa_2048(rng));
/// let dir = Rc::new(PrivateDirectory::new(
///     &forest.empty_name(),
///     Utc::now(),
///     rng,
/// ));
///
/// let node = PrivateNode::Dir(dir);
///
/// println!("Node: {:?}", node);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum PrivateNode {
    File(Rc<PrivateFile>),
    Dir(Rc<PrivateDirectory>),
}

//--------------------------------------------------------------------------------------------------
// Implementations
//--------------------------------------------------------------------------------------------------

impl PrivateNode {
    /// Creates node with upserted modified time.
    ///
    /// # Examples
    ///
    /// ```
    /// use wnfs::{
    ///     private::{
    ///         PrivateDirectory, PrivateNode,
    ///         forest::{hamt::HamtForest, traits::PrivateForest},
    ///     },
    /// };
    /// use chrono::{Utc, Duration, TimeZone};
    /// use std::rc::Rc;
    /// use rand::thread_rng;
    ///
    /// let rng = &mut thread_rng();
    /// let forest = &mut Rc::new(HamtForest::new_rsa_2048(rng));
    /// let dir = Rc::new(PrivateDirectory::new(
    ///     &forest.empty_name(),
    ///     Utc::now(),
    ///     rng,
    /// ));
    /// let node = PrivateNode::Dir(dir);
    ///
    /// let time = Utc::now() + Duration::days(1);
    /// let node = node.upsert_mtime(time);
    ///
    /// let imprecise_time = Utc.timestamp_opt(time.timestamp(), 0).single();
    /// assert_eq!(
    ///     imprecise_time,
    ///     node.as_dir()
    ///         .unwrap()
    ///         .get_metadata()
    ///         .get_modified()
    /// );
    /// ```
    pub fn upsert_mtime(&self, time: DateTime<Utc>) -> Self {
        match self {
            Self::File(file) => {
                let mut file = (**file).clone();
                file.content.metadata.upsert_mtime(time);
                Self::File(Rc::new(file))
            }
            Self::Dir(dir) => {
                let mut dir = (**dir).clone();
                dir.content.metadata.upsert_mtime(time);
                Self::Dir(Rc::new(dir))
            }
        }
    }

    /// Updates bare name ancestry of private sub tree.
    #[async_recursion(?Send)]
    pub(crate) async fn update_ancestry(
        &mut self,
        parent_name: &Name,
        forest: &mut impl PrivateForest,
        store: &impl BlockStore,
        rng: &mut impl CryptoRngCore,
    ) -> Result<()> {
        match self {
            Self::File(file_rc) => {
                let file = Rc::make_mut(file_rc);

                file.prepare_key_rotation(parent_name, forest, store, rng)
                    .await?;
            }
            Self::Dir(dir_rc) => {
                let dir = Rc::make_mut(dir_rc);

                for private_link in &mut dir.content.entries.values_mut() {
                    let mut node = private_link
                        .resolve_node(forest, store, Some(dir.header.name.clone()))
                        .await?
                        .clone();
                    node.update_ancestry(&dir.header.name, forest, store, rng)
                        .await?;
                    *private_link = PrivateLink::from(node);
                }

                dir.prepare_key_rotation(parent_name, rng);
            }
        }
        Ok(())
    }

    /// Gets the header of the node.
    ///
    /// # Examples
    ///
    /// ```
    /// use wnfs::{
    ///     private::{
    ///         PrivateDirectory, PrivateNode,
    ///         forest::{hamt::HamtForest, traits::PrivateForest},
    ///     },
    /// };
    /// use chrono::Utc;
    /// use std::rc::Rc;
    /// use rand::thread_rng;
    ///
    /// let rng = &mut thread_rng();
    /// let forest = &mut Rc::new(HamtForest::new_rsa_2048(rng));
    /// let dir = Rc::new(PrivateDirectory::new(
    ///     &forest.empty_name(),
    ///     Utc::now(),
    ///     rng,
    /// ));
    /// let node = PrivateNode::Dir(Rc::clone(&dir));
    ///
    /// assert_eq!(&dir.header, node.get_header());
    /// ```
    #[inline]
    pub fn get_header(&self) -> &PrivateNodeHeader {
        match self {
            Self::File(file) => &file.header,
            Self::Dir(dir) => &dir.header,
        }
    }

    /// Gets the previous links of the node.
    ///
    /// The previous links are encrypted with the previous revision's
    /// temporal key, so you need to know an 'older' revision of the
    /// skip ratchet to decrypt these.
    ///
    /// The previous links is exactly one Cid in most cases and refers
    /// to the ciphertext Cid from the previous revision that this
    /// node is an update of.
    ///
    /// If this node is a merge-node, it has two or more previous Cids.
    /// A single previous Cid must be from the previous revision, but all
    /// other Cids may appear in even older revisions.
    ///
    /// The previous links is `None`, it doesn't have previous Cids.
    /// The node is malformed if the previous links are `Some`, but
    /// the `BTreeSet` inside is empty.
    #[allow(clippy::mutable_key_type)]
    pub fn get_previous(&self) -> &BTreeSet<(usize, Encrypted<Cid>)> {
        match self {
            Self::File(file) => &file.content.previous,
            Self::Dir(dir) => &dir.content.previous,
        }
    }

    /// Casts a node to a directory.
    ///
    /// # Examples
    ///
    /// ```
    /// use wnfs::{
    ///     private::{
    ///         PrivateDirectory, PrivateNode,
    ///         forest::{hamt::HamtForest, traits::PrivateForest},
    ///     },
    /// };
    /// use chrono::Utc;
    /// use std::rc::Rc;
    /// use rand::thread_rng;
    ///
    /// let rng = &mut thread_rng();
    /// let forest = &mut Rc::new(HamtForest::new_rsa_2048(rng));
    /// let dir = Rc::new(PrivateDirectory::new(
    ///     &forest.empty_name(),
    ///     Utc::now(),
    ///     rng,
    /// ));
    /// let node = PrivateNode::Dir(Rc::clone(&dir));
    ///
    /// assert_eq!(node.as_dir().unwrap(), dir);
    /// ```
    pub fn as_dir(&self) -> Result<Rc<PrivateDirectory>> {
        Ok(match self {
            Self::Dir(dir) => Rc::clone(dir),
            _ => bail!(FsError::NotADirectory),
        })
    }

    /// Casts a node to a mutable directory.
    pub(crate) fn as_dir_mut(&mut self) -> Result<&mut Rc<PrivateDirectory>> {
        Ok(match self {
            Self::Dir(dir) => dir,
            _ => bail!(FsError::NotADirectory),
        })
    }

    /// Casts a node to a file.
    ///
    /// # Examples
    ///
    /// ```
    /// use wnfs::{
    ///     private::{
    ///         PrivateFile, PrivateNode,
    ///         forest::{hamt::HamtForest, traits::PrivateForest},
    ///     },
    /// };
    /// use chrono::Utc;
    /// use std::rc::Rc;
    /// use rand::thread_rng;
    ///
    /// let rng = &mut thread_rng();
    /// let forest = &mut Rc::new(HamtForest::new_rsa_2048(rng));
    /// let file = Rc::new(PrivateFile::new(
    ///     &forest.empty_name(),
    ///     Utc::now(),
    ///     rng,
    /// ));
    /// let node = PrivateNode::File(Rc::clone(&file));
    ///
    /// assert_eq!(node.as_file().unwrap(), file);
    /// ```
    pub fn as_file(&self) -> Result<Rc<PrivateFile>> {
        Ok(match self {
            Self::File(file) => Rc::clone(file),
            _ => bail!(FsError::NotAFile),
        })
    }

    /// Returns true if underlying node is a directory.
    ///
    /// # Examples
    ///
    /// ```
    /// use wnfs::{
    ///     private::{
    ///         PrivateDirectory, PrivateNode,
    ///         forest::{hamt::HamtForest, traits::PrivateForest},
    ///     },
    /// };
    /// use chrono::Utc;
    /// use std::rc::Rc;
    /// use rand::thread_rng;
    ///
    /// let rng = &mut thread_rng();
    /// let forest = &mut Rc::new(HamtForest::new_rsa_2048(rng));
    /// let dir = Rc::new(PrivateDirectory::new(
    ///     &forest.empty_name(),
    ///     Utc::now(),
    ///     rng,
    /// ));
    /// let node = PrivateNode::Dir(dir);
    ///
    /// assert!(node.is_dir());
    /// ```
    pub fn is_dir(&self) -> bool {
        matches!(self, Self::Dir(_))
    }

    /// Returns true if the underlying node is a file.
    ///
    /// # Examples
    ///
    /// ```
    /// use wnfs::{
    ///     private::{
    ///         PrivateFile, PrivateNode,
    ///         forest::{hamt::HamtForest, traits::PrivateForest},
    ///     },
    /// };
    /// use chrono::Utc;
    /// use std::rc::Rc;
    /// use rand::thread_rng;
    ///
    /// let rng = &mut thread_rng();
    /// let forest = &mut Rc::new(HamtForest::new_rsa_2048(rng));
    /// let file = Rc::new(PrivateFile::new(
    ///     &forest.empty_name(),
    ///     Utc::now(),
    ///     rng,
    /// ));
    /// let node = PrivateNode::File(file);
    ///
    /// assert!(node.is_file());
    /// ```
    pub fn is_file(&self) -> bool {
        matches!(self, Self::File(_))
    }

    /// Gets the latest version of the node using exponential search.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::rc::Rc;
    /// use chrono::Utc;
    /// use rand::thread_rng;
    /// use wnfs::{
    ///     private::{
    ///         PrivateRef, PrivateNode, PrivateDirectory,
    ///         forest::{hamt::HamtForest, traits::PrivateForest},
    ///     },
    ///     common::{BlockStore, MemoryBlockStore},
    /// };
    ///
    /// #[async_std::main]
    /// async fn main() {
    ///     let store = &mut MemoryBlockStore::default();
    ///     let rng = &mut thread_rng();
    ///     let forest = &mut Rc::new(HamtForest::new_rsa_2048(rng));
    ///
    ///     let mut init_dir = PrivateDirectory::new_and_store(
    ///         &forest.empty_name(),
    ///         Utc::now(),
    ///         forest,
    ///         store,
    ///         rng
    ///     ).await.unwrap();
    ///
    ///     let dir_clone = &mut Rc::clone(&init_dir);
    ///
    ///     dir_clone
    ///         .mkdir(&["pictures".into(), "cats".into()], true, Utc::now(), forest, store, rng)
    ///         .await
    ///         .unwrap();
    ///
    ///     dir_clone.store(forest, store, rng).await.unwrap();
    ///
    ///     let latest_node = PrivateNode::Dir(init_dir).search_latest(forest, store).await.unwrap();
    ///
    ///     let found_node = latest_node
    ///         .as_dir()
    ///         .unwrap()
    ///         .lookup_node("pictures", true, forest, store)
    ///         .await
    ///         .unwrap();
    ///
    ///     assert!(found_node.is_some());
    /// }
    /// ```
    pub async fn search_latest(
        &self,
        forest: &impl PrivateForest,
        store: &impl BlockStore,
    ) -> Result<PrivateNode> {
        self.search_latest_nodes(forest, store)
            .await?
            .into_iter()
            .next()
            // We expect the latest revision to have found valid nodes.
            // otherwise it's a revision that's filled with other stuff
            // than PrivateNodes, which should be an error.
            .ok_or(FsError::NotFound.into())
    }

    /// Seek ahead to the latest revision in this node's history.
    ///
    /// The result are all nodes from the latest revision, each one
    /// representing an instance of a concurrent write.
    pub async fn search_latest_nodes(
        &self,
        forest: &impl PrivateForest,
        store: &impl BlockStore,
    ) -> Result<Vec<PrivateNode>> {
        let header = self.get_header();
        let setup = forest.get_accumulator_setup();
        let mountpoint = header.name.parent().unwrap_or_else(|| forest.empty_name());

        let current_name = &header.get_revision_name();
        if !forest.has(current_name, store).await? {
            return Ok(vec![self.clone()]);
        }

        // Start an exponential search, starting with a small jump.
        // In many cases, we'll be at the latest revision already, so we only
        // do a single lookup to the next version, most likely realize it's not
        // there and thus stop seeking.
        let mut search = RatchetSeeker::new(header.ratchet.clone(), JumpSize::Small);
        let mut current_header = header.clone();

        loop {
            let current = search.current();
            current_header.ratchet = current.clone();

            let has_curr = forest
                .has(&current_header.get_revision_name(), store)
                .await?;

            let ord = if has_curr {
                Ordering::Less
            } else {
                Ordering::Greater
            };

            if !search.step(ord) {
                break;
            }
        }

        current_header.ratchet = search.current().clone();

        Ok(forest
            .get_multivalue(
                &current_header.derive_revision_ref(setup),
                store,
                Some(mountpoint),
            )
            .collect::<Vec<Result<PrivateNode>>>()
            .await
            .into_iter()
            .filter_map(|result| result.ok()) // Should we filter out errors?
            .collect())
    }

    /// Tries to deserialize and decrypt a PrivateNode at provided PrivateRef
    /// from the PrivateForest.
    ///
    /// In case you're loading this node as a sub-node of another node, you need
    /// to provide the `parent_name`, so it can correctly create proofs relative
    /// to the parent name's base for the private forest.
    ///
    /// In case you're loading this node as the entry point into a WNFS, e.g.
    /// initially from an access key that was shared with you, simply provide `None`.
    /// In short, provide `None` iff
    /// - you have a certificate that gives you write access to exactly this node you're
    ///   loading specifically
    /// - you don't intend to prove writes to third parties.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::rc::Rc;
    /// use chrono::Utc;
    /// use rand::thread_rng;
    /// use wnfs::{
    ///     private::{
    ///         PrivateRef, PrivateNode, PrivateDirectory,
    ///         forest::{hamt::HamtForest, traits::PrivateForest},
    ///     },
    ///     common::{BlockStore, MemoryBlockStore},
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
    ///
    ///     let private_ref = node.store(forest, store, rng).await.unwrap();
    ///
    ///     assert_eq!(
    ///         PrivateNode::load(&private_ref, forest, store, None).await.unwrap(),
    ///         node
    ///     );
    /// }
    /// ```
    pub async fn load(
        private_ref: &PrivateRef,
        forest: &impl PrivateForest,
        store: &impl BlockStore,
        parent_name: Option<Name>,
    ) -> Result<PrivateNode> {
        let cid = match forest
            .get_encrypted_by_hash(&private_ref.revision_name_hash, store)
            .await?
        {
            Some(cids) if cids.contains(&private_ref.content_cid) => private_ref.content_cid,
            _ => return Err(FsError::NotFound.into()),
        };

        let setup = forest.get_accumulator_setup();

        Self::from_cid(cid, &private_ref.temporal_key, store, parent_name, setup).await
    }

    pub(crate) async fn from_cid(
        cid: Cid,
        temporal_key: &TemporalKey,
        store: &impl BlockStore,
        parent_name: Option<Name>,
        setup: &AccumulatorSetup,
    ) -> Result<PrivateNode> {
        let encrypted_bytes = store.get_block(&cid).await?;
        let snapshot_key = temporal_key.derive_snapshot_key();
        let bytes = snapshot_key.decrypt(&encrypted_bytes)?;
        let node: PrivateNodeContentSerializable = serde_ipld_dagcbor::from_slice(&bytes)?;
        Ok(match node {
            PrivateNodeContentSerializable::File(file) => {
                let file = PrivateFile::from_serializable(
                    file,
                    temporal_key,
                    cid,
                    store,
                    parent_name,
                    setup,
                )
                .await?;
                PrivateNode::File(Rc::new(file))
            }
            PrivateNodeContentSerializable::Dir(dir) => {
                let dir = PrivateDirectory::from_serializable(
                    dir,
                    temporal_key,
                    cid,
                    store,
                    parent_name,
                    setup,
                )
                .await?;
                PrivateNode::Dir(Rc::new(dir))
            }
        })
    }

    /// Encrypt and store this private node in a private forest
    /// and return the private ref used for finding and decrypting it again.
    pub async fn store(
        &self,
        forest: &mut impl PrivateForest,
        store: &impl BlockStore,
        rng: &mut impl CryptoRngCore,
    ) -> Result<PrivateRef> {
        match self {
            Self::File(file) => file.store(forest, store, rng).await,
            Self::Dir(dir) => dir.store(forest, store, rng).await,
        }
    }

    /// Returns the private ref, if this node has been `.store()`ed before.
    pub(crate) fn get_private_ref(&self, setup: &AccumulatorSetup) -> Option<PrivateRef> {
        match self {
            Self::File(file) => file.get_private_ref(setup),
            Self::Dir(dir) => dir.get_private_ref(setup),
        }
    }

    pub(crate) fn persisted_as(&self) -> &OnceCell<Cid> {
        match self {
            Self::Dir(dir) => &dir.content.persisted_as,
            Self::File(file) => &file.content.persisted_as,
        }
    }
}

impl Id for PrivateNode {
    fn get_id(&self) -> String {
        match self {
            Self::File(file) => file.get_id(),
            Self::Dir(dir) => dir.get_id(),
        }
    }
}

impl From<PrivateFile> for PrivateNode {
    fn from(file: PrivateFile) -> Self {
        Self::File(Rc::new(file))
    }
}

impl From<PrivateDirectory> for PrivateNode {
    fn from(dir: PrivateDirectory) -> Self {
        Self::Dir(Rc::new(dir))
    }
}

//--------------------------------------------------------------------------------------------------
// Tests
//--------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::private::forest::hamt::HamtForest;
    use rand_chacha::ChaCha12Rng;
    use rand_core::SeedableRng;
    use wnfs_common::MemoryBlockStore;

    #[async_std::test]
    async fn serialized_private_node_can_be_deserialized() {
        let rng = &mut ChaCha12Rng::seed_from_u64(0);
        let content = b"Lorem ipsum dolor sit amet";
        let forest = &mut Rc::new(HamtForest::new_rsa_2048(rng));
        let store = &mut MemoryBlockStore::new();

        let file = Rc::new(
            PrivateFile::with_content(
                &forest.empty_name(),
                Utc::now(),
                content.to_vec(),
                forest,
                store,
                rng,
            )
            .await
            .unwrap(),
        );

        let mut directory = Rc::new(PrivateDirectory::new(&forest.empty_name(), Utc::now(), rng));

        directory
            .mkdir(&["music".into()], true, Utc::now(), forest, store, rng)
            .await
            .unwrap();

        let file_node = PrivateNode::File(Rc::clone(&file));
        let dir_node = PrivateNode::Dir(Rc::clone(&directory));

        let file_private_ref = file_node.store(forest, store, rng).await.unwrap();
        let dir_private_ref = dir_node.store(forest, store, rng).await.unwrap();

        let deserialized_file_node = PrivateNode::load(&file_private_ref, forest, store, None)
            .await
            .unwrap();

        let deserialized_dir_node = PrivateNode::load(&dir_private_ref, forest, store, None)
            .await
            .unwrap();

        assert_eq!(file_node, deserialized_file_node);
        assert_eq!(dir_node, deserialized_dir_node);
    }
}
