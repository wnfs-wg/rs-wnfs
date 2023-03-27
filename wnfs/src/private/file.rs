use super::{
    encrypted::Encrypted, PrivateForest, PrivateNode, PrivateNodeHeader, PrivateRef, SnapshotKey,
    AUTHENTICATION_TAG_SIZE, NONCE_SIZE,
};
use crate::{error::FsError, traits::Id};
use anyhow::Result;
use async_once_cell::OnceCell;
use async_stream::try_stream;
use chrono::{DateTime, Utc};
use futures::{future, AsyncRead, Stream, StreamExt, TryStreamExt};
use libipld::{Cid, IpldCodec};
use rand_core::RngCore;
use semver::Version;
use serde::{de::Error as DeError, Deserialize, Deserializer, Serialize};
use sha3::Sha3_256;
use std::{collections::BTreeSet, iter, rc::Rc};
use wnfs_common::{dagcbor, utils, BlockStore, Metadata, NodeType, MAX_BLOCK_SIZE};
use wnfs_hamt::Hasher;
use wnfs_namefilter::Namefilter;

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
/// use std::rc::Rc;
/// use chrono::Utc;
/// use rand::thread_rng;
/// use wnfs::{
///     private::{PrivateForest, PrivateRef, PrivateFile},
///     common::{MemoryBlockStore, utils::get_random_bytes},
///     namefilter::Namefilter,
/// };
///
/// #[async_std::main]
/// async fn main() {
///     let store = &mut MemoryBlockStore::default();
///     let rng = &mut thread_rng();
///     let forest = &mut Rc::new(PrivateForest::new());
///
///     let file = PrivateFile::with_content(
///         Namefilter::default(),
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
#[derive(Debug, Clone, PartialEq)]
pub struct PrivateFile {
    pub header: PrivateNodeHeader,
    pub(crate) content: PrivateFileContent,
}

#[derive(Debug)]
pub struct PrivateFileContent {
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

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PrivateFileContentSerializable {
    pub r#type: NodeType,
    pub version: Version,
    #[serde(rename = "headerCid")]
    pub header_cid: Cid,
    pub previous: Vec<(usize, Encrypted<Cid>)>,
    pub metadata: Metadata,
    pub content: FileContent,
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
    /// use wnfs::{
    ///     private::PrivateFile,
    ///     namefilter::Namefilter,
    ///     traits::Id
    /// };
    /// use chrono::Utc;
    /// use rand::thread_rng;
    ///
    /// let rng = &mut thread_rng();
    /// let file = PrivateFile::new(
    ///     Namefilter::default(),
    ///     Utc::now(),
    ///     rng,
    /// );
    ///
    /// println!("file = {:?}", file);
    /// ```
    pub fn new(parent_bare_name: Namefilter, time: DateTime<Utc>, rng: &mut impl RngCore) -> Self {
        Self {
            header: PrivateNodeHeader::new(parent_bare_name, rng),
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
    ///     private::{PrivateForest, PrivateRef, PrivateFile},
    ///     common::{MemoryBlockStore, utils::get_random_bytes, MAX_BLOCK_SIZE},
    ///     namefilter::Namefilter,
    /// };
    ///
    /// #[async_std::main]
    /// async fn main() {
    ///     let store = &mut MemoryBlockStore::default();
    ///     let rng = &mut thread_rng();
    ///     let forest = &mut Rc::new(PrivateForest::new());
    ///
    ///     let file = PrivateFile::with_content(
    ///         Namefilter::default(),
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
        parent_bare_name: Namefilter,
        time: DateTime<Utc>,
        content: Vec<u8>,
        forest: &mut Rc<PrivateForest>,
        store: &impl BlockStore,
        rng: &mut impl RngCore,
    ) -> Result<Self> {
        let header = PrivateNodeHeader::new(parent_bare_name, rng);
        let content = Self::prepare_content(&header.bare_name, content, forest, store, rng).await?;

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
    /// use async_std::fs::File;
    /// use chrono::Utc;
    /// use rand::thread_rng;
    /// use wnfs::{
    ///     private::{PrivateForest, PrivateRef, PrivateFile},
    ///     common::{MemoryBlockStore, MAX_BLOCK_SIZE},
    ///     namefilter::Namefilter,
    /// };
    ///
    /// #[async_std::main]
    /// async fn main() {
    ///     let disk_file = File::open("./test/fixtures/Clara Schumann, Scherzo no. 2, Op. 14.mp3")
    ///         .await
    ///         .unwrap();
    ///
    ///     let store = &mut MemoryBlockStore::default();
    ///     let rng = &mut thread_rng();
    ///     let forest = &mut Rc::new(PrivateForest::new());
    ///
    ///     let file = PrivateFile::with_content_streaming(
    ///         Namefilter::default(),
    ///         Utc::now(),
    ///         disk_file,
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
    pub async fn with_content_streaming(
        parent_bare_name: Namefilter,
        time: DateTime<Utc>,
        content: impl AsyncRead + Unpin,
        forest: &mut Rc<PrivateForest>,
        store: &impl BlockStore,
        rng: &mut impl RngCore,
    ) -> Result<Self> {
        let header = PrivateNodeHeader::new(parent_bare_name, rng);
        let content =
            Self::prepare_content_streaming(&header.bare_name, content, forest, store, rng).await?;

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
    /// use std::rc::Rc;
    /// use chrono::Utc;
    /// use rand::thread_rng;
    /// use wnfs::{
    ///     private::{PrivateForest, PrivateRef, PrivateFile},
    ///     common::{MemoryBlockStore, utils::get_random_bytes},
    ///     namefilter::Namefilter,
    /// };
    /// use futures::{future, StreamExt};
    ///
    /// #[async_std::main]
    /// async fn main() {
    ///     let store = &mut MemoryBlockStore::default();
    ///     let rng = &mut thread_rng();
    ///     let forest = &mut Rc::new(PrivateForest::new());
    ///
    ///     let content = get_random_bytes::<100>(rng).to_vec();
    ///     let file = PrivateFile::with_content(
    ///         Namefilter::default(),
    ///         Utc::now(),
    ///         content.clone(),
    ///         forest,
    ///         store,
    ///         rng,
    ///     )
    ///     .await
    ///     .unwrap();
    ///
    ///     let mut stream_content = vec![];
    ///     file.stream_content(0, &forest, store)
    ///         .for_each(|chunk| {
    ///             stream_content.extend_from_slice(&chunk.unwrap());
    ///             future::ready(())
    ///         })
    ///         .await;
    ///
    ///     assert_eq!(content, stream_content);
    /// }
    /// ```
    pub fn stream_content<'a>(
        &'a self,
        index: usize,
        forest: &'a PrivateForest,
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
                    let bare_name = &self.header.bare_name;
                    for label in Self::generate_shard_labels(key, index,  *block_count, bare_name) {
                        let bytes = Self::decrypt_block(key, &label, forest, store).await?;
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
        forest: &'a PrivateForest,
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
    /// use std::rc::Rc;
    /// use chrono::Utc;
    /// use rand::thread_rng;
    /// use wnfs::{
    ///     private::{PrivateFile, PrivateForest, PrivateRef},
    ///     common::{MemoryBlockStore, utils::get_random_bytes},
    ///     namefilter::Namefilter,
    /// };
    ///
    /// #[async_std::main]
    /// async fn main() {
    ///     let store = &mut MemoryBlockStore::default();
    ///     let rng = &mut thread_rng();
    ///     let forest = &mut Rc::new(PrivateForest::new());
    ///
    ///     let content = get_random_bytes::<100>(rng).to_vec();
    ///     let file = PrivateFile::with_content(
    ///         Namefilter::default(),
    ///         Utc::now(),
    ///         content.clone(),
    ///         forest,
    ///         store,
    ///         rng,
    ///     )
    ///     .await
    ///     .unwrap();
    ///
    ///     let mut all_content = file.get_content(forest, store).await.unwrap();
    ///
    ///     assert_eq!(content, all_content);
    /// }
    /// ```
    pub async fn get_content(
        &self,
        forest: &PrivateForest,
        store: &impl BlockStore,
    ) -> Result<Vec<u8>> {
        let mut content = Vec::with_capacity(self.get_content_size_upper_bound());
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
        forest: &mut Rc<PrivateForest>,
        store: &impl BlockStore,
        rng: &mut impl RngCore,
    ) -> Result<()> {
        self.content.metadata = Metadata::new(time);
        self.content.content =
            Self::prepare_content_streaming(&self.header.bare_name, content, forest, store, rng)
                .await?;
        Ok(())
    }

    /// Determines where to put the content of a file. This can either be inline or stored up in chunks in a private forest.
    pub(super) async fn prepare_content(
        bare_name: &Namefilter,
        content: Vec<u8>,
        forest: &mut Rc<PrivateForest>,
        store: &impl BlockStore,
        rng: &mut impl RngCore,
    ) -> Result<FileContent> {
        // TODO(appcypher): Use a better heuristic to determine when to use external storage.
        let key = SnapshotKey::from(utils::get_random_bytes(rng));
        let block_count = (content.len() as f64 / MAX_BLOCK_CONTENT_SIZE as f64).ceil() as usize;

        for (index, label) in
            Self::generate_shard_labels(&key, 0, block_count, bare_name).enumerate()
        {
            let start = index * MAX_BLOCK_CONTENT_SIZE;
            let end = content.len().min((index + 1) * MAX_BLOCK_CONTENT_SIZE);
            let slice = &content[start..end];

            let enc_bytes = key.encrypt(slice, rng)?;
            let content_cid = store.put_block(enc_bytes, IpldCodec::Raw).await?;

            forest
                .put_encrypted(label, Some(content_cid), store)
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
        bare_name: &Namefilter,
        mut content: impl AsyncRead + Unpin,
        forest: &mut Rc<PrivateForest>,
        store: &impl BlockStore,
        rng: &mut impl RngCore,
    ) -> Result<FileContent> {
        let key = SnapshotKey::from(utils::get_random_bytes(rng));

        let mut block_index = 0;

        loop {
            let mut current_block = vec![0u8; MAX_BLOCK_SIZE];
            let nonce = SnapshotKey::generate_nonce(rng);
            current_block[..NONCE_SIZE].copy_from_slice(&nonce);

            // read up to MAX_BLOCK_CONTENT_SIZE content

            let content_end = NONCE_SIZE + MAX_BLOCK_CONTENT_SIZE;
            let (bytes_written, done) =
                utils::read_fully(&mut content, &mut current_block[NONCE_SIZE..content_end])
                    .await?;

            // truncate the vector to its actual length.
            current_block.truncate(bytes_written + NONCE_SIZE);

            let tag = key.encrypt_in_place(&nonce, &mut current_block[NONCE_SIZE..])?;
            current_block.extend_from_slice(&tag);

            let content_cid = store.put_block(current_block, IpldCodec::Raw).await?;

            let label = Self::create_block_label(&key, block_index, bare_name);
            forest
                .put_encrypted(label, Some(content_cid), store)
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
        label: &Namefilter,
        forest: &PrivateForest,
        store: &impl BlockStore,
    ) -> Result<Vec<u8>> {
        let label_hash = &Sha3_256::hash(&label.as_bytes());

        let cids = forest
            .get_encrypted(label_hash, store)
            .await?
            .ok_or(FsError::FileShardNotFound)?;

        let cid = cids
            .iter()
            .next()
            .expect("Expected set with at least a Cid");

        let enc_bytes = store.get_block(cid).await?;
        let bytes = key.decrypt(&enc_bytes)?;

        Ok(bytes)
    }

    /// Generates the labels for the shards of a file.
    fn generate_shard_labels<'a>(
        key: &'a SnapshotKey,
        mut index: usize,
        block_count: usize,
        bare_name: &'a Namefilter,
    ) -> impl Iterator<Item = Namefilter> + 'a {
        iter::from_fn(move || {
            if index >= block_count {
                return None;
            }

            let label = Self::create_block_label(key, index, bare_name);
            index += 1;
            Some(label)
        })
    }

    /// Creates the label for a block of a file.
    fn create_block_label(key: &SnapshotKey, index: usize, bare_name: &Namefilter) -> Namefilter {
        let key_bytes = key.0.as_bytes();
        let key_hash = Sha3_256::hash(&[key_bytes, &index.to_le_bytes()[..]].concat());

        let mut label = bare_name.clone();
        label.add(&key_bytes);
        label.add(&key_hash);
        label.saturate();

        label
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
    pub(crate) fn get_private_ref(&self) -> Option<PrivateRef> {
        self.content.persisted_as.get().map(|content_cid| {
            self.header
                .derive_revision_ref()
                .as_private_ref(*content_cid)
        })
    }

    /// This prepares this file for key rotation, usually for moving or
    /// copying the file to some other place.
    ///
    /// Will reset the ratchet, so a different key is necessary for read access,
    /// will reset the inumber to reset write access,
    /// will update the bare namefilter to match the new parent's namefilter,
    /// so it inherits the write access rules from the new parent and
    /// resets the `persisted_as` pointer.
    /// Will copy and re-encrypt all external content.
    pub(crate) async fn prepare_key_rotation(
        &mut self,
        parent_bare_name: Namefilter,
        forest: &mut Rc<PrivateForest>,
        store: &impl BlockStore,
        rng: &mut impl RngCore,
    ) -> Result<()> {
        let content = self.get_content(forest, store).await?;

        self.header.inumber = utils::get_random_bytes(rng);
        self.header.update_bare_name(parent_bare_name);
        self.header.reset_ratchet(rng);
        self.content.persisted_as = OnceCell::new();

        let content =
            Self::prepare_content(&self.header.bare_name, content, forest, store, rng).await?;
        self.content.content = content;

        Ok(())
    }

    /// Stores this PrivateFile in the PrivateForest.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::rc::Rc;
    /// use chrono::Utc;
    /// use rand::thread_rng;
    /// use wnfs::{
    ///     private::{PrivateForest, PrivateRef, PrivateFile, PrivateNode},
    ///     common::{BlockStore, MemoryBlockStore},
    ///     namefilter::Namefilter,
    /// };
    ///
    /// #[async_std::main]
    /// async fn main() {
    ///     let store = &mut MemoryBlockStore::default();
    ///     let rng = &mut thread_rng();
    ///     let forest = &mut Rc::new(PrivateForest::new());
    ///     let file = Rc::new(PrivateFile::new(
    ///         Namefilter::default(),
    ///         Utc::now(),
    ///         rng,
    ///     ));
    ///
    ///     let private_ref = file.store(forest, store, rng).await.unwrap();
    ///
    ///     let node = PrivateNode::File(Rc::clone(&file));
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
        store: &impl BlockStore,
        rng: &mut impl RngCore,
    ) -> Result<PrivateRef> {
        let header_cid = self.header.store(store).await?;
        let snapshot_key = self.header.derive_temporal_key().derive_snapshot_key();
        let label = self.header.get_saturated_name();

        let content_cid = self
            .content
            .store(header_cid, &snapshot_key, store, rng)
            .await?;

        forest
            .put_encrypted(label, [header_cid, content_cid], store)
            .await?;

        Ok(self
            .header
            .derive_revision_ref()
            .as_private_ref(content_cid))
    }

    /// Wraps the file in a [`PrivateNode`].
    pub fn as_node(self: &Rc<Self>) -> PrivateNode {
        PrivateNode::File(Rc::clone(self))
    }
}

impl PrivateFileContent {
    /// Serializes the file with provided Serde serialilzer.
    pub(crate) fn serialize<S>(&self, serializer: S, header_cid: Cid) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        (PrivateFileContentSerializable {
            r#type: NodeType::PrivateFile,
            version: Version::new(0, 2, 0),
            previous: self.previous.iter().cloned().collect(),
            header_cid,
            metadata: self.metadata.clone(),
            content: self.content.clone(),
        })
        .serialize(serializer)
    }

    /// Deserializes the file with provided Serde deserializer and key.
    pub(crate) fn deserialize<'de, D>(
        deserializer: D,
        from_cid: Cid,
    ) -> Result<(Self, Cid), D::Error>
    where
        D: Deserializer<'de>,
    {
        let PrivateFileContentSerializable {
            r#type,
            version,
            metadata,
            previous,
            header_cid,
            content,
        } = PrivateFileContentSerializable::deserialize(deserializer)?;

        if version.major != 0 || version.minor != 2 {
            return Err(DeError::custom(FsError::UnexpectedVersion(version)));
        }

        if r#type != NodeType::PrivateFile {
            return Err(DeError::custom(FsError::UnexpectedNodeType(r#type)));
        }

        Ok((
            Self {
                persisted_as: OnceCell::new_with(Some(from_cid)),
                previous: previous.into_iter().collect(),
                metadata,
                content,
            },
            header_cid,
        ))
    }

    pub(crate) async fn store(
        &self,
        header_cid: Cid,
        snapshot_key: &SnapshotKey,
        store: &impl BlockStore,
        rng: &mut impl RngCore,
    ) -> Result<Cid> {
        Ok(*self
            .persisted_as
            .get_or_try_init::<anyhow::Error>(async {
                // TODO(matheus23) deduplicate when reworking serialization

                // Serialize node to cbor.
                let ipld = self.serialize(libipld::serde::Serializer, header_cid)?;
                let bytes = dagcbor::encode(&ipld)?;

                // Encrypt bytes with snapshot key.
                let block = snapshot_key.encrypt(&bytes, rng)?;

                // Store content section in blockstore and get Cid.
                store.put_block(block, libipld::IpldCodec::Raw).await
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
    use async_std::fs::File;
    use proptest::test_runner::{RngAlgorithm, TestRng};
    use rand::Rng;
    use wnfs_common::MemoryBlockStore;

    #[async_std::test]
    async fn can_create_empty_file() {
        let store = &mut MemoryBlockStore::default();
        let rng = &mut TestRng::deterministic_rng(RngAlgorithm::ChaCha);
        let forest = &Rc::new(PrivateForest::new());

        let file = PrivateFile::new(Namefilter::new(), Utc::now(), rng);
        let file_content = file.get_content(forest, store).await.unwrap();

        assert!(file_content.is_empty());
    }

    #[async_std::test]
    async fn can_stream_limited_content_from_file() {
        let mut content = vec![0u8; MAX_BLOCK_CONTENT_SIZE * 5];
        rand::thread_rng().fill(&mut content[..]);

        let store = &mut MemoryBlockStore::default();
        let rng = &mut TestRng::deterministic_rng(RngAlgorithm::ChaCha);
        let forest = &mut Rc::new(PrivateForest::new());

        let file = PrivateFile::with_content(
            Namefilter::default(),
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

        let forest = &mut Rc::new(PrivateForest::new());
        let store = &mut MemoryBlockStore::new();
        let rng = &mut TestRng::deterministic_rng(RngAlgorithm::ChaCha);

        let file = PrivateFile::with_content_streaming(
            Namefilter::default(),
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
    use crate::private::{PrivateFile, PrivateForest};
    use async_std::io::Cursor;
    use chrono::Utc;
    use futures::{future, StreamExt};
    use proptest::test_runner::{RngAlgorithm, TestRng};
    use std::rc::Rc;
    use test_strategy::proptest;
    use wnfs_common::{BlockStoreError, MemoryBlockStore};
    use wnfs_namefilter::Namefilter;

    /// Size of the test file at "./test/fixtures/Clara Schumann, Scherzo no. 2, Op. 14.mp3"
    const FIXTURE_SCHERZO_SIZE: usize = 4028150;

    #[proptest(cases = 100)]
    fn can_include_and_get_content_from_file(
        #[strategy(0..(MAX_BLOCK_CONTENT_SIZE * 2))] length: usize,
    ) {
        async_std::task::block_on(async {
            let content = vec![0u8; length];
            let store = &mut MemoryBlockStore::default();
            let rng = &mut TestRng::deterministic_rng(RngAlgorithm::ChaCha);
            let forest = &mut Rc::new(PrivateForest::new());

            let file = PrivateFile::with_content(
                Namefilter::new(),
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

    #[proptest(cases = 100)]
    fn can_include_and_stream_content_from_file(
        #[strategy(0..(MAX_BLOCK_CONTENT_SIZE * 2))] length: usize,
    ) {
        async_std::task::block_on(async {
            let content = vec![0u8; length];
            let store = &mut MemoryBlockStore::default();
            let rng = &mut TestRng::deterministic_rng(RngAlgorithm::ChaCha);
            let forest = &mut Rc::new(PrivateForest::new());

            let file = PrivateFile::with_content(
                Namefilter::new(),
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
            let store = &mut MemoryBlockStore::default();
            let rng = &mut TestRng::deterministic_rng(RngAlgorithm::ChaCha);
            let forest = &mut Rc::new(PrivateForest::new());

            let mut file = PrivateFile::new(Namefilter::new(), Utc::now(), rng);

            file.set_content(
                Utc::now(),
                &mut Cursor::new(vec![5u8; length]),
                forest,
                &mut MemoryBlockStore::default(),
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

            let forest = &mut Rc::new(PrivateForest::new());
            let store = &mut MemoryBlockStore::new();
            let rng = &mut TestRng::deterministic_rng(RngAlgorithm::ChaCha);

            let file = PrivateFile::with_content_streaming(
                Namefilter::default(),
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
