use std::{collections::BTreeMap, rc::Rc};

use anyhow::{bail, ensure, Result};
use chrono::{DateTime, Utc};
use rand_core::RngCore;
use semver::Version;
use serde::{de::Error as DeError, ser::Error as SerError, Deserialize, Deserializer, Serialize};

use super::{
    namefilter::Namefilter, Key, PrivateFile, PrivateForest, PrivateNode, PrivateNodeHeader,
    PrivateRef, RatchetKey,
};

use crate::{
    dagcbor, error, utils, BlockStore, FsError, Id, Metadata, NodeType, PathNodes, PathNodesResult,
};

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
/// use wnfs::{PrivateDirectory, Namefilter};
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
    pub version: Version,
    pub header: PrivateNodeHeader,
    pub metadata: Metadata,
    pub entries: BTreeMap<String, PrivateRef>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PrivateDirectorySerde {
    pub r#type: NodeType,
    pub version: Version,
    pub header: Vec<u8>,
    pub metadata: Metadata,
    pub entries: BTreeMap<String, PrivateRef>,
}

/// The result of an operation applied to a directory.
#[derive(Debug, Clone)]
pub struct PrivateOpResult<T> {
    /// The root directory.
    pub root_dir: Rc<PrivateDirectory>,
    /// The hamt forest.
    pub hamt: Rc<PrivateForest>,
    /// Implementation dependent but it usually the last leaf node operated on.
    pub result: T,
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
    /// use wnfs::{PrivateDirectory, Namefilter};
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
    pub fn new<R: RngCore>(parent_bare_name: Namefilter, time: DateTime<Utc>, rng: &mut R) -> Self {
        Self {
            version: Version::new(0, 2, 0),
            header: PrivateNodeHeader::new(parent_bare_name, rng),
            metadata: Metadata::new(time),
            entries: BTreeMap::new(),
        }
    }

    /// Gets the metadata of the directory
    ///
    /// # Examples
    ///
    /// ```
    /// use wnfs::{PrivateDirectory, Namefilter, Metadata};
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
        &self.metadata
    }

    /// Advances the ratchet.
    pub(crate) fn advance_ratchet(&mut self) {
        self.header.advance_ratchet();
    }

    /// Creates a new `PathNodes` that is not based on an existing file tree.
    pub(crate) fn create_path_nodes<R: RngCore>(
        path_segments: &[String],
        time: DateTime<Utc>,
        parent_bare_name: Namefilter,
        rng: &mut R,
    ) -> PrivatePathNodes {
        let mut working_parent_bare_name = parent_bare_name;
        let path: Vec<(Rc<PrivateDirectory>, String)> = path_segments
            .iter()
            .map(|segment| {
                // Create new private directory.
                let directory = Rc::new(PrivateDirectory::new(
                    std::mem::take(&mut working_parent_bare_name),
                    time,
                    rng,
                ));

                working_parent_bare_name = directory.header.bare_name.clone();

                (directory, segment.clone())
            })
            .collect();

        PrivatePathNodes {
            path,
            tail: Rc::new(PrivateDirectory::new(
                std::mem::take(&mut working_parent_bare_name),
                time,
                rng,
            )),
        }
    }

    /// Uses specified path segments and their existence in the file tree to generate `PathNodes`.
    ///
    /// Supports cases where the entire path does not exist.
    pub(crate) async fn get_path_nodes<B: BlockStore>(
        self: Rc<Self>,
        path_segments: &[String],
        search_latest: bool,
        hamt: &PrivateForest,
        store: &B,
    ) -> Result<PrivatePathNodesResult> {
        use PathNodesResult::*;
        let mut working_node = self;
        let mut path_nodes = Vec::with_capacity(path_segments.len());

        for path_segment in path_segments {
            match working_node
                .lookup_node(path_segment, search_latest, hamt, store)
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

    /// Uses specified path segments to generate `PathNodes`. Creates missing directories as needed.
    pub(crate) async fn get_or_create_path_nodes<B: BlockStore, R: RngCore>(
        self: Rc<Self>,
        path_segments: &[String],
        search_latest: bool,
        time: DateTime<Utc>,
        hamt: &PrivateForest,
        store: &mut B,
        rng: &mut R,
    ) -> Result<PrivatePathNodes> {
        use PathNodesResult::*;
        match self
            .get_path_nodes(path_segments, search_latest, hamt, store)
            .await?
        {
            Complete(path_nodes) => Ok(path_nodes),
            NotADirectory(_, _) => error(FsError::InvalidPath),
            MissingLink(path_so_far, missing_link) => {
                // Get remaining missing path segments.
                let missing_path = path_segments.split_at(path_so_far.path.len() + 1).1;

                // Get tail bare name from `path_so_far`.
                let parent_bare_name = path_so_far.tail.header.bare_name.clone();

                // Create missing directories.
                let missing_path_nodes =
                    Self::create_path_nodes(missing_path, time, parent_bare_name, rng);

                Ok(PrivatePathNodes {
                    path: [
                        path_so_far.path,
                        vec![(path_so_far.tail, missing_link)],
                        missing_path_nodes.path,
                    ]
                    .concat(),
                    tail: missing_path_nodes.tail,
                })
            }
        }
    }

    /// Fix up `PathNodes` so that parents refer to the newly updated children.
    async fn fix_up_path_nodes<B: BlockStore, R: RngCore>(
        path_nodes: PrivatePathNodes,
        hamt: Rc<PrivateForest>,
        store: &mut B,
        rng: &mut R,
    ) -> Result<(Rc<Self>, Rc<PrivateForest>)> {
        let mut working_hamt = Rc::clone(&hamt);
        let mut working_child_dir = {
            let mut tmp = (*path_nodes.tail).clone();
            tmp.advance_ratchet();
            Rc::new(tmp)
        };

        for (parent_dir, segment) in path_nodes.path.iter().rev() {
            let mut parent_dir = (**parent_dir).clone();
            parent_dir.advance_ratchet();
            let child_private_ref = working_child_dir.header.get_private_ref()?;

            parent_dir
                .entries
                .insert(segment.clone(), child_private_ref.clone());

            let parent_dir = Rc::new(parent_dir);

            working_hamt = working_hamt
                .set(
                    working_child_dir.header.get_saturated_name(),
                    &child_private_ref,
                    &PrivateNode::Dir(Rc::clone(&working_child_dir)),
                    store,
                    rng,
                )
                .await?;

            working_child_dir = parent_dir;
        }

        working_hamt = working_hamt
            .set(
                working_child_dir.header.get_saturated_name(),
                &working_child_dir.header.get_private_ref()?,
                &PrivateNode::Dir(Rc::clone(&working_child_dir)),
                store,
                rng,
            )
            .await?;

        Ok((working_child_dir, working_hamt))
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
    ///     private::{PrivateForest, PrivateRef},
    ///     BlockStore, MemoryBlockStore, Namefilter, PrivateDirectory, PrivateOpResult,
    /// };
    ///
    /// #[async_std::main]
    /// async fn main() {
    ///     let store = &mut MemoryBlockStore::default();
    ///     let rng = &mut thread_rng();
    ///     let hamt = Rc::new(PrivateForest::new());
    ///
    ///     let dir = Rc::new(PrivateDirectory::new(
    ///         Namefilter::default(),
    ///         Utc::now(),
    ///         rng,
    ///     ));
    ///
    ///     let PrivateOpResult { hamt, root_dir, .. } = dir
    ///         .mkdir(&["pictures".into(), "cats".into()], true, Utc::now(), hamt, store, rng)
    ///         .await
    ///         .unwrap();
    ///
    ///     let PrivateOpResult { result, .. } = root_dir
    ///         .get_node(&["pictures".into(), "cats".into()], true, hamt, store)
    ///         .await
    ///         .unwrap();
    ///
    ///     assert!(result.is_some());
    /// }
    /// ```
    pub async fn get_node<B: BlockStore>(
        self: Rc<Self>,
        path_segments: &[String],
        search_latest: bool,
        hamt: Rc<PrivateForest>,
        store: &B,
    ) -> Result<PrivateOpResult<Option<PrivateNode>>> {
        use PathNodesResult::*;
        let root_dir = Rc::clone(&self);

        Ok(match path_segments.split_last() {
            Some((path_segment, parent_path)) => {
                match self
                    .get_path_nodes(parent_path, search_latest, &hamt, store)
                    .await?
                {
                    Complete(parent_path_nodes) => {
                        let result = parent_path_nodes
                            .tail
                            .lookup_node(path_segment, search_latest, &hamt, store)
                            .await?;

                        PrivateOpResult {
                            root_dir,
                            hamt,
                            result,
                        }
                    }
                    MissingLink(_, _) => bail!(FsError::NotFound),
                    NotADirectory(_, _) => bail!(FsError::NotFound),
                }
            }
            None => PrivateOpResult {
                root_dir,
                hamt,
                result: Some(PrivateNode::Dir(self)),
            },
        })
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
    ///     private::{PrivateForest, PrivateRef},
    ///     BlockStore, MemoryBlockStore, Namefilter, PrivateDirectory, PrivateOpResult,
    /// };
    ///
    /// #[async_std::main]
    /// async fn main() {
    ///     let store = &mut MemoryBlockStore::default();
    ///     let rng = &mut thread_rng();
    ///     let hamt = Rc::new(PrivateForest::new());
    ///
    ///     let dir = Rc::new(PrivateDirectory::new(
    ///         Namefilter::default(),
    ///         Utc::now(),
    ///         rng,
    ///     ));
    ///
    ///     let content = b"print('hello world')";
    ///
    ///     let PrivateOpResult { hamt, root_dir, .. } = dir
    ///         .write(
    ///             &["code".into(), "hello.py".into()],
    ///             true,
    ///             Utc::now(),
    ///             content.to_vec(),
    ///             hamt,
    ///             store,
    ///             rng
    ///         )
    ///         .await
    ///         .unwrap();
    ///
    ///     let PrivateOpResult { result, .. } = root_dir
    ///         .read(&["code".into(), "hello.py".into()], true, hamt, store)
    ///         .await
    ///         .unwrap();
    ///
    ///     assert_eq!(&result, content);
    /// }
    /// ```
    pub async fn read<B: BlockStore>(
        self: Rc<Self>,
        path_segments: &[String],
        search_latest: bool,
        hamt: Rc<PrivateForest>,
        store: &B,
    ) -> Result<PrivateOpResult<Vec<u8>>> {
        let root_dir = Rc::clone(&self);
        let (path, filename) = utils::split_last(path_segments)?;

        match self
            .get_path_nodes(path, search_latest, &hamt, store)
            .await?
        {
            PathNodesResult::Complete(node_path) => {
                match node_path
                    .tail
                    .lookup_node(filename, search_latest, &hamt, store)
                    .await?
                {
                    Some(PrivateNode::File(file)) => Ok(PrivateOpResult {
                        root_dir,
                        hamt,
                        result: file.content.clone(),
                    }),
                    Some(PrivateNode::Dir(_)) => error(FsError::NotAFile),
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
    ///
    /// use chrono::Utc;
    /// use rand::thread_rng;
    ///
    /// use wnfs::{
    ///     private::{PrivateForest, PrivateRef},
    ///     BlockStore, MemoryBlockStore, Namefilter, PrivateDirectory, PrivateOpResult,
    /// };
    ///
    /// #[async_std::main]
    /// async fn main() {
    ///     let store = &mut MemoryBlockStore::default();
    ///     let rng = &mut thread_rng();
    ///     let hamt = Rc::new(PrivateForest::new());
    ///
    ///     let dir = Rc::new(PrivateDirectory::new(
    ///         Namefilter::default(),
    ///         Utc::now(),
    ///         rng,
    ///     ));
    ///
    ///     let content = b"print('hello world')";
    ///
    ///     let PrivateOpResult { hamt, root_dir, .. } = dir
    ///         .write(
    ///             &["code".into(), "hello.py".into()],
    ///             true,
    ///             Utc::now(),
    ///             content.to_vec(),
    ///             hamt,
    ///             store,
    ///             rng
    ///         )
    ///         .await
    ///         .unwrap();
    ///
    ///     let PrivateOpResult { result, .. } = root_dir
    ///         .read(&["code".into(), "hello.py".into()], true, hamt, store)
    ///         .await
    ///         .unwrap();
    ///
    ///     assert_eq!(&result, content);
    /// }
    /// ```
    #[allow(clippy::too_many_arguments)]
    pub async fn write<B: BlockStore, R: RngCore>(
        self: Rc<Self>,
        path_segments: &[String],
        search_latest: bool,
        time: DateTime<Utc>,
        content: Vec<u8>,
        hamt: Rc<PrivateForest>,
        store: &mut B,
        rng: &mut R,
    ) -> Result<PrivateOpResult<()>> {
        let (directory_path, filename) = utils::split_last(path_segments)?;

        // This will create directories if they don't exist yet
        let mut directory_path_nodes = self
            .get_or_create_path_nodes(directory_path, search_latest, time, &hamt, store, rng)
            .await?;

        let mut directory = (*directory_path_nodes.tail).clone();

        // Modify the file if it already exists, otherwise create a new file with expected content
        let file = match directory
            .lookup_node(filename, search_latest, &hamt, store)
            .await?
        {
            Some(PrivateNode::File(file_before)) => {
                let mut file = (*file_before).clone();
                file.content = content;
                file.metadata.upsert_mtime(time);
                file.header.advance_ratchet();
                file
            }
            Some(PrivateNode::Dir(_)) => bail!(FsError::DirectoryAlreadyExists),
            None => PrivateFile::new(directory.header.bare_name.clone(), time, content, rng),
        };

        let child_private_ref = file.header.get_private_ref()?;
        let hamt = hamt
            .set(
                file.header.get_saturated_name(),
                &child_private_ref,
                &PrivateNode::File(Rc::new(file)),
                store,
                rng,
            )
            .await?;

        // Insert the file into its parent directory
        directory
            .entries
            .insert(filename.to_string(), child_private_ref);

        directory_path_nodes.tail = Rc::new(directory);

        let (root_dir, hamt) =
            Self::fix_up_path_nodes(directory_path_nodes, hamt, store, rng).await?;

        // Fix up the file path
        Ok(PrivateOpResult {
            root_dir,
            hamt,
            result: (),
        })
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
    ///     private::{PrivateForest, PrivateRef},
    ///     BlockStore, MemoryBlockStore, Namefilter, PrivateDirectory, PrivateOpResult,
    /// };
    ///
    /// #[async_std::main]
    /// async fn main() {
    ///     let store = &mut MemoryBlockStore::default();
    ///     let rng = &mut thread_rng();
    ///     let hamt = Rc::new(PrivateForest::new());
    ///
    ///     let dir = Rc::new(PrivateDirectory::new(
    ///         Namefilter::default(),
    ///         Utc::now(),
    ///         rng,
    ///     ));
    ///
    ///     let PrivateOpResult { hamt, root_dir, .. } = dir
    ///         .mkdir(&["pictures".into(), "cats".into()], true, Utc::now(), hamt, store, rng)
    ///         .await
    ///         .unwrap();
    ///
    ///     let node = root_dir.lookup_node("pictures", true, &hamt, store)
    ///         .await
    ///         .unwrap();
    ///
    ///     assert!(node.is_some());
    /// }
    /// ```
    pub async fn lookup_node<'a, B: BlockStore>(
        &self,
        path_segment: &str,
        search_latest: bool,
        hamt: &PrivateForest,
        store: &B,
    ) -> Result<Option<PrivateNode>> {
        Ok(match self.entries.get(path_segment) {
            Some(private_ref) => {
                let private_node = hamt.get(private_ref, store).await?;
                match (search_latest, private_node) {
                    (true, Some(node)) => Some(node.search_latest(hamt, store).await?),
                    (_, node) => node,
                }
            }
            None => None,
        })
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
    ///     private::{PrivateForest, PrivateRef},
    ///     BlockStore, MemoryBlockStore, Namefilter, PrivateDirectory, PrivateOpResult,
    /// };
    ///
    /// #[async_std::main]
    /// async fn main() {
    ///     let store = &mut MemoryBlockStore::default();
    ///     let rng = &mut thread_rng();
    ///     let hamt = Rc::new(PrivateForest::new());
    ///
    ///     let dir = Rc::new(PrivateDirectory::new(
    ///         Namefilter::default(),
    ///         Utc::now(),
    ///         rng,
    ///     ));
    ///
    ///     let PrivateOpResult { hamt, root_dir, .. } = dir
    ///         .mkdir(&["pictures".into(), "cats".into()], true, Utc::now(), hamt, store, rng)
    ///         .await
    ///         .unwrap();
    ///
    ///     let node = root_dir.lookup_node("pictures", true, &hamt, store)
    ///         .await
    ///         .unwrap();
    ///
    ///     assert!(node.is_some());
    /// }
    /// ```
    pub async fn mkdir<B: BlockStore, R: RngCore>(
        self: Rc<Self>,
        path_segments: &[String],
        search_latest: bool,
        time: DateTime<Utc>,
        hamt: Rc<PrivateForest>,
        store: &mut B,
        rng: &mut R,
    ) -> Result<PrivateOpResult<()>> {
        let path_nodes = self
            .get_or_create_path_nodes(path_segments, search_latest, time, &hamt, store, rng)
            .await?;

        let (root_dir, hamt) = Self::fix_up_path_nodes(path_nodes, hamt, store, rng).await?;

        Ok(PrivateOpResult {
            root_dir,
            hamt,
            result: (),
        })
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
    ///     private::{PrivateForest, PrivateRef},
    ///     BlockStore, MemoryBlockStore, Namefilter, PrivateDirectory, PrivateOpResult,
    /// };
    ///
    /// #[async_std::main]
    /// async fn main() {
    ///     let store = &mut MemoryBlockStore::default();
    ///     let rng = &mut thread_rng();
    ///     let hamt = Rc::new(PrivateForest::new());
    ///
    ///     let dir = Rc::new(PrivateDirectory::new(
    ///         Namefilter::default(),
    ///         Utc::now(),
    ///         rng,
    ///     ));
    ///
    ///     let PrivateOpResult { hamt, root_dir, .. } = dir
    ///         .write(
    ///             &["code".into(), "hello.py".into()],
    ///             true,
    ///             Utc::now(),
    ///             b"print('hello world')".to_vec(),
    ///             hamt,
    ///             store,
    ///             rng
    ///         )
    ///         .await
    ///         .unwrap();
    ///
    ///     let PrivateOpResult { hamt, root_dir, .. } = root_dir
    ///         .mkdir(&["code".into(), "bin".into()], true, Utc::now(), hamt, store, rng)
    ///         .await
    ///         .unwrap();
    ///
    ///     let PrivateOpResult { result, .. } = root_dir
    ///         .ls(&["code".into()], true, hamt, store)
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
    pub async fn ls<B: BlockStore>(
        self: Rc<Self>,
        path_segments: &[String],
        search_latest: bool,
        hamt: Rc<PrivateForest>,
        store: &B,
    ) -> Result<PrivateOpResult<Vec<(String, Metadata)>>> {
        let root_dir = Rc::clone(&self);
        match self
            .get_path_nodes(path_segments, search_latest, &hamt, store)
            .await?
        {
            PathNodesResult::Complete(path_nodes) => {
                let mut result = vec![];
                for (name, private_ref) in path_nodes.tail.entries.iter() {
                    match hamt.get(private_ref, store).await? {
                        Some(PrivateNode::File(file)) => {
                            result.push((name.clone(), file.metadata.clone()));
                        }
                        Some(PrivateNode::Dir(dir)) => {
                            result.push((name.clone(), dir.metadata.clone()));
                        }
                        _ => bail!(FsError::NotFound),
                    }
                }
                Ok(PrivateOpResult {
                    root_dir,
                    hamt,
                    result,
                })
            }
            _ => bail!(FsError::NotFound),
        }
    }

    /// Removes a file or directory from the directory.
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
    ///     private::{PrivateForest, PrivateRef},
    ///     BlockStore, MemoryBlockStore, Namefilter, PrivateDirectory, PrivateOpResult,
    /// };
    ///
    /// #[async_std::main]
    /// async fn main() {
    ///     let store = &mut MemoryBlockStore::default();
    ///     let rng = &mut thread_rng();
    ///     let hamt = Rc::new(PrivateForest::new());
    ///
    ///     let dir = Rc::new(PrivateDirectory::new(
    ///         Namefilter::default(),
    ///         Utc::now(),
    ///         rng,
    ///     ));
    ///
    ///     let PrivateOpResult { hamt, root_dir, .. } = dir
    ///         .write(
    ///             &["code".into(), "python".into(), "hello.py".into()],
    ///             true,
    ///             Utc::now(),
    ///             b"print('hello world')".to_vec(),
    ///             hamt,
    ///             store,
    ///             rng
    ///         )
    ///         .await
    ///         .unwrap();
    ///
    ///     let PrivateOpResult { hamt, root_dir, result } = root_dir
    ///         .ls(&["code".into()], true, hamt, store)
    ///         .await
    ///         .unwrap();
    ///
    ///     assert_eq!(result.len(), 1);
    ///
    ///     let PrivateOpResult { hamt, root_dir, .. } = root_dir
    ///         .rm(&["code".into(), "python".into()], true, hamt, store, rng)
    ///         .await
    ///         .unwrap();
    ///
    ///     let PrivateOpResult { result, .. } = root_dir
    ///         .ls(&["code".into()], true, hamt, store)
    ///         .await
    ///         .unwrap();
    ///
    ///     assert_eq!(result.len(), 0);
    /// }
    /// ```
    pub async fn rm<B: BlockStore, R: RngCore>(
        self: Rc<Self>,
        path_segments: &[String],
        search_latest: bool,
        hamt: Rc<PrivateForest>,
        store: &mut B,
        rng: &mut R,
    ) -> Result<PrivateOpResult<PrivateNode>> {
        let (directory_path, node_name) = utils::split_last(path_segments)?;

        let mut directory_path_nodes = match self
            .get_path_nodes(directory_path, search_latest, &hamt, store)
            .await?
        {
            PrivatePathNodesResult::Complete(node_path) => node_path,
            _ => bail!(FsError::NotFound),
        };

        let mut directory = (*directory_path_nodes.tail).clone();

        // Remove the entry from its parent directory
        let removed_node = match directory.entries.remove(node_name) {
            Some(ref private_ref) => hamt.get(private_ref, store).await?.unwrap(),
            None => bail!(FsError::NotFound),
        };

        directory_path_nodes.tail = Rc::new(directory);

        let (root_dir, hamt) =
            Self::fix_up_path_nodes(directory_path_nodes, hamt, store, rng).await?;

        Ok(PrivateOpResult {
            root_dir,
            hamt,
            result: removed_node,
        })
    }

    /// Attaches a node to the specified directory.
    ///
    /// Fixes up the subtree bare names to refer to the new parent.
    #[allow(clippy::too_many_arguments)]
    async fn attach<B: BlockStore, R: RngCore>(
        self: Rc<Self>,
        node: PrivateNode,
        path_segments: &[String],
        search_latest: bool,
        time: DateTime<Utc>,
        hamt: Rc<PrivateForest>,
        store: &mut B,
        rng: &mut R,
    ) -> Result<PrivateOpResult<()>> {
        let (directory_path, filename) = utils::split_last(path_segments)?;

        let mut path_nodes = match self
            .get_path_nodes(directory_path, search_latest, &hamt, store)
            .await?
        {
            PrivatePathNodesResult::Complete(node_path) => node_path,
            _ => bail!(FsError::NotFound),
        };

        let mut directory = (*path_nodes.tail).clone();

        ensure!(
            !directory.entries.contains_key(filename),
            FsError::FileAlreadyExists
        );

        let mut node = node.upsert_mtime(time);

        let hamt = node
            .update_ancestry(directory.header.bare_name.clone(), hamt, store, rng)
            .await?;

        directory
            .entries
            .insert(filename.clone(), node.get_header().get_private_ref()?);

        path_nodes.tail = Rc::new(directory);

        let (root_dir, hamt) = Self::fix_up_path_nodes(path_nodes, hamt, store, rng).await?;

        Ok(PrivateOpResult {
            root_dir,
            result: (),
            hamt,
        })
    }

    /// Moves a file or directory from one path to another.
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
    ///     private::{PrivateForest, PrivateRef},
    ///     BlockStore, MemoryBlockStore, Namefilter, PrivateDirectory, PrivateOpResult,
    /// };
    ///
    /// #[async_std::main]
    /// async fn main() {
    ///     let store = &mut MemoryBlockStore::default();
    ///     let rng = &mut thread_rng();
    ///     let hamt = Rc::new(PrivateForest::new());
    ///
    ///     let dir = Rc::new(PrivateDirectory::new(
    ///         Namefilter::default(),
    ///         Utc::now(),
    ///         rng,
    ///     ));
    ///
    ///     let PrivateOpResult { hamt, root_dir, .. } = dir
    ///         .write(
    ///             &["code".into(), "python".into(), "hello.py".into()],
    ///             true,
    ///             Utc::now(),
    ///             b"print('hello world')".to_vec(),
    ///             hamt,
    ///             store,
    ///             rng
    ///         )
    ///         .await
    ///         .unwrap();
    ///
    ///     let PrivateOpResult { hamt, root_dir, result } = root_dir
    ///         .basic_mv(
    ///             &["code".into(), "python".into(), "hello.py".into()],
    ///             &["code".into(), "hello.py".into()],
    ///             true,
    ///             Utc::now(),
    ///             hamt,
    ///             store,
    ///             rng
    ///         )
    ///         .await
    ///         .unwrap();
    ///
    ///     let PrivateOpResult { result, .. } = root_dir
    ///         .ls(&["code".into()], true, hamt, store)
    ///         .await
    ///         .unwrap();
    ///
    ///     assert_eq!(result.len(), 2);
    /// }
    /// ```
    #[allow(clippy::too_many_arguments)]
    pub async fn basic_mv<B: BlockStore, R: RngCore>(
        self: Rc<Self>,
        path_segments_from: &[String],
        path_segments_to: &[String],
        search_latest: bool,
        time: DateTime<Utc>,
        hamt: Rc<PrivateForest>,
        store: &mut B,
        rng: &mut R,
    ) -> Result<PrivateOpResult<()>> {
        let PrivateOpResult {
            root_dir,
            result: removed_node,
            hamt,
        } = self
            .rm(path_segments_from, search_latest, hamt, store, rng)
            .await?;

        root_dir
            .attach(
                removed_node,
                path_segments_to,
                search_latest,
                time,
                hamt,
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
    ///     private::{PrivateForest, PrivateRef},
    ///     BlockStore, MemoryBlockStore, Namefilter, PrivateDirectory, PrivateOpResult,
    /// };
    ///
    /// #[async_std::main]
    /// async fn main() {
    ///     let store = &mut MemoryBlockStore::default();
    ///     let rng = &mut thread_rng();
    ///     let hamt = Rc::new(PrivateForest::new());
    ///
    ///     let dir = Rc::new(PrivateDirectory::new(
    ///         Namefilter::default(),
    ///         Utc::now(),
    ///         rng,
    ///     ));
    ///
    ///     let PrivateOpResult { hamt, root_dir, .. } = dir
    ///         .write(
    ///             &["code".into(), "python".into(), "hello.py".into()],
    ///             true,
    ///             Utc::now(),
    ///             b"print('hello world')".to_vec(),
    ///             hamt,
    ///             store,
    ///             rng
    ///         )
    ///         .await
    ///         .unwrap();
    ///
    ///     let PrivateOpResult { hamt, root_dir, result } = root_dir
    ///         .cp(
    ///             &["code".into(), "python".into(), "hello.py".into()],
    ///             &["code".into(), "hello.py".into()],
    ///             true,
    ///             Utc::now(),
    ///             hamt,
    ///             store,
    ///             rng
    ///         )
    ///         .await
    ///         .unwrap();
    ///
    ///     let PrivateOpResult { result, .. } = root_dir
    ///         .ls(&["code".into()], true, hamt, store)
    ///         .await
    ///         .unwrap();
    ///
    ///     assert_eq!(result.len(), 2);
    /// }
    /// ```
    #[allow(clippy::too_many_arguments)]
    pub async fn cp<B: BlockStore, R: RngCore>(
        self: Rc<Self>,
        path_segments_from: &[String],
        path_segments_to: &[String],
        search_latest: bool,
        time: DateTime<Utc>,
        hamt: Rc<PrivateForest>,
        store: &mut B,
        rng: &mut R,
    ) -> Result<PrivateOpResult<()>> {
        let PrivateOpResult {
            root_dir,
            result,
            hamt,
        } = self
            .get_node(path_segments_from, search_latest, hamt, store)
            .await?;

        root_dir
            .attach(
                result.ok_or(FsError::NotFound)?,
                path_segments_to,
                search_latest,
                time,
                hamt,
                store,
                rng,
            )
            .await
    }

    /// Serializes the directory with provided Serde serialilzer.
    pub(crate) fn serialize<S, R: RngCore>(
        &self,
        serializer: S,
        rng: &mut R,
    ) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let key = self
            .header
            .get_private_ref()
            .map_err(SerError::custom)?
            .ratchet_key;

        (PrivateDirectorySerde {
            r#type: NodeType::PrivateDirectory,
            version: self.version.clone(),
            header: {
                let cbor_bytes = dagcbor::encode(&self.header).map_err(SerError::custom)?;
                key.0
                    .encrypt(&Key::generate_nonce(rng), &cbor_bytes)
                    .map_err(SerError::custom)?
            },
            metadata: self.metadata.clone(),
            entries: self.entries.clone(),
        })
        .serialize(serializer)
    }

    /// Deserializes the directory with provided Serde deserializer and key.
    pub(crate) fn deserialize<'de, D>(deserializer: D, key: &RatchetKey) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let PrivateDirectorySerde {
            version,
            metadata,
            header,
            entries,
            ..
        } = PrivateDirectorySerde::deserialize(deserializer)?;

        Ok(Self {
            version,
            metadata,
            header: {
                let cbor_bytes = key.0.decrypt(&header).map_err(DeError::custom)?;
                dagcbor::decode(&cbor_bytes).map_err(DeError::custom)?
            },
            entries,
        })
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
mod private_directory_tests {
    use super::*;
    use crate::MemoryBlockStore;
    use proptest::test_runner::{RngAlgorithm, TestRng};

    use test_log::test;

    #[test(async_std::test)]
    async fn look_up_can_fetch_file_added_to_directory() {
        let rng = &mut TestRng::deterministic_rng(RngAlgorithm::ChaCha);
        let root_dir = Rc::new(PrivateDirectory::new(
            Namefilter::default(),
            Utc::now(),
            rng,
        ));
        let store = &mut MemoryBlockStore::default();
        let hamt = Rc::new(PrivateForest::new());

        let content = b"Hello, World!".to_vec();

        let PrivateOpResult { root_dir, hamt, .. } = root_dir
            .write(
                &["text.txt".into()],
                true,
                Utc::now(),
                content.clone(),
                hamt,
                store,
                rng,
            )
            .await
            .unwrap();

        let PrivateOpResult { result, .. } = root_dir
            .read(&["text.txt".into()], true, hamt, store)
            .await
            .unwrap();

        assert_eq!(result, content);
    }

    #[test(async_std::test)]
    async fn look_up_cannot_fetch_file_not_added_to_directory() {
        let rng = &mut TestRng::deterministic_rng(RngAlgorithm::ChaCha);
        let root_dir = Rc::new(PrivateDirectory::new(
            Namefilter::default(),
            Utc::now(),
            rng,
        ));
        let store = &mut MemoryBlockStore::default();
        let hamt = Rc::new(PrivateForest::new());

        let node = root_dir
            .lookup_node("Unknown", true, &hamt, store)
            .await
            .unwrap();

        assert!(node.is_none());
    }

    #[test(async_std::test)]
    async fn mkdir_can_create_new_directory() {
        let rng = &mut TestRng::deterministic_rng(RngAlgorithm::ChaCha);
        let root_dir = Rc::new(PrivateDirectory::new(
            Namefilter::default(),
            Utc::now(),
            rng,
        ));
        let store = &mut MemoryBlockStore::default();
        let hamt = Rc::new(PrivateForest::new());

        let PrivateOpResult { root_dir, hamt, .. } = root_dir
            .mkdir(
                &["tamedun".into(), "pictures".into()],
                true,
                Utc::now(),
                hamt,
                store,
                rng,
            )
            .await
            .unwrap();

        let PrivateOpResult { result, .. } = root_dir
            .get_node(&["tamedun".into(), "pictures".into()], true, hamt, store)
            .await
            .unwrap();

        assert!(result.is_some());
    }

    #[test(async_std::test)]
    async fn ls_can_list_children_under_directory() {
        let rng = &mut TestRng::deterministic_rng(RngAlgorithm::ChaCha);
        let root_dir = Rc::new(PrivateDirectory::new(
            Namefilter::default(),
            Utc::now(),
            rng,
        ));
        let store = &mut MemoryBlockStore::default();
        let hamt = Rc::new(PrivateForest::new());

        let PrivateOpResult { root_dir, hamt, .. } = root_dir
            .mkdir(
                &["tamedun".into(), "pictures".into()],
                true,
                Utc::now(),
                hamt,
                store,
                rng,
            )
            .await
            .unwrap();

        let PrivateOpResult { root_dir, hamt, .. } = root_dir
            .write(
                &["tamedun".into(), "pictures".into(), "puppy.jpg".into()],
                true,
                Utc::now(),
                b"puppy".to_vec(),
                hamt,
                store,
                rng,
            )
            .await
            .unwrap();

        let PrivateOpResult { root_dir, hamt, .. } = root_dir
            .mkdir(
                &["tamedun".into(), "pictures".into(), "cats".into()],
                true,
                Utc::now(),
                hamt,
                store,
                rng,
            )
            .await
            .unwrap();

        let PrivateOpResult { result, .. } = root_dir
            .ls(&["tamedun".into(), "pictures".into()], true, hamt, store)
            .await
            .unwrap();

        assert_eq!(result.len(), 2);
        assert_eq!(result[0].0, String::from("cats"));
        assert_eq!(result[1].0, String::from("puppy.jpg"));
    }

    #[test(async_std::test)]
    async fn rm_can_remove_children_from_directory() {
        let rng = &mut TestRng::deterministic_rng(RngAlgorithm::ChaCha);
        let root_dir = Rc::new(PrivateDirectory::new(
            Namefilter::default(),
            Utc::now(),
            rng,
        ));
        let store = &mut MemoryBlockStore::default();
        let hamt = Rc::new(PrivateForest::new());

        let PrivateOpResult { root_dir, hamt, .. } = root_dir
            .mkdir(
                &["tamedun".into(), "pictures".into()],
                true,
                Utc::now(),
                hamt,
                store,
                rng,
            )
            .await
            .unwrap();

        let PrivateOpResult { root_dir, hamt, .. } = root_dir
            .write(
                &["tamedun".into(), "pictures".into(), "puppy.jpg".into()],
                true,
                Utc::now(),
                b"puppy".to_vec(),
                hamt,
                store,
                rng,
            )
            .await
            .unwrap();

        let PrivateOpResult { root_dir, hamt, .. } = root_dir
            .mkdir(
                &["tamedun".into(), "pictures".into(), "cats".into()],
                true,
                Utc::now(),
                hamt,
                store,
                rng,
            )
            .await
            .unwrap();

        let PrivateOpResult { root_dir, hamt, .. } = root_dir
            .rm(
                &["tamedun".into(), "pictures".into()],
                true,
                hamt,
                store,
                rng,
            )
            .await
            .unwrap();

        let result = root_dir
            .rm(
                &["tamedun".into(), "pictures".into()],
                true,
                hamt,
                store,
                rng,
            )
            .await;

        assert!(result.is_err());
    }

    #[async_std::test]
    async fn read_can_fetch_userland_of_file_added_to_directory() {
        let rng = &mut TestRng::deterministic_rng(RngAlgorithm::ChaCha);
        let root_dir = Rc::new(PrivateDirectory::new(
            Namefilter::default(),
            Utc::now(),
            rng,
        ));
        let store = &mut MemoryBlockStore::default();
        let hamt = Rc::new(PrivateForest::new());

        let PrivateOpResult { root_dir, hamt, .. } = root_dir
            .write(
                &["text.txt".into()],
                true,
                Utc::now(),
                b"text".to_vec(),
                hamt,
                store,
                rng,
            )
            .await
            .unwrap();

        let PrivateOpResult { result, .. } = root_dir
            .read(&["text.txt".into()], true, hamt, store)
            .await
            .unwrap();

        assert_eq!(result, b"text".to_vec());
    }

    #[async_std::test]
    async fn path_nodes_can_generates_new_path_nodes() {
        let store = &mut MemoryBlockStore::default();
        let hamt = Rc::new(PrivateForest::new());
        let rng = &mut TestRng::deterministic_rng(RngAlgorithm::ChaCha);

        let path_nodes = PrivateDirectory::create_path_nodes(
            &["Documents".into(), "Apps".into()],
            Utc::now(),
            Namefilter::default(),
            rng,
        );

        let (root_dir, hamt) =
            PrivateDirectory::fix_up_path_nodes(path_nodes.clone(), hamt, store, rng)
                .await
                .unwrap();

        let result = root_dir
            .get_path_nodes(&["Documents".into(), "Apps".into()], true, &hamt, store)
            .await
            .unwrap();

        match result {
            PathNodesResult::MissingLink(_, segment) => panic!("MissingLink {segment}"),
            PathNodesResult::NotADirectory(_, segment) => panic!("NotADirectory {segment}"),
            PathNodesResult::Complete(path_nodes_2) => {
                assert_eq!(path_nodes.path.len(), path_nodes_2.path.len());
                assert_eq!(path_nodes.path[0].1, path_nodes_2.path[0].1);
                assert_eq!(path_nodes.path[1].1, path_nodes_2.path[1].1);
            }
        }
    }

    #[test(async_std::test)]
    async fn search_latest_finds_the_most_recent() {
        let store = &mut MemoryBlockStore::default();
        let hamt = Rc::new(PrivateForest::new());
        let rng = &mut TestRng::deterministic_rng(RngAlgorithm::ChaCha);

        let root_dir = Rc::new(PrivateDirectory::new(
            Namefilter::default(),
            Utc::now(),
            rng,
        ));

        let path = ["Documents".into(), "file.txt".into()];

        let PrivateOpResult { root_dir, hamt, .. } = root_dir
            .write(&path, true, Utc::now(), b"One".to_vec(), hamt, store, rng)
            .await
            .unwrap();

        let old_root = Rc::clone(&root_dir);

        let PrivateOpResult { root_dir, hamt, .. } = root_dir
            .write(&path, true, Utc::now(), b"Two".to_vec(), hamt, store, rng)
            .await
            .unwrap();

        let new_read = Rc::clone(&root_dir)
            .read(&path, false, Rc::clone(&hamt), store)
            .await
            .unwrap()
            .result;

        let old_read = Rc::clone(&old_root)
            .read(&path, false, Rc::clone(&hamt), store)
            .await
            .unwrap()
            .result;

        let old_read_latest = old_root
            .read(&path, true, Rc::clone(&hamt), store)
            .await
            .unwrap()
            .result;

        let new_read_latest = root_dir
            .read(&path, true, hamt, store)
            .await
            .unwrap()
            .result;

        assert_eq!(&String::from_utf8_lossy(&new_read), "Two");
        assert_eq!(&String::from_utf8_lossy(&old_read), "One");
        assert_eq!(&String::from_utf8_lossy(&old_read_latest), "Two");
        assert_eq!(&String::from_utf8_lossy(&new_read_latest), "Two");
    }

    #[async_std::test]
    async fn cp_can_copy_sub_directory_to_another_valid_location_with_updated_ancestry() {
        let rng = &mut TestRng::deterministic_rng(RngAlgorithm::ChaCha);
        let store = &mut MemoryBlockStore::default();
        let hamt = Rc::new(PrivateForest::new());
        let root_dir = Rc::new(PrivateDirectory::new(
            Namefilter::default(),
            Utc::now(),
            rng,
        ));

        let PrivateOpResult { root_dir, hamt, .. } = root_dir
            .write(
                &["pictures".into(), "cats".into(), "tabby.jpg".into()],
                true,
                Utc::now(),
                b"tabby".to_vec(),
                hamt,
                store,
                rng,
            )
            .await
            .unwrap();

        let PrivateOpResult { root_dir, hamt, .. } = root_dir
            .write(
                &["pictures".into(), "cats".into(), "luna.png".into()],
                true,
                Utc::now(),
                b"luna".to_vec(),
                hamt,
                store,
                rng,
            )
            .await
            .unwrap();

        let PrivateOpResult { root_dir, hamt, .. } = root_dir
            .mkdir(&["images".into()], true, Utc::now(), hamt, store, rng)
            .await
            .unwrap();

        let PrivateOpResult { root_dir, hamt, .. } = root_dir
            .cp(
                &["pictures".into(), "cats".into()],
                &["images".into(), "cats".into()],
                true,
                Utc::now(),
                hamt,
                store,
                rng,
            )
            .await
            .unwrap();

        let PrivateOpResult {
            root_dir,
            hamt,
            result,
        } = root_dir
            .ls(&["images".into()], true, hamt, store)
            .await
            .unwrap();

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].0, String::from("cats"));

        let PrivateOpResult {
            result,
            root_dir,
            hamt,
        } = root_dir
            .ls(&["pictures".into()], true, hamt, store)
            .await
            .unwrap();

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].0, String::from("cats"));

        let PrivateOpResult {
            result,
            root_dir,
            hamt,
        } = root_dir
            .get_node(&["images".into(), "cats".into()], true, hamt, store)
            .await
            .unwrap();

        let cats_bare_name = result.unwrap().get_header().bare_name.clone();

        let images_dir_inumber = root_dir
            .lookup_node("images", true, &hamt, store)
            .await
            .unwrap()
            .unwrap()
            .get_header()
            .inumber;

        let pictures_dir_inumber = root_dir
            .lookup_node("pictures", true, &hamt, store)
            .await
            .unwrap()
            .unwrap()
            .get_header()
            .inumber;

        assert!(cats_bare_name.contains(&images_dir_inumber));
        assert!(!cats_bare_name.contains(&pictures_dir_inumber));
    }

    #[async_std::test]
    async fn mv_can_move_sub_directory_to_another_valid_location_with_updated_ancestry() {
        let rng = &mut TestRng::deterministic_rng(RngAlgorithm::ChaCha);
        let store = &mut MemoryBlockStore::default();
        let hamt = Rc::new(PrivateForest::new());
        let root_dir = Rc::new(PrivateDirectory::new(
            Namefilter::default(),
            Utc::now(),
            rng,
        ));

        let PrivateOpResult { root_dir, hamt, .. } = root_dir
            .write(
                &["pictures".into(), "cats".into(), "tabby.jpg".into()],
                true,
                Utc::now(),
                b"tabby".to_vec(),
                hamt,
                store,
                rng,
            )
            .await
            .unwrap();

        let PrivateOpResult { root_dir, hamt, .. } = root_dir
            .write(
                &["pictures".into(), "cats".into(), "luna.png".into()],
                true,
                Utc::now(),
                b"luna".to_vec(),
                hamt,
                store,
                rng,
            )
            .await
            .unwrap();

        let PrivateOpResult { root_dir, hamt, .. } = root_dir
            .mkdir(&["images".into()], true, Utc::now(), hamt, store, rng)
            .await
            .unwrap();

        let PrivateOpResult { root_dir, hamt, .. } = root_dir
            .basic_mv(
                &["pictures".into(), "cats".into()],
                &["images".into(), "cats".into()],
                true,
                Utc::now(),
                hamt,
                store,
                rng,
            )
            .await
            .unwrap();

        let PrivateOpResult {
            root_dir,
            hamt,
            result,
        } = root_dir
            .ls(&["images".into()], true, hamt, store)
            .await
            .unwrap();

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].0, String::from("cats"));

        let PrivateOpResult {
            result,
            root_dir,
            hamt,
        } = root_dir
            .ls(&["pictures".into()], true, hamt, store)
            .await
            .unwrap();

        assert_eq!(result.len(), 0);

        let PrivateOpResult {
            result,
            root_dir,
            hamt,
        } = root_dir
            .get_node(&["images".into(), "cats".into()], true, hamt, store)
            .await
            .unwrap();

        let cats_bare_name = result.unwrap().get_header().bare_name.clone();

        let images_dir_inumber = root_dir
            .lookup_node("images", true, &hamt, store)
            .await
            .unwrap()
            .unwrap()
            .get_header()
            .inumber;

        let pictures_dir_inumber = root_dir
            .lookup_node("pictures", true, &hamt, store)
            .await
            .unwrap()
            .unwrap()
            .get_header()
            .inumber;

        assert!(cats_bare_name.contains(&images_dir_inumber));
        assert!(!cats_bare_name.contains(&pictures_dir_inumber));
    }

    #[async_std::test]
    async fn mv_cannot_move_sub_directory_to_invalid_location() {
        let rng = &mut TestRng::deterministic_rng(RngAlgorithm::ChaCha);
        let store = &mut MemoryBlockStore::default();
        let hamt = Rc::new(PrivateForest::new());
        let root_dir = Rc::new(PrivateDirectory::new(
            Namefilter::default(),
            Utc::now(),
            rng,
        ));

        let PrivateOpResult { root_dir, hamt, .. } = root_dir
            .mkdir(
                &[
                    "videos".into(),
                    "movies".into(),
                    "anime".into(),
                    "ghibli".into(),
                ],
                true,
                Utc::now(),
                hamt,
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
                hamt,
                store,
                rng,
            )
            .await;

        assert!(result.is_err());
    }

    #[async_std::test]
    async fn mv_can_rename_directories() {
        let rng = &mut TestRng::deterministic_rng(RngAlgorithm::ChaCha);
        let store = &mut MemoryBlockStore::default();
        let hamt = Rc::new(PrivateForest::new());
        let root_dir = Rc::new(PrivateDirectory::new(
            Namefilter::default(),
            Utc::now(),
            rng,
        ));
        let content = b"file".to_vec();

        let PrivateOpResult { root_dir, hamt, .. } = root_dir
            .write(
                &["file.txt".into()],
                true,
                Utc::now(),
                content.clone(),
                hamt,
                store,
                rng,
            )
            .await
            .unwrap();

        let PrivateOpResult { root_dir, hamt, .. } = root_dir
            .basic_mv(
                &["file.txt".into()],
                &["renamed.txt".into()],
                true,
                Utc::now(),
                hamt,
                store,
                rng,
            )
            .await
            .unwrap();

        let PrivateOpResult {
            root_dir,
            hamt,
            result,
        } = root_dir
            .read(&["renamed.txt".into()], true, hamt, store)
            .await
            .unwrap();

        assert!(result == content);

        let result = root_dir
            .lookup_node("file.txt", true, &hamt, store)
            .await
            .unwrap();

        assert!(result.is_none());
    }

    #[async_std::test]
    async fn mv_fails_moving_directories_to_files() {
        let rng = &mut TestRng::deterministic_rng(RngAlgorithm::ChaCha);
        let store = &mut MemoryBlockStore::default();
        let hamt = Rc::new(PrivateForest::new());
        let root_dir = Rc::new(PrivateDirectory::new(
            Namefilter::default(),
            Utc::now(),
            rng,
        ));

        let PrivateOpResult { root_dir, hamt, .. } = root_dir
            .mkdir(
                &["movies".into(), "ghibli".into()],
                true,
                Utc::now(),
                hamt,
                store,
                rng,
            )
            .await
            .unwrap();

        let PrivateOpResult { root_dir, hamt, .. } = root_dir
            .write(
                &["file.txt".into()],
                true,
                Utc::now(),
                b"file".to_vec(),
                hamt,
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
                hamt,
                store,
                rng,
            )
            .await;

        assert!(result.is_err());
    }
}
