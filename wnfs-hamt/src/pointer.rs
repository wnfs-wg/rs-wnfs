use super::{error::HamtError, hash::Hasher, Node, HAMT_VALUES_BUCKET_SIZE};
use crate::serializable::PointerSerializable;
use anyhow::Result;
use async_trait::async_trait;
use serde::{
    de::DeserializeOwned, ser::Error as SerError, Deserialize, Deserializer, Serialize, Serializer,
};
use std::fmt::Debug;
use wnfs_common::{
    utils::{error, Arc, CondSend, CondSync},
    AsyncSerialize, BlockStore, Link,
};

//--------------------------------------------------------------------------------------------------
// Type Definitions
//--------------------------------------------------------------------------------------------------

/// A key-value pair type.
///
/// # Examples
///
/// ```
/// use wnfs_hamt::Pair;
///
/// let pair = Pair::new("key", "value");
///
/// assert_eq!(pair.key, "key");
/// assert_eq!(pair.value, "value");
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pair<K, V> {
    pub key: K,
    pub value: V,
}

/// Each bit in the bitmask of a node maps a `Pointer` in the HAMT structure.
/// A `Pointer` can be either a link to a child node or a collection of key-value pairs.
pub(crate) enum Pointer<K: CondSync, V: CondSync, H: Hasher + CondSync> {
    Values(Vec<Pair<K, V>>),
    Link(Link<Arc<Node<K, V, H>>>),
}

//--------------------------------------------------------------------------------------------------
// Implementations
//--------------------------------------------------------------------------------------------------

impl<K, V> Pair<K, V> {
    /// Create a new `Pair` from a key and value.
    ///
    /// # Examples
    ///
    /// ```
    /// use wnfs_hamt::Pair;
    ///
    /// let pair = Pair::new("key", "value");
    ///
    /// assert_eq!(pair.key, "key");
    /// assert_eq!(pair.value, "value");
    /// ```
    pub fn new(key: K, value: V) -> Self {
        Self { key, value }
    }
}

impl<K: CondSync, V: CondSync, H: Hasher + CondSync> Pointer<K, V, H> {
    /// Converts a Link pointer to a canonical form to ensure consistent tree representation after deletes.
    pub async fn canonicalize(self, store: &impl BlockStore) -> Result<Option<Self>>
    where
        K: DeserializeOwned + Clone + AsRef<[u8]>,
        V: DeserializeOwned + Clone,
        H: CondSync,
    {
        match self {
            Pointer::Link(link) => {
                let node = link.resolve_owned_value(store).await?;
                match node.pointers.len() {
                    0 => Ok(None),
                    1 if matches!(node.pointers[0], Pointer::Values(_)) => {
                        Ok(Some(node.pointers[0].clone()))
                    }
                    2..=HAMT_VALUES_BUCKET_SIZE if node.count_values().is_ok() => {
                        // Collect all the values of the node.
                        let mut values = node
                            .pointers
                            .iter()
                            .filter_map(|p| match p {
                                Pointer::Values(values) => Some(values.clone()),
                                _ => None,
                            })
                            .flatten()
                            .collect::<Vec<_>>();

                        // Bail if it's more values that we can fit into a bucket
                        if values.len() > HAMT_VALUES_BUCKET_SIZE {
                            return Ok(Some(Pointer::Link(Link::from(node))));
                        }

                        values.sort_unstable_by(|a, b| {
                            H::hash(&a.key).partial_cmp(&H::hash(&b.key)).unwrap()
                        });

                        Ok(Some(Pointer::Values(values)))
                    }
                    _ => Ok(Some(Pointer::Link(Link::from(node)))),
                }
            }
            _ => error(HamtError::NonCanonicalizablePointer),
        }
    }

    /// Converts a Pointer to the serializable representation.
    pub(crate) async fn to_serializable<B: BlockStore + ?Sized>(
        &self,
        store: &B,
    ) -> Result<PointerSerializable<K, V>>
    where
        K: Serialize + Clone,
        V: Serialize + Clone,
    {
        Ok(match self {
            Pointer::Values(values) => PointerSerializable::Values(
                values
                    .iter()
                    .map(|pair| (pair.key.clone(), pair.value.clone()))
                    .collect(),
            ),
            Pointer::Link(link) => {
                let cid = link.resolve_cid(store).await?;
                PointerSerializable::Link(*cid)
            }
        })
    }

    /// Constructs a Pointer from its serializable representation.
    pub(crate) fn from_serializable(serializable: PointerSerializable<K, V>) -> Self {
        match serializable {
            PointerSerializable::Values(values) => Self::Values(
                values
                    .into_iter()
                    .map(|(key, value)| Pair { key, value })
                    .collect(),
            ),
            PointerSerializable::Link(cid) => Self::Link(Link::from_cid(cid)),
        }
    }
}

#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
impl<K, V, H: Hasher> AsyncSerialize for Pointer<K, V, H>
where
    K: Serialize + Clone + CondSync,
    V: Serialize + Clone + CondSync,
    H: CondSync,
{
    async fn async_serialize<S, B>(&self, serializer: S, store: &B) -> Result<S::Ok, S::Error>
    where
        S: Serializer + CondSend,
        B: BlockStore + ?Sized,
    {
        self.to_serializable(store)
            .await
            .map_err(SerError::custom)?
            .serialize(serializer)
    }
}

impl<'de, K, V, H: Hasher + CondSync> Deserialize<'de> for Pointer<K, V, H>
where
    K: DeserializeOwned + CondSync,
    V: DeserializeOwned + CondSync,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Self::from_serializable(PointerSerializable::deserialize(
            deserializer,
        )?))
    }
}

impl<K: Clone + CondSync, V: Clone + CondSync, H: Hasher + CondSync> Clone for Pointer<K, V, H> {
    fn clone(&self) -> Self {
        match self {
            Self::Values(arg0) => Self::Values(arg0.clone()),
            Self::Link(arg0) => Self::Link(arg0.clone()),
        }
    }
}

impl<K: Debug + CondSync, V: Debug + CondSync, H: Hasher + CondSync> std::fmt::Debug
    for Pointer<K, V, H>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Values(arg0) => f.debug_tuple("Values").field(arg0).finish(),
            Self::Link(arg0) => f.debug_tuple("Link").field(arg0).finish(),
        }
    }
}

impl<K: CondSync, V: CondSync, H: Hasher + CondSync> Default for Pointer<K, V, H> {
    fn default() -> Self {
        Pointer::Values(Vec::new())
    }
}

impl<K, V, H: Hasher + CondSync> PartialEq for Pointer<K, V, H>
where
    K: PartialEq + CondSync,
    V: PartialEq + CondSync,
{
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Pointer::Values(vals), Pointer::Values(other_vals)) => vals == other_vals,
            (Pointer::Link(link), Pointer::Link(other_link)) => link == other_link,
            _ => false,
        }
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
    async fn pointer_can_encode_decode_as_cbor() {
        let store = &MemoryBlockStore::default();
        let pointer: Pointer<String, i32, blake3::Hasher> = Pointer::Values(vec![
            Pair {
                key: "James".into(),
                value: 4500,
            },
            Pair {
                key: "Peter".into(),
                value: 2000,
            },
        ]);

        let encoded_pointer = async_encode(&pointer, store, DagCborCodec).await.unwrap();
        let decoded_pointer: Pointer<String, i32, blake3::Hasher> =
            decode(encoded_pointer.as_ref(), DagCborCodec).unwrap();

        assert_eq!(pointer, decoded_pointer);
    }
}

#[cfg(test)]
mod snapshot_tests {
    use super::*;
    use wnfs_common::utils::SnapshotBlockStore;

    #[async_std::test]
    async fn test_pointer() {
        let store = &SnapshotBlockStore::default();
        let pointer: Pointer<String, i32, blake3::Hasher> = Pointer::Values(vec![
            Pair {
                key: "James".into(),
                value: 4500,
            },
            Pair {
                key: "Peter".into(),
                value: 2000,
            },
        ]);

        let cid = store.put_async_serializable(&pointer).await.unwrap();
        let ptr = store.get_block_snapshot(&cid).await.unwrap();

        insta::assert_json_snapshot!(ptr);
    }
}
