// TODO(appcypher): Based on ipld_hamt implementation

use std::rc::Rc;

use anyhow::Result;
use async_trait::async_trait;
use libipld::{serde as ipld_serde, Ipld};
use serde::{
    de::{DeserializeOwned, Error as DeError},
    ser::Error as SerError,
    Deserialize, Deserializer, Serialize, Serializer,
};

use crate::{AsyncSerialize, BlockStore, Link};

use super::Node;

//--------------------------------------------------------------------------------------------------
// Type Definitions
//--------------------------------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pair<K, V> {
    pub key: K,
    pub value: V,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Pointer<K, V> {
    Values(Vec<Pair<K, V>>),
    Link(Link<Rc<Node<K, V>>>),
}

//--------------------------------------------------------------------------------------------------
// Implementations
//--------------------------------------------------------------------------------------------------

impl<K, V> Pointer<K, V> {
    pub async fn to_ipld<B: BlockStore + ?Sized>(&self, store: &mut B) -> Result<Ipld>
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

#[async_trait(?Send)]
impl<K, V> AsyncSerialize for Pointer<K, V>
where
    K: Serialize,
    V: Serialize,
{
    async fn async_serialize<S: Serializer, B: BlockStore + ?Sized>(
        &self,
        serializer: S,
        store: &mut B,
    ) -> Result<S::Ok, S::Error> {
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

impl<'de, K, V> Deserialize<'de> for Pointer<K, V>
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

impl<K, V> TryFrom<Ipld> for Pointer<K, V>
where
    K: DeserializeOwned,
    V: DeserializeOwned,
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
                "Expected `Ipld::List` or `Ipld::Link`, got {:#?}",
                other
            )),
        }
    }
}

impl<K, V> Default for Pointer<K, V> {
    fn default() -> Self {
        Pointer::Values(Vec::new())
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
mod pointer_tests {
    use super::*;
    use crate::{dagcbor, MemoryBlockStore};

    #[async_std::test]
    async fn pointer_can_encode_decode_as_cbor() {
        let store = &mut MemoryBlockStore::default();
        let pointer: Pointer<String, i32> = Pointer::Values(vec![
            Pair {
                key: "James".into(),
                value: 4500,
            },
            Pair {
                key: "Peter".into(),
                value: 2000,
            },
        ]);

        let encoded_pointer = dagcbor::async_encode(&pointer, store).await.unwrap();
        let decoded_pointer =
            dagcbor::decode::<Pointer<String, i32>>(encoded_pointer.as_ref()).unwrap();

        assert_eq!(pointer, decoded_pointer);
    }
}
