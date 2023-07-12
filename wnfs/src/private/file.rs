use super::{
    encrypted::Encrypted, forest::traits::PrivateForest, PrivateFileContentSerializable,
    PrivateNode, PrivateNodeContentSerializable, PrivateNodeHeader, PrivateRef, SnapshotKey,
    TemporalKey, AUTHENTICATION_TAG_SIZE, NONCE_SIZE,
};
use crate::{error::FsError, traits::Id, WNFS_VERSION};
use anyhow::{bail, Result};
use async_once_cell::OnceCell;
use async_stream::try_stream;
use chrono::{DateTime, Utc};
use futures::{future, AsyncRead, Stream, StreamExt, TryStreamExt};
use libipld_core::cid::Cid;
use rand_core::CryptoRngCore;
use serde::{Deserialize, Serialize};
use std::{collections::BTreeSet, iter, rc::Rc};
use wnfs_common::{utils, BlockStore, Metadata, CODEC_RAW, MAX_BLOCK_SIZE};
use wnfs_nameaccumulator::{AccumulatorSetup, Name, NameSegment};

//--------------------------------------------------------------------------------------------------
// Constants
//--------------------------------------------------------------------------------------------------

/// The maximum block size is 2 ^ 18 but the first 12 bytes are reserved for the cipher text's initialization vector.
/// The ciphertext then also contains a 16 byte authentication tag.
/// This leaves a maximum of (2 ^ 18) - 12 - 16 = 262,116 bytes for the actual data.
///
/// More on that [here][priv-file].
///
/// [priv-file]: https://github.com/wnfs-wg/spec/blob/matheus23/file-sharding/spec/private-wnfs.md#314-private-file
pub const MAX_BLOCK_CONTENT_SIZE: usize = MAX_BLOCK_SIZE - NONCE_SIZE - AUTHENTICATION_TAG_SIZE;

//--------------------------------------------------------------------------------------------------
// Type Definitions
//--------------------------------------------------------------------------------------------------

/// Represents a file in the WNFS private filesystem.
///
/// # Examples
///
/// ```
/// use anyhow::Result;
/// use std::rc::Rc;
/// use chrono::Utc;
/// use rand::thread_rng;
/// use wnfs::{
///     private::{PrivateFile, forest::{hamt::HamtForest, traits::PrivateForest}},
///     common::{MemoryBlockStore, utils::get_random_bytes},
/// };
///
/// #[async_std::main]
/// async fn main() -> Result<()> {
///     let store = &MemoryBlockStore::new();
///     let rng = &mut thread_rng();
///     let forest = &mut Rc::new(HamtForest::new_rsa_2048(rng));
///
///     let file = PrivateFile::with_content(
///         &forest.empty_name(),
///         Utc::now(),
///         get_random_bytes::<100>(rng).to_vec(),
///         forest,
///         store,
///         rng,
///     )
///     .await?;
///
///     println!("file = {:?}", file);
///
///     Ok(())
/// }
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct PrivateFile {
    pub header: PrivateNodeHeader,
    pub(crate) content: PrivateFileContent,
}

#[derive(Debug)]
pub(crate) struct PrivateFileContent {
    pub(crate) persisted_as: OnceCell<Cid>,
    pub(crate) previous: BTreeSet<(usize, Encrypted<Cid>)>,
    pub(crate) metadata: Metadata,
    pub(crate) content: FileContent,
}

/// The content of a file.
/// It is stored inline or stored in blocks.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum FileContent {
    Inline {
        data: Vec<u8>,
    },
    External {
        key: SnapshotKey,
        block_count: usize,
        block_content_size: usize,
    },
}

//--------------------------------------------------------------------------------------------------
// Implementations
//--------------------------------------------------------------------------------------------------

impl PrivateFile {
    /// Creates an empty file.
    ///
    /// # Examples
    ///
    /// ```
    /// use wnfs::private::{
    ///     PrivateFile, forest::{hamt::HamtForest, traits::PrivateForest},
    /// };
    /// use chrono::Utc;
    /// use rand::thread_rng;
    ///
    /// let rng = &mut thread_rng();
    /// let forest = HamtForest::new_rsa_2048(rng);
    /// let file = PrivateFile::new(
    ///     &forest.empty_name(),
    ///     Utc::now(),
    ///     rng,
    /// );
    ///
    /// println!("file = {:?}", file);
    /// ```
    pub fn new(parent_name: &Name, time: DateTime<Utc>, rng: &mut impl CryptoRngCore) -> Self {
        Self {
            header: PrivateNodeHeader::new(parent_name, rng),
            content: PrivateFileContent {
                persisted_as: OnceCell::new(),
                metadata: Metadata::new(time),
                previous: BTreeSet::new(),
                content: FileContent::Inline { data: vec![] },
            },
        }
    }

    /// Creates a file with provided content.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::rc::Rc;
    /// use chrono::Utc;
    /// use rand::thread_rng;
    /// use wnfs::{
    ///     private::{PrivateFile, forest::{hamt::HamtForest, traits::PrivateForest}},
    ///     common::{MemoryBlockStore, utils::get_random_bytes},
    /// };
    ///
    /// #[async_std::main]
    /// async fn main() {
    ///     let store = &MemoryBlockStore::new();
    ///     let rng = &mut thread_rng();
    ///     let forest = &mut Rc::new(HamtForest::new_rsa_2048(rng));
    ///
    ///     let file = PrivateFile::with_content(
    ///         &forest.empty_name(),
    ///         Utc::now(),
    ///         get_random_bytes::<100>(rng).to_vec(),
    ///         forest,
    ///         store,
    ///         rng,
    ///     )
    ///     .await
    ///     .unwrap();
    ///
    ///     println!("file = {:?}", file);
    /// }
    /// ```
    pub async fn with_content(
        parent_name: &Name,
        time: DateTime<Utc>,
        content: Vec<u8>,
        forest: &mut impl PrivateForest,
        store: &impl BlockStore,
        rng: &mut impl CryptoRngCore,
    ) -> Result<Self> {
        let header = PrivateNodeHeader::new(parent_name, rng);
        let content = Self::prepare_content(&header.name, content, forest, store, rng).await?;

        Ok(Self {
            header,
            content: PrivateFileContent {
                persisted_as: OnceCell::new(),
                metadata: Metadata::new(time),
                previous: BTreeSet::new(),
                content,
            },
        })
    }

    /// Creates a file with provided content as a stream.
    ///
    /// Depending on the BlockStore implementation this will
    /// use essentially O(1) memory (roughly `2 * MAX_BLOCK_CONTENT_SIZE` bytes).
    ///
    /// # Examples
    ///
    /// ```
    /// use std::rc::Rc;
    /// use anyhow::Result;
    /// use async_std::fs::File;
    /// use chrono::Utc;
    /// use rand::thread_rng;
    /// use wnfs::{
    ///     private::{PrivateFile, forest::{hamt::HamtForest, traits::PrivateForest}},
    ///     common::MemoryBlockStore,
    /// };
    ///
    /// #[async_std::main]
    /// async fn main() -> Result<()> {
    ///     let disk_file = File::open("./test/fixtures/Clara Schumann, Scherzo no. 2, Op. 14.mp3").await?;
    ///
    ///     let store = &MemoryBlockStore::new();
    ///     let rng = &mut thread_rng();
    ///     let forest = &mut Rc::new(HamtForest::new_rsa_2048(rng));
    ///
    ///     let file = PrivateFile::with_content_streaming(
    ///         &forest.empty_name(),
    ///         Utc::now(),
    ///         disk_file,
    ///         forest,
    ///         store,
    ///         rng,
    ///     )
    ///     .await?;
    ///
    ///     println!("file = {:?}", file);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn with_content_streaming(
        parent_name: &Name,
        time: DateTime<Utc>,
        content: impl AsyncRead + Unpin,
        forest: &mut impl PrivateForest,
        store: &impl BlockStore,
        rng: &mut impl CryptoRngCore,
    ) -> Result<Self> {
        let header = PrivateNodeHeader::new(parent_name, rng);
        let content =
            Self::prepare_content_streaming(&header.name, content, forest, store, rng).await?;

        Ok(Self {
            header,
            content: PrivateFileContent {
                persisted_as: OnceCell::new(),
                metadata: Metadata::new(time),
                previous: BTreeSet::new(),
                content,
            },
        })
    }

    /// Streams the content of a file as chunk of blocks.
    ///
    /// # Examples
    ///
    /// ```
    /// use anyhow::Result;
    /// use std::rc::Rc;
    /// use chrono::Utc;
    /// use rand::thread_rng;
    /// use wnfs::{
    ///     private::{PrivateFile, forest::{hamt::HamtForest, traits::PrivateForest}},
    ///     common::{MemoryBlockStore, utils::get_random_bytes},
    /// };
    /// use futures::{future, TryStreamExt};
    ///
    /// #[async_std::main]
    /// async fn main() -> Result<()> {
    ///     let store = &MemoryBlockStore::new();
    ///     let rng = &mut thread_rng();
    ///     let forest = &mut Rc::new(HamtForest::new_rsa_2048(rng));
    ///
    ///     let content = get_random_bytes::<100>(rng).to_vec();
    ///     let file = PrivateFile::with_content(
    ///         &forest.empty_name(),
    ///         Utc::now(),
    ///         content.clone(),
    ///         forest,
    ///         store,
    ///         rng,
    ///     )
    ///     .await?;
    ///
    ///     let mut stream_content = vec![];
    ///     file.stream_content(0, forest, store)
    ///         .try_for_each(|chunk| {
    ///             stream_content.extend_from_slice(&chunk);
    ///             future::ready(Ok(()))
    ///         })
    ///         .await?;
    ///
    ///     assert_eq!(content, stream_content);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn stream_content<'a>(
        &'a self,
        index: usize,
        forest: &'a impl PrivateForest,
        store: &'a impl BlockStore,
    ) -> impl Stream<Item = Result<Vec<u8>>> + 'a {
        Box::pin(try_stream! {
            match &self.content.content {
                FileContent::Inline { data } => {
                    if index != 0 {
                        Err(FsError::FileShardNotFound)?
                    }

                    yield data.clone()
                },
                FileContent::External {
                    key,
                    block_count,
                    ..
                } => {
                    let name = &self.header.name;
                    for name in Self::generate_shard_labels(key, index, *block_count, name) {
                        let bytes = Self::decrypt_block(key, &name, forest, store).await?;
                        yield bytes
                    }
                }
            }
        })
    }

    /// Reads a number of bytes starting from a given offset.
    pub async fn read_at<'a>(
        &'a self,
        offset: usize,
        size: usize,
        forest: &'a impl PrivateForest,
        store: &'a impl BlockStore,
    ) -> Result<Vec<u8>> {
        let block_content_size = MAX_BLOCK_CONTENT_SIZE;
        let chunk_size_upper_bound = (self.get_content_size_upper_bound() - offset).min(size);
        if chunk_size_upper_bound == 0 {
            return Ok(vec![]);
        }
        let first_block = offset / block_content_size;
        let last_block = (offset + size) / block_content_size;
        let mut bytes = Vec::with_capacity(chunk_size_upper_bound);
        let mut content_stream = self.stream_content(first_block, forest, store).enumerate();
        while let Some((i, chunk)) = content_stream.next().await {
            let chunk = chunk?;
            let index = first_block + i;
            let from = if index == first_block {
                (offset - index * block_content_size).min(chunk.len())
            } else {
                0
            };
            let to = if index == last_block {
                (offset + size - index * block_content_size).min(chunk.len())
            } else {
                chunk.len()
            };
            bytes.extend_from_slice(&chunk[from..to]);
            if index == last_block {
                break;
            }
        }
        Ok(bytes)
    }

    /// Gets the metadata of the file
    pub fn get_metadata(&self) -> &Metadata {
        &self.content.metadata
    }

    /// Gets the entire content of a file.
    ///
    /// # Examples
    ///
    /// ```
    /// use anyhow::Result;
    /// use std::rc::Rc;
    /// use chrono::Utc;
    /// use rand::thread_rng;
    /// use wnfs::{
    ///     private::{PrivateFile, forest::{hamt::HamtForest, traits::PrivateForest}},
    ///     common::{MemoryBlockStore, utils::get_random_bytes},
    /// };
    ///
    /// #[async_std::main]
    /// async fn main() -> Result<()> {
    ///     let store = &MemoryBlockStore::new();
    ///     let rng = &mut thread_rng();
    ///     let forest = &mut Rc::new(HamtForest::new_rsa_2048(rng));
    ///
    ///     let content = get_random_bytes::<100>(rng).to_vec();
    ///     let file = PrivateFile::with_content(
    ///         &forest.empty_name(),
    ///         Utc::now(),
    ///         content.clone(),
    ///         forest,
    ///         store,
    ///         rng,
    ///     )
    ///     .await?;
    ///
    ///     let mut all_content = file.get_content(forest, store).await?;
    ///
    ///     assert_eq!(content, all_content);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_content(
        &self,
        forest: &impl PrivateForest,
        store: &impl BlockStore,
    ) -> Result<Vec<u8>> {
        // We're not using Vec::with_capacity here because
        // a call to get_content instead of stream_content seems to
        // indicate that the content is small enough to fit into
        // memory. So let's keep allocations low.
        let mut content = Vec::new();
        self.stream_content(0, forest, store)
            .try_for_each(|chunk| {
                content.extend_from_slice(&chunk);
                future::ready(Ok(()))
            })
            .await?;
        Ok(content)
    }

    /// Sets the content of a file.
    pub async fn set_content(
        &mut self,
        time: DateTime<Utc>,
        content: impl AsyncRead + Unpin,
        forest: &mut impl PrivateForest,
        store: &impl BlockStore,
        rng: &mut impl CryptoRngCore,
    ) -> Result<()> {
        self.content.metadata = Metadata::new(time);
        self.content.content =
            Self::prepare_content_streaming(&self.header.name, content, forest, store, rng).await?;
        Ok(())
    }

    /// Determines where to put the content of a file. This can either be inline or stored up in chunks in a private forest.
    pub(super) async fn prepare_content(
        file_name: &Name,
        content: Vec<u8>,
        forest: &mut impl PrivateForest,
        store: &impl BlockStore,
        rng: &mut impl CryptoRngCore,
    ) -> Result<FileContent> {
        // TODO(appcypher): Use a better heuristic to determine when to use external storage.
        let key = SnapshotKey::from(utils::get_random_bytes(rng));
        let block_count = (content.len() as f64 / MAX_BLOCK_CONTENT_SIZE as f64).ceil() as usize;

        for (index, name) in
            Self::generate_shard_labels(&key, 0, block_count, file_name).enumerate()
        {
            let start = index * MAX_BLOCK_CONTENT_SIZE;
            let end = content.len().min((index + 1) * MAX_BLOCK_CONTENT_SIZE);
            let slice = &content[start..end];

            let enc_bytes = key.encrypt(slice, rng)?;
            let content_cid = store.put_block(enc_bytes, CODEC_RAW).await?;

            forest
                .put_encrypted(&name, Some(content_cid), store)
                .await?;
        }

        Ok(FileContent::External {
            key,
            block_count,
            block_content_size: MAX_BLOCK_CONTENT_SIZE,
        })
    }

    /// Drains the content streamed-in and puts it into the private forest
    /// as blocks of encrypted data.
    /// Returns an external `FileContent` that contains necessary information
    /// to later retrieve the data.
    pub(super) async fn prepare_content_streaming(
        file_name: &Name,
        mut content: impl AsyncRead + Unpin,
        forest: &mut impl PrivateForest,
        store: &impl BlockStore,
        rng: &mut impl CryptoRngCore,
    ) -> Result<FileContent> {
        let key = SnapshotKey::from(utils::get_random_bytes(rng));
        let file_revision_name = Self::create_revision_name(file_name, &key);

        let mut block_index = 0;

        loop {
            let mut current_block = vec![0u8; MAX_BLOCK_SIZE];
            let nonce = SnapshotKey::generate_nonce(rng);
            current_block[..NONCE_SIZE].copy_from_slice(nonce.as_ref());

            // read up to MAX_BLOCK_CONTENT_SIZE content

            let content_end = NONCE_SIZE + MAX_BLOCK_CONTENT_SIZE;
            let (bytes_written, done) =
                utils::read_fully(&mut content, &mut current_block[NONCE_SIZE..content_end])
                    .await?;

            // truncate the vector to its actual length.
            current_block.truncate(bytes_written + NONCE_SIZE);

            let tag = key.encrypt_in_place(&nonce, &mut current_block[NONCE_SIZE..])?;
            current_block.extend_from_slice(tag.as_ref());

            let content_cid = store.put_block(current_block, CODEC_RAW).await?;

            let name = Self::create_block_label(&key, block_index, &file_revision_name);
            forest
                .put_encrypted(&name, Some(content_cid), store)
                .await?;

            block_index += 1;

            if done {
                break;
            }
        }

        Ok(FileContent::External {
            key,
            block_count: block_index,
            block_content_size: MAX_BLOCK_CONTENT_SIZE,
        })
    }

    /// Gets the upper bound of a file content size.
    pub fn get_content_size_upper_bound(&self) -> usize {
        match &self.content.content {
            FileContent::Inline { data } => data.len(),
            FileContent::External {
                block_count,
                block_content_size,
                ..
            } => block_count * block_content_size,
        }
    }

    /// Decrypts a block of a file's content.
    async fn decrypt_block(
        key: &SnapshotKey,
        name: &Name,
        forest: &impl PrivateForest,
        store: &impl BlockStore,
    ) -> Result<Vec<u8>> {
        let cid = forest
            .get_encrypted(name, store)
            .await?
            .ok_or(FsError::FileShardNotFound)?
            .iter()
            .next()
            .expect("Expected set with at least a one cid");

        let enc_bytes = store.get_block(cid).await?;
        let bytes = key.decrypt(&enc_bytes)?;

        Ok(bytes)
    }

    /// Generates the labels for the shards of a file.
    fn generate_shard_labels<'a>(
        key: &'a SnapshotKey,
        mut index: usize,
        block_count: usize,
        file_block_name: &'a Name,
    ) -> impl Iterator<Item = Name> + 'a {
        let file_revision_name = Self::create_revision_name(file_block_name, key);
        iter::from_fn(move || {
            if index >= block_count {
                return None;
            }

            let label = Self::create_block_label(key, index, &file_revision_name);
            index += 1;
            Some(label)
        })
    }

    fn create_revision_name(file_block_name: &Name, key: &SnapshotKey) -> Name {
        let revision_segment = NameSegment::new_hashed(key.0.as_bytes());
        file_block_name.with_segments_added(Some(revision_segment))
    }

    /// Creates the label for a block of a file.
    fn create_block_label(key: &SnapshotKey, index: usize, file_revision_name: &Name) -> Name {
        let mut vec = Vec::with_capacity(40);
        vec.extend(key.0.as_bytes()); // 32 bytes
        vec.extend((index as u64).to_le_bytes()); // 8 bytes
        let block_segment = NameSegment::new_hashed(vec);

        file_revision_name.with_segments_added(Some(block_segment))
    }

    /// This should be called to prepare a node for modifications,
    /// if it's meant to be a successor revision of the current revision.
    ///
    /// This doesn't have any effect if the current state hasn't been `.store()`ed yet.
    /// Otherwise, it clones itself, stores its current CID in the previous links and
    /// advances its ratchet.
    pub(crate) fn prepare_next_revision<'a>(self: &'a mut Rc<Self>) -> Result<&'a mut Self> {
        let previous_cid = match self.content.persisted_as.get() {
            Some(cid) => *cid,
            None => {
                // The current revision wasn't written yet.
                // There's no point in advancing the revision even further.
                return Ok(Rc::make_mut(self));
            }
        };

        let temporal_key = self.header.derive_temporal_key();
        let previous_link = (1, Encrypted::from_value(previous_cid, &temporal_key)?);
        let cloned = Rc::make_mut(self);

        // We make sure to clear any cached states.
        cloned.content.persisted_as = OnceCell::new();
        cloned.content.previous = [previous_link].into_iter().collect();
        cloned.header.advance_ratchet();

        Ok(cloned)
    }

    /// Returns the private ref, if this file has been `.store()`ed before.
    pub(crate) fn derive_private_ref(&self, setup: &AccumulatorSetup) -> Option<PrivateRef> {
        self.content.persisted_as.get().map(|content_cid| {
            self.header
                .derive_revision_ref(setup)
                .into_private_ref(*content_cid)
        })
    }

    /// This prepares this file for key rotation, usually for moving or
    /// copying the file to some other place.
    ///
    /// Will reset the ratchet, so a different key is necessary for read access,
    /// will reset the inumber to reset write access,
    /// will update the name to be the sub-name of given parent name,
    /// so it inherits the write access rules from the new parent and
    /// resets the `persisted_as` pointer.
    /// Will copy and re-encrypt all external content.
    pub(crate) async fn prepare_key_rotation(
        &mut self,
        parent_name: &Name,
        forest: &mut impl PrivateForest,
        store: &impl BlockStore,
        rng: &mut impl CryptoRngCore,
    ) -> Result<()> {
        let content = self.get_content(forest, store).await?;

        self.header.inumber = NameSegment::new(rng);
        self.header.update_name(parent_name);
        self.header.reset_ratchet(rng);
        self.content.persisted_as = OnceCell::new();

        let content = Self::prepare_content(&self.header.name, content, forest, store, rng).await?;
        self.content.content = content;

        Ok(())
    }

    /// Stores this PrivateFile in the PrivateForest.
    pub(crate) async fn store(
        &self,
        forest: &mut impl PrivateForest,
        store: &impl BlockStore,
        rng: &mut impl CryptoRngCore,
    ) -> Result<PrivateRef> {
        let setup = &forest.get_accumulator_setup().clone();
        let header_cid = self.header.store(store, setup).await?;
        let temporal_key = self.header.derive_temporal_key();
        let snapshot_key = temporal_key.derive_snapshot_key();
        let name_with_revision = self.header.get_revision_name();

        let content_cid = self
            .content
            .store(header_cid, &snapshot_key, store, rng)
            .await?;

        forest
            .put_encrypted(&name_with_revision, [header_cid, content_cid], store)
            .await?;

        Ok(self
            .header
            .derive_revision_ref(setup)
            .into_private_ref(content_cid))
    }

    /// Creates a new [`PrivateFile`] from a [`PrivateFileContentSerializable`].
    pub(crate) async fn from_serializable(
        serializable: PrivateFileContentSerializable,
        temporal_key: &TemporalKey,
        cid: Cid,
        store: &impl BlockStore,
        parent_name: Option<Name>,
        setup: &AccumulatorSetup,
    ) -> Result<Self> {
        if serializable.version.major != 0 || serializable.version.minor != 2 {
            bail!(FsError::UnexpectedVersion(serializable.version));
        }

        let content = PrivateFileContent {
            persisted_as: OnceCell::new_with(Some(cid)),
            previous: serializable.previous.into_iter().collect(),
            metadata: serializable.metadata,
            content: serializable.content,
        };

        let header = PrivateNodeHeader::load(
            &serializable.header_cid,
            temporal_key,
            store,
            parent_name,
            setup,
        )
        .await?;
        Ok(Self { header, content })
    }

    /// Wraps the file in a [`PrivateNode`].
    pub fn as_node(self: &Rc<Self>) -> PrivateNode {
        PrivateNode::File(Rc::clone(self))
    }
}

impl PrivateFileContent {
    /// Serializes the file to a dag-cbor representation.
    pub(crate) fn to_dag_cbor(&self, header_cid: Cid) -> Result<Vec<u8>> {
        Ok(serde_ipld_dagcbor::to_vec(
            &PrivateNodeContentSerializable::File(PrivateFileContentSerializable {
                version: WNFS_VERSION,
                previous: self.previous.iter().cloned().collect(),
                header_cid,
                metadata: self.metadata.clone(),
                content: self.content.clone(),
            }),
        )?)
    }

    pub(crate) async fn store(
        &self,
        header_cid: Cid,
        snapshot_key: &SnapshotKey,
        store: &impl BlockStore,
        rng: &mut impl CryptoRngCore,
    ) -> Result<Cid> {
        Ok(*self
            .persisted_as
            .get_or_try_init::<anyhow::Error>(async {
                // TODO(matheus23) deduplicate when reworking serialization

                // Serialize node to cbor.
                let bytes = self.to_dag_cbor(header_cid)?;

                // Encrypt bytes with snapshot key.
                let block = snapshot_key.encrypt(&bytes, rng)?;

                // Store content section in blockstore and get Cid.
                store.put_block(block, CODEC_RAW).await
            })
            .await?)
    }
}

impl PartialEq for PrivateFileContent {
    fn eq(&self, other: &Self) -> bool {
        self.previous == other.previous
            && self.metadata == other.metadata
            && self.content == other.content
    }
}

impl Clone for PrivateFileContent {
    fn clone(&self) -> Self {
        Self {
            persisted_as: OnceCell::new_with(self.persisted_as.get().cloned()),
            previous: self.previous.clone(),
            metadata: self.metadata.clone(),
            content: self.content.clone(),
        }
    }
}

impl Id for PrivateFile {
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
    use crate::private::forest::hamt::HamtForest;
    use async_std::fs::File;
    use rand::Rng;
    use rand_chacha::ChaCha12Rng;
    use rand_core::SeedableRng;
    use wnfs_common::MemoryBlockStore;

    #[async_std::test]
    async fn can_create_empty_file() {
        let store = &MemoryBlockStore::new();
        let rng = &mut ChaCha12Rng::seed_from_u64(0);
        let forest = &Rc::new(HamtForest::new_rsa_2048(rng));

        let file = PrivateFile::new(&forest.empty_name(), Utc::now(), rng);
        let file_content = file.get_content(forest, store).await.unwrap();

        assert!(file_content.is_empty());
    }

    #[async_std::test]
    async fn can_stream_limited_content_from_file() {
        let mut content = vec![0u8; MAX_BLOCK_CONTENT_SIZE * 5];
        rand::thread_rng().fill(&mut content[..]);

        let store = &MemoryBlockStore::new();
        let rng = &mut ChaCha12Rng::seed_from_u64(0);
        let forest = &mut Rc::new(HamtForest::new_rsa_2048(rng));

        let file = PrivateFile::with_content(
            &forest.empty_name(),
            Utc::now(),
            content.clone(),
            forest,
            store,
            rng,
        )
        .await
        .unwrap();

        let mut collected_content = Vec::new();
        let mut block_limit = 2;
        file.stream_content(2, forest, store)
            .for_each(|chunk| {
                if block_limit == 0 {
                    return future::ready(());
                }

                collected_content.extend_from_slice(&chunk.unwrap());
                block_limit -= 1;
                future::ready(())
            })
            .await;

        assert_eq!(
            collected_content,
            content[2 * MAX_BLOCK_CONTENT_SIZE..4 * MAX_BLOCK_CONTENT_SIZE]
        );
    }

    #[async_std::test]
    async fn can_construct_file_from_stream() {
        let disk_file = File::open("./test/fixtures/Clara Schumann, Scherzo no. 2, Op. 14.mp3")
            .await
            .unwrap();

        let store = &MemoryBlockStore::new();
        let rng = &mut ChaCha12Rng::seed_from_u64(0);
        let forest = &mut Rc::new(HamtForest::new_rsa_2048(rng));

        let file = PrivateFile::with_content_streaming(
            &forest.empty_name(),
            Utc::now(),
            disk_file,
            forest,
            store,
            rng,
        )
        .await
        .unwrap();

        assert!(
            matches!(file.content.content, FileContent::External { block_count, .. } if block_count > 0)
        );
    }
}

#[cfg(test)]
mod proptests {
    use super::MAX_BLOCK_CONTENT_SIZE;
    use crate::private::{
        forest::{hamt::HamtForest, traits::PrivateForest},
        PrivateFile,
    };
    use async_std::io::Cursor;
    use chrono::Utc;
    use futures::{future, StreamExt};
    use rand_chacha::ChaCha12Rng;
    use rand_core::SeedableRng;
    use std::rc::Rc;
    use test_strategy::proptest;
    use wnfs_common::{BlockStoreError, MemoryBlockStore};

    /// Size of the test file at "./test/fixtures/Clara Schumann, Scherzo no. 2, Op. 14.mp3"
    const FIXTURE_SCHERZO_SIZE: usize = 4028150;

    #[proptest(cases = 100)]
    fn can_include_and_get_content_from_file(
        #[strategy(0..(MAX_BLOCK_CONTENT_SIZE * 2))] length: usize,
    ) {
        async_std::task::block_on(async {
            let content = vec![0u8; length];
            let store = &MemoryBlockStore::new();
            let rng = &mut ChaCha12Rng::seed_from_u64(0);
            let forest = &mut Rc::new(HamtForest::new_rsa_2048(rng));

            let file = PrivateFile::with_content(
                &forest.empty_name(),
                Utc::now(),
                content.clone(),
                forest,
                store,
                rng,
            )
            .await
            .unwrap();

            let collected_content = file.get_content(forest, store).await.unwrap();

            assert_eq!(collected_content, content);
        })
    }

    #[proptest(cases = 10)]
    fn can_include_and_stream_content_from_file(
        #[strategy(0..(MAX_BLOCK_CONTENT_SIZE * 2))] length: usize,
    ) {
        async_std::task::block_on(async {
            let content = vec![0u8; length];
            let store = &MemoryBlockStore::new();
            let rng = &mut ChaCha12Rng::seed_from_u64(0);
            let forest = &mut Rc::new(HamtForest::new_rsa_2048(rng));

            let file = PrivateFile::with_content(
                &forest.empty_name(),
                Utc::now(),
                content.clone(),
                forest,
                store,
                rng,
            )
            .await
            .unwrap();

            let mut collected_content = Vec::new();
            file.stream_content(0, forest, store)
                .for_each(|chunk| {
                    collected_content.extend_from_slice(&chunk.unwrap());
                    future::ready(())
                })
                .await;

            assert_eq!(collected_content, content);
        })
    }

    #[proptest(cases = 100)]
    fn can_propagate_missing_chunk_error(
        #[strategy(0..(MAX_BLOCK_CONTENT_SIZE * 2))] length: usize,
    ) {
        async_std::task::block_on(async {
            let store = &MemoryBlockStore::new();
            let rng = &mut ChaCha12Rng::seed_from_u64(0);
            let forest = &mut Rc::new(HamtForest::new_rsa_2048(rng));

            let mut file = PrivateFile::new(&forest.empty_name(), Utc::now(), rng);

            file.set_content(
                Utc::now(),
                &mut Cursor::new(vec![5u8; length]),
                forest,
                &MemoryBlockStore::default(),
                rng,
            )
            .await
            .unwrap();

            let error = file
                .get_content(forest, store)
                .await
                .expect_err("Expected error");

            let error = error.downcast_ref::<BlockStoreError>().unwrap();

            assert!(matches!(error, BlockStoreError::CIDNotFound(_)));
        })
    }

    #[proptest(cases = 10)]
    fn can_read_section_of_file(
        #[strategy(0..FIXTURE_SCHERZO_SIZE)] size: usize,
        #[strategy(0..FIXTURE_SCHERZO_SIZE)] offset: usize,
    ) {
        use async_std::{io::SeekFrom, prelude::*};
        async_std::task::block_on(async {
            let size = size.min(FIXTURE_SCHERZO_SIZE - offset);
            let mut disk_file = async_std::fs::File::open(
                "./test/fixtures/Clara Schumann, Scherzo no. 2, Op. 14.mp3",
            )
            .await
            .unwrap();

            let rng = &mut ChaCha12Rng::seed_from_u64(0);
            let forest = &mut Rc::new(HamtForest::new_rsa_2048(rng));
            let store = &MemoryBlockStore::new();

            let file = PrivateFile::with_content_streaming(
                &forest.empty_name(),
                Utc::now(),
                disk_file.clone(),
                forest,
                store,
                rng,
            )
            .await
            .unwrap();

            let mut source_content = vec![0u8; size];
            disk_file
                .seek(SeekFrom::Start(offset as u64))
                .await
                .unwrap();
            disk_file.read_exact(&mut source_content).await.unwrap();
            let wnfs_content = file.read_at(offset, size, forest, store).await.unwrap();

            assert_eq!(source_content, wnfs_content);
        })
    }
}
