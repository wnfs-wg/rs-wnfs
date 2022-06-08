// TODO(appcypher): Based on ipld_hamt implementation

use std::{borrow::Cow, rc::Rc};

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
    Pair, Pointer,
};

//--------------------------------------------------------------------------------------------------
// Constants
//--------------------------------------------------------------------------------------------------

const HAMT_BITMASK_SIZE: usize = 16;
const HAMT_BITMASK_BYTES: usize = HAMT_BITMASK_SIZE / 8;

//--------------------------------------------------------------------------------------------------
// Type Definitions
//--------------------------------------------------------------------------------------------------

pub type BitMaskType = [u8; HAMT_BITMASK_BYTES];

#[derive(Debug, Clone)]
pub struct Node<K, V> {
    pub(crate) bitmask: BitArray<BitMaskType>,
    pub(crate) pointers: Vec<Pointer<K, V>>,
}

//--------------------------------------------------------------------------------------------------
// Implementations
//--------------------------------------------------------------------------------------------------

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
        let mut hashquartets = HashQuartets::new(hash);
        self.modify_value(&mut hashquartets, key, value, store)
            .await
    }

    pub fn is_empty(&self) -> bool {
        self.bitmask.is_empty()
    }

    pub async fn get<B: BlockStore>(key: &K, store: &B) {
        let hash = &Sha3_256::generate_hash(&key);

        todo!()
    }

    #[async_recursion(?Send)]
    pub async fn modify_value<'a, 'b, B: BlockStore>(
        self: &'b Rc<Self>,
        hashquartets: &mut HashQuartets<'a>,
        key: K,
        value: V,
        store: &B,
    ) -> Result<Rc<Self>> {
        let index = hashquartets.next()?;

        // No existing values at this point.
        if !self.bitmask[index as usize] {
            let mut cloned = (**self).clone();
            cloned.insert_child(index as u32, key, value);
            return Ok(Rc::new(cloned));
        }

        let value_index = self.get_value_index(index as u32);
        Ok(match &self.pointers[value_index] {
            Pointer::Values(values) => {
                if let Some(i) = values.iter().position(|p| p.key == key) {
                    let mut cloned = (**self).clone();
                    let mut values_cloned = (*values).clone();
                    values_cloned[i] = Pair { key, value };
                    cloned.pointers[value_index] = Pointer::Values(values_cloned);
                    Rc::new(cloned)
                } else {
                    Rc::clone(self)
                }
            }
            Pointer::Link(link) => {
                let child = Rc::clone(link.resolve(store).await?);
                let new_child = child.modify_value(hashquartets, key, value, store).await?;
                let mut self_cloned = (**self).clone();
                self_cloned.pointers[value_index] = Pointer::Link(Link::from(new_child));
                Rc::new(self_cloned)
            }
        })
    }

    pub fn insert_child(&mut self, index: u32, key: K, value: V) {
        self.pointers.insert(
            self.get_value_index(index),
            Pointer::Values(vec![Pair { key, value }]),
        );

        self.bitmask.set(index as usize, true);
    }

    fn get_value_index(&self, bit_pos: u32) -> usize {
        let mut mask = bitarr!(u8, Lsb0; 1; 2);
        mask.shift_right(HAMT_BITMASK_SIZE - bit_pos as usize);
        (mask & self.bitmask).count_ones()
    }

    #[async_recursion(?Send)]
    pub async fn flush<'a, B: BlockStore>(self: &'a Rc<Self>, store: &mut B) -> Result<()> {
        for pointer in self.pointers.iter() {
            if let Pointer::Link(link) = pointer {
                if let Some(node) = link.get_value() {
                    Rc::clone(node).flush(store).await?
                }

                link.seal(store).await?;
            }
        }

        Ok(())
    }

    pub async fn search<'a, B: BlockStore>(
        self: &'a Rc<Self>,
        key: &K,
        store: &B,
    ) -> Result<Option<&'a Pair<K, V>>> {
        let hash = &Sha3_256::generate_hash(&key);
        let mut hashquartets = HashQuartets::new(hash);
        self.get_value(&mut hashquartets, key, store).await
    }

    #[async_recursion(?Send)]
    pub async fn get_value<'a, 'b, B: BlockStore>(
        self: &'a Rc<Self>,
        hashquartets: &'b mut HashQuartets,
        key: &K,
        store: &B,
    ) -> Result<Option<&'a Pair<K, V>>> {
        let index = hashquartets.next()?;

        if !self.bitmask[index as usize] {
            return Ok(None);
        }

        let value_index = self.get_value_index(index as u32);
        match &self.pointers[value_index] {
            Pointer::Values(values) => Ok(values.iter().find(|kv| key.eq(&kv.key))),
            Pointer::Link(link) => {
                let child = link.resolve(store).await?;
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

//--------------------------------------------------------------------------------------------------
// Tests
//--------------------------------------------------------------------------------------------------

#[cfg(test)]
mod hamt_node_tests {
    use crate::{store_put, BlockStore, MemoryBlockStore};
    use std::rc::Rc;

    use super::Node;

    #[async_std::test]
    async fn example_test() {
        let mut store = MemoryBlockStore::default();
        let hamt = Rc::new(Node::default())
            .set("abc".to_string(), "def".to_string(), &mut store)
            .await
            .unwrap();
        hamt.flush(&mut store);
        let cid = store_put(&*hamt, &mut store).await.unwrap();
        println!("{cid}");
        println!("{:02X?}", store.get_block(&cid).await.unwrap().as_ref());
    }
}
