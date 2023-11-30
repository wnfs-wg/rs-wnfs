//! Public fs file node.

use super::{PublicFileSerializable, PublicNodeSerializable};
use crate::{error::FsError, is_readable_wnfs_version, traits::Id, WNFS_VERSION};
use anyhow::{bail, Result};
use async_once_cell::OnceCell;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use futures::{AsyncRead, AsyncReadExt};
use libipld_core::cid::Cid;
use serde::{
    de::Error as DeError, ser::Error as SerError, Deserialize, Deserializer, Serialize, Serializer,
};
use std::{collections::BTreeSet, io::SeekFrom};
use tokio::io::AsyncSeekExt;
use tokio_util::compat::{FuturesAsyncReadCompatExt, TokioAsyncReadCompatExt};
use wnfs_common::{
    utils::{Arc, CondSend},
    AsyncSerialize, BlockStore, Metadata, RemembersCid,
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
    pub metadata: Metadata,
    userland: FileUserland,
    pub previous: BTreeSet<Cid>,
}

#[derive(Debug, Clone, PartialEq)]
enum FileUserland {
    Loaded(UnixFsFile),
    Stored(Cid),
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
            userland: FileUserland::Loaded(UnixFsFile::empty()),
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
            userland: FileUserland::Loaded(UnixFsFile::load(&content_cid, store).await?),
            previous: BTreeSet::new(),
        })
    }

    pub async fn with_content_rc(
        time: DateTime<Utc>,
        content: Vec<u8>,
        store: &impl BlockStore,
    ) -> Result<Arc<Self>> {
        Ok(Arc::new(Self::with_content(time, content, store).await?))
    }

    pub async fn with_content_streaming<'a>(
        time: DateTime<Utc>,
        content: impl AsyncRead + Send + 'a,
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
            userland: FileUserland::Loaded(UnixFsFile::load(&content_cid, store).await?),
            previous: BTreeSet::new(),
        })
    }

    pub async fn with_content_streaming_rc<'a>(
        time: DateTime<Utc>,
        content: impl AsyncRead + Send + 'a,
        store: &'a impl BlockStore,
    ) -> Result<Arc<Self>> {
        Ok(Arc::new(
            Self::with_content_streaming(time, content, store).await?,
        ))
    }

    pub fn copy_content_from(&mut self, other: &Self, time: DateTime<Utc>) {
        self.metadata.upsert_mtime(time);
        self.userland = other.userland.clone();
    }

    pub async fn stream_content<'a>(
        &'a self,
        byte_offset: u64,
        store: &'a impl BlockStore,
    ) -> Result<impl AsyncRead + Send + 'a> {
        let mut reader = self
            .userland
            .get_cloned(store)
            .await?
            .into_content_reader(store, None)?;
        reader.seek(SeekFrom::Start(byte_offset)).await?;
        Ok(TokioAsyncReadCompatExt::compat(reader))
    }

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
        let userland = UnixFsFile::load(&content_cid, store).await?;

        let file = self.prepare_next_revision();
        file.metadata.upsert_mtime(time);
        file.userland = FileUserland::Loaded(userland);

        Ok(())
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

    /// Stores file in provided block store.
    ///
    /// # Examples
    ///
    /// ```
    /// use wnfs::{
    ///     public::PublicFile,
    ///     traits::Id,
    ///     common::MemoryBlockStore
    /// };
    /// use chrono::Utc;
    /// use libipld_core::cid::Cid;
    ///
    /// #[async_std::main]
    /// async fn main() {
    ///     let store = &MemoryBlockStore::new();
    ///     let file = PublicFile::new(Utc::now());
    ///
    ///     file.store(store).await.unwrap();
    /// }
    /// ```
    pub async fn store(&self, store: &impl BlockStore) -> Result<Cid> {
        Ok(*self
            .persisted_as
            .get_or_try_init(store.put_async_serializable(self))
            .await?)
    }

    /// Creates a new file from a serializable.
    pub(crate) fn from_serializable(serializable: PublicFileSerializable) -> Result<Self> {
        if !is_readable_wnfs_version(&serializable.version) {
            bail!(FsError::UnexpectedVersion(serializable.version))
        }

        Ok(Self {
            persisted_as: OnceCell::new(),
            metadata: serializable.metadata,
            userland: FileUserland::Stored(serializable.userland),
            previous: serializable.previous.iter().cloned().collect(),
        })
    }
}

#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
impl AsyncSerialize for PublicFile {
    async fn async_serialize<S, B>(&self, serializer: S, store: &B) -> Result<S::Ok, S::Error>
    where
        S: Serializer + CondSend,
        S::Error: CondSend,
        B: BlockStore,
    {
        let userland = self
            .userland
            .get_stored(store)
            .await
            .map_err(SerError::custom)?;

        PublicNodeSerializable::File(PublicFileSerializable {
            version: WNFS_VERSION,
            metadata: self.metadata.clone(),
            userland,
            previous: self.previous.iter().cloned().collect(),
        })
        .serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for PublicFile {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        match PublicNodeSerializable::deserialize(deserializer)? {
            PublicNodeSerializable::File(file) => {
                PublicFile::from_serializable(file).map_err(DeError::custom)
            }
            _ => Err(DeError::custom(FsError::InvalidDeserialization(
                "Expected directory".into(),
            ))),
        }
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

impl RemembersCid for PublicFile {
    fn persisted_as(&self) -> &OnceCell<Cid> {
        &self.persisted_as
    }
}

impl FileUserland {
    async fn get_cloned(&self, store: &impl BlockStore) -> Result<UnixFsFile> {
        match self {
            Self::Loaded(file) => Ok(file.clone()),
            Self::Stored(cid) => UnixFsFile::load(cid, store).await,
        }
    }

    async fn get_stored(&self, store: &impl BlockStore) -> Result<Cid> {
        match self {
            Self::Loaded(file) => file.encode()?.store(store).await,
            Self::Stored(cid) => Ok(*cid),
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
