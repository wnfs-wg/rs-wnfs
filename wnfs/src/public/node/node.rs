//! Public node system in-memory representation.

use super::PublicNodeSerializable;
use crate::{
    error::FsError,
    public::{PublicDirectory, PublicFile},
    traits::Id,
};
use anyhow::{bail, Result};
use async_once_cell::OnceCell;
use chrono::{DateTime, Utc};
use libipld_core::cid::Cid;
use std::{cmp::Ordering, collections::BTreeSet};
use wnfs_common::{utils::Arc, BlockStore, Storable};

//--------------------------------------------------------------------------------------------------
// Type Definitions
//--------------------------------------------------------------------------------------------------

/// A node in the WNFS public file system. This can either be a file or a directory.
///
/// # Examples
///
/// ```
/// use wnfs::public::{PublicDirectory, PublicNode};
/// use chrono::Utc;
///
/// let dir = PublicDirectory::new_rc(Utc::now());
/// let node = PublicNode::Dir(dir);
///
/// println!("Node: {:?}", node);
/// ```
#[derive(Debug, Clone)]
pub enum PublicNode {
    File(Arc<PublicFile>),
    Dir(Arc<PublicDirectory>),
}

//--------------------------------------------------------------------------------------------------
// Implementations
//--------------------------------------------------------------------------------------------------

impl PublicNode {
    /// Creates node with upserted modified time.
    ///
    /// # Examples
    ///
    /// ```
    /// use wnfs::public::{PublicDirectory, PublicNode};
    /// use chrono::{Utc, Duration, TimeZone};
    ///
    /// let dir = PublicDirectory::new_rc(Utc::now());
    /// let node = &mut PublicNode::Dir(dir);
    ///
    /// let time = Utc::now();
    /// node.upsert_mtime(time);
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
    pub fn upsert_mtime(&mut self, time: DateTime<Utc>) {
        match self {
            Self::File(file) => {
                Arc::make_mut(file).metadata.upsert_mtime(time);
            }
            Self::Dir(dir) => {
                Arc::make_mut(dir).metadata.upsert_mtime(time);
            }
        }
    }

    /// Creates node with updated previous pointer value.
    ///
    /// # Examples
    ///
    /// ```
    /// use wnfs::public::{PublicDirectory, PublicNode};
    /// use chrono::Utc;
    /// use libipld_core::cid::Cid;
    /// use std::{sync::Arc, collections::BTreeSet};
    ///
    /// let dir = PublicDirectory::new_rc(Utc::now());
    /// let node = PublicNode::Dir(dir);
    ///
    /// let new_cids = [Cid::default()];
    /// let node = node.update_previous(new_cids.to_vec());
    ///
    /// assert_eq!(
    ///     &BTreeSet::from(new_cids),
    ///     node.as_dir()
    ///         .unwrap()
    ///         .get_previous()
    /// );
    /// ```
    pub fn update_previous(&self, cids: Vec<Cid>) -> Self {
        match self {
            Self::File(file) => {
                let mut file = (**file).clone();
                file.previous = cids.into_iter().collect();
                Self::File(Arc::new(file))
            }
            Self::Dir(dir) => {
                let mut dir = (**dir).clone();
                dir.previous = cids.into_iter().collect();
                Self::Dir(Arc::new(dir))
            }
        }
    }

    /// Gets previous ancestor of a node.
    ///
    /// # Examples
    ///
    /// ```
    /// use wnfs::public::{PublicDirectory, PublicNode};
    /// use chrono::Utc;
    ///
    /// let dir = PublicDirectory::new_rc(Utc::now());
    /// let node = PublicNode::Dir(dir);
    ///
    /// assert_eq!(
    ///     node.get_previous(),
    ///     node.as_dir()
    ///         .unwrap()
    ///         .get_previous()
    /// );
    /// ```
    pub fn get_previous(&self) -> &BTreeSet<Cid> {
        match self {
            Self::File(file) => file.get_previous(),
            Self::Dir(dir) => dir.get_previous(),
        }
    }

    /// Casts a node to a directory.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::sync::Arc;
    /// use wnfs::public::{PublicDirectory, PublicNode};
    /// use chrono::Utc;
    ///
    /// let dir = PublicDirectory::new_rc(Utc::now());
    /// let node = PublicNode::Dir(Arc::clone(&dir));
    ///
    /// assert_eq!(node.as_dir().unwrap(), dir);
    /// ```
    pub fn as_dir(&self) -> Result<Arc<PublicDirectory>> {
        Ok(match self {
            Self::Dir(dir) => Arc::clone(dir),
            _ => bail!(FsError::NotADirectory),
        })
    }

    /// Casts a node to a mutable directory.
    pub(crate) fn as_dir_mut(&mut self) -> Result<&mut Arc<PublicDirectory>> {
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
    /// use std::sync::Arc;
    /// use wnfs::public::{PublicFile, PublicNode};
    /// use chrono::Utc;
    ///
    /// let file = PublicFile::new_rc(Utc::now());
    /// let node = PublicNode::File(Arc::clone(&file));
    ///
    /// assert_eq!(node.as_file().unwrap(), file);
    /// ```
    pub fn as_file(&self) -> Result<Arc<PublicFile>> {
        Ok(match self {
            Self::File(file) => Arc::clone(file),
            _ => bail!(FsError::NotAFile),
        })
    }

    /// Tries to resolve this node as a file. Fails with `NotAFile` otherwise.
    pub fn as_file_mut(&mut self) -> Result<&mut Arc<PublicFile>> {
        match self {
            Self::File(file) => Ok(file),
            _ => bail!(FsError::NotAFile),
        }
    }

    /// Returns true if underlying node is a directory.
    ///
    /// # Examples
    ///
    /// ```
    /// use wnfs::public::{PublicDirectory, PublicNode};
    /// use chrono::Utc;
    ///
    /// let dir = PublicDirectory::new_rc(Utc::now());
    /// let node = PublicNode::Dir(dir);
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
    /// use wnfs::public::{PublicFile, PublicNode};
    /// use chrono::Utc;
    ///
    /// let file = PublicFile::new_rc(Utc::now());
    /// let node = PublicNode::File(file);
    ///
    /// assert!(node.is_file());
    /// ```
    pub fn is_file(&self) -> bool {
        matches!(self, Self::File(_))
    }

    /// Comparing the merkle clocks of this node to the other node
    pub async fn causal_compare(
        &self,
        other: &Self,
        store: &impl BlockStore,
    ) -> Result<Option<Ordering>> {
        async fn next_previous_set(
            previous_set: BTreeSet<Cid>,
            visited_cids: &mut BTreeSet<Cid>,
            store: &impl BlockStore,
        ) -> Result<BTreeSet<Cid>> {
            let mut previous = BTreeSet::new();

            for cid in previous_set {
                let node = PublicNode::load(&cid, store).await?;
                previous.extend(
                    node.get_previous()
                        .iter()
                        .filter(|cid| visited_cids.insert(**cid))
                        .cloned(),
                );
            }

            Ok(previous)
        }

        let our_root = self.store(store).await?;
        let other_root = other.store(store).await?;

        if our_root == other_root {
            return Ok(Some(Ordering::Equal));
        }

        let mut our_previous_set = self.get_previous().clone();
        let mut other_previous_set = other.get_previous().clone();

        let mut our_visited = BTreeSet::new();
        let mut other_visited = BTreeSet::new();

        loop {
            if other_previous_set.contains(&our_root) {
                return Ok(Some(Ordering::Less));
            }

            if our_previous_set.contains(&other_root) {
                return Ok(Some(Ordering::Greater));
            }

            // early return optimization:
            // If one "previous CIDs frontier" is entirely within the other's visited set,
            // then it for sure can't hit the other root, so we know they diverged.
            let our_is_subset = our_previous_set.is_subset(&other_visited);
            let other_is_subset = other_previous_set.is_subset(&our_visited);
            if our_is_subset || other_is_subset {
                return Ok(None);
            }

            our_previous_set = next_previous_set(our_previous_set, &mut our_visited, store).await?;
            other_previous_set =
                next_previous_set(other_previous_set, &mut other_visited, store).await?;

            if our_previous_set.is_empty() && other_previous_set.is_empty() {
                return Ok(None); // No common causal history
            }
        }
    }
}

impl Id for PublicNode {
    fn get_id(&self) -> String {
        match self {
            PublicNode::File(file) => file.get_id(),
            PublicNode::Dir(dir) => dir.get_id(),
        }
    }
}

impl PartialEq for PublicNode {
    fn eq(&self, other: &PublicNode) -> bool {
        match (self, other) {
            (Self::File(self_file), Self::File(other_file)) => {
                Arc::ptr_eq(self_file, other_file) || self_file == other_file
            }
            (Self::Dir(self_dir), Self::Dir(other_dir)) => {
                Arc::ptr_eq(self_dir, other_dir) || self_dir == other_dir
            }
            _ => false,
        }
    }
}

impl From<PublicFile> for PublicNode {
    fn from(file: PublicFile) -> Self {
        Self::File(Arc::new(file))
    }
}

impl From<PublicDirectory> for PublicNode {
    fn from(dir: PublicDirectory) -> Self {
        Self::Dir(Arc::new(dir))
    }
}

impl Storable for PublicNode {
    type Serializable = PublicNodeSerializable;

    async fn to_serializable(&self, store: &impl BlockStore) -> Result<Self::Serializable> {
        Ok(match self {
            Self::File(file) => file.to_serializable(store).await?,
            Self::Dir(dir) => dir.to_serializable(store).await?,
        })
    }

    async fn from_serializable(
        cid: Option<&Cid>,
        serializable: Self::Serializable,
    ) -> Result<Self> {
        // TODO(matheus23) this is weird, refactor?
        Ok(match serializable {
            PublicNodeSerializable::File(file) => Self::File(Arc::new(
                PublicFile::from_serializable(cid, PublicNodeSerializable::File(file)).await?,
            )),
            PublicNodeSerializable::Dir(dir) => Self::Dir(Arc::new(
                PublicDirectory::from_serializable(cid, PublicNodeSerializable::Dir(dir)).await?,
            )),
        })
    }

    fn persisted_as(&self) -> Option<&OnceCell<Cid>> {
        match self {
            PublicNode::File(file) => file.as_ref().persisted_as(),
            PublicNode::Dir(dir) => dir.as_ref().persisted_as(),
        }
    }
}

//--------------------------------------------------------------------------------------------------
// Tests
//--------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use crate::public::{PublicDirectory, PublicFile, PublicNode};
    use chrono::Utc;
    use testresult::TestResult;
    use wnfs_common::{MemoryBlockStore, Storable};

    #[async_std::test]
    async fn serialized_public_node_can_be_deserialized() -> TestResult {
        let store = &MemoryBlockStore::new();
        let dir_node: PublicNode = PublicDirectory::new(Utc::now()).into();
        let file_node: PublicNode = PublicFile::new(Utc::now()).into();

        // We add a round-trip, because... userland records whether it was newly created/loaded
        let file_node = PublicNode::load(&file_node.store(store).await?, store).await?;

        let dir_cid = dir_node.store(store).await?;
        let file_cid = file_node.store(store).await?;

        let loaded_file_node = PublicNode::load(&file_cid, store).await?;
        let loaded_dir_node = PublicNode::load(&dir_cid, store).await?;

        assert_eq!(loaded_file_node, file_node);
        assert_eq!(loaded_dir_node, dir_node);

        Ok(())
    }
}

#[cfg(test)]
mod snapshot_tests {
    use super::*;
    use chrono::TimeZone;
    use wnfs_common::utils::SnapshotBlockStore;

    #[async_std::test]
    async fn public_file_and_directory_nodes() {
        let store = &SnapshotBlockStore::default();
        let time = Utc.with_ymd_and_hms(1970, 1, 1, 0, 0, 0).unwrap();

        let dir_node: PublicNode = PublicDirectory::new(time).into();
        let file_node: PublicNode = PublicFile::new(time).into();

        let dir_cid = dir_node.store(store).await.unwrap();
        let file_cid = file_node.store(store).await.unwrap();

        let dir = store.get_block_snapshot(&dir_cid).await.unwrap();
        let file = store.get_block_snapshot(&file_cid).await.unwrap();

        insta::assert_json_snapshot!(dir);
        insta::assert_json_snapshot!(file);
    }

    #[async_std::test]
    async fn public_fs() {
        let store = &SnapshotBlockStore::default();
        let time = Utc.with_ymd_and_hms(1970, 1, 1, 0, 0, 0).unwrap();

        let paths = [
            vec!["text.txt".into()],
            vec!["music".into(), "jazz".into()],
            vec!["videos".into(), "movies".into(), "anime".into()],
        ];

        let root_dir = &mut PublicDirectory::new_rc(time);
        let _ = root_dir.store(store).await.unwrap();

        for path in paths.iter() {
            root_dir
                .write(path, b"Hello, World!".to_vec(), time, store)
                .await
                .unwrap();
        }

        let cid = root_dir.store(store).await.unwrap();

        let values = store.get_dag_snapshot(cid).await.unwrap();
        insta::assert_json_snapshot!(values)
    }
}
