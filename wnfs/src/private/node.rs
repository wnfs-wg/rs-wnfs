use super::{
    encrypted::Encrypted, hamt::Hasher, namefilter::Namefilter, AesKey, PrivateDirectory,
    PrivateFile, PrivateForest, PrivateRef,
};
use crate::{utils, BlockStore, FsError, HashOutput, Id, NodeType, HASH_BYTE_SIZE};
use anyhow::{bail, Result};
use async_recursion::async_recursion;
use chrono::{DateTime, Utc};
use libipld::{cbor::DagCborCodec, prelude::Decode, Cid, Ipld};
use rand_core::RngCore;
use serde::{Deserialize, Serialize};
use sha3::Sha3_256;
use skip_ratchet::{seek::JumpSize, Ratchet, RatchetSeeker};
use std::{cmp::Ordering, collections::BTreeSet, fmt::Debug, io::Cursor, rc::Rc};

//--------------------------------------------------------------------------------------------------
// Type Definitions
//--------------------------------------------------------------------------------------------------

pub type INumber = HashOutput;

/// Represents a node in the WNFS private file system. This can either be a file or a directory.
///
/// # Examples
///
/// ```
/// use wnfs::{PrivateDirectory, PrivateNode, Namefilter};
/// use chrono::Utc;
/// use std::rc::Rc;
/// use rand::thread_rng;
///
/// let rng = &mut thread_rng();
/// let dir = Rc::new(PrivateDirectory::new(
///     Namefilter::default(),
///     Utc::now(),
///     rng,
/// ));
///
/// let node = PrivateNode::Dir(dir);
///
/// println!("Node: {:?}", node);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum PrivateNode {
    File(Rc<PrivateFile>),
    Dir(Rc<PrivateDirectory>),
}

/// The key used to encrypt the content of a node.
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct ContentKey(pub AesKey);

/// The key used to encrypt the header section of a node.
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct RevisionKey(pub AesKey);

/// The two types of keys.
#[derive(Debug, Serialize, Deserialize)]
pub enum KeyType {
    Temporal(ContentKey),
    Snapshot(RevisionKey),
}

/// This is the header of a private node. It contains secret information about the node which includes
/// the inumber, the ratchet, and the namefilter.
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
/// println!("Header: {:?}", file.header);
/// ```
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PrivateNodeHeader {
    /// A unique identifier of the node.
    pub(crate) inumber: INumber,
    /// Used both for versioning and deriving keys for that enforces privacy.
    pub(crate) ratchet: Ratchet,
    /// Used for ancestry checks and as a key for the private forest.
    pub(crate) bare_name: Namefilter,
}

//--------------------------------------------------------------------------------------------------
// Implementations
//--------------------------------------------------------------------------------------------------

impl PrivateNode {
    /// Creates node with upserted modified time.
    ///
    /// # Examples
    ///
    /// ```
    /// use wnfs::{PrivateDirectory, PrivateNode, Namefilter};
    /// use chrono::{Utc, Duration, TimeZone};
    /// use std::rc::Rc;
    /// use rand::thread_rng;
    ///
    /// let rng = &mut thread_rng();
    /// let dir = Rc::new(PrivateDirectory::new(
    ///     Namefilter::default(),
    ///     Utc::now(),
    ///     rng,
    /// ));
    /// let node = PrivateNode::Dir(dir);
    ///
    /// let time = Utc::now() + Duration::days(1);
    /// let node = node.upsert_mtime(time);
    ///
    /// let imprecise_time = Utc.timestamp_opt(time.timestamp(), 0).single();
    /// assert_eq!(
    ///     imprecise_time,
    ///     node.as_dir()
    ///         .unwrap()
    ///         .get_metadata()
    ///         .get_modified()
    /// );
    /// ```
    pub fn upsert_mtime(&self, time: DateTime<Utc>) -> Self {
        match self {
            Self::File(file) => {
                let mut file = (**file).clone();
                file.metadata.upsert_mtime(time);
                Self::File(Rc::new(file))
            }
            Self::Dir(dir) => {
                let mut dir = (**dir).clone();
                dir.metadata.upsert_mtime(time);
                Self::Dir(Rc::new(dir))
            }
        }
    }

    /// Updates bare name ancestry of private sub tree.
    #[async_recursion(?Send)]
    pub(crate) async fn update_ancestry(
        &mut self,
        parent_bare_name: Namefilter,
        mut forest: Rc<PrivateForest>,
        store: &mut impl BlockStore,
        rng: &mut impl RngCore,
    ) -> Result<Rc<PrivateForest>> {
        match self {
            Self::File(file) => {
                let mut file = (**file).clone();

                forest = file
                    .prepare_key_rotation(parent_bare_name, forest, store, rng)
                    .await?;

                *self = Self::File(Rc::new(file));
            }
            Self::Dir(old_dir) => {
                let mut dir = (**old_dir).clone();

                for (name, private_ref) in &old_dir.entries {
                    let mut node = forest
                        .get(private_ref, PrivateForest::resolve_lowest, store)
                        .await?
                        .ok_or(FsError::NotFound)?;

                    forest = node
                        .update_ancestry(dir.header.bare_name.clone(), forest, store, rng)
                        .await?;

                    dir.entries
                        .insert(name.clone(), node.get_header().derive_private_ref());
                }

                dir.prepare_key_rotation(parent_bare_name, rng);

                *self = Self::Dir(Rc::new(dir));
            }
        };

        let header = self.get_header();

        forest
            .put(
                header.get_saturated_name(),
                &header.derive_private_ref(),
                self,
                store,
                rng,
            )
            .await
    }

    /// Gets the header of the node.
    ///
    /// # Examples
    ///
    /// ```
    /// use wnfs::{PrivateDirectory, PrivateNode, Namefilter};
    /// use chrono::Utc;
    /// use std::rc::Rc;
    /// use rand::thread_rng;
    ///
    /// let rng = &mut thread_rng();
    /// let dir = Rc::new(PrivateDirectory::new(
    ///     Namefilter::default(),
    ///     Utc::now(),
    ///     rng,
    /// ));
    /// let node = PrivateNode::Dir(Rc::clone(&dir));
    ///
    /// assert_eq!(&dir.header, node.get_header());
    /// ```
    #[inline]
    pub fn get_header(&self) -> &PrivateNodeHeader {
        match self {
            Self::File(file) => &file.header,
            Self::Dir(dir) => &dir.header,
        }
    }

    /// Gets the previous links of the node.
    ///
    /// The previous links are encrypted with the previous revision's
    /// revision key, so you need to know an 'older' revision of the
    /// skip ratchet to decrypt these.
    ///
    /// The previous links is exactly one Cid in most cases and refers
    /// to the ciphertext Cid from the previous revision that this
    /// node is an update of.
    ///
    /// If this node is a merge-node, it has two or more previous Cids.
    /// A single previous Cid must be from the previous revision, but all
    /// other Cids may appear in even older revisions.
    ///
    /// The previous links is `None`, it doesn't have previous Cids.
    /// The node is malformed if the previous links are `Some`, but
    /// the `BTreeSet` inside is empty.
    pub fn get_previous(&self) -> &Option<Encrypted<BTreeSet<Cid>>> {
        match self {
            Self::File(file) => &file.previous,
            Self::Dir(dir) => &dir.previous,
        }
    }

    /// Casts a node to a directory.
    ///
    /// # Examples
    ///
    /// ```
    /// use wnfs::{PrivateDirectory, PrivateNode, Namefilter};
    /// use chrono::Utc;
    /// use std::rc::Rc;
    /// use rand::thread_rng;
    ///
    /// let rng = &mut thread_rng();
    /// let dir = Rc::new(PrivateDirectory::new(
    ///     Namefilter::default(),
    ///     Utc::now(),
    ///     rng,
    /// ));
    /// let node = PrivateNode::Dir(Rc::clone(&dir));
    ///
    /// assert_eq!(node.as_dir().unwrap(), dir);
    /// ```
    pub fn as_dir(&self) -> Result<Rc<PrivateDirectory>> {
        Ok(match self {
            Self::Dir(dir) => Rc::clone(dir),
            _ => bail!(FsError::NotADirectory),
        })
    }

    /// Casts a node to a file.
    ///
    /// # Examples
    ///
    /// ```
    /// use wnfs::{PrivateFile, PrivateNode, Namefilter};
    /// use chrono::Utc;
    /// use std::rc::Rc;
    /// use rand::thread_rng;
    ///
    /// let rng = &mut thread_rng();
    /// let file = Rc::new(PrivateFile::new(
    ///     Namefilter::default(),
    ///     Utc::now(),
    ///     rng,
    /// ));
    /// let node = PrivateNode::File(Rc::clone(&file));
    ///
    /// assert_eq!(node.as_file().unwrap(), file);
    /// ```
    pub fn as_file(&self) -> Result<Rc<PrivateFile>> {
        Ok(match self {
            Self::File(file) => Rc::clone(file),
            _ => bail!(FsError::NotAFile),
        })
    }

    /// Returns true if underlying node is a directory.
    ///
    /// # Examples
    ///
    /// ```
    /// use wnfs::{PrivateDirectory, PrivateNode, Namefilter};
    /// use chrono::Utc;
    /// use std::rc::Rc;
    /// use rand::thread_rng;
    ///
    /// let rng = &mut thread_rng();
    /// let dir = Rc::new(PrivateDirectory::new(
    ///     Namefilter::default(),
    ///     Utc::now(),
    ///     rng,
    /// ));
    /// let node = PrivateNode::Dir(dir);
    ///
    /// assert!(node.is_dir());
    /// ```
    pub fn is_dir(&self) -> bool {
        matches!(self, Self::Dir(_))
    }

    /// Returns true if the underlying node is a file.
    ///
    /// # Examples
    ///
    /// ```
    /// use wnfs::{PrivateFile, PrivateNode, Namefilter};
    /// use chrono::Utc;
    /// use std::rc::Rc;
    /// use rand::thread_rng;
    ///
    /// let rng = &mut thread_rng();
    /// let file = Rc::new(PrivateFile::new(
    ///     Namefilter::default(),
    ///     Utc::now(),
    ///     rng,
    /// ));
    /// let node = PrivateNode::File(file);
    ///
    /// assert!(node.is_file());
    /// ```
    pub fn is_file(&self) -> bool {
        matches!(self, Self::File(_))
    }

    /// Gets the latest version of the node using exponential search.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::rc::Rc;
    /// use chrono::Utc;
    /// use rand::thread_rng;
    /// use wnfs::{
    ///     private::{PrivateForest, PrivateRef, PrivateNode},
    ///     BlockStore, MemoryBlockStore, Namefilter, PrivateDirectory, PrivateOpResult,
    /// };
    ///
    /// #[async_std::main]
    /// async fn main() {
    ///     let store = &mut MemoryBlockStore::default();
    ///     let rng = &mut thread_rng();
    ///     let forest = Rc::new(PrivateForest::new());
    ///
    ///     let PrivateOpResult { forest, root_dir: init_dir, .. } = PrivateDirectory::new_and_store(
    ///         Default::default(),
    ///         Utc::now(),
    ///         forest,
    ///         store,
    ///         rng
    ///     ).await.unwrap();
    ///
    ///     let PrivateOpResult { forest, root_dir, .. } = Rc::clone(&init_dir)
    ///         .mkdir(&["pictures".into(), "cats".into()], true, Utc::now(), forest, store, rng)
    ///         .await
    ///         .unwrap();
    ///
    ///     let latest_node = PrivateNode::Dir(init_dir).search_latest(&forest, store).await.unwrap();
    ///
    ///     let found_node = latest_node
    ///         .as_dir()
    ///         .unwrap()
    ///         .lookup_node("pictures", true, &forest, store)
    ///         .await
    ///         .unwrap();
    ///
    ///     assert!(found_node.is_some());
    /// }
    /// ```
    pub async fn search_latest(
        &self,
        forest: &PrivateForest,
        store: &impl BlockStore,
    ) -> Result<PrivateNode> {
        let header = self.get_header();

        let private_ref = &header.derive_private_ref();
        if !forest.has(&private_ref.saturated_name_hash, store).await? {
            return Ok(self.clone());
        }

        // Start an exponential search, starting with a small jump.
        // In many cases, we'll be at the latest revision already, so we only
        // do a single lookup to the next version, most likely realize it's not
        // there and thus stop seeking.
        let mut search = RatchetSeeker::new(header.ratchet.clone(), JumpSize::Small);
        let mut current_header = header.clone();

        loop {
            let current = search.current();
            current_header.ratchet = current.clone();

            let has_curr = forest
                .has(
                    &current_header.derive_private_ref().saturated_name_hash,
                    store,
                )
                .await?;

            let ord = if has_curr {
                Ordering::Less
            } else {
                Ordering::Greater
            };

            if !search.step(ord) {
                break;
            }
        }

        current_header.ratchet = search.current().clone();

        let latest_private_ref = current_header.derive_private_ref();

        match forest
            .get(&latest_private_ref, PrivateForest::resolve_lowest, store)
            .await?
        {
            Some(node) => Ok(node),
            None => unreachable!(),
        }
    }

    pub(crate) async fn load(
        cid: Cid,
        private_ref: &PrivateRef,
        store: &impl BlockStore,
    ) -> Result<PrivateNode> {
        // Fetch encrypted bytes from blockstore.
        let enc_bytes = store.get_block(&cid).await?;

        // Decrypt bytes
        let cbor_bytes = private_ref.content_key.0.decrypt(&enc_bytes)?;

        // Deserialize
        PrivateNode::deserialize_from_cbor(&cbor_bytes, &private_ref.revision_key, cid)
    }

    pub(crate) async fn store(
        &self,
        store: &mut impl BlockStore,
        rng: &mut impl RngCore,
    ) -> Result<Cid> {
        match self {
            PrivateNode::File(file) => file.store(store, rng).await,
            PrivateNode::Dir(dir) => dir.store(store, rng).await,
        }
    }

    /// Deserializes the node from dag-cbor bytes.
    pub(crate) fn deserialize_from_cbor(
        bytes: &[u8],
        key: &RevisionKey,
        from_cid: Cid,
    ) -> Result<Self> {
        let ipld = Ipld::decode(DagCborCodec, &mut Cursor::new(bytes))?;
        (ipld, key, from_cid).try_into()
    }
}

impl TryFrom<(Ipld, &RevisionKey, Cid)> for PrivateNode {
    type Error = anyhow::Error;

    fn try_from(triple: (Ipld, &RevisionKey, Cid)) -> Result<Self> {
        match triple {
            (Ipld::Map(map), key, from_cid) => {
                let r#type: NodeType = map
                    .get("type")
                    .ok_or(FsError::MissingNodeType)?
                    .try_into()?;

                Ok(match r#type {
                    NodeType::PrivateFile => {
                        PrivateNode::from(PrivateFile::deserialize(Ipld::Map(map), key, from_cid)?)
                    }
                    NodeType::PrivateDirectory => PrivateNode::from(PrivateDirectory::deserialize(
                        Ipld::Map(map),
                        key,
                        from_cid,
                    )?),
                    other => bail!(FsError::UnexpectedNodeType(other)),
                })
            }
            other => bail!("Expected `Ipld::Map` got {:#?}", other),
        }
    }
}

impl Id for PrivateNode {
    fn get_id(&self) -> String {
        match self {
            Self::File(file) => file.get_id(),
            Self::Dir(dir) => dir.get_id(),
        }
    }
}

impl From<PrivateFile> for PrivateNode {
    fn from(file: PrivateFile) -> Self {
        Self::File(Rc::new(file))
    }
}

impl From<PrivateDirectory> for PrivateNode {
    fn from(dir: PrivateDirectory) -> Self {
        Self::Dir(Rc::new(dir))
    }
}

impl PrivateNodeHeader {
    /// Creates a new PrivateNodeHeader.
    pub(crate) fn new(parent_bare_name: Namefilter, rng: &mut impl RngCore) -> Self {
        let inumber = crate::utils::get_random_bytes::<HASH_BYTE_SIZE>(rng);
        let ratchet_seed = crate::utils::get_random_bytes::<HASH_BYTE_SIZE>(rng);

        Self {
            bare_name: {
                let mut namefilter = parent_bare_name;
                namefilter.add(&inumber);
                namefilter
            },
            ratchet: Ratchet::zero(ratchet_seed),
            inumber,
        }
    }

    /// Creates a new PrivateNodeHeader with provided seed.
    pub(crate) fn with_seed(
        parent_bare_name: Namefilter,
        ratchet_seed: HashOutput,
        inumber: HashOutput,
    ) -> Self {
        Self {
            bare_name: {
                let mut namefilter = parent_bare_name;
                namefilter.add(&inumber);
                namefilter
            },
            ratchet: Ratchet::zero(ratchet_seed),
            inumber,
        }
    }

    /// Advances the ratchet.
    pub(crate) fn advance_ratchet(&mut self) {
        self.ratchet.inc();
    }

    /// Updates the bare name of the node.
    pub(crate) fn update_bare_name(&mut self, parent_bare_name: Namefilter) {
        self.bare_name = {
            let mut namefilter = parent_bare_name;
            namefilter.add(&self.inumber);
            namefilter
        };
    }

    /// Resets the ratchet.
    pub(crate) fn reset_ratchet(&mut self, rng: &mut impl RngCore) {
        self.ratchet = Ratchet::zero(utils::get_random_bytes(rng))
    }

    /// Derives the private ref of the current header.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::rc::Rc;
    /// use wnfs::{PrivateFile, Namefilter, Id};
    /// use chrono::Utc;
    /// use rand::thread_rng;
    ///
    /// let rng = &mut thread_rng();
    /// let file = Rc::new(PrivateFile::new(
    ///     Namefilter::default(),
    ///     Utc::now(),
    ///     rng,
    /// ));
    /// let private_ref = file.header.derive_private_ref();
    ///
    /// println!("Private ref: {:?}", private_ref);
    /// ```
    pub fn derive_private_ref(&self) -> PrivateRef {
        let revision_key = self.derive_revision_key();
        let saturated_name_hash = Sha3_256::hash(&self.get_saturated_name_with_key(&revision_key));

        PrivateRef {
            saturated_name_hash,
            content_key: AesKey::new(Sha3_256::hash(&revision_key.0.as_bytes())).into(),
            revision_key,
        }
    }

    /// Derives the revision key.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::rc::Rc;
    /// use wnfs::{PrivateFile, Namefilter, Id};
    /// use chrono::Utc;
    /// use rand::thread_rng;
    ///
    /// let rng = &mut thread_rng();
    /// let file = Rc::new(PrivateFile::new(
    ///     Namefilter::default(),
    ///     Utc::now(),
    ///     rng,
    /// ));
    /// let revision_key = file.header.derive_revision_key();
    ///
    /// println!("Revision Key: {:?}", revision_key);
    /// ```
    #[inline]
    pub fn derive_revision_key(&self) -> RevisionKey {
        AesKey::new(self.ratchet.derive_key()).into()
    }

    /// Derives the content key.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::rc::Rc;
    /// use wnfs::{PrivateFile, Namefilter, Id};
    /// use chrono::Utc;
    /// use rand::thread_rng;
    ///
    /// let rng = &mut thread_rng();
    /// let file = Rc::new(PrivateFile::new(
    ///     Namefilter::default(),
    ///     Utc::now(),
    ///     rng,
    /// ));
    /// let content_key = file.header.derive_content_key();
    ///
    /// println!("Content Key: {:?}", content_key);
    /// ```
    #[inline]
    pub fn derive_content_key(&self) -> ContentKey {
        AesKey::new(Sha3_256::hash(&self.ratchet.derive_key())).into()
    }

    /// Gets the saturated namefilter for this node using the provided ratchet key.
    pub(crate) fn get_saturated_name_with_key(&self, revision_key: &RevisionKey) -> Namefilter {
        let mut name = self.bare_name.clone();
        name.add(&revision_key.0.as_bytes());
        name.saturate();
        name
    }

    /// Gets the saturated namefilter for this node.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::rc::Rc;
    /// use wnfs::{PrivateFile, Namefilter, private::AesKey};
    /// use chrono::Utc;
    /// use rand::thread_rng;
    ///
    /// let rng = &mut thread_rng();
    /// let file = Rc::new(PrivateFile::new(
    ///     Namefilter::default(),
    ///     Utc::now(),
    ///     rng,
    /// ));
    /// let saturated_name = file.header.get_saturated_name();
    ///
    /// println!("Saturated name: {:?}", saturated_name);
    /// ```
    #[inline]
    pub fn get_saturated_name(&self) -> Namefilter {
        let revision_key = self.derive_revision_key();
        self.get_saturated_name_with_key(&revision_key)
    }
}

impl Debug for PrivateNodeHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut inumber_str = String::from("0x");
        for byte in self.inumber {
            inumber_str.push_str(&format!("{byte:02X}"));
        }

        f.debug_struct("PrivateRef")
            .field("inumber", &inumber_str)
            .field("ratchet", &self.ratchet)
            .field("bare_name", &self.bare_name)
            .finish()
    }
}

impl From<AesKey> for RevisionKey {
    fn from(key: AesKey) -> Self {
        Self(key)
    }
}

impl From<[u8; 32]> for RevisionKey {
    fn from(key: [u8; 32]) -> Self {
        Self(AesKey::new(key))
    }
}

impl From<&Ratchet> for RevisionKey {
    fn from(ratchet: &Ratchet) -> Self {
        Self::from(AesKey::new(ratchet.derive_key()))
    }
}

impl From<AesKey> for ContentKey {
    fn from(key: AesKey) -> Self {
        Self(key)
    }
}

impl From<ContentKey> for AesKey {
    fn from(key: ContentKey) -> Self {
        key.0
    }
}

impl RevisionKey {
    pub fn derive_content_key(&self) -> ContentKey {
        let RevisionKey(key) = self;
        ContentKey(AesKey::new(Sha3_256::hash(&key.as_bytes())))
    }
}

//--------------------------------------------------------------------------------------------------
// Tests
//--------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use proptest::test_runner::{RngAlgorithm, TestRng};

    use crate::MemoryBlockStore;

    use super::*;

    #[async_std::test]
    async fn serialized_private_node_can_be_deserialized() {
        let rng = &mut TestRng::deterministic_rng(RngAlgorithm::ChaCha);
        let content = b"Lorem ipsum dolor sit amet";
        let forest = Rc::new(PrivateForest::new());
        let store = &mut MemoryBlockStore::new();

        let (file, _) = PrivateFile::with_content(
            Namefilter::default(),
            Utc::now(),
            content.to_vec(),
            forest,
            store,
            rng,
        )
        .await
        .unwrap();

        let file = PrivateNode::File(Rc::new(file));
        let private_ref = file.get_header().derive_private_ref();
        let cid = file.store(store, rng).await.unwrap();

        let deserialized_node = PrivateNode::load(cid, &private_ref, store).await.unwrap();

        assert_eq!(file, deserialized_node);
    }
}
