use std::{collections::BTreeMap, rc::Rc, str::FromStr};

use anyhow::Result;
use async_trait::async_trait;
use libipld::{serde as ipld_serde, Ipld};
use semver::Version;
use serde::{
    de::{DeserializeOwned, Error as DeError},
    ser::Error as SerError,
    Deserialize, Deserializer, Serialize, Serializer,
};

use crate::{AsyncSerialize, BlockStore};

use super::{Node, HAMT_VERSION};

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
#[derive(Debug, Clone, PartialEq)]
pub struct Hamt<K, V> {
    pub root: Rc<Node<K, V>>,
    pub version: Version,
}

//--------------------------------------------------------------------------------------------------
// Implementations
//--------------------------------------------------------------------------------------------------

impl<K, V> Hamt<K, V> {
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
    pub fn with_root(root: Rc<Node<K, V>>) -> Self {
        Self {
            root,
            version: HAMT_VERSION,
        }
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
impl<K, V> AsyncSerialize for Hamt<K, V>
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
            other => Err(format!("Expected `Ipld::Map`, got {:#?}", other)),
        }
    }
}

impl<K, V> Default for Hamt<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

//--------------------------------------------------------------------------------------------------
// Tests
//--------------------------------------------------------------------------------------------------

#[cfg(test)]
mod hamt_tests {
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
