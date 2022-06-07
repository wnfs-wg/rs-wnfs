use std::rc::Rc;

use anyhow::Result;
use bitvec::array::BitArray;
use bitvec::bitarr;
use bitvec::order::Lsb0;
use serde::{Serialize, Serializer};
use sha3::{Digest, Sha3_256};

use crate::BlockStore;

use super::{hashbits::HashBits, Pointer};

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

// key = 0x83279d6298d6a9f
// // 0x83479d6298d6a9f
// // 0x81279d6298d6a9f

// Node{
//     bitmask: 0000_0001_0000_0000
//     pointers: vec![Values(vec![value])]
// }

impl<K, V> Node<K, V>
where
    K: AsRef<[u8]> + Clone,
    V: Clone,
{
    pub async fn set<B: BlockStore>(
        self: Rc<Self>,
        key: K,
        value: V,
        store: &mut B,
    ) -> Result<Rc<Self>> {
        let hash = &hash(&key);
        let mut hashbits = HashBits::new(hash);
        self.modify_value(&mut hashbits, key, value, store).await?;

        todo!()
    }

    pub async fn get<B: BlockStore>(key: &K, store: &B) {
        let hash = hash(key);

        todo!()
    }

    pub async fn modify_value<'a, B: BlockStore>(
        self: Rc<Self>,
        hashed_key: &mut HashBits<'a>,
        key: K,
        value: V,
        store: &B,
    ) -> Result<Rc<Self>> {
        let index = hashed_key.next(4)?;

        // No existing values at this point.
        if !self.bitmask[index as usize] {
            let mut cloned = (*self).clone();
            cloned.insert_child(index, key, value);
            return Ok(Rc::new(cloned));
        }

        // let value_index = self.get_value_index(index);
        // match self.pointers[value_index] {
        //     Pointer::Values(_) => todo!(),
        //     Pointer::NodeLink(link) => {
        //         let child = link.get(store).await?;
        //     }
        // }

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

        (mask & &self.bitmask).count_ones()
    }

    pub async fn search<B: BlockStore>(key: &K, store: &B) -> Result<Option<V>> {
        todo!()
    }
}

pub fn hash<K: AsRef<[u8]>>(key: &K) -> Vec<u8> {
    let mut hasher = Sha3_256::default();
    hasher.update(key.as_ref());
    hasher.finalize().as_slice().to_vec()
}
