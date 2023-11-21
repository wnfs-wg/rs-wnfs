use super::{error::HamtError, hash::Hasher, Node, HAMT_VALUES_BUCKET_SIZE};
use anyhow::Result;
use async_trait::async_trait;
use libipld::{serde as ipld_serde, Ipld};
use serde::{
    de::{DeserializeOwned, Error as DeError},
    ser::Error as SerError,
    Deserialize, Deserializer, Serialize, Serializer,
};
use std::{fmt::Debug, sync::Arc};
use wnfs_common::{utils::error, AsyncSerialize, BlockStore, Link};

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
pub(crate) enum Pointer<K: Sync + Send, V: Sync + Send, H: Hasher + Sync + Send> {
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

impl<K: Sync + Send, V: Sync + Send, H: Hasher + Send + Sync> Pointer<K, V, H> {
    /// Converts a Link pointer to a canonical form to ensure consistent tree representation after deletes.
    pub async fn canonicalize(self, store: &impl BlockStore) -> Result<Option<Self>>
    where
        K: DeserializeOwned + Clone + AsRef<[u8]>,
        V: DeserializeOwned + Clone,
        H: Sync + Send,
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

    /// Converts a Pointer to an IPLD object.
    pub async fn to_ipld<B: BlockStore + ?Sized + Sync>(&self, store: &B) -> Result<Ipld>
    where
        K: Serialize,
        V: Serialize,
    {
        Ok(match self {
            Pointer::Values(values) => ipld_serde::to_ipld(values)?,
            Pointer::Link(link) => ipld_serde::to_ipld(link.resolve_cid(store).await?)?,
        })
    }
}

#[async_trait]
impl<K, V, H: Hasher> AsyncSerialize for Pointer<K, V, H>
where
    K: Serialize + Sync + Send,
    V: Serialize + Sync + Send,
    H: Sync + Send,
{
    async fn async_serialize<S, B>(&self, serializer: S, store: &B) -> Result<S::Ok, S::Error>
    where
        S: Serializer + Send,
        B: BlockStore + ?Sized + Sync,
    {
        match self {
            Pointer::Values(vals) => vals.serialize(serializer),
            Pointer::Link(link) => link
                .resolve_cid(store)
                .await
                .map_err(SerError::custom)?
                .serialize(serializer),
        }
    }
}

impl<'de, K, V, H: Hasher + Send + Sync> Deserialize<'de> for Pointer<K, V, H>
where
    K: DeserializeOwned + Send + Sync,
    V: DeserializeOwned + Send + Sync,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ipld::deserialize(deserializer).and_then(|ipld| ipld.try_into().map_err(DeError::custom))
    }
}

impl<K, V, H: Hasher + Send + Sync> TryFrom<Ipld> for Pointer<K, V, H>
where
    K: DeserializeOwned + Send + Sync,
    V: DeserializeOwned + Send + Sync,
{
    type Error = String;

    fn try_from(ipld: Ipld) -> Result<Self, Self::Error> {
        match ipld {
            ipld_list @ Ipld::List(_) => {
                let values: Vec<Pair<K, V>> =
                    Deserialize::deserialize(ipld_list).map_err(|error| error.to_string())?;
                Ok(Self::Values(values))
            }
            Ipld::Link(cid) => Ok(Self::Link(Link::from_cid(cid))),
            other => Err(format!(
                "Expected `Ipld::List` or `Ipld::Link`, got {other:?}",
            )),
        }
    }
}

impl<K: Clone + Sync + Send, V: Clone + Sync + Send, H: Hasher + Sync + Send> Clone
    for Pointer<K, V, H>
{
    fn clone(&self) -> Self {
        match self {
            Self::Values(arg0) => Self::Values(arg0.clone()),
            Self::Link(arg0) => Self::Link(arg0.clone()),
        }
    }
}

impl<K: Debug + Sync + Send, V: Debug + Sync + Send, H: Hasher + Sync + Send> std::fmt::Debug
    for Pointer<K, V, H>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Values(arg0) => f.debug_tuple("Values").field(arg0).finish(),
            Self::Link(arg0) => f.debug_tuple("Link").field(arg0).finish(),
        }
    }
}

impl<K: Sync + Send, V: Sync + Send, H: Hasher + Sync + Send> Default for Pointer<K, V, H> {
    fn default() -> Self {
        Pointer::Values(Vec::new())
    }
}

impl<K, V, H: Hasher + Send + Sync> PartialEq for Pointer<K, V, H>
where
    K: PartialEq + Send + Sync,
    V: PartialEq + Send + Sync,
{
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Pointer::Values(vals), Pointer::Values(other_vals)) => vals == other_vals,
            (Pointer::Link(link), Pointer::Link(other_link)) => link == other_link,
            _ => false,
        }
    }
}

impl<'de, K, V> Deserialize<'de> for Pair<K, V>
where
    K: DeserializeOwned,
    V: DeserializeOwned,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let (key, value) = <(K, V)>::deserialize(deserializer)?;
        Ok(Pair { key, value })
    }
}

impl<K, V> Serialize for Pair<K, V>
where
    K: Serialize,
    V: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        (&self.key, &self.value).serialize(serializer)
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
