use std::pin::Pin;
use std::rc::Rc;

use crate::BlockStore;
use anyhow::Result;
use async_recursion::async_recursion;
use bitvec::array::BitArray;
use bitvec::bitarr;
use bitvec::order::Lsb0;
use libipld::serde::to_ipld;
use libipld::{cbor::DagCborCodec, codec::Encode};
use serde::{
    de::{Deserialize, DeserializeOwned},
    ser::{self, SerializeTuple},
    Deserializer, Serialize, Serializer,
};
use sha3::Sha3_256;

use super::{
    hash::{GenerateHash, HashBits},
    Pointer,
};

#[derive(Debug, Default, Clone)]
pub struct Node<K, V> {
    pub(crate) bitmask: BitArray<[u16; 1]>,
    pub(crate) pointers: Vec<Pointer<K, V>>,
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
        let (bitmask, pointers): ([u16; 1], _) = Deserialize::deserialize(deserializer)?;
        Ok(Node {
            bitmask: BitArray::<[u16; 1]>::from(bitmask),
            pointers,
        })
    }
}

impl<K, V> Node<K, V>
where
    K: DeserializeOwned + Serialize + AsRef<[u8]> + Clone,
    V: DeserializeOwned + Serialize + Clone,
{
    pub async fn set<B: BlockStore>(
        self: Rc<Self>,
        key: K,
        value: V,
        store: &mut B,
    ) -> Result<Rc<Self>> {
        let hash = &Sha3_256::generate_hash(&key);
        let mut hashbits = HashBits::new(hash);
        self.modify_value(&mut hashbits, key, value, store).await?;

        todo!()
    }

    pub async fn get<B: BlockStore>(key: &K, store: &B) {
        let hash = &Sha3_256::generate_hash(&key);

        todo!()
    }

    pub async fn modify_value<'a, B: BlockStore>(
        self: Rc<Self>,
        hashbits: &mut HashBits<'a>,
        key: K,
        value: V,
        store: &B,
    ) -> Result<Rc<Self>> {
        let index = hashbits.next()?;

        // No existing values at this point.
        if !self.bitmask[index as usize] {
            let mut cloned = (*self).clone();
            cloned.insert_child(index as u32, key, value);
            return Ok(Rc::new(cloned));
        }

        let value_index = self.get_value_index(index as u32);
        match &self.pointers[value_index] {
            Pointer::Values(_) => todo!(),
            Pointer::NodeLink(link) => {
                // let child = link.get(store).await?;
                todo!()
            }
        }

        todo!()
    }

    // 0x55f...
    // 0x5
    // bitmask: 0000_0000_0001_0000
    // pointers: Values([(k, v)]) idx 0
    // 0x3...
    // bitmask: 0000_0000_0001_0100
    // pointers: Values([(k2, v2), (k, v)]) idx 0
    // 0x4
    // bitmask: 0000_0000_0001_1100
    //        & 0000_0000_0000_1111
    //        = 0000_0000_0000_1100 -> count_ones -> 2
    // pointers: Values([(k2, v2), (k3, v3), (k, v)]) idx 1

    pub fn insert_child(&mut self, index: u32, key: K, value: V) {
        self.pointers.insert(
            self.get_value_index(index),
            Pointer::Values(vec![(key, value)]),
        );

        self.bitmask.set(index as usize, true);
    }

    fn get_value_index(&self, bit_pos: u32) -> usize {
        let mut mask = bitarr!(u16, Lsb0; 1; 1);

        mask.shift_right(16 - bit_pos as usize);

        assert_eq!(mask.count_ones(), bit_pos as usize);

        (mask & self.bitmask).count_ones()
    }

    #[async_recursion(?Send)]
    pub(crate) async fn flush<'a, B: BlockStore>(self: &'a Rc<Self>, store: &mut B) -> Result<()> {
        for pointer in self.pointers.iter() {
            if let Pointer::NodeLink(link) = pointer {
                match link.get_cached() {
                    Some(node) => Rc::clone(node).flush(store).await?,
                    None => {}
                };
                link.seal(store).await?;
            }
        }

        Ok(())
    }

    pub async fn search<B: BlockStore>(key: &K, store: &B) -> Result<Option<V>> {
        todo!()
    }
}
