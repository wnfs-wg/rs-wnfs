use super::{
    encrypted::Encrypted, namefilter::Namefilter, AesKey, PrivateForest, PrivateNodeHeader,
    SnapshotKey, AUTHENTICATION_TAG_SIZE, NONCE_SIZE,
};
use crate::{
    utils, utils::get_random_bytes, BlockStore, FsError, Hasher, Id, Metadata, NodeType,
    PrivateNode, MAX_BLOCK_SIZE,
};
use anyhow::Result;
use async_once_cell::OnceCell;
use async_stream::try_stream;
use chrono::{DateTime, Utc};
use futures::{future, AsyncRead, Stream, StreamExt};
use libipld::{cbor::DagCborCodec, prelude::Encode, Cid, IpldCodec};
use rand_core::RngCore;
use semver::Version;
use serde::{de::Error as DeError, Deserialize, Deserializer, Serialize};
use sha3::Sha3_256;
use std::{collections::BTreeSet, iter, rc::Rc};

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
///     private::{PrivateForest, PrivateRef},
///     MemoryBlockStore, Namefilter, PrivateFile,
///     utils::get_random_bytes,
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
    pub content: PrivateFileContent,
}

#[derive(Debug)]
pub struct PrivateFileContent {
    persisted_as: OnceCell<Cid>,
    pub previous: BTreeSet<(usize, Encrypted<Cid>)>,
    pub metadata: Metadata,
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
    /// use wnfs::{PrivateFile, Namefilter, Id};
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
    ///     private::{PrivateForest, PrivateRef},
    ///     MemoryBlockStore, Namefilter, PrivateFile,
    ///     utils::get_random_bytes, MAX_BLOCK_SIZE
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
        store: &mut impl BlockStore,
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
    ///     private::{PrivateForest, PrivateRef},
    ///     MemoryBlockStore, Namefilter, PrivateFile,
    ///     MAX_BLOCK_SIZE
    /// };
    ///
    /// #[async_std::main]
    /// async fn main() {
    ///     let disk_file = File::open("./test/fixtures/Clara Schumann, Scherzo no. 2, Op. 14.mp3")
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
        store: &mut impl BlockStore,
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
    ///     private::{PrivateForest, PrivateRef},
    ///     MemoryBlockStore, Namefilter, PrivateFile,
    ///     utils::get_random_bytes,
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
    ///     private::{PrivateForest, PrivateRef},
    ///     MemoryBlockStore, Namefilter, PrivateFile,
    ///     utils::get_random_bytes,
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
            .for_each(|chunk| {
                content.extend_from_slice(&chunk.unwrap());
                future::ready(())
            })
            .await;
        Ok(content)
    }

    /// Determines where to put the content of a file. This can either be inline or stored up in chunks in a private forest.
    pub(super) async fn prepare_content(
        bare_name: &Namefilter,
        content: Vec<u8>,
        forest: &mut Rc<PrivateForest>,
        store: &mut impl BlockStore,
        rng: &mut impl RngCore,
    ) -> Result<FileContent> {
        // TODO(appcypher): Use a better heuristic to determine when to use external storage.
        let key = SnapshotKey(AesKey::new(get_random_bytes(rng)));
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
        store: &mut impl BlockStore,
        rng: &mut impl RngCore,
    ) -> Result<FileContent> {
        let key = SnapshotKey(AesKey::new(get_random_bytes(rng)));

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
    pub(crate) fn get_content_size_upper_bound(&self) -> usize {
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
    /// It will store the current revision in the given `BlockStore` to
    /// retrieve its CID and put that into the `previous` links,
    /// as well as advancing the ratchet and resetting the `persisted_as` pointer.
    pub(crate) async fn prepare_next_revision(
        self: Rc<Self>,
        store: &mut impl BlockStore,
        rng: &mut impl RngCore,
    ) -> Result<Self> {
        let temporal_key = self.header.derive_temporal_key();
        let snapshot_key = temporal_key.derive_snapshot_key();
        let header_cid = self.header.store(store).await?;
        let content_cid = self
            .content
            .store(header_cid, &snapshot_key, store, rng)
            .await?;

        let mut cloned = Rc::try_unwrap(self).unwrap_or_else(|rc| (*rc).clone());

        cloned.content.persisted_as = OnceCell::new(); // Also done in `.clone()`, but need this to work in case try_unwrap optimizes.
        cloned.content.previous.clear();
        cloned
            .content
            .previous
            .insert((1, Encrypted::from_value(content_cid, &temporal_key)?));

        cloned.header.advance_ratchet();

        Ok(cloned)
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
        store: &mut impl BlockStore,
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

    pub(crate) async fn store(
        &self,
        store: &mut impl BlockStore,
        rng: &mut impl RngCore,
    ) -> Result<(Cid, Cid)> {
        let header_cid = self.header.store(store).await?;

        let snapshot_key = self.header.derive_snapshot_key();

        let content_cid = self
            .content
            .store(header_cid, &snapshot_key, store, rng)
            .await?;

        Ok((header_cid, content_cid))
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
        store: &mut impl BlockStore,
        rng: &mut impl RngCore,
    ) -> Result<Cid> {
        Ok(*self
            .persisted_as
            .get_or_try_init::<anyhow::Error>(async {
                // TODO(matheus23) deduplicate when reworking serialization

                // Serialize node to cbor.
                let ipld = self.serialize(libipld::serde::Serializer, header_cid)?;
                let mut bytes = Vec::new();
                ipld.encode(DagCborCodec, &mut bytes)?;

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
    use crate::{utils::test_setup, MemoryBlockStore};
    use async_std::fs::File;
    use proptest::test_runner::{RngAlgorithm, TestRng};
    use rand::Rng;

    #[async_std::test]
    async fn can_create_empty_file() {
        let (file, _) = test_setup::private!(file);
        let (ref forest, ref store) = test_setup::init!(forest, store);
        let file_content = file.get_content(forest, store).await.unwrap();

        assert!(file_content.is_empty());
    }

    #[async_std::test]
    async fn can_stream_limited_content_from_file() {
        let mut content = vec![0u8; MAX_BLOCK_CONTENT_SIZE * 5];
        rand::thread_rng().fill(&mut content[..]);

        let (file, (ref mut forest, ref store, _)) = test_setup::private!(file, content.clone());

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
    use crate::utils::test_setup;
    use futures::{future, StreamExt};
    use test_strategy::proptest;

    #[proptest(cases = 100)]
    fn can_include_and_get_content_from_file(
        #[strategy(0..(MAX_BLOCK_CONTENT_SIZE * 2))] length: usize,
    ) {
        async_std::task::block_on(async {
            let content = vec![0u8; length];
            let (file, (ref forest, ref store, _)) = test_setup::private!(file, content.clone());
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
            let (file, (ref forest, ref store, _)) = test_setup::private!(file, content.clone());

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
}
