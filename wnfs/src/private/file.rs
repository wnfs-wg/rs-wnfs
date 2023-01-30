use super::{
    encrypted::Encrypted, namefilter::Namefilter, AesKey, ContentKey, PrivateForest,
    PrivateNodeHeader, RevisionKey, AUTHENTICATION_TAG_SIZE, NONCE_SIZE,
};
use crate::{
    dagcbor, utils, utils::get_random_bytes, BlockStore, FsError, Hasher, Id, Metadata, NodeType,
    MAX_BLOCK_SIZE,
};
use anyhow::Result;
use async_once_cell::OnceCell;
use async_stream::try_stream;
use chrono::{DateTime, Utc};
use futures::{Stream, StreamExt};
use libipld::{cbor::DagCborCodec, prelude::Encode, Cid, IpldCodec};
use rand_core::RngCore;
use semver::Version;
use serde::{de::Error as DeError, ser::Error as SerError, Deserialize, Deserializer, Serialize};
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
///     let forest = Rc::new(PrivateForest::new());
///
///     let (file, _) = PrivateFile::with_content(
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
#[derive(Debug)]
pub struct PrivateFile {
    persisted_as: OnceCell<Cid>,
    pub header: PrivateNodeHeader,
    pub content: PrivateFileContent,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PrivateFileContent {
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
        key: ContentKey,
        block_count: usize,
        block_content_size: usize,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PrivateFileSerializable {
    pub r#type: NodeType,
    pub version: Version,
    pub header: Vec<u8>,
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
            persisted_as: OnceCell::new(),
            header: PrivateNodeHeader::new(parent_bare_name, rng),
            content: PrivateFileContent {
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
    ///     let forest = Rc::new(PrivateForest::new());
    ///
    ///     let (file, _) = PrivateFile::with_content(
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
        forest: Rc<PrivateForest>,
        store: &mut impl BlockStore,
        rng: &mut impl RngCore,
    ) -> Result<(Self, Rc<PrivateForest>)> {
        let header = PrivateNodeHeader::new(parent_bare_name, rng);
        let (content, forest) =
            Self::prepare_content(&header.bare_name, content, forest, store, rng).await?;

        Ok((
            Self {
                persisted_as: OnceCell::new(),
                header,
                content: PrivateFileContent {
                    metadata: Metadata::new(time),
                    previous: BTreeSet::new(),
                    content,
                },
            },
            forest,
        ))
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
    /// use futures::{StreamExt};
    ///
    /// #[async_std::main]
    /// async fn main() {
    ///     let store = &mut MemoryBlockStore::default();
    ///     let rng = &mut thread_rng();
    ///     let forest = Rc::new(PrivateForest::new());
    ///
    ///     let content = get_random_bytes::<100>(rng).to_vec();
    ///     let (file, forest) = PrivateFile::with_content(
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
    ///     let mut stream = file.stream_content(0, &forest, store);
    ///     while let Some(block) = stream.next().await {
    ///         stream_content.extend_from_slice(&block.unwrap());
    ///     }
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
    ///     let forest = Rc::new(PrivateForest::new());
    ///
    ///     let content = get_random_bytes::<100>(rng).to_vec();
    ///     let (file, forest) = PrivateFile::with_content(
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
    ///     let mut all_content = file.get_content(&forest, store).await.unwrap();
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
        let mut stream = self.stream_content(0, forest, store);
        while let Some(bytes) = stream.next().await {
            content.extend_from_slice(&bytes?);
        }
        Ok(content)
    }

    /// Determines where to put the content of a file. This can either be inline or stored up in chunks in a private forest.
    pub(super) async fn prepare_content(
        bare_name: &Namefilter,
        content: Vec<u8>,
        mut forest: Rc<PrivateForest>,
        store: &mut impl BlockStore,
        rng: &mut impl RngCore,
    ) -> Result<(FileContent, Rc<PrivateForest>)> {
        // TODO(appcypher): Use a better heuristic to determine when to use external storage.
        let key = ContentKey(AesKey::new(get_random_bytes(rng)));
        let block_count = (content.len() as f64 / MAX_BLOCK_CONTENT_SIZE as f64).ceil() as usize;

        for (index, label) in
            Self::generate_shard_labels(&key, 0, block_count, bare_name).enumerate()
        {
            let start = index * MAX_BLOCK_CONTENT_SIZE;
            let end = content.len().min((index + 1) * MAX_BLOCK_CONTENT_SIZE);
            let slice = &content[start..end];

            let enc_bytes = key.encrypt(slice, rng)?;
            let content_cid = store.put_block(enc_bytes, IpldCodec::Raw).await?;

            forest = forest.put_encrypted(label, content_cid, store).await?;
        }

        Ok((
            FileContent::External {
                key,
                block_count,
                block_content_size: MAX_BLOCK_CONTENT_SIZE,
            },
            forest,
        ))
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
        key: &ContentKey,
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
        key: &'a ContentKey,
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
    fn create_block_label(key: &ContentKey, index: usize, bare_name: &Namefilter) -> Namefilter {
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
        let cid = self.store(store, rng).await?;

        let mut cloned = Rc::try_unwrap(self).unwrap_or_else(|rc| (*rc).clone());
        let revision_key = cloned.header.derive_private_ref().revision_key;

        cloned.persisted_as = OnceCell::new(); // Also done in `.clone()`, but need this to work in case try_unwrap optimizes.
        cloned.content.previous.clear();
        cloned
            .content
            .previous
            .insert((1, Encrypted::from_value(cid, &revision_key)?));

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
        forest: Rc<PrivateForest>,
        store: &mut impl BlockStore,
        rng: &mut impl RngCore,
    ) -> Result<Rc<PrivateForest>> {
        let content = self.get_content(&forest, store).await?;

        self.header.inumber = utils::get_random_bytes(rng);
        self.header.update_bare_name(parent_bare_name);
        self.header.reset_ratchet(rng);
        self.persisted_as = OnceCell::new();

        let (content, forest) =
            Self::prepare_content(&self.header.bare_name, content, forest, store, rng).await?;
        self.content.content = content;

        Ok(forest)
    }

    /// Serializes the file with provided Serde serialilzer.
    pub(crate) fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let key = self.header.derive_private_ref().revision_key;

        (PrivateFileSerializable {
            r#type: NodeType::PrivateFile,
            version: Version::new(0, 2, 0),
            header: {
                let cbor_bytes = dagcbor::encode(&self.header).map_err(SerError::custom)?;
                key.key_wrap_encrypt(&cbor_bytes)
                    .map_err(SerError::custom)?
            },
            previous: self.content.previous.iter().cloned().collect(),
            metadata: self.content.metadata.clone(),
            content: self.content.content.clone(),
        })
        .serialize(serializer)
    }

    /// Deserializes the file with provided Serde deserializer and key.
    pub(crate) fn deserialize<'de, D>(
        deserializer: D,
        key: &RevisionKey,
        from_cid: Cid,
    ) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let PrivateFileSerializable {
            version,
            metadata,
            header,
            previous,
            content,
            ..
        } = PrivateFileSerializable::deserialize(deserializer)?;

        if version.major != 0 || version.minor != 2 {
            return Err(DeError::custom(FsError::InvalidDeserialization(format!(
                "Couldn't deserialize file: Expected version 0.2.0 but got {}",
                version.to_string()
            ))));
        }

        Ok(Self {
            persisted_as: OnceCell::new_with(Some(from_cid)),
            header: {
                let cbor_bytes = key.key_wrap_decrypt(&header).map_err(DeError::custom)?;
                dagcbor::decode(&cbor_bytes).map_err(DeError::custom)?
            },
            content: PrivateFileContent {
                previous: previous.into_iter().collect(),
                metadata,
                content,
            },
        })
    }

    pub(crate) async fn store(
        &self,
        store: &mut impl BlockStore,
        rng: &mut impl RngCore,
    ) -> Result<Cid> {
        let cid = self
            .persisted_as
            .get_or_try_init::<anyhow::Error>(async {
                // TODO(matheus23) deduplicate when reworking serialization
                let private_ref = &self.header.derive_private_ref();

                // Serialize node to cbor.
                let ipld = self.serialize(libipld::serde::Serializer)?;
                let mut bytes = Vec::new();
                ipld.encode(DagCborCodec, &mut bytes)?;

                // Encrypt bytes with content key.
                let enc_bytes = private_ref
                    .revision_key
                    .derive_content_key()
                    .encrypt(&bytes, rng)?;

                // Store content section in blockstore and get Cid.
                store.put_block(enc_bytes, libipld::IpldCodec::Raw).await
            })
            .await?;

        Ok(*cid)
    }
}

impl PartialEq for PrivateFile {
    fn eq(&self, other: &Self) -> bool {
        self.header == other.header && self.content == other.content
    }
}

impl Clone for PrivateFile {
    fn clone(&self) -> Self {
        Self {
            persisted_as: OnceCell::new(),
            header: self.header.clone(),
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
    use crate::utils::test_setup;
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

        let (file, (ref forest, ref store, _)) = test_setup::private!(file, content.clone());

        let mut collected_content = Vec::new();
        let mut stream = file.stream_content(2, forest, store);
        let mut block_limit = 2;
        while let Some(chunk) = stream.next().await {
            if block_limit == 0 {
                break;
            }

            collected_content.extend_from_slice(&chunk.unwrap());
            block_limit -= 1;
        }

        assert_eq!(
            collected_content,
            content[2 * MAX_BLOCK_CONTENT_SIZE..4 * MAX_BLOCK_CONTENT_SIZE]
        );
    }
}

#[cfg(test)]
mod proptests {
    use super::MAX_BLOCK_CONTENT_SIZE;
    use crate::utils::test_setup;
    use futures::StreamExt;
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
            let mut stream = file.stream_content(0, forest, store);
            while let Some(chunk) = stream.next().await {
                collected_content.extend_from_slice(&chunk.unwrap());
            }

            assert_eq!(collected_content, content);
        })
    }
}
