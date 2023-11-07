use super::{KeyValueChange, Node, HAMT_VERSION};
use crate::Hasher;
use anyhow::Result;
use async_trait::async_trait;
use libipld::{serde as ipld_serde, Ipld};
use semver::Version;
use serde::{
    de::{DeserializeOwned, Error as DeError},
    ser::Error as SerError,
    Deserialize, Deserializer, Serialize, Serializer,
};
use std::{collections::BTreeMap, hash::Hash, str::FromStr, sync::Arc};
use wnfs_common::{AsyncSerialize, BlockStore, Link};

//--------------------------------------------------------------------------------------------------
// Type Definitions
//--------------------------------------------------------------------------------------------------

/// Hash Array Mapped Trie (HAMT) is an implementation of an associative array that combines the characteristics
/// of a hash table and an array mapped trie.
///
/// This type wraps the actual implementation which can be found in the [`Node`](crate::Node).
///
/// # Examples
///
/// ```
/// use wnfs_hamt::Hamt;
///
/// let hamt = Hamt::<String, usize>::new();
/// println!("HAMT: {:?}", hamt);
/// ```
#[derive(Debug, Clone)]
pub struct Hamt<K, V, H = blake3::Hasher>
where
    H: Hasher,
{
    pub root: Arc<Node<K, V, H>>,
    pub version: Version,
}

//--------------------------------------------------------------------------------------------------
// Implementations
//--------------------------------------------------------------------------------------------------

impl<K, V, H: Hasher> Hamt<K, V, H> {
    /// Creates a new empty HAMT.
    ///
    /// # Examples
    ///
    /// ```
    /// use wnfs_hamt::Hamt;
    ///
    /// let hamt = Hamt::<String, usize>::new();
    /// println!("HAMT: {:?}", hamt);
    /// ```
    pub fn new() -> Self {
        Self {
            root: Arc::new(Node::default()),
            version: HAMT_VERSION,
        }
    }

    /// Creates a new `HAMT` with the given root node.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::sync::Arc;
    /// use wnfs_hamt::{Hamt, Node};
    ///
    /// let hamt = Hamt::<String, usize>::with_root(Arc::new(Node::default()));
    ///
    /// println!("HAMT: {:?}", hamt);
    /// ```
    pub fn with_root(root: Arc<Node<K, V, H>>) -> Self {
        Self {
            root,
            version: HAMT_VERSION,
        }
    }

    /// Gets the difference between two HAMTs at the key-value level.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::sync::Arc;
    /// use wnfs_hamt::{Hamt, Node};
    /// use wnfs_common::MemoryBlockStore;
    ///
    /// #[async_std::main]
    /// async fn main() {
    ///     let store = &MemoryBlockStore::default();
    ///
    ///     let main_hamt = Hamt::<String, usize>::with_root({
    ///         let mut node = Arc::new(Node::default());
    ///         node.set("foo".into(), 400, store).await.unwrap();
    ///         node.set("bar".into(), 500, store).await.unwrap();
    ///         node
    ///     });
    ///
    ///     let other_hamt = Hamt::<String, usize>::with_root({
    ///         let mut node = Arc::new(Node::default());
    ///         node.set("foo".into(), 200, store).await.unwrap();
    ///         node.set("qux".into(), 600, store).await.unwrap();
    ///         node
    ///     });
    ///
    ///     let diff = main_hamt.diff(&other_hamt, store).await.unwrap();
    ///
    ///     println!("diff: {:#?}", diff);
    /// }
    pub async fn diff(
        &self,
        other: &Self,
        store: &impl BlockStore,
    ) -> Result<Vec<KeyValueChange<K, V>>>
    where
        K: DeserializeOwned + Clone + Eq + Hash + AsRef<[u8]>,
        V: DeserializeOwned + Clone + Eq,
        H: Clone + 'static,
    {
        super::diff(
            Link::from(Arc::clone(&self.root)),
            Link::from(Arc::clone(&other.root)),
            store,
        )
        .await
    }

    async fn to_ipld<B: BlockStore + ?Sized>(&self, store: &B) -> Result<Ipld>
    where
        K: Serialize,
        V: Serialize,
    {
        Ok(Ipld::Map(BTreeMap::from([
            ("root".into(), self.root.to_ipld(store).await?),
            ("version".into(), ipld_serde::to_ipld(&self.version)?),
            ("structure".into(), ipld_serde::to_ipld("hamt")?),
        ])))
    }
}

#[async_trait(?Send)]
impl<K, V, H: Hasher> AsyncSerialize for Hamt<K, V, H>
where
    K: Serialize,
    V: Serialize,
{
    async fn async_serialize<S, B>(&self, serializer: S, store: &B) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        B: BlockStore + ?Sized,
    {
        self.to_ipld(store)
            .await
            .map_err(SerError::custom)?
            .serialize(serializer)
    }
}

impl<'de, K, V> Deserialize<'de> for Hamt<K, V>
where
    K: DeserializeOwned,
    V: DeserializeOwned,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ipld::deserialize(deserializer).and_then(|ipld| ipld.try_into().map_err(DeError::custom))
    }
}

impl<K, V> TryFrom<Ipld> for Hamt<K, V>
where
    K: DeserializeOwned,
    V: DeserializeOwned,
{
    type Error = String;

    fn try_from(ipld: Ipld) -> Result<Self, Self::Error> {
        match ipld {
            Ipld::Map(mut map) => {
                let root = Arc::new(
                    Node::<K, V>::deserialize(map.remove("root").ok_or("Missing root")?)
                        .map_err(|e| e.to_string())?,
                );

                let version = match map.get("version").ok_or("Missing version")? {
                    Ipld::String(v) => Version::from_str(v).map_err(|e| e.to_string())?,
                    _ => return Err("`version` is not a string".into()),
                };

                Ok(Self { root, version })
            }
            other => Err(format!("Expected `Ipld::Map`, got {other:#?}")),
        }
    }
}

impl<K, V, H: Hasher> Default for Hamt<K, V, H> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K, V, H> PartialEq for Hamt<K, V, H>
where
    K: PartialEq,
    V: PartialEq,
    H: Hasher,
{
    fn eq(&self, other: &Self) -> bool {
        self.root == other.root && self.version == other.version
    }
}

//--------------------------------------------------------------------------------------------------
// Tests
//--------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use libipld::cbor::DagCborCodec;
    use wnfs_common::{async_encode, decode, MemoryBlockStore};

    #[async_std::test]
    async fn hamt_can_encode_decode_as_cbor() {
        let store = &MemoryBlockStore::default();
        let root = Arc::new(Node::default());
        let hamt: Hamt<String, i32> = Hamt::with_root(root);

        let encoded_hamt = async_encode(&hamt, store, DagCborCodec).await.unwrap();
        let decoded_hamt: Hamt<String, i32> = decode(encoded_hamt.as_ref(), DagCborCodec).unwrap();

        assert_eq!(hamt, decoded_hamt);
    }
}

#[cfg(test)]
mod snapshot_tests {
    use super::*;
    use wnfs_common::utils::SnapshotBlockStore;

    #[async_std::test]
    async fn test_hamt() {
        let store = &SnapshotBlockStore::default();
        let node = &mut Arc::new(Node::<[u8; 4], String>::default());
        for i in 0..99_u32 {
            node.set(i.to_le_bytes(), i.to_string(), store)
                .await
                .unwrap();
        }

        let hamt = Hamt::with_root(Arc::clone(node));
        let cid = store.put_async_serializable(&hamt).await.unwrap();
        let hamt = store.get_block_snapshot(&cid).await.unwrap();

        insta::assert_json_snapshot!(hamt);
    }
}
