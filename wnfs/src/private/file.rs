use super::{namefilter::Namefilter, Key, PrivateForest, PrivateNodeHeader};
use crate::{
    dagcbor, utils::get_random_bytes, BlockStore, FsError, Hasher, Id, Metadata, NodeType,
    MAX_BLOCK_SIZE,
};
use anyhow::Result;
use async_stream::try_stream;
use chrono::{DateTime, Utc};
use futures::{Stream, StreamExt};
use rand_core::RngCore;
use semver::Version;
use serde::{de::Error as DeError, ser::Error as SerError, Deserialize, Deserializer, Serialize};
use sha3::Sha3_256;
use std::{iter, rc::Rc};

//--------------------------------------------------------------------------------------------------
// Constants
//--------------------------------------------------------------------------------------------------



//--------------------------------------------------------------------------------------------------
// Type Definitions
//--------------------------------------------------------------------------------------------------

/// Represents a file in the WNFS private filesystem.
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
///     b"hello world".to_vec(),
///     rng,
/// );
///
/// println!("file = {:?}", file);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct PrivateFile {
    pub version: Version,
    pub header: PrivateNodeHeader,
    pub metadata: Metadata,
    pub(crate) content: FileContent,
}

/// TODO(appcypher): Document
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum FileContent {
    Inline {
        data: Vec<u8>,
    },
    External {
        key: Key,
        block_count: usize,
        block_content_size: usize,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PrivateFileSerializable {
    pub r#type: NodeType,
    pub version: Version,
    pub header: Vec<u8>,
    pub metadata: Metadata,
    pub content: Vec<u8>,
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
    /// let file = PrivateFile::empty(
    ///     Namefilter::default(),
    ///     Utc::now(),
    ///     rng,
    /// );
    ///
    /// println!("file = {:?}", file);
    /// ```
    pub async fn empty<R: RngCore>(
        parent_bare_name: Namefilter,
        time: DateTime<Utc>,
        rng: &mut R,
    ) -> Self {
        Self {
            version: Version::new(0, 2, 0),
            metadata: Metadata::new(time),
            header: PrivateNodeHeader::new(parent_bare_name, rng),
            content: FileContent::Inline { data: vec![] },
        }
    }

    /// Creates a file with specified content.
    ///
    /// # Examples
    ///
    /// ```
    /// use wnfs::{PrivateFile, Namefilter, Id};
    /// use chrono::Utc;
    /// use rand::thread_rng;
    ///
    /// let rng = &mut thread_rng();
    /// let file = PrivateFile::empty(
    ///     Namefilter::default(),
    ///     Utc::now(),
    ///     rng,
    /// );
    ///
    /// println!("file = {:?}", file);
    /// ```
    pub async fn with_content<B: BlockStore, R: RngCore>(
        parent_bare_name: Namefilter,
        time: DateTime<Utc>,
        content: Vec<u8>,
        hamt: Rc<PrivateForest>,
        store: &mut B,
        rng: &mut R,
    ) -> Result<(Self, Rc<PrivateForest>)> {
        let header = PrivateNodeHeader::new(parent_bare_name, rng);
        let (content, hamt) =
            Self::prepare_content(&header.bare_name, content, hamt, store, rng).await?;

        Ok((
            Self {
                version: Version::new(0, 2, 0),
                metadata: Metadata::new(time),
                header,
                content,
            },
            hamt,
        ))
    }

    /// TODO(appcypher): Document
    pub fn stream_content<'a, B: BlockStore>(
        &'a self,
        hamt: &'a Rc<PrivateForest>,
        store: &'a B,
    ) -> impl Stream<Item = Result<Vec<u8>>> + 'a {
        Box::pin(try_stream! {
            match &self.content {
                FileContent::Inline { data } => {
                    yield data.clone();
                },
                FileContent::External {
                    key,
                    block_count,
                    ..
                } => {
                    let bare_name = &self.header.bare_name;
                    for label in Self::generate_shard_labels(key, *block_count, bare_name) {
                        println!("stream label = {:?}", label);

                        let bytes = Self::decrypt_block(key, &label, hamt, store).await?;
                        yield bytes
                    }
                }
            }
        })
    }

    /// TODO(appcypher): Document
    pub async fn get_content<B: BlockStore>(
        &self,
        hamt: &Rc<PrivateForest>,
        store: &B,
    ) -> Result<Vec<u8>> {
        let mut content = Vec::with_capacity(self.get_max_content_size());
        while let Some(bytes) = self.stream_content(hamt, store).next().await {
            content.extend_from_slice(&bytes?);
        }
        Ok(content)
    }

    /// TODO(appcypher): Document
    pub async fn get_block_at<B: BlockStore>(
        &self,
        index: usize,
        hamt: &Rc<PrivateForest>,
        store: &mut B,
    ) -> Result<Option<Vec<u8>>> {
        if let FileContent::External {
            key, block_count, ..
        } = &self.content
        {
            if index >= *block_count {
                return Ok(None);
            }

            let label = Self::create_block_label(key, *block_count, &self.header.bare_name);
            let bytes = Self::decrypt_block(key, &label, hamt, store).await?;
            return Ok(Some(bytes));
        }

        Ok(None)
    }

    /// TODO(appcypher): Document
    pub(crate) async fn prepare_content<B: BlockStore, R: RngCore>(
        bare_name: &Namefilter,
        content: Vec<u8>,
        mut hamt: Rc<PrivateForest>,
        store: &mut B,
        rng: &mut R,
    ) -> Result<(FileContent, Rc<PrivateForest>)> {
        // TODO(appcypher): Use a better heuristic to determine when to use external storage.
        if content.len() <= MAX_INLINE_CONTENT_SIZE {
            return Ok((FileContent::Inline { data: content }, hamt));
        }

        let key = Key(get_random_bytes(rng));
        let block_count = (content.len() as f64 / MAX_BLOCK_CONTENT_SIZE as f64).ceil() as usize;

        for (index, label) in Self::generate_shard_labels(&key, block_count, bare_name).enumerate()
        {
            println!("index = {}", index);
            println!("label = {:?}", label);

            let start = index * MAX_BLOCK_CONTENT_SIZE;
            let end = content.len().min((index + 1) * MAX_BLOCK_CONTENT_SIZE);
            let slice = &content[start..end];

            println!("slice = {:?}", slice);

            let enc_bytes = key.encrypt(&Key::generate_nonce(rng), slice)?;
            let content_cid = store.put_block(enc_bytes, libipld::IpldCodec::Raw).await?;

            hamt = hamt.put_encrypted(label, content_cid, store).await?;
        }

        Ok((
            FileContent::External {
                key,
                block_count,
                block_content_size: MAX_BLOCK_CONTENT_SIZE,
            },
            hamt,
        ))
    }

    /// TODO(appcypher): Document
    fn get_max_content_size(&self) -> usize {
        match &self.content {
            FileContent::Inline { data } => data.len(),
            FileContent::External {
                block_count,
                block_content_size,
                ..
            } => block_count * block_content_size,
        }
    }

    /// TODO(appcypher): Document
    async fn decrypt_block<B: BlockStore>(
        key: &Key,
        label: &Namefilter,
        hamt: &Rc<PrivateForest>,
        store: &B,
    ) -> Result<Vec<u8>> {
        let label_hash = &Sha3_256::hash(&label.as_bytes());

        let cids = hamt
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

    /// TODO(appcypher): Document
    fn generate_shard_labels<'a>(
        key: &'a Key,
        mut block_count: usize,
        bare_name: &'a Namefilter,
    ) -> impl Iterator<Item = Namefilter> + 'a {
        iter::from_fn(move || {
            if block_count < 1 {
                return None;
            }

            let label = Self::create_block_label(key, block_count, bare_name);
            block_count -= 1;
            Some(label)
        })
    }

    /// TODO(appcypher): Document
    fn create_block_label(key: &Key, index: usize, bare_name: &Namefilter) -> Namefilter {
        println!("key = {:?}", key);
        let key_bytes = key.as_bytes();
        let key_hash = Sha3_256::hash(&[key_bytes, &index.to_le_bytes()[..]].concat());

        println!("key_hash = {:?}", key_hash);

        let mut label = bare_name.clone();
        label.add(&key_bytes);
        label.add(&key_hash);

        label
    }

    /// Serializes the file with provided Serde serialilzer.
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
            .revision_key;

        (PrivateFileSerializable {
            r#type: NodeType::PrivateFile,
            version: self.version.clone(),
            header: {
                let cbor_bytes = dagcbor::encode(&self.header).map_err(SerError::custom)?;
                key.0
                    .encrypt(&Key::generate_nonce(rng), &cbor_bytes)
                    .map_err(SerError::custom)?
            },
            metadata: self.metadata.clone(),
            content: self.content.clone(),
        })
        .serialize(serializer)
    }

    /// Deserializes the file with provided Serde deserializer and key.
    pub(crate) fn deserialize<'de, D>(deserializer: D, key: &RevisionKey) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let PrivateFileSerializable {
            version,
            metadata,
            header,
            content,
            ..
        } = PrivateFileSerializable::deserialize(deserializer)?;

        Ok(Self {
            version,
            metadata,
            header: {
                let cbor_bytes = key.0.decrypt(&header).map_err(DeError::custom)?;
                dagcbor::decode(&cbor_bytes).map_err(DeError::custom)?
            },
            content,
        })
    }
}

impl Id for PrivateFile {
    fn get_id(&self) -> String {
        format!("{:p}", &self.header)
    }
}

#[cfg(test)]
mod prop_tests {
    use super::*;
    use crate::utils::test_setup;
    use test_strategy::proptest;

    // #[proptest]
    #[async_std::test]
    async fn can_create_empty_file() {
        let _ = test_setup::init!(rng);
        let (file, _) = test_setup::private!(file, vec![]);

        println!("file = {:?}", file);
    }
}
