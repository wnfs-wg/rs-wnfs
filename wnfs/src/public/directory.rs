//! Public fs directory node.

use super::{PublicFile, PublicLink, PublicNode};
use crate::{
    error::FsError,
    traits::{Id, PrepareMut},
    utils,
};
use anyhow::{bail, ensure, Result};
use async_once_cell::OnceCell;
use async_recursion::async_recursion;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use libipld::Cid;
use semver::Version;
use serde::{
    de::Error as DeError, ser::Error as SerError, Deserialize, Deserializer, Serialize, Serializer,
};
use std::{
    collections::{BTreeMap, BTreeSet},
    rc::Rc,
};
use wnfs_common::{
    utils::error, AsyncSerialize, BlockStore, Metadata, NodeType, PathNodes, PathNodesResult,
    RemembersCid,
};

//--------------------------------------------------------------------------------------------------
// Type Definitions
//--------------------------------------------------------------------------------------------------

pub type PublicPathNodes = PathNodes<PublicDirectory>;
pub type PublicPathNodesResult = PathNodesResult<PublicDirectory>;

/// Represents a directory in the WNFS public filesystem.
///
/// # Examples
///
/// ```
/// use wnfs::public::PublicDirectory;
/// use chrono::Utc;
///
/// let dir = PublicDirectory::new(Utc::now());
///
/// println!("Directory: {:?}", dir);
/// ```
#[derive(Debug)]
pub struct PublicDirectory {
    persisted_as: OnceCell<Cid>,
    pub metadata: Metadata,
    pub userland: BTreeMap<String, PublicLink>,
    pub previous: BTreeSet<Cid>,
}

#[derive(Serialize, Deserialize)]
struct PublicDirectorySerializable {
    r#type: NodeType,
    version: Version,
    metadata: Metadata,
    userland: BTreeMap<String, Cid>,
    previous: Vec<Cid>,
}

/// The result of an basic get operation.
enum SearchState<T> {
    Missing(T, usize),
    NotADir(T, usize),
    Found(T),
}

//--------------------------------------------------------------------------------------------------
// Implementations
//--------------------------------------------------------------------------------------------------

impl PublicDirectory {
    /// Creates a new directory with provided time.
    ///
    /// # Examples
    ///
    /// ```
    /// use wnfs::public::PublicDirectory;
    /// use chrono::Utc;
    ///
    /// let dir = PublicDirectory::new(Utc::now());
    ///
    /// println!("Directory: {:?}", dir);
    /// ```
    pub fn new(time: DateTime<Utc>) -> Self {
        Self {
            persisted_as: OnceCell::new(),
            metadata: Metadata::new(time),
            userland: BTreeMap::new(),
            previous: BTreeSet::new(),
        }
    }

    /// Gets the previous Cids.
    ///
    /// # Examples
    ///
    /// ```
    /// use wnfs::public::PublicDirectory;
    /// use std::{rc::Rc, collections::BTreeSet};
    /// use chrono::Utc;
    ///
    /// let dir = Rc::new(PublicDirectory::new(Utc::now()));
    ///
    /// assert_eq!(dir.get_previous(), &BTreeSet::new());
    /// ```
    #[inline]
    pub fn get_previous<'a>(self: &'a Rc<Self>) -> &'a BTreeSet<Cid> {
        &self.previous
    }

    /// Gets the metadata.
    ///
    /// # Examples
    ///
    /// ```
    /// use wnfs::{public::PublicDirectory, common::Metadata};
    /// use std::rc::Rc;
    /// use chrono::Utc;
    ///
    /// let time = Utc::now();
    /// let dir = Rc::new(PublicDirectory::new(time));
    ///
    /// assert_eq!(dir.get_metadata(), &Metadata::new(time));
    /// ```
    #[inline]
    pub fn get_metadata<'a>(self: &'a Rc<Self>) -> &'a Metadata {
        &self.metadata
    }

    async fn get_leaf_dir<'a>(
        &'a self,
        path_segments: &[String],
        store: &impl BlockStore,
    ) -> Result<SearchState<&'a Self>> {
        let mut working_dir = self;
        for (depth, segment) in path_segments.iter().enumerate() {
            match working_dir.lookup_node(segment, store).await? {
                Some(PublicNode::Dir(ref directory)) => {
                    working_dir = directory;
                }
                Some(_) => return Ok(SearchState::NotADir(working_dir, depth)),
                None => return Ok(SearchState::Missing(working_dir, depth)),
            }
        }

        Ok(SearchState::Found(working_dir))
    }

    async fn get_leaf_dir_mut<'a>(
        self: &'a mut Rc<Self>,
        path_segments: &[String],
        store: &impl BlockStore,
    ) -> Result<SearchState<&'a mut Self>> {
        let mut working_dir = self.prepare_mut();
        for (depth, segment) in path_segments.iter().enumerate() {
            match working_dir.lookup_node(segment, store).await? {
                Some(PublicNode::Dir(_)) => {
                    // We need this repeated lookup because Rust borrowck can't handle
                    // this mut borrow case yet without resorting to the unstable -Zpolonius flag.
                    // https://github.com/rust-lang/rust/issues/51545
                    working_dir = working_dir
                        .lookup_node_mut(segment, store)
                        .await
                        .unwrap()
                        .unwrap()
                        .as_dir_mut()
                        .unwrap()
                        .prepare_mut()
                }
                Some(_) => return Ok(SearchState::NotADir(working_dir, depth)),
                None => return Ok(SearchState::Missing(working_dir, depth)),
            };
        }

        Ok(SearchState::Found(working_dir))
    }

    async fn get_or_create_leaf_dir_mut<'a>(
        self: &'a mut Rc<Self>,
        path_segments: &[String],
        time: DateTime<Utc>,
        store: &impl BlockStore,
    ) -> Result<&'a mut Self> {
        match self.get_leaf_dir_mut(path_segments, store).await? {
            SearchState::Found(dir) => Ok(dir),
            SearchState::Missing(mut dir, depth) => {
                for segment in &path_segments[depth..] {
                    dir = Rc::make_mut(
                        dir.userland
                            .entry(segment.to_string())
                            .or_insert_with(|| PublicLink::with_dir(Self::new(time)))
                            .resolve_value_mut(store)
                            .await
                            .unwrap()
                            .as_dir_mut()
                            .unwrap(),
                    );
                }

                Ok(dir)
            }
            SearchState::NotADir(_, _) => bail!(FsError::NotADirectory),
        }
    }

    /// Follows a path and fetches the node at the end of the path.
    ///
    /// # Examples
    ///
    /// ```
    /// use wnfs::{
    ///     public::PublicDirectory,
    ///     common::MemoryBlockStore
    /// };
    /// use std::rc::Rc;
    /// use chrono::Utc;
    ///
    /// #[async_std::main]
    /// async fn main() {
    ///     let dir = &mut Rc::new(PublicDirectory::new(Utc::now()));
    ///     let store = MemoryBlockStore::default();
    ///
    ///     dir
    ///         .mkdir(&["pictures".into(), "cats".into()], Utc::now(), &store)
    ///         .await
    ///         .unwrap();
    ///
    ///     let result = dir
    ///         .get_node(&["pictures".into()], &store)
    ///         .await
    ///         .unwrap();
    ///
    ///     assert!(result.is_some());
    /// }
    /// ```
    pub async fn get_node<'a>(
        &'a self,
        path_segments: &[String],
        store: &impl BlockStore,
    ) -> Result<Option<&'a PublicNode>> {
        let Some((tail, path)) = path_segments.split_last() else {
            bail!(FsError::InvalidPath);
        };

        let SearchState::Found(dir) = self.get_leaf_dir(path, store).await? else {
            bail!(FsError::NotFound);
        };

        dir.lookup_node(tail, store).await
    }

    /// Looks up a node by its path name in the current directory.
    ///
    /// # Examples
    ///
    /// ```
    /// use wnfs::{
    ///     public::PublicDirectory,
    ///     traits::Id,
    ///     common::MemoryBlockStore
    /// };
    /// use std::rc::Rc;
    /// use chrono::Utc;
    ///
    /// #[async_std::main]
    /// async fn main() {
    ///     let dir = &mut Rc::new(PublicDirectory::new(Utc::now()));
    ///     let mut store = MemoryBlockStore::default();
    ///
    ///     dir
    ///         .mkdir(&["pictures".into(), "cats".into()], Utc::now(), &store)
    ///         .await
    ///         .unwrap();
    ///
    ///     let node = dir.lookup_node("pictures", &store).await.unwrap();
    ///
    ///     assert!(node.is_some());
    /// }
    /// ```
    pub async fn lookup_node<'a>(
        &'a self,
        path_segment: &str,
        store: &impl BlockStore,
    ) -> Result<Option<&'a PublicNode>> {
        Ok(match self.userland.get(path_segment) {
            Some(link) => Some(link.resolve_value(store).await?),
            None => None,
        })
    }

    pub async fn lookup_node_mut<'a>(
        &'a mut self,
        path_segment: &str,
        store: &impl BlockStore,
    ) -> Result<Option<&'a mut PublicNode>> {
        Ok(match self.userland.get_mut(path_segment) {
            Some(link) => Some(link.resolve_value_mut(store).await?),
            None => None,
        })
    }

    #[async_recursion(?Send)]
    /// Stores directory in provided block store.
    ///
    /// This function can be recursive if the directory contains other directories.
    ///
    /// # Examples
    ///
    /// ```
    /// use wnfs::{
    ///     public::PublicDirectory,
    ///     traits::Id,
    ///     common::{BlockStore, MemoryBlockStore}
    /// };
    /// use chrono::Utc;
    ///
    /// #[async_std::main]
    /// async fn main() {
    ///     let store = &mut MemoryBlockStore::default();
    ///     let dir = PublicDirectory::new(Utc::now());
    ///
    ///     let cid = dir.store(store).await.unwrap();
    ///
    ///     assert_eq!(
    ///         dir,
    ///         store.get_deserializable(&cid).await.unwrap()
    ///     );
    /// }
    /// ```
    pub async fn store(&self, store: &mut impl BlockStore) -> Result<Cid> {
        Ok(*self
            .persisted_as
            .get_or_try_init(store.put_async_serializable(self))
            .await?)
    }

    /// Reads specified file content from the directory.
    ///
    /// # Examples
    ///
    /// ```
    /// use wnfs::{
    ///     public::PublicDirectory,
    ///     common::MemoryBlockStore
    /// };
    /// use libipld::cid::Cid;
    /// use std::rc::Rc;
    /// use chrono::Utc;
    ///
    /// #[async_std::main]
    /// async fn main() {
    ///     let dir = &mut Rc::new(PublicDirectory::new(Utc::now()));
    ///     let store = &mut MemoryBlockStore::default();
    ///     let cid = Cid::default();
    ///
    ///     dir
    ///         .write(
    ///             &["pictures".into(), "cats".into(), "tabby.png".into()],
    ///             cid,
    ///             Utc::now(),
    ///             store
    ///         )
    ///         .await
    ///         .unwrap();
    ///
    ///     let result = dir
    ///         .read(&["pictures".into(), "cats".into(), "tabby.png".into()], store)
    ///         .await
    ///         .unwrap();
    ///
    ///     assert_eq!(result, cid);
    /// }
    /// ```
    pub async fn read(&self, path_segments: &[String], store: &impl BlockStore) -> Result<Cid> {
        let (path, filename) = utils::split_last(path_segments)?;
        match self.get_leaf_dir(path, store).await? {
            SearchState::Found(dir) => match dir.lookup_node(filename, store).await? {
                Some(PublicNode::File(file)) => Ok(file.userland),
                Some(_) => error(FsError::NotAFile),
                None => error(FsError::NotFound),
            },
            _ => error(FsError::NotFound),
        }
    }

    /// Writes a file to the directory.
    ///
    /// # Examples
    ///
    /// ```
    /// use wnfs::{
    ///     public::PublicDirectory,
    ///     common::MemoryBlockStore
    /// };
    /// use libipld::cid::Cid;
    /// use std::rc::Rc;
    /// use chrono::Utc;
    ///
    /// #[async_std::main]
    /// async fn main() {
    ///     let dir = &mut Rc::new(PublicDirectory::new(Utc::now()));
    ///     let store = &MemoryBlockStore::default();
    ///
    ///     dir
    ///         .write(
    ///             &["pictures".into(), "cats".into(), "tabby.png".into()],
    ///             Cid::default(),
    ///             Utc::now(),
    ///             store
    ///         )
    ///         .await
    ///         .unwrap();
    /// }
    /// ```
    pub async fn write(
        self: &mut Rc<Self>,
        path_segments: &[String],
        content_cid: Cid,
        time: DateTime<Utc>,
        store: &impl BlockStore,
    ) -> Result<()> {
        let (path, filename) = utils::split_last(path_segments)?;
        let dir = self.get_or_create_leaf_dir_mut(path, time, store).await?;

        match dir.lookup_node_mut(filename, store).await? {
            Some(PublicNode::File(file)) => {
                let file = file.prepare_mut();
                file.userland = content_cid;
                file.metadata.upsert_mtime(time);
            }
            Some(PublicNode::Dir(_)) => bail!(FsError::DirectoryAlreadyExists),
            None => {
                dir.userland.insert(
                    filename.to_string(),
                    PublicLink::with_file(PublicFile::new(time, content_cid)),
                );
            }
        }

        Ok(())
    }

    /// Creates a new directory at the specified path.
    ///
    /// # Examples
    ///
    /// ```
    /// use wnfs::{
    ///     public::PublicDirectory,
    ///     traits::Id,
    ///     common::MemoryBlockStore
    /// };
    /// use std::rc::Rc;
    /// use chrono::Utc;
    ///
    /// #[async_std::main]
    /// async fn main() {
    ///     let dir = &mut Rc::new(PublicDirectory::new(Utc::now()));
    ///     let store = MemoryBlockStore::default();
    ///
    ///     dir
    ///         .mkdir(&["pictures".into(), "cats".into()], Utc::now(), &store)
    ///         .await
    ///         .unwrap();
    ///
    ///     let result = dir
    ///         .ls(&["pictures".into()], &store)
    ///         .await
    ///         .unwrap();
    ///
    ///     assert_eq!(result.len(), 1);
    ///     assert_eq!(result[0].0, "cats");
    /// }
    /// ```
    ///
    /// This method acts like `mkdir -p` in Unix because it creates intermediate directories if they do not exist.
    pub async fn mkdir(
        self: &mut Rc<Self>,
        path_segments: &[String],
        time: DateTime<Utc>,
        store: &impl BlockStore,
    ) -> Result<()> {
        let _ = self
            .get_or_create_leaf_dir_mut(path_segments, time, store)
            .await?;

        Ok(())
    }

    /// Returns names and metadata of directory's immediate children.
    ///
    /// # Examples
    ///
    /// ```
    /// use wnfs::{
    ///     public::PublicDirectory,
    ///     common::MemoryBlockStore
    /// };
    /// use libipld::cid::Cid;
    /// use std::rc::Rc;
    /// use chrono::Utc;
    ///
    /// #[async_std::main]
    /// async fn main() {
    ///     let dir = &mut Rc::new(PublicDirectory::new(Utc::now()));
    ///     let store = MemoryBlockStore::default();
    ///
    ///     dir
    ///         .write(
    ///             &["pictures".into(), "cats".into(), "tabby.png".into()],
    ///             Cid::default(),
    ///             Utc::now(),
    ///             &store
    ///         )
    ///         .await
    ///         .unwrap();
    ///
    ///     let result = dir
    ///         .ls(&["pictures".into(), "cats".into()], &store)
    ///         .await
    ///         .unwrap();
    ///
    ///     assert_eq!(result.len(), 1);
    ///     assert_eq!(result[0].0, "tabby.png");
    /// }
    /// ```
    pub async fn ls(
        &self,
        path_segments: &[String],
        store: &impl BlockStore,
    ) -> Result<Vec<(String, Metadata)>> {
        match self.get_leaf_dir(path_segments, store).await? {
            SearchState::Found(dir) => {
                let mut result = vec![];
                for (name, link) in dir.userland.iter() {
                    match link.resolve_value(store).await? {
                        PublicNode::File(file) => {
                            result.push((name.clone(), file.metadata.clone()));
                        }
                        PublicNode::Dir(dir) => {
                            result.push((name.clone(), dir.metadata.clone()));
                        }
                    }
                }
                Ok(result)
            }
            SearchState::NotADir(_, _) => bail!(FsError::NotADirectory),
            _ => bail!(FsError::NotFound),
        }
    }

    /// Removes a file or directory from the directory.
    ///
    /// # Examples
    ///
    /// ```
    /// use wnfs::{
    ///     public::PublicDirectory,
    ///     common::MemoryBlockStore
    /// };
    /// use libipld::cid::Cid;
    /// use std::rc::Rc;
    /// use chrono::Utc;
    ///
    /// #[async_std::main]
    /// async fn main() {
    ///     let dir = &mut Rc::new(PublicDirectory::new(Utc::now()));
    ///     let store = MemoryBlockStore::default();
    ///
    ///     dir
    ///         .write(
    ///             &["pictures".into(), "cats".into(), "tabby.png".into()],
    ///             Cid::default(),
    ///             Utc::now(),
    ///             &store
    ///         )
    ///         .await
    ///         .unwrap();
    ///
    ///     let result = dir
    ///         .ls(&["pictures".into()], &store)
    ///         .await
    ///         .unwrap();
    ///
    ///     assert_eq!(result.len(), 1);
    ///
    ///     dir
    ///         .rm(&["pictures".into(), "cats".into()], &store)
    ///         .await
    ///         .unwrap();
    ///
    ///     let result = dir
    ///         .ls(&["pictures".into()], &store)
    ///         .await
    ///         .unwrap();
    ///
    ///     assert_eq!(result.len(), 0);
    /// }
    /// ```
    pub async fn rm(
        self: &mut Rc<Self>,
        path_segments: &[String],
        store: &impl BlockStore,
    ) -> Result<PublicNode> {
        let (path, node_name) = utils::split_last(path_segments)?;

        let SearchState::Found(dir) = self.get_leaf_dir_mut(path, store).await? else {
            bail!(FsError::NotFound)
        };

        let removed_node = match dir.userland.remove(node_name) {
            Some(link) => link.resolve_owned_value(store).await?,
            None => bail!(FsError::NotFound),
        };

        Ok(removed_node)
    }

    /// Moves a file or directory from one path to another.
    ///
    /// This function requires stating the destination name explicitly.
    ///
    /// # Examples
    ///
    /// ```
    /// use wnfs::{
    ///     public::PublicDirectory,
    ///     common::MemoryBlockStore
    /// };
    /// use libipld::cid::Cid;
    /// use std::rc::Rc;
    /// use chrono::Utc;
    ///
    /// #[async_std::main]
    /// async fn main() {
    ///     let dir = &mut Rc::new(PublicDirectory::new(Utc::now()));
    ///     let store = MemoryBlockStore::default();
    ///
    ///     dir
    ///         .write(
    ///             &["pictures".into(), "cats".into(), "tabby.png".into()],
    ///             Cid::default(),
    ///             Utc::now(),
    ///             &store
    ///         )
    ///         .await
    ///         .unwrap();
    ///
    ///     dir
    ///         .basic_mv(
    ///             &["pictures".into(), "cats".into()],
    ///             &["cats".into()],
    ///             Utc::now(),
    ///             &store
    ///         )
    ///         .await
    ///         .unwrap();
    ///
    ///     let result = dir
    ///         .ls(&[], &store)
    ///         .await
    ///         .unwrap();
    ///
    ///     assert_eq!(result.len(), 2);
    /// }
    /// ```
    pub async fn basic_mv(
        self: &mut Rc<Self>,
        path_segments_from: &[String],
        path_segments_to: &[String],
        time: DateTime<Utc>,
        store: &impl BlockStore,
    ) -> Result<()> {
        let (path, filename) = utils::split_last(path_segments_to)?;
        let mut removed_node = self.rm(path_segments_from, store).await?;

        let SearchState::Found(dir) = self.get_leaf_dir_mut(path, store).await? else {
            bail!(FsError::NotFound);
        };

        ensure!(
            !dir.userland.contains_key(filename),
            FsError::FileAlreadyExists
        );

        removed_node.upsert_mtime(time);

        dir.userland
            .insert(filename.clone(), PublicLink::new(removed_node));

        Ok(())
    }
}

impl PrepareMut for PublicDirectory {
    /// Takes care of creating previous links, in case the current
    /// directory was previously `.store()`ed.
    /// In any case it'll try to give you ownership of the directory if possible,
    /// otherwise it clones.
    fn prepare_mut<'a>(self: &'a mut Rc<PublicDirectory>) -> &'a mut PublicDirectory {
        let Some(previous_cid) = self.persisted_as.get().cloned() else {
            return Rc::make_mut(self);
        };

        let cloned = Rc::make_mut(self);
        cloned.persisted_as = OnceCell::new();
        cloned.previous = [previous_cid].into_iter().collect();
        cloned
    }
}

impl Id for PublicDirectory {
    fn get_id(&self) -> String {
        format!("{:p}", &self.metadata)
    }
}

impl PartialEq for PublicDirectory {
    fn eq(&self, other: &Self) -> bool {
        self.metadata == other.metadata
            && self.userland == other.userland
            && self.previous == other.previous
    }
}

impl Clone for PublicDirectory {
    fn clone(&self) -> Self {
        Self {
            persisted_as: OnceCell::new_with(self.persisted_as.get().cloned()),
            metadata: self.metadata.clone(),
            userland: self.userland.clone(),
            previous: self.previous.clone(),
        }
    }
}

impl RemembersCid for PublicDirectory {
    fn persisted_as(&self) -> &OnceCell<Cid> {
        &self.persisted_as
    }
}

/// Implements async deserialization for serde serializable types.
#[async_trait(?Send)]
impl AsyncSerialize for PublicDirectory {
    async fn async_serialize<S, B>(&self, serializer: S, store: &mut B) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        B: BlockStore + ?Sized,
    {
        let encoded_userland = {
            let mut map = BTreeMap::new();
            for (name, link) in self.userland.iter() {
                map.insert(
                    name.clone(),
                    *link.resolve_cid(store).await.map_err(SerError::custom)?,
                );
            }
            map
        };

        (PublicDirectorySerializable {
            r#type: NodeType::PublicDirectory,
            version: Version::new(0, 2, 0),
            metadata: self.metadata.clone(),
            userland: encoded_userland,
            previous: self.previous.iter().cloned().collect(),
        })
        .serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for PublicDirectory {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let PublicDirectorySerializable {
            r#type,
            version,
            metadata,
            userland,
            previous,
        } = PublicDirectorySerializable::deserialize(deserializer)?;

        if version.major != 0 || version.minor != 2 {
            return Err(DeError::custom(FsError::UnexpectedVersion(version)));
        }

        if r#type != NodeType::PublicDirectory {
            return Err(DeError::custom(FsError::UnexpectedNodeType(r#type)));
        }

        let userland = userland
            .into_iter()
            .map(|(name, cid)| (name, PublicLink::from_cid(cid)))
            .collect();

        Ok(Self {
            persisted_as: OnceCell::new(),
            metadata,
            userland,
            previous: previous.iter().cloned().collect(),
        })
    }
}

//--------------------------------------------------------------------------------------------------
// Tests
//--------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use libipld::Ipld;
    use wnfs_common::{dagcbor, MemoryBlockStore};

    #[async_std::test]
    async fn look_up_can_fetch_file_added_to_directory() {
        let root_dir = &mut Rc::new(PublicDirectory::new(Utc::now()));
        let store = MemoryBlockStore::default();
        let content_cid = Cid::default();
        let time = Utc::now();

        root_dir
            .write(&["text.txt".into()], content_cid, time, &store)
            .await
            .unwrap();

        let node = root_dir.lookup_node("text.txt", &store).await.unwrap();

        assert!(node.is_some());

        assert_eq!(
            node,
            Some(&PublicNode::File(Rc::new(PublicFile::new(
                time,
                content_cid
            ))))
        );
    }

    #[async_std::test]
    async fn look_up_cannot_fetch_file_not_added_to_directory() {
        let root = PublicDirectory::new(Utc::now());
        let store = MemoryBlockStore::default();

        let node = root.lookup_node("Unknown", &store).await;

        assert!(node.is_ok());

        assert_eq!(node.unwrap(), None);
    }

    #[async_std::test]
    async fn directory_added_to_store_can_be_retrieved() {
        let root = PublicDirectory::new(Utc::now());
        let store = &mut MemoryBlockStore::default();

        let cid = root.store(store).await.unwrap();

        let encoded_dir = store.get_block(&cid).await.unwrap();
        let deserialized_dir = dagcbor::decode::<PublicDirectory>(encoded_dir.as_ref()).unwrap();

        assert_eq!(root, deserialized_dir);
    }

    #[async_std::test]
    async fn directory_can_encode_decode_as_cbor() {
        let root = PublicDirectory::new(Utc::now());
        let store = &mut MemoryBlockStore::default();

        let encoded_dir = dagcbor::async_encode(&root, store).await.unwrap();
        let decoded_dir = dagcbor::decode::<PublicDirectory>(encoded_dir.as_ref()).unwrap();

        assert_eq!(root, decoded_dir);
    }

    #[async_std::test]
    async fn mkdir_can_create_new_directory() {
        let time = Utc::now();
        let store = MemoryBlockStore::default();
        let root_dir = &mut Rc::new(PublicDirectory::new(time));

        root_dir
            .mkdir(&["tamedun".into(), "pictures".into()], time, &store)
            .await
            .unwrap();

        let result = root_dir
            .get_node(&["tamedun".into(), "pictures".into()], &store)
            .await
            .unwrap();

        assert!(result.is_some());
    }

    #[async_std::test]
    async fn ls_can_list_children_under_directory() {
        let time = Utc::now();
        let store = MemoryBlockStore::default();
        let root_dir = &mut Rc::new(PublicDirectory::new(time));

        root_dir
            .mkdir(&["tamedun".into(), "pictures".into()], time, &store)
            .await
            .unwrap();

        root_dir
            .write(
                &["tamedun".into(), "pictures".into(), "puppy.jpg".into()],
                Cid::default(),
                time,
                &store,
            )
            .await
            .unwrap();

        root_dir
            .mkdir(
                &["tamedun".into(), "pictures".into(), "cats".into()],
                time,
                &store,
            )
            .await
            .unwrap();

        let result = root_dir
            .ls(&["tamedun".into(), "pictures".into()], &store)
            .await
            .unwrap();

        assert_eq!(result.len(), 2);

        assert_eq!(result[0].0, String::from("cats"));

        assert_eq!(result[1].0, String::from("puppy.jpg"));
    }

    #[async_std::test]
    async fn rm_can_remove_children_from_directory() {
        let time = Utc::now();
        let store = MemoryBlockStore::default();
        let mut root_dir = Rc::new(PublicDirectory::new(time));

        root_dir
            .mkdir(&["tamedun".into(), "pictures".into()], time, &store)
            .await
            .unwrap();

        root_dir
            .write(
                &["tamedun".into(), "pictures".into(), "puppy.jpg".into()],
                Cid::default(),
                time,
                &store,
            )
            .await
            .unwrap();

        root_dir
            .mkdir(
                &["tamedun".into(), "pictures".into(), "cats".into()],
                time,
                &store,
            )
            .await
            .unwrap();

        let result = root_dir
            .rm(&["tamedun".into(), "pictures".into()], &store)
            .await;

        assert!(result.is_ok());

        let result = root_dir
            .rm(&["tamedun".into(), "pictures".into()], &store)
            .await;

        assert!(result.is_err());
    }

    #[async_std::test]
    async fn read_can_fetch_userland_of_file_added_to_directory() {
        let store = MemoryBlockStore::default();
        let content_cid = Cid::default();
        let time = Utc::now();
        let mut root_dir = Rc::new(PublicDirectory::new(time));

        root_dir
            .write(&["text.txt".into()], content_cid, time, &store)
            .await
            .unwrap();

        let result = root_dir.read(&["text.txt".into()], &store).await.unwrap();

        assert_eq!(result, content_cid);
    }

    #[async_std::test]
    async fn mv_can_move_sub_directory_to_another_valid_location() {
        let time = Utc::now();
        let store = MemoryBlockStore::default();
        let mut root_dir = Rc::new(PublicDirectory::new(time));

        root_dir
            .write(
                &["pictures".into(), "cats".into(), "tabby.jpg".into()],
                Cid::default(),
                time,
                &store,
            )
            .await
            .unwrap();

        root_dir
            .write(
                &["pictures".into(), "cats".into(), "luna.png".into()],
                Cid::default(),
                time,
                &store,
            )
            .await
            .unwrap();

        root_dir
            .mkdir(&["images".into()], time, &store)
            .await
            .unwrap();

        root_dir
            .basic_mv(
                &["pictures".into(), "cats".into()],
                &["images".into(), "cats".into()],
                Utc::now(),
                &store,
            )
            .await
            .unwrap();

        let result = root_dir.ls(&["images".into()], &store).await.unwrap();

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].0, String::from("cats"));

        let result = root_dir.ls(&["pictures".into()], &store).await.unwrap();

        assert_eq!(result.len(), 0);
    }

    #[async_std::test]
    async fn mv_cannot_move_sub_directory_to_invalid_location() {
        let time = Utc::now();
        let store = MemoryBlockStore::default();
        let mut root_dir = Rc::new(PublicDirectory::new(time));

        root_dir
            .mkdir(
                &[
                    "videos".into(),
                    "movies".into(),
                    "anime".into(),
                    "ghibli".into(),
                ],
                time,
                &store,
            )
            .await
            .unwrap();

        let result = root_dir
            .basic_mv(
                &["videos".into(), "movies".into()],
                &["videos".into(), "movies".into(), "anime".into()],
                Utc::now(),
                &store,
            )
            .await;

        assert!(result.is_err());
    }

    #[async_std::test]
    async fn mv_can_rename_directories() {
        let time = Utc::now();
        let store = MemoryBlockStore::default();
        let root_dir = &mut Rc::new(PublicDirectory::new(time));

        root_dir
            .write(&["file.txt".into()], Cid::default(), time, &store)
            .await
            .unwrap();

        root_dir
            .basic_mv(
                &["file.txt".into()],
                &["renamed.txt".into()],
                Utc::now(),
                &store,
            )
            .await
            .unwrap();

        let result = root_dir
            .read(&["renamed.txt".into()], &store)
            .await
            .unwrap();

        assert!(result == Cid::default());
    }

    #[async_std::test]
    async fn mv_fails_moving_directories_to_files() {
        let time = Utc::now();
        let store = MemoryBlockStore::default();
        let root_dir = &mut Rc::new(PublicDirectory::new(time));

        root_dir
            .mkdir(&["movies".into(), "ghibli".into()], time, &store)
            .await
            .unwrap();

        root_dir
            .write(&["file.txt".into()], Cid::default(), time, &store)
            .await
            .unwrap();

        let result = root_dir
            .basic_mv(
                &["movies".into(), "ghibli".into()],
                &["file.txt".into()],
                Utc::now(),
                &store,
            )
            .await;

        assert!(result.is_err());
    }

    #[async_std::test]
    async fn previous_links_get_set() {
        let time = Utc::now();
        let store = &mut MemoryBlockStore::default();
        let root_dir = &mut Rc::new(PublicDirectory::new(time));

        let previous_cid = root_dir.store(store).await.unwrap();
        let root_dir_clone = &mut Rc::clone(root_dir);
        root_dir_clone
            .mkdir(&["test".into()], time, store)
            .await
            .unwrap();

        let ipld = root_dir_clone.async_serialize_ipld(store).await.unwrap();
        match ipld {
            Ipld::Map(map) => match map.get("previous") {
                Some(Ipld::List(previous)) => {
                    assert_eq!(previous, &vec![Ipld::Link(previous_cid)]);
                }
                _ => panic!("Expected 'previous' key to be a list"),
            },
            _ => panic!("Expected map!"),
        }
    }

    #[async_std::test]
    async fn prepare_mut_shortcuts_if_possible() {
        let time = Utc::now();
        let store = &mut MemoryBlockStore::default();
        let root_dir = &mut Rc::new(PublicDirectory::new(time));

        let previous_cid = &root_dir.store(store).await.unwrap();
        let next_root_dir = root_dir.prepare_mut();
        let next_root_dir_clone = &mut Rc::new(next_root_dir.clone());
        let yet_another_dir = next_root_dir_clone.prepare_mut();

        assert_eq!(
            yet_another_dir.previous.iter().collect::<Vec<_>>(),
            vec![previous_cid]
        );
    }
}
