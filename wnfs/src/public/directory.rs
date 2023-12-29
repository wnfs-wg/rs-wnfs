//! Public fs directory node.

use super::{
    PublicDirectorySerializable, PublicFile, PublicLink, PublicNode, PublicNodeSerializable,
};
use crate::{
    error::FsError, is_readable_wnfs_version, traits::Id, utils, SearchResult, WNFS_VERSION,
};
use anyhow::{bail, ensure, Result};
use async_once_cell::OnceCell;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use libipld_core::cid::Cid;
use std::collections::{BTreeMap, BTreeSet};
use wnfs_common::{
    utils::{error, Arc},
    BlockStore, Metadata, NodeType, Storable,
};

//--------------------------------------------------------------------------------------------------
// Type Definitions
//--------------------------------------------------------------------------------------------------

/// A directory in the WNFS public file system.
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
    pub(crate) metadata: Metadata,
    pub(crate) userland: BTreeMap<String, PublicLink>,
    pub(crate) previous: BTreeSet<Cid>,
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

    /// Creates an `Arc` wrapped directory.
    ///
    /// # Examples
    ///
    /// ```
    /// use wnfs::public::PublicDirectory;
    /// use chrono::Utc;
    ///
    /// let dir = PublicDirectory::new_rc(Utc::now());
    ///
    /// println!("Directory: {:?}", dir);
    /// ```
    pub fn new_rc(time: DateTime<Utc>) -> Arc<Self> {
        Arc::new(Self::new(time))
    }

    /// Gets the previous Cids.
    ///
    /// # Examples
    ///
    /// ```
    /// use wnfs::public::PublicDirectory;
    /// use std::{sync::Arc, collections::BTreeSet};
    /// use chrono::Utc;
    ///
    /// let dir = PublicDirectory::new_rc(Utc::now());
    ///
    /// assert_eq!(dir.get_previous(), &BTreeSet::new());
    /// ```
    #[inline]
    pub fn get_previous<'a>(self: &'a Arc<Self>) -> &'a BTreeSet<Cid> {
        &self.previous
    }

    /// Gets the metadata.
    ///
    /// # Examples
    ///
    /// ```
    /// use wnfs::{public::PublicDirectory, common::Metadata};
    /// use chrono::Utc;
    ///
    /// let time = Utc::now();
    /// let dir = PublicDirectory::new_rc(time);
    ///
    /// assert_eq!(dir.get_metadata(), &Metadata::new(time));
    /// ```
    #[inline]
    pub fn get_metadata<'a>(self: &'a Arc<Self>) -> &'a Metadata {
        &self.metadata
    }

    /// Returns a mutable reference to this directory's metadata.
    pub fn get_metadata_mut(&mut self) -> &mut Metadata {
        &mut self.metadata
    }

    /// Returns a mutable reference to this directory's metadata and ratchets forward the history, if necessary.
    pub fn get_metadata_mut_rc<'a>(self: &'a mut Arc<Self>) -> &'a mut Metadata {
        self.prepare_next_revision().get_metadata_mut()
    }

    /// Takes care of creating previous links, in case the current
    /// directory was previously `.store()`ed.
    /// In any case it'll try to give you ownership of the directory if possible,
    /// otherwise it clones.
    pub(crate) fn prepare_next_revision<'a>(self: &'a mut Arc<Self>) -> &'a mut Self {
        let Some(previous_cid) = self.persisted_as.get().cloned() else {
            return Arc::make_mut(self);
        };

        let cloned = Arc::make_mut(self);
        cloned.persisted_as = OnceCell::new();
        cloned.previous = [previous_cid].into_iter().collect();
        cloned
    }

    async fn get_leaf_dir<'a>(
        &'a self,
        path_segments: &[String],
        store: &impl BlockStore,
    ) -> Result<SearchResult<&'a Self>> {
        let mut working_dir = self;
        for (depth, segment) in path_segments.iter().enumerate() {
            match working_dir.lookup_node(segment, store).await? {
                Some(PublicNode::Dir(directory)) => {
                    working_dir = directory.as_ref();
                }
                Some(_) => return Ok(SearchResult::NotADir(working_dir, depth)),
                None => return Ok(SearchResult::Missing(working_dir, depth)),
            }
        }

        Ok(SearchResult::Found(working_dir))
    }

    async fn get_leaf_dir_mut<'a>(
        self: &'a mut Arc<Self>,
        path_segments: &[String],
        store: &impl BlockStore,
    ) -> Result<SearchResult<&'a mut Self>> {
        // TODO(matheus23) actually set the modification time of all these nodes
        let mut working_dir = self.prepare_next_revision();
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
                        .prepare_next_revision()
                }
                Some(_) => return Ok(SearchResult::NotADir(working_dir, depth)),
                None => return Ok(SearchResult::Missing(working_dir, depth)),
            };
        }

        Ok(SearchResult::Found(working_dir))
    }

    async fn get_or_create_leaf_dir_mut<'a>(
        self: &'a mut Arc<Self>,
        path_segments: &[String],
        time: DateTime<Utc>,
        store: &impl BlockStore,
    ) -> Result<&'a mut Self> {
        match self.get_leaf_dir_mut(path_segments, store).await? {
            SearchResult::Found(dir) => Ok(dir),
            SearchResult::Missing(mut dir, depth) => {
                for segment in &path_segments[depth..] {
                    dir = Arc::make_mut(
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
            SearchResult::NotADir(_, _) => bail!(FsError::NotADirectory),
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
    /// use chrono::Utc;
    ///
    /// #[async_std::main]
    /// async fn main() {
    ///     let dir = &mut PublicDirectory::new_rc(Utc::now());
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
            return Ok(None);
        };

        let SearchResult::Found(dir) = self.get_leaf_dir(path, store).await? else {
            return Ok(None);
        };

        dir.lookup_node(tail, store).await
    }

    /// Opens a file at given path, or creates a new one if it was missing.
    /// Also creates the intermediate directories if they didn't exist before.
    /// Updates the modification time for everything on the path.
    ///
    /// # Examples
    ///
    /// ```
    /// use anyhow::Result;
    /// use wnfs::{
    ///     public::PublicDirectory,
    ///     common::MemoryBlockStore
    /// };
    /// use std::sync::Arc;
    /// use chrono::Utc;
    /// use wnfs_common::libipld::Ipld;
    ///
    /// #[async_std::main]
    /// async fn main() -> Result<()> {
    ///     let dir = &mut Arc::new(PublicDirectory::new(Utc::now()));
    ///     let store = &MemoryBlockStore::new();
    ///
    ///     // Gain a mutable file reference
    ///     let path = &["Documents".into(), "Notes.md".into()];
    ///     let file = dir.open_file_mut(path, Utc::now(), store).await?;
    ///
    ///     let metadata = Ipld::String("Hello Metadata!".into());
    ///     file.get_metadata_mut().put("custom-metadata", metadata.clone());
    ///
    ///     // We can later look up the file again
    ///     let file = dir.get_node(path, store).await?.unwrap().as_file()?;
    ///
    ///     assert_eq!(file.get_metadata().get("custom-metadata"), Some(&metadata));
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn open_file_mut<'a>(
        self: &'a mut Arc<Self>,
        path_segments: &[String],
        time: DateTime<Utc>,
        store: &'a impl BlockStore,
    ) -> Result<&'a mut PublicFile> {
        let (path, filename) = utils::split_last(path_segments)?;

        // Resolve the path to an entry
        let file_ref = self
            .get_or_create_leaf_dir_mut(path, time, store)
            .await?
            .userland
            .entry(filename.clone())
            // Create a file, if it doesn't exist yet
            .or_insert_with(|| PublicLink::with_file(PublicFile::new(time)))
            // Get a mutable ref out of the directory entry
            .resolve_value_mut(store)
            .await?
            .as_file_mut()?
            .prepare_next_revision();

        file_ref.metadata.upsert_mtime(time);

        Ok(file_ref)
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
    /// use chrono::Utc;
    ///
    /// #[async_std::main]
    /// async fn main() {
    ///     let dir = &mut PublicDirectory::new_rc(Utc::now());
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

    /// Looks up a node by its path name in the current directory.
    async fn lookup_node_mut<'a>(
        &'a mut self,
        path_segment: &str,
        store: &impl BlockStore,
    ) -> Result<Option<&'a mut PublicNode>> {
        Ok(match self.userland.get_mut(path_segment) {
            Some(link) => Some(link.resolve_value_mut(store).await?),
            None => None,
        })
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
    /// use chrono::Utc;
    ///
    /// #[async_std::main]
    /// async fn main() {
    ///     let dir = &mut PublicDirectory::new_rc(Utc::now());
    ///     let store = &MemoryBlockStore::default();
    ///     let content = b"Hello, World!".to_vec();
    ///
    ///     dir
    ///         .write(
    ///             &["pictures".into(), "cats".into(), "tabby.png".into()],
    ///             content.clone(),
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
    ///     assert_eq!(result, content);
    /// }
    /// ```
    pub async fn read(&self, path_segments: &[String], store: &impl BlockStore) -> Result<Vec<u8>> {
        let (path, filename) = utils::split_last(path_segments)?;
        match self.get_leaf_dir(path, store).await? {
            SearchResult::Found(dir) => match dir.lookup_node(filename, store).await? {
                Some(PublicNode::File(file)) => Ok(file.read_at(0, None, store).await?),
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
    /// use chrono::Utc;
    /// use anyhow::Result;
    ///
    /// #[async_std::main]
    /// async fn main() -> Result<()> {
    ///     let dir = &mut PublicDirectory::new_rc(Utc::now());
    ///     let store = &MemoryBlockStore::new();
    ///
    ///     dir
    ///         .write(
    ///             &["pictures".into(), "cats".into(), "tabby.png".into()],
    ///             b"Hello, World!".to_vec(),
    ///             Utc::now(),
    ///             store
    ///         )
    ///         .await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn write(
        self: &mut Arc<Self>,
        path_segments: &[String],
        content: Vec<u8>,
        time: DateTime<Utc>,
        store: &impl BlockStore,
    ) -> Result<()> {
        let (path, filename) = utils::split_last(path_segments)?;
        let dir = self.get_or_create_leaf_dir_mut(path, time, store).await?;

        match dir.lookup_node_mut(filename, store).await? {
            Some(PublicNode::File(file)) => file.set_content(time, content, store).await?,
            Some(PublicNode::Dir(_)) => bail!(FsError::DirectoryAlreadyExists),
            None => {
                dir.userland.insert(
                    filename.to_string(),
                    PublicLink::with_file(PublicFile::with_content(time, content, store).await?),
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
    /// use chrono::Utc;
    ///
    /// #[async_std::main]
    /// async fn main() {
    ///     let dir = &mut PublicDirectory::new_rc(Utc::now());
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
        self: &mut Arc<Self>,
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
    /// use libipld_core::cid::Cid;
    /// use chrono::Utc;
    ///
    /// #[async_std::main]
    /// async fn main() {
    ///     let dir = &mut PublicDirectory::new_rc(Utc::now());
    ///     let store = MemoryBlockStore::default();
    ///
    ///     dir
    ///         .write(
    ///             &["pictures".into(), "cats".into(), "tabby.png".into()],
    ///             b"Hello, world!".to_vec(),
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
            SearchResult::Found(dir) => {
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
            SearchResult::NotADir(_, _) => bail!(FsError::NotADirectory),
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
    /// use chrono::Utc;
    /// use anyhow::Result;
    ///
    /// #[async_std::main]
    /// async fn main() -> Result<()> {
    ///     let dir = &mut PublicDirectory::new_rc(Utc::now());
    ///     let store = &MemoryBlockStore::new();
    ///
    ///     dir
    ///         .write(
    ///             &["pictures".into(), "cats".into(), "tabby.png".into()],
    ///             b"Hello, World!".to_vec(),
    ///             Utc::now(),
    ///             store
    ///         )
    ///         .await?;
    ///
    ///     let result = dir
    ///         .ls(&["pictures".into()], store)
    ///         .await?;
    ///
    ///     assert_eq!(result.len(), 1);
    ///
    ///     dir
    ///         .rm(&["pictures".into(), "cats".into()], store)
    ///         .await?;
    ///
    ///     let result = dir
    ///         .ls(&["pictures".into()], store)
    ///         .await?;
    ///
    ///     assert_eq!(result.len(), 0);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn rm(
        self: &mut Arc<Self>,
        path_segments: &[String],
        store: &impl BlockStore,
    ) -> Result<PublicNode> {
        // TODO(matheus23) set modification time
        let (path, node_name) = utils::split_last(path_segments)?;

        let SearchResult::Found(dir) = self.get_leaf_dir_mut(path, store).await? else {
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
    /// use libipld_core::cid::Cid;
    /// use chrono::Utc;
    ///
    /// #[async_std::main]
    /// async fn main() {
    ///     let dir = &mut PublicDirectory::new_rc(Utc::now());
    ///     let store = MemoryBlockStore::default();
    ///
    ///     dir
    ///         .write(
    ///             &["pictures".into(), "cats".into(), "tabby.png".into()],
    ///             b"Hello, World!".to_vec(),
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
        self: &mut Arc<Self>,
        path_segments_from: &[String],
        path_segments_to: &[String],
        time: DateTime<Utc>,
        store: &impl BlockStore,
    ) -> Result<()> {
        let (path, filename) = utils::split_last(path_segments_to)?;
        let mut removed_node = self.rm(path_segments_from, store).await?;

        let SearchResult::Found(dir) = self.get_leaf_dir_mut(path, store).await? else {
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

    /// Copies a file or directory from one path to another.
    ///
    /// # Examples
    ///
    /// ```
    /// use anyhow::Result;
    /// use libipld_core::cid::Cid;
    /// use chrono::Utc;
    /// use wnfs::{
    ///     public::PublicDirectory,
    ///     common::{BlockStore, MemoryBlockStore},
    /// };
    ///
    /// #[async_std::main]
    /// async fn main() -> Result<()> {
    ///     let dir = &mut PublicDirectory::new_rc(Utc::now());
    ///     let store = &MemoryBlockStore::new();
    ///
    ///     dir
    ///         .write(
    ///             &["code".into(), "python".into(), "hello.py".into()],
    ///             b"Hello, world!".to_vec(),
    ///             Utc::now(),
    ///             store
    ///         )
    ///         .await?;
    ///
    ///     dir
    ///         .cp(
    ///             &["code".into(), "python".into(), "hello.py".into()],
    ///             &["code".into(), "hello.py".into()],
    ///             Utc::now(),
    ///             store
    ///         )
    ///         .await?;
    ///
    ///     let result = dir
    ///         .ls(&["code".into()], store)
    ///         .await?;
    ///
    ///     assert_eq!(result.len(), 2);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn cp(
        self: &mut Arc<Self>,
        path_segments_from: &[String],
        path_segments_to: &[String],
        time: DateTime<Utc>,
        store: &impl BlockStore,
    ) -> Result<()> {
        let (path, filename) = utils::split_last(path_segments_to)?;
        let Some(mut node) = self.get_node(path_segments_from, store).await?.cloned() else {
            bail!(FsError::NotFound);
        };

        let SearchResult::Found(dir) = self.get_leaf_dir_mut(path, store).await? else {
            bail!(FsError::NotFound);
        };

        ensure!(
            !dir.userland.contains_key(filename),
            FsError::FileAlreadyExists
        );

        node.upsert_mtime(time);

        dir.userland.insert(filename.clone(), PublicLink::new(node));

        Ok(())
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
            persisted_as: self
                .persisted_as
                .get()
                .cloned()
                .map(OnceCell::new_with)
                .unwrap_or_default(),
            metadata: self.metadata.clone(),
            userland: self.userland.clone(),
            previous: self.previous.clone(),
        }
    }
}

#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
impl Storable for PublicDirectory {
    type Serializable = PublicNodeSerializable;

    async fn to_serializable(&self, store: &impl BlockStore) -> Result<Self::Serializable> {
        let userland = {
            let mut map = BTreeMap::new();
            for (name, link) in self.userland.iter() {
                map.insert(name.clone(), link.resolve_cid(store).await?);
            }
            map
        };

        Ok(PublicNodeSerializable::Dir(PublicDirectorySerializable {
            version: WNFS_VERSION,
            metadata: self.metadata.clone(),
            userland,
            previous: self.previous.iter().cloned().collect(),
        }))
    }

    async fn from_serializable(
        cid: Option<&Cid>,
        serializable: Self::Serializable,
    ) -> Result<Self> {
        let PublicNodeSerializable::Dir(serializable) = serializable else {
            bail!(FsError::UnexpectedNodeType(NodeType::PublicFile));
        };

        if !is_readable_wnfs_version(&serializable.version) {
            bail!(FsError::UnexpectedVersion(serializable.version))
        }

        let userland = serializable
            .userland
            .into_iter()
            .map(|(name, cid)| (name, PublicLink::from_cid(cid)))
            .collect();

        Ok(Self {
            persisted_as: cid.cloned().map(OnceCell::new_with).unwrap_or_default(),
            metadata: serializable.metadata,
            userland,
            previous: serializable.previous.iter().cloned().collect(),
        })
    }

    fn persisted_as(&self) -> Option<&OnceCell<Cid>> {
        Some(&self.persisted_as)
    }
}

//--------------------------------------------------------------------------------------------------
// Tests
//--------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use libipld_core::ipld::Ipld;
    use testresult::TestResult;
    use wnfs_common::MemoryBlockStore;

    #[async_std::test]
    async fn look_up_can_fetch_file_added_to_directory() -> TestResult {
        let root_dir = &mut PublicDirectory::new_rc(Utc::now());
        let store = &MemoryBlockStore::new();
        let time = Utc::now();

        root_dir
            .write(&["text.txt".into()], b"Hello World!".to_vec(), time, store)
            .await?;

        let node = root_dir.lookup_node("text.txt", store).await?;

        assert_eq!(
            node,
            Some(&PublicNode::File(
                PublicFile::with_content_rc(time, b"Hello World!".to_vec(), store).await?
            ))
        );

        Ok(())
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
    async fn get_node_can_fetch_node_from_root_dir() -> TestResult {
        let time = Utc::now();
        let store = &MemoryBlockStore::new();
        let root_dir = &mut PublicDirectory::new_rc(time);

        root_dir
            .mkdir(&["pictures".into(), "dogs".into()], time, store)
            .await?;

        root_dir
            .write(
                &["pictures".into(), "cats".into(), "tabby.jpg".into()],
                b"Hello".to_vec(),
                time,
                store,
            )
            .await?;

        assert!(root_dir
            .get_node(
                &["pictures".into(), "cats".into(), "tabby.jpg".into()],
                store
            )
            .await?
            .is_some());

        assert!(root_dir
            .get_node(
                &["pictures".into(), "cats".into(), "tabby.jpeg".into()],
                store
            )
            .await?
            .is_none());

        assert!(root_dir
            .get_node(
                &["images".into(), "parrots".into(), "coco.png".into()],
                store
            )
            .await?
            .is_none());

        assert!(root_dir
            .get_node(
                &["pictures".into(), "dogs".into(), "bingo.jpg".into()],
                store
            )
            .await?
            .is_none());

        Ok(())
    }

    #[async_std::test]
    async fn mkdir_can_create_new_directory() -> TestResult {
        let time = Utc::now();
        let store = &MemoryBlockStore::new();
        let root_dir = &mut PublicDirectory::new_rc(time);

        root_dir
            .mkdir(&["tamedun".into(), "pictures".into()], time, store)
            .await?;

        let result = root_dir
            .get_node(&["tamedun".into(), "pictures".into()], store)
            .await?;

        assert!(result.is_some());

        Ok(())
    }

    #[async_std::test]
    async fn ls_can_list_children_under_directory() -> TestResult {
        let time = Utc::now();
        let store = &MemoryBlockStore::new();
        let root_dir = &mut PublicDirectory::new_rc(time);

        root_dir
            .mkdir(&["tamedun".into(), "pictures".into()], time, store)
            .await?;

        root_dir
            .write(
                &["tamedun".into(), "pictures".into(), "puppy.jpg".into()],
                b"Hello".to_vec(),
                time,
                store,
            )
            .await?;

        root_dir
            .mkdir(
                &["tamedun".into(), "pictures".into(), "cats".into()],
                time,
                store,
            )
            .await?;

        let result = root_dir
            .ls(&["tamedun".into(), "pictures".into()], store)
            .await?;

        assert_eq!(result.len(), 2);

        assert_eq!(result[0].0, String::from("cats"));

        assert_eq!(result[1].0, String::from("puppy.jpg"));

        Ok(())
    }

    #[async_std::test]
    async fn rm_can_remove_children_from_directory() -> TestResult {
        let time = Utc::now();
        let store = &MemoryBlockStore::new();
        let mut root_dir = PublicDirectory::new_rc(time);

        root_dir
            .mkdir(&["tamedun".into(), "pictures".into()], time, store)
            .await?;

        root_dir
            .write(
                &["tamedun".into(), "pictures".into(), "puppy.jpg".into()],
                b"Hello".to_vec(),
                time,
                store,
            )
            .await?;

        root_dir
            .mkdir(
                &["tamedun".into(), "pictures".into(), "cats".into()],
                time,
                store,
            )
            .await?;

        let result = root_dir
            .rm(&["tamedun".into(), "pictures".into()], store)
            .await;

        assert!(result.is_ok());

        let result = root_dir
            .rm(&["tamedun".into(), "pictures".into()], store)
            .await;

        assert!(result.is_err());

        Ok(())
    }

    #[async_std::test]
    async fn read_can_fetch_userland_of_file_added_to_directory() -> TestResult {
        let store = &MemoryBlockStore::new();
        let time = Utc::now();
        let content = b"Hello".to_vec();
        let mut root_dir = PublicDirectory::new_rc(time);

        root_dir
            .write(&["text.txt".into()], content.clone(), time, store)
            .await?;

        let result = root_dir.read(&["text.txt".into()], store).await?;

        assert_eq!(result, content);

        Ok(())
    }

    #[async_std::test]
    async fn mv_can_move_sub_directory_to_another_valid_location() -> TestResult {
        let time = Utc::now();
        let store = &MemoryBlockStore::new();
        let mut root_dir = PublicDirectory::new_rc(time);

        root_dir
            .write(
                &["pictures".into(), "cats".into(), "tabby.jpg".into()],
                b"Hello".to_vec(),
                time,
                store,
            )
            .await?;

        root_dir
            .write(
                &["pictures".into(), "cats".into(), "luna.png".into()],
                b"Hello".to_vec(),
                time,
                store,
            )
            .await?;

        root_dir.mkdir(&["images".into()], time, store).await?;

        root_dir
            .basic_mv(
                &["pictures".into(), "cats".into()],
                &["images".into(), "cats".into()],
                Utc::now(),
                store,
            )
            .await?;

        let result = root_dir.ls(&["images".into()], store).await?;

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].0, String::from("cats"));

        let result = root_dir.ls(&["pictures".into()], store).await?;

        assert_eq!(result.len(), 0);

        Ok(())
    }

    #[async_std::test]
    async fn mv_cannot_move_sub_directory_to_invalid_location() {
        let time = Utc::now();
        let store = MemoryBlockStore::default();
        let mut root_dir = PublicDirectory::new_rc(time);

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
    async fn mv_can_rename_directories() -> TestResult {
        let time = Utc::now();
        let store = &MemoryBlockStore::new();
        let root_dir = &mut PublicDirectory::new_rc(time);

        root_dir
            .write(&["file.txt".into()], b"Hello".to_vec(), time, store)
            .await?;

        root_dir
            .basic_mv(
                &["file.txt".into()],
                &["renamed.txt".into()],
                Utc::now(),
                store,
            )
            .await?;

        let result = root_dir.read(&["renamed.txt".into()], store).await?;

        assert_eq!(result, b"Hello".to_vec());

        Ok(())
    }

    #[async_std::test]
    async fn mv_fails_moving_directories_to_files() -> TestResult {
        let time = Utc::now();
        let store = &MemoryBlockStore::new();
        let root_dir = &mut PublicDirectory::new_rc(time);

        root_dir
            .mkdir(&["movies".into(), "ghibli".into()], time, store)
            .await?;

        root_dir
            .write(&["file.txt".into()], b"Hello".to_vec(), time, store)
            .await?;

        let result = root_dir
            .basic_mv(
                &["movies".into(), "ghibli".into()],
                &["file.txt".into()],
                Utc::now(),
                store,
            )
            .await;

        assert!(result.is_err());

        Ok(())
    }

    #[async_std::test]
    async fn previous_links_get_set() {
        let time = Utc::now();
        let store = &MemoryBlockStore::default();
        let root_dir = &mut PublicDirectory::new_rc(time);
        let previous_cid = root_dir.store(store).await.unwrap();

        root_dir.mkdir(&["test".into()], time, store).await.unwrap();

        let ipld = store
            .get_deserializable::<Ipld>(&root_dir.store(store).await.unwrap())
            .await
            .unwrap();
        match ipld {
            Ipld::Map(map) => match map.get("wnfs/pub/dir") {
                Some(Ipld::Map(content)) => match content.get("previous") {
                    Some(Ipld::List(previous)) => {
                        assert_eq!(previous, &vec![Ipld::Link(previous_cid)]);
                    }
                    _ => panic!("Expected 'previous' key to be a list"),
                },
                _ => panic!("Expected 'wnfs/pub/dir' key in the map"),
            },
            _ => panic!("Expected map!"),
        }
    }

    #[async_std::test]
    async fn prepare_next_revision_shortcuts_if_possible() {
        let time = Utc::now();
        let store = &MemoryBlockStore::default();
        let root_dir = &mut PublicDirectory::new_rc(time);

        let previous_cid = &root_dir.store(store).await.unwrap();
        let next_root_dir = root_dir.prepare_next_revision();
        let next_root_dir_clone = &mut Arc::new(next_root_dir.clone());
        let yet_another_dir = next_root_dir_clone.prepare_next_revision();

        assert_eq!(
            yet_another_dir.previous.iter().collect::<Vec<_>>(),
            vec![previous_cid]
        );
    }
}

#[cfg(test)]
mod snapshot_tests {
    use super::*;
    use chrono::TimeZone;
    use wnfs_common::utils::SnapshotBlockStore;

    #[async_std::test]
    async fn test_empty_directory() {
        let store = &SnapshotBlockStore::default();
        let time = Utc.with_ymd_and_hms(1970, 1, 1, 0, 0, 0).unwrap();

        let root_dir = &mut PublicDirectory::new_rc(time);
        let cid = root_dir.store(store).await.unwrap();

        let dir = store.get_block_snapshot(&cid).await.unwrap();

        insta::assert_json_snapshot!(dir);
    }

    #[async_std::test]
    async fn test_directory_with_children() {
        let store = &SnapshotBlockStore::default();
        let time = Utc.with_ymd_and_hms(1970, 1, 1, 0, 0, 0).unwrap();

        let root_dir = &mut PublicDirectory::new_rc(time);
        let paths = [
            vec!["text.txt".into()],
            vec!["music".into(), "jazz".into()],
            vec!["videos".into(), "movies".into(), "anime".into()],
        ];

        for path in paths.iter() {
            root_dir
                .write(path, b"Hello".to_vec(), time, store)
                .await
                .unwrap();
        }

        let cid = root_dir.store(store).await.unwrap();

        let dir = store.get_block_snapshot(&cid).await.unwrap();
        insta::assert_json_snapshot!(dir);
    }

    #[async_std::test]
    async fn test_directory_with_previous_links() {
        let store = &SnapshotBlockStore::default();
        let time = Utc.with_ymd_and_hms(1970, 1, 1, 0, 0, 0).unwrap();

        let paths = [
            vec!["text.txt".into()],
            vec!["music".into(), "jazz".into()],
            vec!["videos".into(), "movies".into(), "anime".into()],
        ];

        let root_dir = &mut PublicDirectory::new_rc(time);
        let _ = root_dir.store(store).await.unwrap();

        assert!(root_dir.persisted_as().and_then(OnceCell::get).is_some());

        for path in paths.iter() {
            root_dir
                .write(path, b"Hello".to_vec(), time, store)
                .await
                .unwrap();
        }

        let cid = root_dir.store(store).await.unwrap();
        let dir = store.get_block_snapshot(&cid).await.unwrap();

        insta::assert_json_snapshot!(dir);
    }
}
