// TODO(appcypher): Based on ipld_hamt implementation

use std::rc::Rc;

use crate::{BlockStore, Link};
use anyhow::Result;
use async_recursion::async_recursion;
use bitvec::array::BitArray;
use bitvec::bitarr;
use bitvec::order::Lsb0;
use serde::{
    de::{Deserialize, DeserializeOwned},
    Deserializer, Serialize, Serializer,
};
use sha3::Sha3_256;

use super::{
    hash::{GenerateHash, HashQuartets},
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
    pub async fn set<B: BlockStore>(
        self: Rc<Self>,
        key: K,
        value: V,
        store: &mut B,
    ) -> Result<Rc<Self>> {
        let hash = &Sha3_256::generate_hash(&key);
        self.modify_value(&mut HashQuartets::new(hash), key, value, store)
            .await
    }

    pub async fn get<'a, B: BlockStore>(
        self: &'a Rc<Self>,
        key: &K,
        store: &B,
    ) -> Result<Option<&'a V>> {
        Ok(self.search(key, store).await?.map(|pair| &pair.value))
    }

    /// Recursively resolves all nodes under the directory to Cids.
    #[async_recursion(?Send)]
    pub async fn flush<'a, B: BlockStore>(self: &'a Rc<Self>, store: &mut B) -> Result<()> {
        for pointer in self.pointers.iter() {
            if let Pointer::Link(link) = pointer {
                if let Some(node) = link.get_value() {
                    node.flush(store).await?
                }

                link.resolve_cid(store).await?;
            }
        }

        Ok(())
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
        self.bitmask.set(value_index, true);
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
        let mut mask = bitarr!(u8, Lsb0; 1; 2);
        mask.shift_right(HAMT_BITMASK_SIZE - bit_index);
        (mask & self.bitmask).count_ones()
    }

    /// M
    #[async_recursion(?Send)]
    async fn modify_value<'a, 'b, B: BlockStore>(
        self: &'b Rc<Self>,
        hashquartets: &mut HashQuartets<'a>,
        key: K,
        value: V,
        store: &B,
    ) -> Result<Rc<Self>> {
        let bit_index = hashquartets.next()? as usize;

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
                let new_child = child.modify_value(hashquartets, key, value, store).await?;
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
        let mut hashquartets = HashQuartets::new(hash);
        self.get_value(&mut hashquartets, key, store).await
    }

    #[async_recursion(?Send)]
    async fn get_value<'a, 'b, B: BlockStore>(
        self: &'a Rc<Self>,
        hashquartets: &'b mut HashQuartets,
        key: &K,
        store: &B,
    ) -> Result<Option<&'a Pair<K, V>>> {
        let bit_index = hashquartets.next()? as usize;

        if !self.bitmask[bit_index] {
            return Ok(None);
        }

        let value_index = self.get_value_index(bit_index);
        match &self.pointers[value_index] {
            Pointer::Values(values) => Ok(values.iter().find(|kv| key.eq(&kv.key))),
            Pointer::Link(link) => {
                let child = link.resolve_value(store).await?;
                child.get_value(hashquartets, key, store).await
            }
        }
    }
}

impl<K, V> Default for Node<K, V> {
    fn default() -> Self {
        Node {
            bitmask: BitArray::ZERO,
            pointers: Vec::new(),
        }
    }
}

impl<K, V> Serialize for Node<K, V>
where
    K: Serialize,
    V: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        (&self.bitmask.as_raw_slice(), &self.pointers).serialize(serializer)
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
    use crate::{BlockStore, MemoryBlockStore};
    use std::rc::Rc;

    use super::Node;

    #[async_std::test]
    async fn example_test() {
        let mut store = MemoryBlockStore::default();

        let node = Rc::new(Node::default())
            .set("abc".to_string(), "def".to_string(), &mut store)
            .await
            .unwrap();

        node.flush(&mut store);

        let cid = store.put_serializable(&*node).await.unwrap();

        println!("{:02X?}", store.get_block(&cid).await.unwrap().as_ref());
    }
}
