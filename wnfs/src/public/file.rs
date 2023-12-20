//! Public fs file node.

use super::{PublicFileSerializable, PublicNodeSerializable};
use crate::{error::FsError, is_readable_wnfs_version, traits::Id, WNFS_VERSION};
use anyhow::{bail, Result};
use async_once_cell::OnceCell;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use futures::{AsyncRead, AsyncReadExt};
use libipld_core::cid::Cid;
use std::{collections::BTreeSet, io::SeekFrom};
use tokio::io::AsyncSeekExt;
use tokio_util::compat::{FuturesAsyncReadCompatExt, TokioAsyncReadCompatExt};
use wnfs_common::{
    utils::{Arc, CondSend},
    BlockStore, Link, Metadata, NodeType, Storable,
};
use wnfs_unixfs_file::{builder::FileBuilder, unixfs::UnixFsFile};

/// A file in the WNFS public file system.
///
/// # Examples
///
/// ```
/// use wnfs::public::PublicFile;
/// use chrono::Utc;
///
/// let file = PublicFile::new(Utc::now());
///
/// println!("File: {:?}", file);
/// ```
#[derive(Debug)]
pub struct PublicFile {
    persisted_as: OnceCell<Cid>,
    pub(crate) metadata: Metadata,
    pub(crate) userland: Link<UnixFsFile>,
    pub(crate) previous: BTreeSet<Cid>,
}

//--------------------------------------------------------------------------------------------------
// Implementations
//--------------------------------------------------------------------------------------------------

impl PublicFile {
    /// Creates a new, empty file.
    ///
    /// # Examples
    ///
    /// ```
    /// use wnfs::{public::PublicFile, common::MemoryBlockStore};
    /// use chrono::Utc;
    ///
    /// let file = PublicFile::new(Utc::now());
    ///
    /// println!("File: {:?}", file);
    /// ```
    pub fn new(time: DateTime<Utc>) -> Self {
        Self {
            persisted_as: OnceCell::new(),
            metadata: Metadata::new(time),
            userland: Link::from(UnixFsFile::empty()),
            previous: BTreeSet::new(),
        }
    }

    /// Creates an `Arc` wrapped empty file, a shorthand wrapper around `PublicFile::new`.
    pub fn new_rc(time: DateTime<Utc>) -> Arc<Self> {
        Arc::new(Self::new(time))
    }

    /// Creates a file with given content bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// use anyhow::Result;
    /// use wnfs::{public::PublicFile, common::MemoryBlockStore};
    /// use chrono::Utc;
    ///
    /// #[async_std::main]
    /// async fn main() -> Result<()> {
    ///     let store = &MemoryBlockStore::new();
    ///     let content = b"Hello, World!".to_vec();
    ///     let file = PublicFile::with_content(Utc::now(), content, store).await?;
    ///
    ///     println!("File: {:?}", file);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn with_content(
        time: DateTime<Utc>,
        content: Vec<u8>,
        store: &impl BlockStore,
    ) -> Result<Self> {
        let content_cid = FileBuilder::new()
            .content_bytes(content)
            .build()?
            .store(store)
            .await?;

        Ok(Self {
            persisted_as: OnceCell::new(),
            metadata: Metadata::new(time),
            userland: Link::from_cid(content_cid),
            previous: BTreeSet::new(),
        })
    }

    /// A convenience wrapper around `with_content` that also wraps the result in an `Arc`.
    pub async fn with_content_rc(
        time: DateTime<Utc>,
        content: Vec<u8>,
        store: &impl BlockStore,
    ) -> Result<Arc<Self>> {
        Ok(Arc::new(Self::with_content(time, content, store).await?))
    }

    /// Creates a file similar to `with_content`, but allowing streaming in the file.
    ///
    /// This is useful to keep memory usage low when importing bigger files, it should
    /// use only O(log n) memory.
    ///
    /// # Examples
    ///
    /// ```
    /// use anyhow::Result;
    /// use async_std::fs::File;
    /// use chrono::Utc;
    /// use wnfs::{
    ///     public::PublicFile,
    ///     common::MemoryBlockStore,
    /// };
    ///
    /// #[async_std::main]
    /// async fn main() -> Result<()> {
    ///     let disk_file = File::open("./test/fixtures/Clara Schumann, Scherzo no. 2, Op. 14.mp3").await?;
    ///     let store = &MemoryBlockStore::new();
    ///     let file = PublicFile::with_content_streaming(
    ///         Utc::now(),
    ///         disk_file,
    ///         store,
    ///     )
    ///     .await?;
    ///
    ///     println!("file = {:?}", file);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn with_content_streaming<'a>(
        time: DateTime<Utc>,
        content: impl AsyncRead + CondSend + 'a,
        store: &'a impl BlockStore,
    ) -> Result<Self> {
        let content_cid = FileBuilder::new()
            .content_reader(FuturesAsyncReadCompatExt::compat(content))
            .build()?
            .store(store)
            .await?;

        Ok(Self {
            persisted_as: OnceCell::new(),
            metadata: Metadata::new(time),
            userland: Link::from_cid(content_cid),
            previous: BTreeSet::new(),
        })
    }

    /// Convenience wrapper around `with_content_streaming` that additionally
    /// wraps the result in an `Arc`.
    pub async fn with_content_streaming_rc<'a>(
        time: DateTime<Utc>,
        content: impl AsyncRead + CondSend + 'a,
        store: &'a impl BlockStore,
    ) -> Result<Arc<Self>> {
        Ok(Arc::new(
            Self::with_content_streaming(time, content, store).await?,
        ))
    }

    /// Copy the contents from another file to this file.
    /// This is an O(1) operation, as WNFS is a copy-on-write file system.
    ///
    /// # Example
    ///
    /// ```
    /// use anyhow::Result;
    /// use chrono::Utc;
    /// use wnfs::{
    ///     public::{PublicDirectory, PublicFile},
    ///     common::{MemoryBlockStore, utils::get_random_bytes},
    /// };
    ///
    /// #[async_std::main]
    /// async fn main() -> Result<()> {
    ///     let store = &MemoryBlockStore::new();
    ///
    ///     let file = PublicFile::with_content(
    ///         Utc::now(),
    ///         get_random_bytes::<100>(&mut rand_core::OsRng).to_vec(),
    ///         store,
    ///     )
    ///     .await?;
    ///
    ///     let root_dir = &mut PublicDirectory::new_rc(Utc::now());
    ///
    ///     let copy = root_dir
    ///         .open_file_mut(&["some".into(), "copy.txt".into()], Utc::now(), store)
    ///         .await?;
    ///
    ///     copy.copy_content_from(&file, Utc::now());
    ///
    ///     assert_eq!(file.read_at(0, None, store).await?, copy.read_at(0, None, store).await?);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn copy_content_from(&mut self, other: &Self, time: DateTime<Utc>) {
        self.metadata.upsert_mtime(time);
        self.userland = other.userland.clone();
    }

    /// Stream out the content of this file starting from given byte offset.
    ///
    /// # Examples
    ///
    /// ```
    /// use anyhow::Result;
    /// use chrono::Utc;
    /// use wnfs::{
    ///     public::PublicFile,
    ///     common::{MemoryBlockStore, utils::get_random_bytes},
    /// };
    ///
    /// #[async_std::main]
    /// async fn main() -> Result<()> {
    ///     let store = &MemoryBlockStore::new();
    ///     let content = b"Hello, World!\n".repeat(1000).to_vec();
    ///     let file = PublicFile::with_content(Utc::now(), content, store).await?;
    ///
    ///     let mut content_stream = file.stream_content(0, store).await?;
    ///
    ///     // Pipe file contents to stdout
    ///     let mut stdout = async_std::io::stdout();
    ///     futures::io::copy(&mut content_stream, &mut stdout).await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn stream_content<'a>(
        &'a self,
        byte_offset: u64,
        store: &'a impl BlockStore,
    ) -> Result<impl AsyncRead + CondSend + 'a> {
        let mut reader = self
            .userland
            .resolve_value(store)
            .await?
            .clone()
            .into_content_reader(store, None)?;
        reader.seek(SeekFrom::Start(byte_offset)).await?;
        Ok(TokioAsyncReadCompatExt::compat(reader))
    }

    /// Read the contents of this file.
    /// You can provide a byte offset from which to start reading,
    /// and you can provide a maximum amount of bytes you want to read.
    ///
    /// For more advanced cases, consider using `stream_content` instead.
    ///
    /// # Examples
    ///
    /// ```
    /// use anyhow::Result;
    /// use chrono::Utc;
    /// use wnfs::{
    ///     public::PublicFile,
    ///     common::{MemoryBlockStore, utils::get_random_bytes},
    /// };
    ///
    /// #[async_std::main]
    /// async fn main() -> Result<()> {
    ///     let store = &MemoryBlockStore::new();
    ///     let content = b"Hello, World!\n".repeat(1000).to_vec();
    ///     let file = PublicFile::with_content(Utc::now(), content, store).await?;
    ///
    ///     let content = file.read_at(14, Some(28), store).await?;
    ///
    ///     assert_eq!(content, b"Hello, World!\nHello, World!\n");
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn read_at<'a>(
        &'a self,
        byte_offset: u64,
        len_limit: Option<usize>,
        store: &'a impl BlockStore,
    ) -> Result<Vec<u8>> {
        let mut reader = self.stream_content(byte_offset, store).await?;
        if let Some(len) = len_limit {
            let mut buffer = vec![0; len];
            reader.read_exact(&mut buffer).await?;
            Ok(buffer)
        } else {
            let mut buffer = Vec::new();
            reader.read_to_end(&mut buffer).await?;
            Ok(buffer)
        }
    }

    /// Gets the entire content of a file.
    ///
    /// # Examples
    ///
    /// ```
    /// use anyhow::Result;
    /// use rand_chacha::ChaCha12Rng;
    /// use rand_core::SeedableRng;
    /// use chrono::Utc;
    /// use wnfs::{
    ///     public::PublicFile,
    ///     common::{MemoryBlockStore, utils::get_random_bytes},
    /// };
    ///
    /// #[async_std::main]
    /// async fn main() -> Result<()> {
    ///     let store = &MemoryBlockStore::new();
    ///     let rng = &mut ChaCha12Rng::from_entropy();
    ///     let content = get_random_bytes::<100>(rng).to_vec();
    ///     let file = PublicFile::with_content(
    ///         Utc::now(),
    ///         content.clone(),
    ///         store,
    ///     )
    ///     .await?;
    ///
    ///     let mut all_content = file.get_content(store).await?;
    ///
    ///     assert_eq!(content, all_content);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_content(&self, store: &impl BlockStore) -> Result<Vec<u8>> {
        self.read_at(0, None, store).await
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

    /// Writes a new content cid to the file.
    /// This will create a new revision of the file.
    pub async fn set_content(
        self: &mut Arc<Self>,
        time: DateTime<Utc>,
        content: Vec<u8>,
        store: &impl BlockStore,
    ) -> Result<()> {
        let content_cid = FileBuilder::new()
            .content_bytes(content)
            .build()?
            .store(store)
            .await?;

        let file = self.prepare_next_revision();
        file.metadata.upsert_mtime(time);
        file.userland = Link::from_cid(content_cid);

        Ok(())
    }

    /// Gets the content cid of the file.
    pub async fn get_raw_content_cid(&self, store: &impl BlockStore) -> Cid {
        let content_cid: Result<Cid> = self.userland.resolve_cid(store).await;
        content_cid.unwrap()
    }

    /// Gets the previous value of the file.
    ///
    /// # Examples
    ///
    /// ```
    /// use wnfs::{public::PublicDirectory, traits::Id};
    /// use chrono::Utc;
    ///
    /// let dir = PublicDirectory::new(Utc::now());
    ///
    /// println!("id = {}", dir.get_id());
    /// ```
    pub fn get_previous(&self) -> &BTreeSet<Cid> {
        &self.previous
    }

    /// Gets the metadata of the file
    pub fn get_metadata(&self) -> &Metadata {
        &self.metadata
    }

    /// Returns a mutable reference to metadata for this file.
    pub fn get_metadata_mut(&mut self) -> &mut Metadata {
        &mut self.metadata
    }

    /// Returns a mutable reference to this file's metadata and ratchets forward the history, if necessary.
    pub fn get_metadata_mut_rc<'a>(self: &'a mut Arc<Self>) -> &'a mut Metadata {
        self.prepare_next_revision().get_metadata_mut()
    }
}

#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
impl Storable for PublicFile {
    type Serializable = PublicNodeSerializable;

    async fn to_serializable(&self, store: &impl BlockStore) -> Result<Self::Serializable> {
        Ok(PublicNodeSerializable::File(PublicFileSerializable {
            version: WNFS_VERSION,
            metadata: self.metadata.clone(),
            userland: self.userland.resolve_cid(store).await?,
            previous: self.previous.iter().cloned().collect(),
        }))
    }

    async fn from_serializable(
        cid: Option<&Cid>,
        serializable: Self::Serializable,
    ) -> Result<Self> {
        let PublicNodeSerializable::File(serializable) = serializable else {
            bail!(FsError::UnexpectedNodeType(NodeType::PublicDirectory));
        };

        if !is_readable_wnfs_version(&serializable.version) {
            bail!(FsError::UnexpectedVersion(serializable.version))
        }

        Ok(Self {
            persisted_as: cid.cloned().map(OnceCell::new_with).unwrap_or_default(),
            metadata: serializable.metadata,
            userland: Link::from_cid(serializable.userland),
            previous: serializable.previous.iter().cloned().collect(),
        })
    }

    fn persisted_as(&self) -> Option<&OnceCell<Cid>> {
        Some(&self.persisted_as)
    }
}

impl Id for PublicFile {
    fn get_id(&self) -> String {
        format!("{:p}", &self.metadata)
    }
}

impl PartialEq for PublicFile {
    fn eq(&self, other: &Self) -> bool {
        self.metadata == other.metadata
            && self.userland == other.userland
            && self.previous == other.previous
    }
}

impl Clone for PublicFile {
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

//--------------------------------------------------------------------------------------------------
// Tests
//--------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use wnfs_common::MemoryBlockStore;

    #[async_std::test]
    async fn previous_links_get_set() {
        let time = Utc::now();
        let store = &MemoryBlockStore::default();

        let file = &mut PublicFile::new_rc(time);
        let previous_cid = &file.store(store).await.unwrap();
        let next_file = file.prepare_next_revision();

        assert_eq!(
            next_file.previous.iter().collect::<Vec<_>>(),
            vec![previous_cid]
        );
    }

    #[async_std::test]
    async fn prepare_next_revision_shortcuts_if_possible() {
        let time = Utc::now();
        let store = &MemoryBlockStore::default();

        let file = &mut PublicFile::new_rc(time);
        let previous_cid = &file.store(store).await.unwrap();
        let next_file = file.prepare_next_revision();
        let next_file_clone = &mut Arc::new(next_file.clone());
        let yet_another_file = next_file_clone.prepare_next_revision();

        assert_eq!(
            yet_another_file.previous.iter().collect::<Vec<_>>(),
            vec![previous_cid]
        );
    }
}

#[cfg(test)]
mod snapshot_tests {
    use super::*;
    use chrono::TimeZone;
    use testresult::TestResult;
    use wnfs_common::utils::SnapshotBlockStore;

    #[async_std::test]
    async fn test_simple_file() -> TestResult {
        let store = &SnapshotBlockStore::default();
        let time = Utc.with_ymd_and_hms(1970, 1, 1, 0, 0, 0).unwrap();

        let file = &mut PublicFile::new_rc(time);
        let cid = file.store(store).await?;

        let file = store.get_block_snapshot(&cid).await?;

        insta::assert_json_snapshot!(file);

        Ok(())
    }

    #[async_std::test]
    async fn test_file_with_previous_links() -> TestResult {
        let store = &SnapshotBlockStore::default();
        let time = Utc.with_ymd_and_hms(1970, 1, 1, 0, 0, 0).unwrap();

        let file = &mut PublicFile::new_rc(time);
        let _ = file.store(store).await?;

        file.set_content(time, b"Hello, World!".to_vec(), store)
            .await?;
        let cid = file.store(store).await?;

        let file = store.get_block_snapshot(&cid).await?;

        insta::assert_json_snapshot!(file);

        Ok(())
    }
}
