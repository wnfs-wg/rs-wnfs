use super::{diff, KeyValueChange, Node, NodeChange, HAMT_VERSION};
use crate::{AsyncSerialize, BlockStore, Hasher, Link};
use anyhow::Result;
use async_trait::async_trait;
use libipld::{serde as ipld_serde, Ipld};
use semver::Version;
use serde::{
    de::{DeserializeOwned, Error as DeError},
    ser::Error as SerError,
    Deserialize, Deserializer, Serialize, Serializer,
};
use sha3::Sha3_256;
use std::{collections::BTreeMap, fmt, hash::Hash, rc::Rc, str::FromStr};

//--------------------------------------------------------------------------------------------------
// Type Definitions
//--------------------------------------------------------------------------------------------------

/// Hash Array Mapped Trie (HAMT) is an implementation of an associative array that combines the characteristics
/// of a hash table and an array mapped trie.
///
/// This type wraps the actual implementation which can be found in the [`Node`](crate::private::Node).
///
/// # Examples
///
/// ```
/// use wnfs::private::Hamt;
///
/// let hamt = Hamt::<String, usize>::new();
/// println!("HAMT: {:?}", hamt);
/// ```
#[derive(Debug, Clone)]
pub struct Hamt<K, V, H = Sha3_256>
where
    H: Hasher,
{
    pub root: Rc<Node<K, V, H>>,
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
    /// use wnfs::private::Hamt;
    ///
    /// let hamt = Hamt::<String, usize>::new();
    /// println!("HAMT: {:?}", hamt);
    /// ```
    pub fn new() -> Self {
        Self {
            root: Rc::new(Node::default()),
            version: HAMT_VERSION,
        }
    }

    /// Creates a new `HAMT` with the given root node.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::rc::Rc;
    /// use wnfs::private::{Hamt, Node};
    ///
    /// let hamt = Hamt::<String, usize>::with_root(Rc::new(Node::default()));
    /// println!("HAMT: {:?}", hamt);
    /// ```
    pub fn with_root(root: Rc<Node<K, V, H>>) -> Self {
        Self {
            root,
            version: HAMT_VERSION,
        }
    }

    /// TODO(appcypher): Add docs.
    pub async fn node_diff<B: BlockStore>(
        &self,
        other: &Self,
        depth: Option<u8>,
        store: &mut B,
    ) -> Result<Vec<NodeChange>>
    where
        K: DeserializeOwned + Clone + fmt::Debug + Eq + Hash + AsRef<[u8]>,
        V: DeserializeOwned + Clone + fmt::Debug + Eq,
        H: Clone + fmt::Debug + 'static,
    {
        if self.version == other.version {
            return diff::node_diff(
                Link::from(Rc::clone(&self.root)),
                Link::from(Rc::clone(&other.root)),
                depth,
                store,
            )
            .await;
        }

        Ok(vec![])
    }

    /// TODO(appcypher): Add docs.
    pub async fn kv_diff<B: BlockStore>(
        &self,
        other: &Self,
        depth: Option<u8>,
        store: &mut B,
    ) -> Result<Vec<KeyValueChange<K, V>>>
    where
        K: DeserializeOwned + Clone + fmt::Debug + Eq + Hash + AsRef<[u8]>,
        V: DeserializeOwned + Clone + fmt::Debug + Eq,
        H: Clone + fmt::Debug + 'static,
    {
        if self.version == other.version {
            return diff::kv_diff(
                Link::from(Rc::clone(&self.root)),
                Link::from(Rc::clone(&other.root)),
                depth,
                store,
            )
            .await;
        }

        Ok(vec![])
    }

    async fn to_ipld<B: BlockStore + ?Sized>(&self, store: &mut B) -> Result<Ipld>
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
    async fn async_serialize<S, B>(&self, serializer: S, store: &mut B) -> Result<S::Ok, S::Error>
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
                let root = Rc::new(
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
    use crate::{dagcbor, MemoryBlockStore};

    #[async_std::test]
    async fn hamt_can_encode_decode_as_cbor() {
        let store = &mut MemoryBlockStore::default();
        let root = Rc::new(Node::default());
        let hamt: Hamt<String, i32> = Hamt::with_root(root);

        let encoded_hamt = dagcbor::async_encode(&hamt, store).await.unwrap();
        let decoded_hamt = dagcbor::decode::<Hamt<String, i32>>(encoded_hamt.as_ref()).unwrap();

        assert_eq!(hamt, decoded_hamt);
    }
}
