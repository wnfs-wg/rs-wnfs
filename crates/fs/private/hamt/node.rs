// TODO(appcypher): Based on ipld_hamt implementation

use std::{fmt::Debug, rc::Rc};

use crate::{AsyncSerialize, BlockStore, Link};
use anyhow::Result;
use async_recursion::async_recursion;
use async_trait::async_trait;
use bitvec::array::BitArray;

use libipld::{serde as ipld_serde, Ipld};
use serde::{
    de::{Deserialize, DeserializeOwned},
    ser::Error as SerError,
    Deserializer, Serialize, Serializer,
};
use sha3::Sha3_256;

use super::{
    hash::{GenerateHash, HashNibbles},
    Pair, Pointer, HAMT_BITMASK_BYTES, HAMT_BITMASK_SIZE,
};

//--------------------------------------------------------------------------------------------------
// Type Definitions
//--------------------------------------------------------------------------------------------------

pub type BitMaskType = [u8; HAMT_BITMASK_BYTES];

#[derive(Debug, Clone, PartialEq)]
pub struct Node<K, V> {
    pub(crate) bitmask: BitArray<BitMaskType>,
    pub(crate) pointers: Vec<Pointer<K, V>>,
}

//--------------------------------------------------------------------------------------------------
// Implementations
//--------------------------------------------------------------------------------------------------

impl<K, V> Node<K, V>
where
    K: DeserializeOwned + Serialize + AsRef<[u8]> + Clone + Eq,
    V: DeserializeOwned + Serialize + Clone,
{
    /// Sets a new value at the given key.
    pub async fn set<B: BlockStore>(
        self: Rc<Self>,
        key: K,
        value: V,
        store: &mut B,
    ) -> Result<Rc<Self>> {
        let hash = &Sha3_256::generate_hash(&key);
        self.modify_value(&mut HashNibbles::new(hash), key, value, store)
            .await
    }

    /// Gets the value at the given key.
    pub async fn get<'a, B: BlockStore>(
        self: &'a Rc<Self>,
        key: &K,
        store: &B,
    ) -> Result<Option<&'a V>> {
        Ok(self.search(key, store).await?.map(|pair| &pair.value))
    }

    /// Checks if the node is empty.
    pub fn is_empty(&self) -> bool {
        self.bitmask.is_empty()
    }

    /// Inserts a new pointer at specified index.
    ///
    /// NOTE: Internal use only. It mutates the node.
    fn insert_child(&mut self, bit_index: usize, key: K, value: V) {
        let value_index = self.get_value_index(bit_index);
        self.pointers
            .insert(value_index, Pointer::Values(vec![Pair { key, value }]));
        self.bitmask.set(bit_index, true);
    }

    /// Removes a pointer at specified index.
    ///
    /// NOTE: Internal use only. It mutates the node.
    fn remove_child<B: BlockStore>(&mut self, bit_index: usize) {
        let value_index = self.get_value_index(bit_index);
        self.pointers.remove(value_index);
        self.bitmask.set(value_index, true);
    }

    /// Calculates the value index from the bitmask index.
    fn get_value_index(&self, bit_index: usize) -> usize {
        let mut mask = BitArray::<BitMaskType>::new([0xff, 0xff]);
        mask.shift_left(HAMT_BITMASK_SIZE - bit_index);
        assert_eq!(mask.count_ones(), bit_index);
        (mask & self.bitmask).count_ones()
    }

    #[async_recursion(?Send)]
    async fn modify_value<'a, 'b, B: BlockStore>(
        self: &'b Rc<Self>,
        hashnibbles: &mut HashNibbles<'a>,
        key: K,
        value: V,
        store: &B,
    ) -> Result<Rc<Self>> {
        let bit_index = hashnibbles.next()? as usize;

        // No existing values at this point.
        if !self.bitmask[bit_index] {
            let mut cloned = (**self).clone();
            cloned.insert_child(bit_index, key, value);
            return Ok(Rc::new(cloned));
        }

        let value_index = self.get_value_index(bit_index);
        Ok(match &self.pointers[value_index] {
            Pointer::Values(values) => {
                let mut cloned = (**self).clone();
                let mut values_cloned = (*values).clone();

                if let Some(i) = values.iter().position(|p| p.key == key) {
                    values_cloned[i] = Pair { key, value };
                } else {
                    values_cloned.insert(value_index, Pair { key, value });
                }

                cloned.pointers[value_index] = Pointer::Values(values_cloned);
                Rc::new(cloned)
            }
            Pointer::Link(link) => {
                let child = Rc::clone(link.resolve_value(store).await?);
                let new_child = child.modify_value(hashnibbles, key, value, store).await?;
                let mut self_cloned = (**self).clone();
                self_cloned.pointers[value_index] = Pointer::Link(Link::from(new_child));
                Rc::new(self_cloned)
            }
        })
    }

    async fn search<'a, B: BlockStore>(
        self: &'a Rc<Self>,
        key: &K,
        store: &B,
    ) -> Result<Option<&'a Pair<K, V>>> {
        let hash = &Sha3_256::generate_hash(&key);
        let mut hashnibbles = HashNibbles::new(hash);
        self.get_value(&mut hashnibbles, key, store).await
    }

    #[async_recursion(?Send)]
    async fn get_value<'a, 'b, B: BlockStore>(
        self: &'a Rc<Self>,
        hashnibbles: &'b mut HashNibbles,
        key: &K,
        store: &B,
    ) -> Result<Option<&'a Pair<K, V>>> {
        let bit_index = hashnibbles.next()? as usize;

        if !self.bitmask[bit_index] {
            return Ok(None);
        }

        let value_index = self.get_value_index(bit_index);
        match &self.pointers[value_index] {
            Pointer::Values(values) => Ok(values.iter().find(|kv| key.eq(&kv.key))),
            Pointer::Link(link) => {
                let child = link.resolve_value(store).await?;
                child.get_value(hashnibbles, key, store).await
            }
        }
    }
}

impl<K, V> Node<K, V> {
    pub async fn to_ipld<B: BlockStore + ?Sized>(&self, store: &mut B) -> Result<Ipld>
    where
        K: Serialize,
        V: Serialize,
    {
        let bitmask_ipld = ipld_serde::to_ipld(&self.bitmask.as_raw_slice())?;
        let pointers_ipld = {
            let mut tmp = Vec::with_capacity(self.pointers.len());
            for pointer in self.pointers.iter() {
                tmp.push(pointer.to_ipld(store).await?);
            }
            Ipld::List(tmp)
        };

        Ok(Ipld::List(vec![bitmask_ipld, pointers_ipld]))
    }
}

impl<K, V> Default for Node<K, V> {
    fn default() -> Self {
        Node {
            bitmask: BitArray::ZERO,
            pointers: Vec::with_capacity(HAMT_BITMASK_SIZE),
        }
    }
}

#[async_trait(?Send)]
impl<K, V> AsyncSerialize for Node<K, V>
where
    K: Serialize,
    V: Serialize,
{
    async fn async_serialize<S: Serializer, B: BlockStore + ?Sized>(
        &self,
        serializer: S,
        store: &mut B,
    ) -> Result<S::Ok, S::Error> {
        let ipld = self.to_ipld(store).await.map_err(SerError::custom)?;
        ipld.serialize(serializer)
    }
}

impl<'de, K, V> Deserialize<'de> for Node<K, V>
where
    K: DeserializeOwned,
    V: DeserializeOwned,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let (bitmask, pointers): (BitMaskType, _) = Deserialize::deserialize(deserializer)?;
        Ok(Node {
            bitmask: BitArray::<BitMaskType>::from(bitmask),
            pointers,
        })
    }
}

//--------------------------------------------------------------------------------------------------
// Tests
//--------------------------------------------------------------------------------------------------

#[cfg(test)]
mod hamt_node_tests {
    use super::*;
    use crate::{dagcbor, MemoryBlockStore};

    #[async_std::test]
    async fn node_can_insert_pair_and_retrieve() {
        let mut store = MemoryBlockStore::default();
        let node = Rc::new(Node::<String, (i32, f64)>::default());

        let node = node
            .set("pill".into(), (10, 0.315), &mut store)
            .await
            .unwrap();
        let value = node.get(&"pill".into(), &mut store).await.unwrap().unwrap();

        assert_eq!(value, &(10, 0.315));
    }

    #[async_std::test]
    async fn node_can_encode_decode_as_cbor() {
        let store = &mut MemoryBlockStore::default();
        let node: Rc<Node<String, i32>> = Rc::new(Node::default());

        let node = node.set("James".into(), 4500, store).await.unwrap();
        let node = node.set("Peter".into(), 2000, store).await.unwrap();

        let encoded_node = dagcbor::async_encode(&node, store).await.unwrap();
        let decoded_node = dagcbor::decode::<Node<String, i32>>(encoded_node.as_ref()).unwrap();

        assert_eq!(*node, decoded_node);
    }
}
