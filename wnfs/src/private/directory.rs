use super::{
    encrypted::Encrypted, link::PrivateLink, INumber, PrivateFile, PrivateForest, PrivateNode,
    PrivateNodeHeader, PrivateRef, PrivateRefSerializable, TemporalKey,
};
use crate::{error::FsError, traits::Id, SearchResult};
use anyhow::{bail, ensure, Result};
use async_once_cell::OnceCell;
use chrono::{DateTime, Utc};
use libipld::Cid;
use rand_core::{CryptoRngCore, RngCore};
use semver::Version;
use serde::{de::Error as DeError, ser::Error as SerError, Deserialize, Deserializer, Serialize};
use std::{
    collections::{BTreeMap, BTreeSet},
    fmt::Debug,
    rc::Rc,
};
use wnfs_common::{
    dagcbor, utils::error, BlockStore, HashOutput, Metadata, NodeType, PathNodes, PathNodesResult,
};
use wnfs_nameaccumulator::{AccumulatorSetup, Name, NameSegment};

//--------------------------------------------------------------------------------------------------
// Type Definitions
//--------------------------------------------------------------------------------------------------

pub type PrivatePathNodes = PathNodes<PrivateDirectory>;
pub type PrivatePathNodesResult = PathNodesResult<PrivateDirectory>;

/// Represents a directory in the WNFS private filesystem.
///
/// # Examples
///
/// ```
/// use wnfs::{private::PrivateDirectory, namefilter::Namefilter};
/// use chrono::Utc;
/// use rand::thread_rng;
///
/// let rng = &mut thread_rng();
/// let dir = PrivateDirectory::new(
///     Namefilter::default(),
///     Utc::now(),
///     rng,
/// );
///
/// println!("dir = {:?}", dir);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct PrivateDirectory {
    pub header: PrivateNodeHeader,
    pub(crate) content: PrivateDirectoryContent,
}

#[derive(Debug)]
pub struct PrivateDirectoryContent {
    pub(crate) persisted_as: OnceCell<Cid>,
    pub(crate) previous: BTreeSet<(usize, Encrypted<Cid>)>,
    pub(crate) metadata: Metadata,
    pub(crate) entries: BTreeMap<String, PrivateLink>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PrivateDirectoryContentSerializable {
    pub r#type: NodeType,
    pub version: Version,
    pub previous: Vec<(usize, Encrypted<Cid>)>,
    #[serde(rename = "headerCid")]
    pub header_cid: Cid,
    pub metadata: Metadata,
    pub entries: BTreeMap<String, PrivateRefSerializable>,
}

//--------------------------------------------------------------------------------------------------
// Implementations
//--------------------------------------------------------------------------------------------------

impl PrivateDirectory {
    /// Creates a new directory with provided details.
    ///
    /// # Examples
    ///
    /// ```
    /// use wnfs::{private::PrivateDirectory, namefilter::Namefilter};
    /// use chrono::Utc;
    /// use rand::thread_rng;
    ///
    /// let rng = &mut thread_rng();
    /// let dir = PrivateDirectory::new(
    ///     Namefilter::default(),
    ///     Utc::now(),
    ///     rng,
    /// );
    ///
    /// println!("dir = {:?}", dir);
    /// ```
    pub fn new(parent_name: &Name, time: DateTime<Utc>, rng: &mut impl RngCore) -> Self {
        Self {
            header: PrivateNodeHeader::new(parent_name, rng),
            content: PrivateDirectoryContent {
                persisted_as: OnceCell::new(),
                previous: BTreeSet::new(),
                metadata: Metadata::new(time),
                entries: BTreeMap::new(),
            },
        }
    }

    /// Creates a new directory with the ratchet seed and inumber provided.
    ///
    /// # Examples
    ///
    /// ```
    /// use wnfs::{private::PrivateDirectory, namefilter::Namefilter};
    /// use chrono::Utc;
    /// use rand::{thread_rng, Rng};
    ///
    /// let rng = &mut thread_rng();
    /// let dir = PrivateDirectory::with_seed(
    ///     Namefilter::default(),
    ///     Utc::now(),
    ///     rng.gen::<[u8; 32]>(),
    ///     rng.gen::<[u8; 32]>(),
    /// );
    ///
    /// println!("dir = {:?}", dir);
    /// ```
    pub fn with_seed(
        parent_name: &Name,
        time: DateTime<Utc>,
        ratchet_seed: HashOutput,
        inumber: INumber,
    ) -> Self {
        Self {
            header: PrivateNodeHeader::with_seed(parent_name, ratchet_seed, inumber),
            content: PrivateDirectoryContent {
                persisted_as: OnceCell::new(),
                metadata: Metadata::new(time),
                previous: BTreeSet::new(),
                entries: BTreeMap::new(),
            },
        }
    }

    /// This contstructor creates a new private directory and stores it in a provided `PrivateForest`.
    pub async fn new_and_store(
        parent_name: &Name,
        time: DateTime<Utc>,
        forest: &mut Rc<PrivateForest>,
        store: &mut impl BlockStore,
        rng: &mut impl RngCore,
    ) -> Result<Rc<Self>> {
        let dir = Rc::new(Self::new(parent_name, time, rng));
        dir.store(forest, store, rng).await?;
        Ok(dir)
    }

    /// This contstructor creates a new private directory and stores it in a provided `PrivateForest` but
    /// with user-provided ratchet seed and inumber provided.
    pub async fn new_with_seed_and_store<B: BlockStore, R: RngCore>(
        parent_name: &Name,
        time: DateTime<Utc>,
        ratchet_seed: HashOutput,
        inumber: INumber,
        forest: &mut Rc<PrivateForest>,
        store: &mut B,
        rng: &mut R,
    ) -> Result<Rc<Self>> {
        let dir = Rc::new(Self::with_seed(parent_name, time, ratchet_seed, inumber));
        dir.store(forest, store, rng).await?;
        Ok(dir)
    }

    /// Uses specified path segments and their existence in the file tree to generate `PathNodes`.
    ///
    /// Supports cases where the entire path does not exist.
    pub(crate) async fn get_path_nodes(
        self: Rc<Self>,
        path_segments: &[String],
        search_latest: bool,
        forest: &PrivateForest,
        store: &impl BlockStore,
    ) -> Result<PrivatePathNodesResult> {
        use PathNodesResult::*;
        let mut working_node = self;
        let mut path_nodes = Vec::with_capacity(path_segments.len());

        for path_segment in path_segments {
            match working_node
                .lookup_node(path_segment, search_latest, forest, store)
                .await?
            {
                Some(PrivateNode::Dir(ref directory)) => {
                    path_nodes.push((Rc::clone(&working_node), path_segment.clone()));
                    working_node = Rc::clone(directory);
                }
                Some(_) => {
                    let path_nodes = PrivatePathNodes {
                        path: path_nodes,
                        tail: Rc::clone(&working_node),
                    };

                    return Ok(NotADirectory(path_nodes, path_segment.clone()));
                }
                None => {
                    let path_nodes = PrivatePathNodes {
                        path: path_nodes,
                        tail: Rc::clone(&working_node),
                    };

                    return Ok(MissingLink(path_nodes, path_segment.clone()));
                }
            }
        }

        Ok(Complete(PrivatePathNodes {
            path: path_nodes,
            tail: Rc::clone(&working_node),
        }))
    }

    /// Gets the metadata of the directory
    ///
    /// # Examples
    ///
    /// ```
    /// use wnfs::{private::PrivateDirectory, namefilter::Namefilter, common::Metadata};
    /// use chrono::Utc;
    /// use rand::thread_rng;
    /// use std::rc::Rc;
    ///
    /// let rng = &mut thread_rng();
    /// let time = Utc::now();
    /// let dir = Rc::new(PrivateDirectory::new(
    ///     Namefilter::default(),
    ///     time,
    ///     rng,
    /// ));
    ///
    /// assert_eq!(dir.get_metadata(), &Metadata::new(time));
    /// ```
    #[inline]
    pub fn get_metadata<'a>(self: &'a Rc<Self>) -> &'a Metadata {
        &self.content.metadata
    }

    /// Looks up a node by its path name in the current directory.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::rc::Rc;
    ///
    /// use chrono::Utc;
    /// use rand::thread_rng;
    ///
    /// use wnfs::{
    ///     private::{PrivateForest, PrivateRef, PrivateDirectory},
    ///     common::{BlockStore, MemoryBlockStore},
    ///     namefilter::Namefilter,
    /// };
    ///
    /// #[async_std::main]
    /// async fn main() {
    ///     let store = &mut MemoryBlockStore::default();
    ///     let rng = &mut thread_rng();
    ///     let forest = &mut Rc::new(PrivateForest::new());
    ///     let root_dir = &mut Rc::new(PrivateDirectory::new(
    ///         Namefilter::default(),
    ///         Utc::now(),
    ///         rng,
    ///     ));
    ///
    ///     root_dir
    ///         .mkdir(&["pictures".into(), "cats".into()], true, Utc::now(), forest, store, rng)
    ///         .await
    ///         .unwrap();
    ///
    ///     let node = root_dir.lookup_node("pictures", true, forest, store)
    ///         .await
    ///         .unwrap();
    ///
    ///     assert!(node.is_some());
    /// }
    /// ```
    pub async fn lookup_node(
        &self,
        path_segment: &str,
        search_latest: bool,
        forest: &PrivateForest,
        store: &impl BlockStore,
    ) -> Result<Option<PrivateNode>> {
        Ok(match self.content.entries.get(path_segment) {
            Some(private_link) => {
                let private_node = private_link
                    .resolve_node(forest, store, Some(&self.header.name))
                    .await?;
                if search_latest {
                    Some(private_node.search_latest(forest, store).await?)
                } else {
                    Some(private_node.clone())
                }
            }
            None => None,
        })
    }

    /// Looks up a node by its path name in the current directory.
    pub(crate) async fn lookup_node_mut<'a>(
        &'a mut self,
        path_segment: &str,
        search_latest: bool,
        forest: &PrivateForest,
        store: &impl BlockStore,
    ) -> Result<Option<&'a mut PrivateNode>> {
        Ok(match self.content.entries.get_mut(path_segment) {
            Some(private_link) => {
                let private_node = private_link
                    .resolve_node_mut(forest, store, Some(&self.header.name))
                    .await?;
                if search_latest {
                    *private_node = private_node.search_latest(forest, store).await?;
                }

                Some(private_node)
            }
            None => None,
        })
    }

    pub(crate) async fn get_leaf_dir(
        self: &Rc<Self>,
        path_segments: &[String],
        search_latest: bool,
        forest: &PrivateForest,
        store: &impl BlockStore,
    ) -> Result<SearchResult<Rc<Self>>> {
        let mut working_dir = Rc::clone(self);
        for (depth, segment) in path_segments.iter().enumerate() {
            match working_dir
                .lookup_node(segment, search_latest, forest, store)
                .await?
            {
                Some(PrivateNode::Dir(directory)) => {
                    working_dir = Rc::clone(&directory);
                }
                Some(_) => return Ok(SearchResult::NotADir(working_dir, depth)),
                None => return Ok(SearchResult::Missing(working_dir, depth)),
            }
        }

        Ok(SearchResult::Found(working_dir))
    }

    pub(crate) async fn get_leaf_dir_mut<'a>(
        self: &'a mut Rc<Self>,
        path_segments: &[String],
        search_latest: bool,
        forest: &PrivateForest,
        store: &impl BlockStore,
    ) -> Result<SearchResult<&'a mut Self>> {
        let mut working_dir = self.prepare_next_revision()?;
        for (depth, segment) in path_segments.iter().enumerate() {
            match working_dir
                .lookup_node(segment, search_latest, forest, store)
                .await?
            {
                Some(PrivateNode::Dir(_)) => {
                    // We need this repeated lookup because Rust borrowck can't handle
                    // this mut borrow case yet without resorting to the unstable -Zpolonius flag.
                    // https://github.com/rust-lang/rust/issues/51545
                    working_dir = working_dir
                        .lookup_node_mut(segment, search_latest, forest, store)
                        .await
                        .unwrap()
                        .unwrap()
                        .as_dir_mut()
                        .unwrap()
                        .prepare_next_revision()?
                }
                Some(_) => return Ok(SearchResult::NotADir(working_dir, depth)),
                None => return Ok(SearchResult::Missing(working_dir, depth)),
            };
        }

        Ok(SearchResult::Found(working_dir))
    }

    pub(crate) async fn get_or_create_leaf_dir_mut<'a>(
        self: &'a mut Rc<Self>,
        path_segments: &[String],
        time: DateTime<Utc>,
        search_latest: bool,
        forest: &PrivateForest,
        store: &impl BlockStore,
        rng: &mut impl RngCore,
    ) -> Result<&'a mut Self> {
        match self
            .get_leaf_dir_mut(path_segments, search_latest, forest, store)
            .await?
        {
            SearchResult::Found(dir) => Ok(dir),
            SearchResult::Missing(mut dir, depth) => {
                for segment in &path_segments[depth..] {
                    dir = Rc::make_mut(
                        dir.content
                            .entries
                            .entry(segment.to_string())
                            .or_insert_with(|| {
                                PrivateLink::with_dir(Self::new(&dir.header.name, time, rng))
                            })
                            .resolve_node_mut(forest, store, Some(&dir.header.name))
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

    /// This should be called to prepare a node for modifications,
    /// if it's meant to be a successor revision of the current revision.
    ///
    /// This doesn't have any effect if the current state hasn't been `.store()`ed yet.
    /// Otherwise, it clones itself, stores its current CID in the previous links and
    /// advances its ratchet.
    pub(crate) fn prepare_next_revision<'a>(self: &'a mut Rc<Self>) -> Result<&'a mut Self> {
        let Some(previous_cid) = self.content.persisted_as.get().cloned() else {
            // The current revision wasn't written yet.
            // There's no point in advancing the revision even further.
            return Ok(Rc::make_mut(self));
        };

        let temporal_key = self.header.derive_temporal_key();
        let previous_link = (1, Encrypted::from_value(previous_cid, &temporal_key)?);
        let mut cloned = Rc::make_mut(self);

        // We make sure to clear any cached states.
        cloned.content.persisted_as = OnceCell::new();
        cloned.content.previous = [previous_link].into_iter().collect();
        cloned.header.advance_ratchet();

        Ok(cloned)
    }

    /// Returns the private ref, if this directory has been `.store()`ed before.
    pub(crate) fn get_private_ref(&self, setup: &AccumulatorSetup) -> Option<PrivateRef> {
        self.content.persisted_as.get().map(|content_cid| {
            self.header
                .derive_revision_ref(setup)
                .as_private_ref(*content_cid)
        })
    }

    /// This prepares this directory for key rotation, usually for moving or
    /// copying the directory to some other place.
    ///
    /// Will reset the ratchet, so a different key is necessary for read access,
    /// will reset the inumber to reset write access,
    /// will update the bare namefilter to match the new parent's namefilter,
    /// so it inherits the write access rules from the new parent and
    /// resets the `persisted_as` pointer.
    pub(crate) fn prepare_key_rotation(
        &mut self,
        parent_name: &Name,
        rng: &mut impl CryptoRngCore,
    ) {
        self.header.inumber = NameSegment::new(rng);
        self.header.update_bare_name(parent_name);
        self.header.reset_ratchet(rng);
        self.content.persisted_as = OnceCell::new();
    }

    /// Follows a path and fetches the node at the end of the path.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::rc::Rc;
    ///
    /// use chrono::Utc;
    /// use rand::thread_rng;
    ///
    /// use wnfs::{
    ///     private::{PrivateForest, PrivateRef, PrivateDirectory},
    ///     common::{BlockStore, MemoryBlockStore},
    ///     namefilter::Namefilter,
    /// };
    ///
    /// #[async_std::main]
    /// async fn main() {
    ///     let store = &mut MemoryBlockStore::default();
    ///     let rng = &mut thread_rng();
    ///     let forest = &mut Rc::new(PrivateForest::new());
    ///     let root_dir = &mut Rc::new(PrivateDirectory::new(
    ///         Namefilter::default(),
    ///         Utc::now(),
    ///         rng,
    ///     ));
    ///
    ///     root_dir
    ///         .mkdir(&["pictures".into(), "cats".into()], true, Utc::now(), forest, store, rng)
    ///         .await
    ///         .unwrap();
    ///
    ///     let result = root_dir
    ///         .get_node(&["pictures".into(), "cats".into()], true, forest, store)
    ///         .await
    ///         .unwrap();
    ///
    ///     assert!(result.is_some());
    /// }
    /// ```
    pub async fn get_node(
        self: &Rc<Self>,
        path_segments: &[String],
        search_latest: bool,
        forest: &PrivateForest,
        store: &impl BlockStore,
    ) -> Result<Option<PrivateNode>> {
        let Some((tail, path)) = path_segments.split_last() else {
            bail!(FsError::InvalidPath);
        };

        let SearchResult::Found(dir) = self.get_leaf_dir(path,  search_latest, forest, store).await? else {
            bail!(FsError::NotFound);
        };

        dir.lookup_node(tail, search_latest, forest, store).await
    }

    /// Reads specified file content from the directory.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::rc::Rc;
    ///
    /// use chrono::Utc;
    /// use rand::thread_rng;
    ///
    /// use wnfs::{
    ///     private::{PrivateForest, PrivateRef, PrivateDirectory},
    ///     common::{BlockStore, MemoryBlockStore},
    ///     namefilter::Namefilter,
    /// };
    ///
    /// #[async_std::main]
    /// async fn main() {
    ///     let store = &mut MemoryBlockStore::default();
    ///     let rng = &mut thread_rng();
    ///     let forest = &mut Rc::new(PrivateForest::new());
    ///     let root_dir = &mut Rc::new(PrivateDirectory::new(
    ///         Namefilter::default(),
    ///         Utc::now(),
    ///         rng,
    ///     ));
    ///
    ///     let content = b"print('hello world')";
    ///
    ///     root_dir
    ///         .write(
    ///             &["code".into(), "hello.py".into()],
    ///             true,
    ///             Utc::now(),
    ///             content.to_vec(),
    ///             forest,
    ///             store,
    ///             rng
    ///         )
    ///         .await
    ///         .unwrap();
    ///
    ///     let result = root_dir
    ///         .read(&["code".into(), "hello.py".into()], true, forest, store)
    ///         .await
    ///         .unwrap();
    ///
    ///     assert_eq!(&result, content);
    /// }
    /// ```
    pub async fn read(
        self: &Rc<Self>,
        path_segments: &[String],
        search_latest: bool,
        forest: &PrivateForest,
        store: &impl BlockStore,
    ) -> Result<Vec<u8>> {
        let (path, filename) = crate::utils::split_last(path_segments)?;
        match self
            .get_leaf_dir(path, search_latest, forest, store)
            .await?
        {
            SearchResult::Found(dir) => {
                match dir
                    .lookup_node(filename, search_latest, forest, store)
                    .await?
                {
                    Some(PrivateNode::File(file)) => Ok(file.get_content(forest, store).await?),
                    Some(_) => error(FsError::NotAFile),
                    None => error(FsError::NotFound),
                }
            }
            _ => error(FsError::NotFound),
        }
    }

    /// Writes a file to the directory.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::rc::Rc;
    /// use chrono::Utc;
    /// use rand::thread_rng;
    /// use wnfs::{
    ///     private::{PrivateForest, PrivateRef, PrivateDirectory},
    ///     common::{BlockStore, MemoryBlockStore},
    ///     namefilter::Namefilter,
    /// };
    ///
    /// #[async_std::main]
    /// async fn main() {
    ///     let store = &mut MemoryBlockStore::default();
    ///     let rng = &mut thread_rng();
    ///     let forest = &mut Rc::new(PrivateForest::new());
    ///     let root_dir = &mut Rc::new(PrivateDirectory::new(
    ///         Namefilter::default(),
    ///         Utc::now(),
    ///         rng,
    ///     ));
    ///
    ///     let content = b"print('hello world')";
    ///
    ///     root_dir
    ///         .write(
    ///             &["code".into(), "hello.py".into()],
    ///             true,
    ///             Utc::now(),
    ///             content.to_vec(),
    ///             forest,
    ///             store,
    ///             rng
    ///         )
    ///         .await
    ///         .unwrap();
    ///
    ///     let result = root_dir
    ///         .read(&["code".into(), "hello.py".into()], true, forest, store)
    ///         .await
    ///         .unwrap();
    ///
    ///     assert_eq!(&result, content);
    /// }
    /// ```
    #[allow(clippy::too_many_arguments)]
    pub async fn write(
        self: &mut Rc<Self>,
        path_segments: &[String],
        search_latest: bool,
        time: DateTime<Utc>,
        content: Vec<u8>,
        forest: &mut Rc<PrivateForest>,
        store: &mut impl BlockStore,
        rng: &mut impl RngCore,
    ) -> Result<()> {
        let (path, filename) = crate::utils::split_last(path_segments)?;
        let dir = self
            .get_or_create_leaf_dir_mut(path, time, search_latest, forest, store, rng)
            .await?;

        match dir
            .lookup_node_mut(filename, search_latest, forest, store)
            .await?
        {
            Some(PrivateNode::File(file)) => {
                let file = file.prepare_next_revision()?;
                let content =
                    PrivateFile::prepare_content(&file.header.name, content, forest, store, rng)
                        .await?;
                file.content.content = content;
                file.content.metadata.upsert_mtime(time);
            }
            Some(PrivateNode::Dir(_)) => bail!(FsError::DirectoryAlreadyExists),
            None => {
                let file =
                    PrivateFile::with_content(&dir.header.name, time, content, forest, store, rng)
                        .await?;
                let link = PrivateLink::with_file(file);
                dir.content.entries.insert(filename.to_string(), link);
            }
        };

        Ok(())
    }

    /// Gets the latest version of the directory using exponential search.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::rc::Rc;
    /// use chrono::Utc;
    /// use rand::thread_rng;
    /// use wnfs::{
    ///     private::{PrivateForest, PrivateRef, PrivateNode, PrivateDirectory},
    ///     common::{BlockStore, MemoryBlockStore},
    ///     namefilter::Namefilter,
    /// };
    ///
    /// #[async_std::main]
    /// async fn main() {
    ///     let store = &mut MemoryBlockStore::default();
    ///     let rng = &mut thread_rng();
    ///     let forest = &mut Rc::new(PrivateForest::new());
    ///     let mut init_dir = PrivateDirectory::new_and_store(
    ///         Default::default(),
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
    ///     let latest_dir = init_dir.search_latest(forest, store).await.unwrap();
    ///
    ///     let found_node = latest_dir
    ///         .lookup_node("pictures", true, forest, store)
    ///         .await
    ///         .unwrap();
    ///
    ///     assert!(found_node.is_some());
    /// }
    /// ```
    #[inline]
    pub async fn search_latest(
        self: Rc<Self>,
        forest: &PrivateForest,
        store: &impl BlockStore,
    ) -> Result<Rc<Self>> {
        PrivateNode::Dir(self)
            .search_latest(forest, store)
            .await?
            .as_dir()
    }

    /// Creates a new directory at the specified path.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::rc::Rc;
    ///
    /// use chrono::Utc;
    /// use rand::thread_rng;
    ///
    /// use wnfs::{
    ///     private::{PrivateForest, PrivateRef, PrivateDirectory},
    ///     common::{BlockStore, MemoryBlockStore},
    ///     namefilter::Namefilter,
    /// };
    ///
    /// #[async_std::main]
    /// async fn main() {
    ///     let store = &mut MemoryBlockStore::default();
    ///     let rng = &mut thread_rng();
    ///     let forest = &mut Rc::new(PrivateForest::new());
    ///     let root_dir = &mut Rc::new(PrivateDirectory::new(
    ///         Namefilter::default(),
    ///         Utc::now(),
    ///         rng,
    ///     ));
    ///
    ///     root_dir
    ///         .mkdir(&["pictures".into(), "cats".into()], true, Utc::now(), forest, store, rng)
    ///         .await
    ///         .unwrap();
    ///
    ///     let node = root_dir.lookup_node("pictures", true, forest, store)
    ///         .await
    ///         .unwrap();
    ///
    ///     assert!(node.is_some());
    /// }
    /// ```
    pub async fn mkdir(
        self: &mut Rc<Self>,
        path_segments: &[String],
        search_latest: bool,
        time: DateTime<Utc>,
        forest: &PrivateForest,
        store: &impl BlockStore,
        rng: &mut impl RngCore,
    ) -> Result<()> {
        let _ = self
            .get_or_create_leaf_dir_mut(path_segments, time, search_latest, forest, store, rng)
            .await?;

        Ok(())
    }

    /// Returns names and metadata of directory's immediate children.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::rc::Rc;
    ///
    /// use chrono::Utc;
    /// use rand::thread_rng;
    ///
    /// use wnfs::{
    ///     private::{PrivateForest, PrivateRef, PrivateDirectory},
    ///     common::{BlockStore, MemoryBlockStore},
    ///     namefilter::Namefilter,
    /// };
    ///
    /// #[async_std::main]
    /// async fn main() {
    ///     let store = &mut MemoryBlockStore::default();
    ///     let rng = &mut thread_rng();
    ///     let forest = &mut Rc::new(PrivateForest::new());
    ///     let root_dir = &mut Rc::new(PrivateDirectory::new(
    ///         Namefilter::default(),
    ///         Utc::now(),
    ///         rng,
    ///     ));
    ///
    ///     root_dir
    ///         .write(
    ///             &["code".into(), "hello.py".into()],
    ///             true,
    ///             Utc::now(),
    ///             b"print('hello world')".to_vec(),
    ///             forest,
    ///             store,
    ///             rng
    ///         )
    ///         .await
    ///         .unwrap();
    ///
    ///     root_dir
    ///         .mkdir(&["code".into(), "bin".into()], true, Utc::now(), forest, store, rng)
    ///         .await
    ///         .unwrap();
    ///
    ///     let result = root_dir
    ///         .ls(&["code".into()], true, forest, store)
    ///         .await
    ///         .unwrap();
    ///
    ///     assert_eq!(result.len(), 2);
    ///     assert_eq!(
    ///         result.iter().map(|t| &t.0).collect::<Vec<_>>(),
    ///         ["bin", "hello.py"]
    ///     );
    /// }
    /// ```
    pub async fn ls(
        self: &Rc<Self>,
        path_segments: &[String],
        search_latest: bool,
        forest: &PrivateForest,
        store: &impl BlockStore,
    ) -> Result<Vec<(String, Metadata)>> {
        match self
            .get_leaf_dir(path_segments, search_latest, forest, store)
            .await?
        {
            SearchResult::Found(dir) => {
                let mut result = vec![];
                for (name, link) in dir.content.entries.iter() {
                    match link
                        .resolve_node(forest, store, Some(&dir.header.name))
                        .await?
                    {
                        PrivateNode::File(file) => {
                            result.push((name.clone(), file.content.metadata.clone()));
                        }
                        PrivateNode::Dir(dir) => {
                            result.push((name.clone(), dir.content.metadata.clone()));
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
    /// use std::rc::Rc;
    /// use chrono::Utc;
    /// use rand::thread_rng;
    /// use wnfs::{
    ///     private::{PrivateForest, PrivateRef, PrivateDirectory},
    ///     common::{BlockStore, MemoryBlockStore},
    ///     namefilter::Namefilter,
    /// };
    ///
    /// #[async_std::main]
    /// async fn main() {
    ///     let store = &mut MemoryBlockStore::default();
    ///     let rng = &mut thread_rng();
    ///     let forest = &mut Rc::new(PrivateForest::new());
    ///     let root_dir = &mut Rc::new(PrivateDirectory::new(
    ///         Namefilter::default(),
    ///         Utc::now(),
    ///         rng,
    ///     ));
    ///
    ///     root_dir
    ///         .write(
    ///             &["code".into(), "python".into(), "hello.py".into()],
    ///             true,
    ///             Utc::now(),
    ///             b"print('hello world')".to_vec(),
    ///             forest,
    ///             store,
    ///             rng
    ///         )
    ///         .await
    ///         .unwrap();
    ///
    ///     let result = root_dir
    ///         .ls(&["code".into()], true, forest, store)
    ///         .await
    ///         .unwrap();
    ///
    ///     assert_eq!(result.len(), 1);
    ///
    ///     root_dir
    ///         .rm(&["code".into(), "python".into()], true, forest, store)
    ///         .await
    ///         .unwrap();
    ///
    ///     let result = root_dir
    ///         .ls(&["code".into()], true, forest, store)
    ///         .await
    ///         .unwrap();
    ///
    ///     assert_eq!(result.len(), 0);
    /// }
    /// ```
    pub async fn rm(
        self: &mut Rc<Self>,
        path_segments: &[String],
        search_latest: bool,
        forest: &PrivateForest,
        store: &impl BlockStore,
    ) -> Result<PrivateNode> {
        let (path, node_name) = crate::utils::split_last(path_segments)?;
        let SearchResult::Found(dir) = self.get_leaf_dir_mut(path, search_latest, forest, store).await? else {
            bail!(FsError::NotFound)
        };

        let removed_node = match dir.content.entries.remove(node_name) {
            Some(link) => {
                link.resolve_owned_node(forest, store, Some(&dir.header.name))
                    .await?
            }
            None => bail!(FsError::NotFound),
        };

        Ok(removed_node)
    }

    /// Attaches a node to the specified directory.
    ///
    /// Fixes up the subtree bare names to refer to the new parent.
    #[allow(clippy::too_many_arguments)]
    async fn attach(
        self: &mut Rc<Self>,
        mut node: PrivateNode,
        path_segments: &[String],
        search_latest: bool,
        time: DateTime<Utc>,
        forest: &mut Rc<PrivateForest>,
        store: &mut impl BlockStore,
        rng: &mut impl CryptoRngCore,
    ) -> Result<()> {
        let (path, node_name) = crate::utils::split_last(path_segments)?;
        let SearchResult::Found(dir) = self.get_leaf_dir_mut(path, search_latest, forest, store).await? else {
            bail!(FsError::NotFound);
        };

        ensure!(
            !dir.content.entries.contains_key(node_name),
            FsError::FileAlreadyExists
        );

        node.upsert_mtime(time);
        node.update_ancestry(&dir.header.name, forest, store, rng)
            .await?;

        dir.content
            .entries
            .insert(node_name.clone(), PrivateLink::from(node));

        Ok(())
    }

    /// Moves a file or directory from one path to another.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::rc::Rc;
    /// use chrono::Utc;
    /// use rand::thread_rng;
    ///
    /// use wnfs::{
    ///     private::{PrivateForest, PrivateRef, PrivateDirectory},
    ///     common::{BlockStore, MemoryBlockStore},
    ///     namefilter::Namefilter,
    /// };
    ///
    /// #[async_std::main]
    /// async fn main() {
    ///     let store = &mut MemoryBlockStore::default();
    ///     let rng = &mut thread_rng();
    ///     let forest = &mut Rc::new(PrivateForest::new());
    ///     let root_dir = &mut Rc::new(PrivateDirectory::new(
    ///         Namefilter::default(),
    ///         Utc::now(),
    ///         rng,
    ///     ));
    ///
    ///     root_dir
    ///         .write(
    ///             &["code".into(), "python".into(), "hello.py".into()],
    ///             true,
    ///             Utc::now(),
    ///             b"print('hello world')".to_vec(),
    ///             forest,
    ///             store,
    ///             rng
    ///         )
    ///         .await
    ///         .unwrap();
    ///
    ///     let result = root_dir
    ///         .basic_mv(
    ///             &["code".into(), "python".into(), "hello.py".into()],
    ///             &["code".into(), "hello.py".into()],
    ///             true,
    ///             Utc::now(),
    ///             forest,
    ///             store,
    ///             rng
    ///         )
    ///         .await
    ///         .unwrap();
    ///
    ///     let result = root_dir
    ///         .ls(&["code".into()], true, forest, store)
    ///         .await
    ///         .unwrap();
    ///
    ///     assert_eq!(result.len(), 2);
    /// }
    /// ```
    #[allow(clippy::too_many_arguments)]
    pub async fn basic_mv(
        self: &mut Rc<Self>,
        path_segments_from: &[String],
        path_segments_to: &[String],
        search_latest: bool,
        time: DateTime<Utc>,
        forest: &mut Rc<PrivateForest>,
        store: &mut impl BlockStore,
        rng: &mut impl CryptoRngCore,
    ) -> Result<()> {
        let removed_node = self
            .rm(path_segments_from, search_latest, forest, store)
            .await?;

        self.attach(
            removed_node,
            path_segments_to,
            search_latest,
            time,
            forest,
            store,
            rng,
        )
        .await
    }

    /// Copies a file or directory from one path to another.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::rc::Rc;
    ///
    /// use chrono::Utc;
    /// use rand::thread_rng;
    ///
    /// use wnfs::{
    ///     private::{PrivateForest, PrivateRef, PrivateDirectory},
    ///     common::{BlockStore, MemoryBlockStore},
    ///     namefilter::Namefilter,
    /// };
    ///
    /// #[async_std::main]
    /// async fn main() {
    ///     let store = &mut MemoryBlockStore::default();
    ///     let rng = &mut thread_rng();
    ///     let forest = &mut Rc::new(PrivateForest::new());
    ///     let root_dir = &mut Rc::new(PrivateDirectory::new(
    ///         Namefilter::default(),
    ///         Utc::now(),
    ///         rng,
    ///     ));
    ///
    ///     root_dir
    ///         .write(
    ///             &["code".into(), "python".into(), "hello.py".into()],
    ///             true,
    ///             Utc::now(),
    ///             b"print('hello world')".to_vec(),
    ///             forest,
    ///             store,
    ///             rng
    ///         )
    ///         .await
    ///         .unwrap();
    ///
    ///     let result = root_dir
    ///         .cp(
    ///             &["code".into(), "python".into(), "hello.py".into()],
    ///             &["code".into(), "hello.py".into()],
    ///             true,
    ///             Utc::now(),
    ///             forest,
    ///             store,
    ///             rng
    ///         )
    ///         .await
    ///         .unwrap();
    ///
    ///     let result = root_dir
    ///         .ls(&["code".into()], true, forest, store)
    ///         .await
    ///         .unwrap();
    ///
    ///     assert_eq!(result.len(), 2);
    /// }
    /// ```
    #[allow(clippy::too_many_arguments)]
    pub async fn cp(
        self: &mut Rc<Self>,
        path_segments_from: &[String],
        path_segments_to: &[String],
        search_latest: bool,
        time: DateTime<Utc>,
        forest: &mut Rc<PrivateForest>,
        store: &mut impl BlockStore,
        rng: &mut impl CryptoRngCore,
    ) -> Result<()> {
        let result = self
            .get_node(path_segments_from, search_latest, forest, store)
            .await?;

        self.attach(
            result.ok_or(FsError::NotFound)?,
            path_segments_to,
            search_latest,
            time,
            forest,
            store,
            rng,
        )
        .await
    }

    /// Stores this PrivateDirectory in the PrivateForest.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::rc::Rc;
    /// use chrono::Utc;
    /// use rand::thread_rng;
    /// use wnfs::{
    ///     private::{PrivateForest, PrivateRef, PrivateNode, PrivateDirectory},
    ///     common::{BlockStore, MemoryBlockStore},
    ///     namefilter::Namefilter,
    /// };
    ///
    /// #[async_std::main]
    /// async fn main() {
    ///     let store = &mut MemoryBlockStore::default();
    ///     let rng = &mut thread_rng();
    ///     let forest = &mut Rc::new(PrivateForest::new());
    ///     let dir = &mut Rc::new(PrivateDirectory::new(
    ///         Namefilter::default(),
    ///         Utc::now(),
    ///         rng,
    ///     ));
    ///
    ///     let private_ref = dir.store(forest, store, rng).await.unwrap();
    ///
    ///     let node = PrivateNode::Dir(Rc::clone(&dir));
    ///
    ///     assert_eq!(
    ///         PrivateNode::load(&private_ref, forest, store).await.unwrap(),
    ///         node
    ///     );
    /// }
    /// ```
    pub async fn store(
        &self,
        forest: &mut Rc<PrivateForest>,
        store: &mut impl BlockStore,
        rng: &mut impl RngCore,
    ) -> Result<PrivateRef> {
        let setup = &forest.get_accumulator_setup().clone();
        let header_cid = self.header.store(store, setup).await?;
        let temporal_key = self.header.derive_temporal_key();
        let label = self.header.get_name(setup);

        let content_cid = self
            .content
            .store(header_cid, &temporal_key, forest, store, rng)
            .await?;

        forest
            .put_encrypted(label, [header_cid, content_cid], store)
            .await?;

        Ok(self
            .header
            .derive_revision_ref(setup)
            .as_private_ref(content_cid))
    }

    /// Wraps the directory in a [`PrivateNode`].
    pub fn as_node(self: &Rc<Self>) -> PrivateNode {
        PrivateNode::Dir(Rc::clone(self))
    }
}

impl PrivateDirectoryContent {
    /// Serializes the directory with provided Serde serialilzer.
    pub(crate) async fn serialize<S>(
        &self,
        serializer: S,
        temporal_key: &TemporalKey,
        header_cid: Cid,
        forest: &mut Rc<PrivateForest>,
        store: &mut impl BlockStore,
        rng: &mut impl RngCore,
    ) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut entries = BTreeMap::new();

        for (name, private_link) in self.entries.iter() {
            let private_ref_serializable = private_link
                .resolve_ref(forest, store, rng)
                .await
                .map_err(SerError::custom)?
                .to_serializable(temporal_key)
                .map_err(SerError::custom)?;
            entries.insert(name.clone(), private_ref_serializable);
        }

        (PrivateDirectoryContentSerializable {
            r#type: NodeType::PrivateDirectory,
            version: Version::new(0, 2, 0),
            previous: self.previous.iter().cloned().collect(),
            header_cid,
            metadata: self.metadata.clone(),
            entries,
        })
        .serialize(serializer)
    }

    /// Deserializes the directory with provided Serde deserializer and temporal key.
    pub(crate) fn deserialize<'de, D>(
        deserializer: D,
        temporal_key: &TemporalKey,
        from_cid: Cid,
    ) -> Result<(Self, Cid), D::Error>
    where
        D: Deserializer<'de>,
    {
        let PrivateDirectoryContentSerializable {
            r#type,
            version,
            metadata,
            previous,
            header_cid,
            entries: entries_encrypted,
        } = PrivateDirectoryContentSerializable::deserialize(deserializer)?;

        if version.major != 0 || version.minor != 2 {
            return Err(DeError::custom(FsError::UnexpectedVersion(version)));
        }

        if r#type != NodeType::PrivateDirectory {
            return Err(DeError::custom(FsError::UnexpectedNodeType(r#type)));
        }

        let mut entries = BTreeMap::new();

        for (name, private_ref_serializable) in entries_encrypted {
            let private_ref = PrivateRef::from_serializable(private_ref_serializable, temporal_key)
                .map_err(DeError::custom)?;
            entries.insert(name, PrivateLink::from_ref(private_ref));
        }

        Ok((
            Self {
                persisted_as: OnceCell::new_with(Some(from_cid)),
                metadata,
                previous: previous.into_iter().collect(),
                entries,
            },
            header_cid,
        ))
    }

    /// Encrypts the directory contents by
    /// - wrapping all subdirectory temporal keys given the current temporal key
    /// - encrypting the whole directory using the snapshot key derived from the temporal key.
    ///
    /// The resulting ciphertext is then stored in the given BlockStore. Its CID is finally returned.
    ///
    /// Randomness is required for randomized encryption.
    ///
    /// The header cid is required as it's not stored in the PrivateDirectoryContent itself, but
    /// stored in the serialized format.
    pub async fn store(
        &self,
        header_cid: Cid,
        temporal_key: &TemporalKey,
        forest: &mut Rc<PrivateForest>,
        store: &mut impl BlockStore,
        rng: &mut impl RngCore,
    ) -> Result<Cid> {
        Ok(*self
            .persisted_as
            .get_or_try_init::<anyhow::Error>(async {
                // TODO(matheus23) deduplicate when reworking serialization (see file.rs)
                let snapshot_key = temporal_key.derive_snapshot_key();

                // Serialize node to cbor.
                let ipld = self
                    .serialize(
                        libipld::serde::Serializer,
                        temporal_key,
                        header_cid,
                        forest,
                        store,
                        rng,
                    )
                    .await?;
                let bytes = dagcbor::encode(&ipld)?;

                // Encrypt bytes with snapshot key.
                let block = snapshot_key.encrypt(&bytes, rng)?;

                // Store content section in blockstore and get Cid.
                store.put_block(block, libipld::IpldCodec::Raw).await
            })
            .await?)
    }
}

impl PartialEq for PrivateDirectoryContent {
    fn eq(&self, other: &Self) -> bool {
        self.previous == other.previous
            && self.metadata == other.metadata
            && self.entries == other.entries
    }
}

impl Clone for PrivateDirectoryContent {
    fn clone(&self) -> Self {
        Self {
            persisted_as: OnceCell::new_with(self.persisted_as.get().cloned()),
            previous: self.previous.clone(),
            metadata: self.metadata.clone(),
            entries: self.entries.clone(),
        }
    }
}

impl Id for PrivateDirectory {
    fn get_id(&self) -> String {
        format!("{:p}", &self.header)
    }
}

//--------------------------------------------------------------------------------------------------
// Tests
//--------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::test_runner::{RngAlgorithm, TestRng};
    use rand::thread_rng;
    use rand_chacha::ChaCha12Rng;
    use rand_core::SeedableRng;
    use test_log::test;
    use wnfs_common::{utils, MemoryBlockStore};

    #[test(async_std::test)]
    async fn can_create_directories_deterministically_with_user_provided_seeds() {
        let rng = &mut TestRng::deterministic_rng(RngAlgorithm::ChaCha);
        let setup = &AccumulatorSetup::from_rsa_2048(rng);
        let ratchet_seed = utils::get_random_bytes::<32>(rng);
        let inumber = NameSegment::new(rng);

        let dir1 =
            PrivateDirectory::with_seed(&Name::empty(), Utc::now(), ratchet_seed, inumber.clone());

        let dir2 = PrivateDirectory::with_seed(&Name::empty(), Utc::now(), ratchet_seed, inumber);

        assert_eq!(
            dir1.header.derive_temporal_key(),
            dir2.header.derive_temporal_key()
        );

        assert_eq!(
            dir1.header.get_name_hash(setup),
            dir2.header.get_name_hash(setup)
        );
    }

    #[test(async_std::test)]
    async fn look_up_can_fetch_file_added_to_directory() {
        let rng = &mut TestRng::deterministic_rng(RngAlgorithm::ChaCha);
        let root_dir = &mut Rc::new(PrivateDirectory::new(&Name::empty(), Utc::now(), rng));
        let store = &mut MemoryBlockStore::default();
        let forest = &mut Rc::new(PrivateForest::new_rsa_2048(rng));

        let content = b"Hello, World!".to_vec();

        root_dir
            .write(
                &["text.txt".into()],
                true,
                Utc::now(),
                content.clone(),
                forest,
                store,
                rng,
            )
            .await
            .unwrap();

        let result = root_dir
            .read(&["text.txt".into()], true, forest, store)
            .await
            .unwrap();

        assert_eq!(result, content);
    }

    #[test(async_std::test)]
    async fn look_up_cannot_fetch_file_not_added_to_directory() {
        let rng = &mut TestRng::deterministic_rng(RngAlgorithm::ChaCha);
        let root_dir = Rc::new(PrivateDirectory::new(&Name::empty(), Utc::now(), rng));
        let store = &MemoryBlockStore::default();
        let forest = &Rc::new(PrivateForest::new_rsa_2048(rng));

        let node = root_dir
            .lookup_node("Unknown", true, forest, store)
            .await
            .unwrap();

        assert!(node.is_none());
    }

    #[test(async_std::test)]
    async fn mkdir_can_create_new_directory() {
        let rng = &mut TestRng::deterministic_rng(RngAlgorithm::ChaCha);
        let root_dir = &mut Rc::new(PrivateDirectory::new(&Name::empty(), Utc::now(), rng));
        let store = &mut MemoryBlockStore::default();
        let forest = &mut Rc::new(PrivateForest::new_rsa_2048(rng));

        root_dir
            .mkdir(
                &["tamedun".into(), "pictures".into()],
                true,
                Utc::now(),
                forest,
                store,
                rng,
            )
            .await
            .unwrap();

        let result = root_dir
            .get_node(&["tamedun".into(), "pictures".into()], true, forest, store)
            .await
            .unwrap();

        assert!(result.is_some());
    }

    #[test(async_std::test)]
    async fn ls_can_list_children_under_directory() {
        let rng = &mut TestRng::deterministic_rng(RngAlgorithm::ChaCha);
        let root_dir = &mut Rc::new(PrivateDirectory::new(&Name::empty(), Utc::now(), rng));
        let store = &mut MemoryBlockStore::default();
        let forest = &mut Rc::new(PrivateForest::new_rsa_2048(rng));

        root_dir
            .mkdir(
                &["tamedun".into(), "pictures".into()],
                true,
                Utc::now(),
                forest,
                store,
                rng,
            )
            .await
            .unwrap();

        root_dir
            .write(
                &["tamedun".into(), "pictures".into(), "puppy.jpg".into()],
                true,
                Utc::now(),
                b"puppy".to_vec(),
                forest,
                store,
                rng,
            )
            .await
            .unwrap();

        root_dir
            .mkdir(
                &["tamedun".into(), "pictures".into(), "cats".into()],
                true,
                Utc::now(),
                forest,
                store,
                rng,
            )
            .await
            .unwrap();

        let result = root_dir
            .ls(&["tamedun".into(), "pictures".into()], true, forest, store)
            .await
            .unwrap();

        assert_eq!(result.len(), 2);
        assert_eq!(result[0].0, String::from("cats"));
        assert_eq!(result[1].0, String::from("puppy.jpg"));
    }

    #[test(async_std::test)]
    async fn rm_can_remove_children_from_directory() {
        let rng = &mut TestRng::deterministic_rng(RngAlgorithm::ChaCha);
        let root_dir = &mut Rc::new(PrivateDirectory::new(&Name::empty(), Utc::now(), rng));
        let store = &mut MemoryBlockStore::default();
        let forest = &mut Rc::new(PrivateForest::new_rsa_2048(rng));

        root_dir
            .mkdir(
                &["tamedun".into(), "pictures".into()],
                true,
                Utc::now(),
                forest,
                store,
                rng,
            )
            .await
            .unwrap();

        root_dir
            .write(
                &["tamedun".into(), "pictures".into(), "puppy.jpg".into()],
                true,
                Utc::now(),
                b"puppy".to_vec(),
                forest,
                store,
                rng,
            )
            .await
            .unwrap();

        root_dir
            .mkdir(
                &["tamedun".into(), "pictures".into(), "cats".into()],
                true,
                Utc::now(),
                forest,
                store,
                rng,
            )
            .await
            .unwrap();

        root_dir
            .rm(&["tamedun".into(), "pictures".into()], true, forest, store)
            .await
            .unwrap();

        let result = root_dir
            .rm(&["tamedun".into(), "pictures".into()], true, forest, store)
            .await;

        assert!(result.is_err());
    }

    #[async_std::test]
    async fn read_can_fetch_userland_of_file_added_to_directory() {
        let rng = &mut TestRng::deterministic_rng(RngAlgorithm::ChaCha);
        let root_dir = &mut Rc::new(PrivateDirectory::new(&Name::empty(), Utc::now(), rng));
        let store = &mut MemoryBlockStore::default();
        let forest = &mut Rc::new(PrivateForest::new_rsa_2048(rng));

        root_dir
            .write(
                &["text.txt".into()],
                true,
                Utc::now(),
                b"text".to_vec(),
                forest,
                store,
                rng,
            )
            .await
            .unwrap();

        let result = root_dir
            .read(&["text.txt".into()], true, forest, store)
            .await
            .unwrap();

        assert_eq!(result, b"text".to_vec());
    }

    #[test(async_std::test)]
    async fn search_latest_finds_the_most_recent() {
        let rng = &mut rand::thread_rng();
        let store = &mut MemoryBlockStore::default();
        let forest = &mut Rc::new(PrivateForest::new_rsa_2048(rng));
        let root_dir = &mut Rc::new(PrivateDirectory::new(&Name::empty(), Utc::now(), rng));

        let path = ["Documents".into(), "file.txt".into()];

        root_dir
            .write(
                &path,
                false,
                Utc::now(),
                b"One".to_vec(),
                forest,
                store,
                rng,
            )
            .await
            .unwrap();

        root_dir.store(forest, store, rng).await.unwrap();

        let old_root = &Rc::clone(root_dir);

        root_dir
            .write(&path, true, Utc::now(), b"Two".to_vec(), forest, store, rng)
            .await
            .unwrap();

        root_dir.store(forest, store, rng).await.unwrap();

        let new_read = root_dir.read(&path, false, forest, store).await.unwrap();

        let old_read = Rc::clone(old_root)
            .read(&path, false, forest, store)
            .await
            .unwrap();

        let old_read_latest = old_root.read(&path, true, forest, store).await.unwrap();
        let new_read_latest = root_dir.read(&path, true, forest, store).await.unwrap();

        assert_eq!(&String::from_utf8_lossy(&new_read), "Two");
        assert_eq!(&String::from_utf8_lossy(&old_read), "One");
        assert_eq!(&String::from_utf8_lossy(&old_read_latest), "Two");
        assert_eq!(&String::from_utf8_lossy(&new_read_latest), "Two");
    }

    #[async_std::test]
    async fn cp_can_copy_sub_directory_to_another_valid_location_with_updated_ancestry() {
        let rng = &mut ChaCha12Rng::seed_from_u64(0);
        let store = &mut MemoryBlockStore::default();
        let forest = &mut Rc::new(PrivateForest::new_rsa_2048(rng));
        let root_dir = &mut Rc::new(PrivateDirectory::new(&Name::empty(), Utc::now(), rng));

        root_dir
            .write(
                &["pictures".into(), "cats".into(), "tabby.jpg".into()],
                true,
                Utc::now(),
                b"tabby".to_vec(),
                forest,
                store,
                rng,
            )
            .await
            .unwrap();

        root_dir
            .write(
                &["pictures".into(), "cats".into(), "luna.png".into()],
                true,
                Utc::now(),
                b"luna".to_vec(),
                forest,
                store,
                rng,
            )
            .await
            .unwrap();

        root_dir
            .mkdir(&["images".into()], true, Utc::now(), forest, store, rng)
            .await
            .unwrap();

        root_dir
            .cp(
                &["pictures".into(), "cats".into()],
                &["images".into(), "cats".into()],
                true,
                Utc::now(),
                forest,
                store,
                rng,
            )
            .await
            .unwrap();

        let result = root_dir
            .ls(&["images".into()], true, forest, store)
            .await
            .unwrap();

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].0, String::from("cats"));

        let result = root_dir
            .ls(&["pictures".into()], true, forest, store)
            .await
            .unwrap();

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].0, String::from("cats"));

        let _result = root_dir
            .get_node(&["images".into(), "cats".into()], true, forest, store)
            .await
            .unwrap();

        // TODO(matheus23)
        // let cats_bare_name = result.unwrap().get_header().bare_name.clone();

        // let images_dir_inumber = root_dir
        //     .lookup_node("images", true, forest, store)
        //     .await
        //     .unwrap()
        //     .unwrap()
        //     .get_header()
        //     .inumber;

        // let pictures_dir_inumber = root_dir
        //     .lookup_node("pictures", true, forest, store)
        //     .await
        //     .unwrap()
        //     .unwrap()
        //     .get_header()
        //     .inumber;

        // assert!(cats_bare_name.contains(&images_dir_inumber));
        // assert!(!cats_bare_name.contains(&pictures_dir_inumber));
    }

    #[async_std::test]
    async fn mv_can_move_sub_directory_to_another_valid_location_with_updated_ancestry() {
        let rng = &mut thread_rng();
        let store = &mut MemoryBlockStore::default();
        let forest = &mut Rc::new(PrivateForest::new_rsa_2048(rng));
        let root_dir = &mut Rc::new(PrivateDirectory::new(&Name::empty(), Utc::now(), rng));

        root_dir
            .write(
                &["pictures".into(), "cats".into(), "tabby.jpg".into()],
                true,
                Utc::now(),
                b"tabby".to_vec(),
                forest,
                store,
                rng,
            )
            .await
            .unwrap();

        root_dir
            .write(
                &["pictures".into(), "cats".into(), "luna.png".into()],
                true,
                Utc::now(),
                b"luna".to_vec(),
                forest,
                store,
                rng,
            )
            .await
            .unwrap();

        root_dir
            .mkdir(&["images".into()], true, Utc::now(), forest, store, rng)
            .await
            .unwrap();

        root_dir
            .basic_mv(
                &["pictures".into(), "cats".into()],
                &["images".into(), "cats".into()],
                true,
                Utc::now(),
                forest,
                store,
                rng,
            )
            .await
            .unwrap();

        let result = root_dir
            .ls(&["images".into()], true, forest, store)
            .await
            .unwrap();

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].0, String::from("cats"));

        let result = root_dir
            .ls(&["pictures".into()], true, forest, store)
            .await
            .unwrap();

        assert_eq!(result.len(), 0);

        let _result = root_dir
            .get_node(&["images".into(), "cats".into()], true, forest, store)
            .await
            .unwrap();

        // TODO(matheus23)
        // let cats_bare_name = result.unwrap().get_header().bare_name.clone();

        // let images_dir_inumber = root_dir
        //     .lookup_node("images", true, forest, store)
        //     .await
        //     .unwrap()
        //     .unwrap()
        //     .get_header()
        //     .inumber;

        // let pictures_dir_inumber = root_dir
        //     .lookup_node("pictures", true, forest, store)
        //     .await
        //     .unwrap()
        //     .unwrap()
        //     .get_header()
        //     .inumber;

        // assert!(cats_bare_name.contains(&images_dir_inumber));
        // assert!(!cats_bare_name.contains(&pictures_dir_inumber));
    }

    #[async_std::test]
    async fn mv_cannot_move_sub_directory_to_invalid_location() {
        let rng = &mut thread_rng();
        let store = &mut MemoryBlockStore::default();
        let forest = &mut Rc::new(PrivateForest::new_rsa_2048(rng));
        let root_dir = &mut Rc::new(PrivateDirectory::new(&Name::empty(), Utc::now(), rng));

        root_dir
            .mkdir(
                &[
                    "videos".into(),
                    "movies".into(),
                    "anime".into(),
                    "ghibli".into(),
                ],
                true,
                Utc::now(),
                forest,
                store,
                rng,
            )
            .await
            .unwrap();

        let result = root_dir
            .basic_mv(
                &["videos".into(), "movies".into()],
                &["videos".into(), "movies".into(), "anime".into()],
                true,
                Utc::now(),
                forest,
                store,
                rng,
            )
            .await;

        assert!(result.is_err());
    }

    #[async_std::test]
    async fn mv_can_rename_directories() {
        let rng = &mut thread_rng();
        let store = &mut MemoryBlockStore::default();
        let forest = &mut Rc::new(PrivateForest::new_rsa_2048(rng));
        let root_dir = &mut Rc::new(PrivateDirectory::new(&Name::empty(), Utc::now(), rng));
        let content = b"file".to_vec();

        root_dir
            .write(
                &["file.txt".into()],
                true,
                Utc::now(),
                content.clone(),
                forest,
                store,
                rng,
            )
            .await
            .unwrap();

        root_dir
            .basic_mv(
                &["file.txt".into()],
                &["renamed.txt".into()],
                true,
                Utc::now(),
                forest,
                store,
                rng,
            )
            .await
            .unwrap();

        let result = root_dir
            .read(&["renamed.txt".into()], true, forest, store)
            .await
            .unwrap();

        assert!(result == content);

        let result = root_dir
            .lookup_node("file.txt", true, forest, store)
            .await
            .unwrap();

        assert!(result.is_none());
    }

    #[async_std::test]
    async fn mv_fails_moving_directories_to_files() {
        let rng = &mut thread_rng();
        let store = &mut MemoryBlockStore::default();
        let forest = &mut Rc::new(PrivateForest::new_rsa_2048(rng));
        let root_dir = &mut Rc::new(PrivateDirectory::new(&Name::empty(), Utc::now(), rng));

        root_dir
            .mkdir(
                &["movies".into(), "ghibli".into()],
                true,
                Utc::now(),
                forest,
                store,
                rng,
            )
            .await
            .unwrap();

        root_dir
            .write(
                &["file.txt".into()],
                true,
                Utc::now(),
                b"file".to_vec(),
                forest,
                store,
                rng,
            )
            .await
            .unwrap();

        let result = root_dir
            .basic_mv(
                &["movies".into(), "ghibli".into()],
                &["file.txt".into()],
                true,
                Utc::now(),
                forest,
                store,
                rng,
            )
            .await;

        assert!(result.is_err());
    }

    #[async_std::test]
    async fn write_doesnt_generate_previous_link() {
        let rng = &mut thread_rng();
        let store = &mut MemoryBlockStore::new();
        let forest = &mut Rc::new(PrivateForest::new_rsa_2048(rng));
        let old_dir = &mut Rc::new(PrivateDirectory::new(&Name::empty(), Utc::now(), rng));

        let new_dir = &mut Rc::clone(old_dir);
        new_dir
            .write(
                &["file.txt".into()],
                false,
                Utc::now(),
                b"Hello".to_vec(),
                forest,
                store,
                rng,
            )
            .await
            .unwrap();

        assert!(old_dir.content.previous.is_empty());
        assert!(new_dir.content.previous.is_empty());
    }

    #[async_std::test]
    async fn store_before_write_generates_previous_link() {
        let rng = &mut thread_rng();
        let store = &mut MemoryBlockStore::new();
        let forest = &mut Rc::new(PrivateForest::new_rsa_2048(rng));
        let old_dir = &mut Rc::new(PrivateDirectory::new(&Name::empty(), Utc::now(), rng));
        old_dir.store(forest, store, rng).await.unwrap();

        let new_dir = &mut Rc::clone(old_dir);
        new_dir
            .write(
                &["file.txt".into()],
                false,
                Utc::now(),
                b"Hello".to_vec(),
                forest,
                store,
                rng,
            )
            .await
            .unwrap();

        assert!(old_dir.content.previous.is_empty());
        assert_eq!(new_dir.content.previous.len(), 1);
    }
}
