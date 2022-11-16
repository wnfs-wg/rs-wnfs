use std::{fmt::Debug, marker::PhantomData, rc::Rc};

use crate::{private::HAMT_VALUES_BUCKET_SIZE, AsyncSerialize, BlockStore, HashOutput, Link};
use anyhow::{bail, Result};
use async_recursion::async_recursion;
use async_trait::async_trait;
use bitvec::array::BitArray;

use futures::future::LocalBoxFuture;
use libipld::{serde as ipld_serde, Ipld};
use log::debug;
use serde::{
    de::{Deserialize, DeserializeOwned},
    ser::Error as SerError,
    Deserializer, Serialize, Serializer,
};
use sha3::Sha3_256;

use super::{
    error::HamtError,
    hash::{HashNibbles, Hasher},
    Pair, Pointer, HAMT_BITMASK_BIT_SIZE, HAMT_BITMASK_BYTE_SIZE,
};

//--------------------------------------------------------------------------------------------------
// Type Definitions
//--------------------------------------------------------------------------------------------------

/// The bitmask used by the HAMT which 16-bit, [u8; 2] type.
pub type BitMaskType = [u8; HAMT_BITMASK_BYTE_SIZE];

/// Represents a node in the HAMT tree structure.
///
/// # Examples
///
/// ```
/// use std::rc::Rc;
/// use wnfs::{private::Node, MemoryBlockStore};
///
/// let store = &mut MemoryBlockStore::new();
/// let node = Rc::new(Node::<String, usize>::default());
///
/// assert!(node.is_empty());
/// ```
#[derive(Debug, Clone)]
pub struct Node<K, V, H = Sha3_256>
where
    H: Hasher,
{
    pub(crate) bitmask: BitArray<BitMaskType>,
    pub(crate) pointers: Vec<Pointer<K, V, H>>,
    hasher: PhantomData<H>,
}

//--------------------------------------------------------------------------------------------------
// Implementations
//--------------------------------------------------------------------------------------------------

impl<K, V, H> Node<K, V, H>
where
    H: Hasher + Clone + 'static,
{
    /// Sets a new value at the given key.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::rc::Rc;
    /// use wnfs::{private::Node, MemoryBlockStore};
    ///
    /// #[async_std::main]
    /// async fn main() {
    ///     let store = &mut MemoryBlockStore::new();
    ///     let node = Rc::new(Node::<String, usize>::default());
    ///
    ///     let node = node.set("key".into(), 42, store).await.unwrap();
    ///     assert_eq!(node.get(&String::from("key"), store).await.unwrap(), Some(&42));
    /// }
    /// ```
    pub async fn set<B: BlockStore>(self: Rc<Self>, key: K, value: V, store: &B) -> Result<Rc<Self>>
    where
        K: DeserializeOwned + Clone + AsRef<[u8]>,
        V: DeserializeOwned + Clone,
    {
        let hash = &H::hash(&key);
        debug!("set: hash = {:02x?}", hash);
        self.set_value(&mut HashNibbles::new(hash), key, value, store)
            .await
    }

    /// Gets the value at the given key.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::rc::Rc;
    /// use wnfs::{private::Node, MemoryBlockStore};
    ///
    /// #[async_std::main]
    /// async fn main() {
    ///     let store = &mut MemoryBlockStore::new();
    ///     let node = Rc::new(Node::<String, usize>::default());
    ///
    ///     let node = node.set("key".into(), 42, store).await.unwrap();
    ///     assert_eq!(node.get(&String::from("key"), store).await.unwrap(), Some(&42));
    /// }
    /// ```
    pub async fn get<'a, B: BlockStore>(&'a self, key: &K, store: &B) -> Result<Option<&'a V>>
    where
        K: DeserializeOwned + AsRef<[u8]>,
        V: DeserializeOwned,
    {
        let hash = &H::hash(key);
        debug!("get: hash = {:02x?}", hash);
        Ok(self
            .get_value(&mut HashNibbles::new(hash), store)
            .await?
            .map(|pair| &pair.value))
    }

    /// Removes the value at the given key.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::rc::Rc;
    /// use wnfs::{private::Node, Pair, MemoryBlockStore};
    ///
    /// #[async_std::main]
    /// async fn main() {
    ///     let store = &mut MemoryBlockStore::new();
    ///     let node = Rc::new(Node::<String, usize>::default());
    ///
    ///     let node = node.set("key".into(), 42, store).await.unwrap();
    ///     assert_eq!(node.get(&String::from("key"), store).await.unwrap(), Some(&42));
    ///
    ///     let (node, value) = node.remove(&String::from("key"), store).await.unwrap();
    ///     assert_eq!(value, Some(Pair::new("key".into(), 42)));
    ///     assert_eq!(node.get(&String::from("key"), store).await.unwrap(), None);
    /// }
    /// ```
    pub async fn remove<B: BlockStore>(
        self: Rc<Self>,
        key: &K,
        store: &B,
    ) -> Result<(Rc<Self>, Option<Pair<K, V>>)>
    where
        K: DeserializeOwned + Clone + AsRef<[u8]>,
        V: DeserializeOwned + Clone,
    {
        let hash = &H::hash(key);
        debug!("remove: hash = {:02x?}", hash);
        self.remove_value(&mut HashNibbles::new(hash), store).await
    }

    /// Gets the value at the key matching the provided hash.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::rc::Rc;
    /// use sha3::Sha3_256;
    /// use wnfs::{private::Node, Hasher, MemoryBlockStore};
    ///
    /// #[async_std::main]
    /// async fn main() {
    ///     let store = &mut MemoryBlockStore::new();
    ///     let node = Rc::new(Node::<String, usize>::default());
    ///
    ///     let node = node.set("key".into(), 42, store).await.unwrap();
    ///
    ///     let key_hash = &Sha3_256::hash(&String::from("key"));
    ///     assert_eq!(node.get_by_hash(key_hash, store).await.unwrap(), Some(&42));
    /// }
    /// ```
    pub async fn get_by_hash<'a, B: BlockStore>(
        &'a self,
        hash: &HashOutput,
        store: &B,
    ) -> Result<Option<&'a V>>
    where
        K: DeserializeOwned + AsRef<[u8]>,
        V: DeserializeOwned,
    {
        debug!("get_by_hash: hash = {:02x?}", hash);
        Ok(self
            .get_value(&mut HashNibbles::new(hash), store)
            .await?
            .map(|pair| &pair.value))
    }

    /// Removes the value at the key matching the provided hash.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::rc::Rc;
    /// use sha3::Sha3_256;
    /// use wnfs::{private::Node, Hasher, Pair, MemoryBlockStore};
    ///
    /// #[async_std::main]
    /// async fn main() {
    ///     let store = &mut MemoryBlockStore::new();
    ///     let node = Rc::new(Node::<String, usize>::default());
    ///
    ///     let node = node.set("key".into(), 42, store).await.unwrap();
    ///     assert_eq!(node.get(&String::from("key"), store).await.unwrap(), Some(&42));
    ///
    ///     let key_hash = &Sha3_256::hash(&String::from("key"));
    ///     let (node, value) = node.remove_by_hash(key_hash, store).await.unwrap();
    ///
    ///     assert_eq!(value, Some(Pair::new("key".into(), 42)));
    ///     assert_eq!(node.get(&String::from("key"), store).await.unwrap(), None);
    /// }
    /// ```
    pub async fn remove_by_hash<B: BlockStore>(
        self: Rc<Self>,
        hash: &HashOutput,
        store: &B,
    ) -> Result<(Rc<Self>, Option<Pair<K, V>>)>
    where
        K: DeserializeOwned + Clone + AsRef<[u8]>,
        V: DeserializeOwned + Clone,
    {
        self.remove_value(&mut HashNibbles::new(hash), store).await
    }

    /// Checks if the node is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::rc::Rc;
    /// use wnfs::{private::Node, MemoryBlockStore};
    ///
    /// #[async_std::main]
    /// async fn main() {
    ///     let store = &mut MemoryBlockStore::new();
    ///
    ///     let node = Rc::new(Node::<String, usize>::default());
    ///     assert!(node.is_empty());
    ///
    ///     let node = node.set("key".into(), 42, store).await.unwrap();
    ///     assert!(!node.is_empty());
    /// }
    /// ```
    pub fn is_empty(&self) -> bool {
        self.bitmask.count_ones() == 0
    }

    /// Calculates the value index from the bitmask index.
    fn get_value_index(&self, bit_index: usize) -> usize {
        let shift_amount = HAMT_BITMASK_BIT_SIZE - bit_index;
        let mask = if shift_amount < HAMT_BITMASK_BIT_SIZE {
            let mut tmp = BitArray::<BitMaskType>::new([0xff, 0xff]);
            tmp.shift_left(shift_amount);
            tmp
        } else {
            BitArray::ZERO
        };
        assert_eq!(mask.count_ones(), bit_index);
        (mask & self.bitmask).count_ones()
    }

    fn set_value<'a, B: BlockStore>(
        self: Rc<Self>,
        hashnibbles: &'a mut HashNibbles,
        key: K,
        value: V,
        store: &'a B,
    ) -> LocalBoxFuture<'a, Result<Rc<Self>>>
    where
        K: DeserializeOwned + Clone + AsRef<[u8]> + 'a,
        V: DeserializeOwned + Clone + 'a,
        H: 'a,
    {
        Box::pin(async move {
            let bit_index = hashnibbles.try_next()?;
            let value_index = self.get_value_index(bit_index);

            debug!(
                "set_value: bit_index = {}, value_index = {}",
                bit_index, value_index
            );

            let mut node = Rc::try_unwrap(self).unwrap_or_else(|rc| (*rc).clone());

            // If the bit is not set yet, insert a new pointer.
            if !node.bitmask[bit_index] {
                node.pointers
                    .insert(value_index, Pointer::Values(vec![Pair { key, value }]));

                node.bitmask.set(bit_index, true);

                return Ok(Rc::new(node));
            }

            match &mut node.pointers[value_index] {
                Pointer::Values(values) => {
                    if let Some(i) = values
                        .iter()
                        .position(|p| &H::hash(&p.key) == hashnibbles.digest)
                    {
                        // If the key is already present, update the value.
                        values[i] = Pair::new(key, value);
                    } else {
                        // Otherwise, insert the new value if bucket is not full. Create new node if it is.
                        if values.len() < HAMT_VALUES_BUCKET_SIZE {
                            // Insert in order of key.
                            let index = values
                                .iter()
                                .position(|p| &H::hash(&p.key) > hashnibbles.digest)
                                .unwrap_or(values.len());
                            values.insert(index, Pair::new(key, value));
                        } else {
                            // If values has reached threshold, we need to create a node link that splits it.
                            let mut sub_node = Rc::new(Node::<K, V, H>::default());
                            let cursor = hashnibbles.get_cursor();
                            // We can take because
                            // Pointer::Values() gets replaced with Pointer::Link at the end
                            let values = std::mem::take(values);
                            for Pair { key, value } in
                                values.into_iter().chain(Some(Pair::new(key, value)))
                            {
                                let hash = &H::hash(&key);
                                let hashnibbles = &mut HashNibbles::with_cursor(hash, cursor);
                                sub_node =
                                    sub_node.set_value(hashnibbles, key, value, store).await?;
                            }
                            node.pointers[value_index] = Pointer::Link(Link::from(sub_node));
                        }
                    }
                }
                Pointer::Link(link) => {
                    let child = Rc::clone(link.resolve_value(store).await?);
                    let child = child.set_value(hashnibbles, key, value, store).await?;
                    node.pointers[value_index] = Pointer::Link(Link::from(child));
                }
            }

            Ok(Rc::new(node))
        })
    }

    #[async_recursion(?Send)]
    async fn get_value<'a, B: BlockStore>(
        &'a self,
        hashnibbles: &mut HashNibbles,
        store: &B,
    ) -> Result<Option<&'a Pair<K, V>>>
    where
        K: DeserializeOwned + AsRef<[u8]>,
        V: DeserializeOwned,
    {
        let bit_index = hashnibbles.try_next()?;

        // If the bit is not set yet, return None.
        if !self.bitmask[bit_index] {
            return Ok(None);
        }

        let value_index = self.get_value_index(bit_index);
        match &self.pointers[value_index] {
            Pointer::Values(values) => Ok({
                values
                    .iter()
                    .find(|p| &H::hash(&p.key) == hashnibbles.digest)
            }),
            Pointer::Link(link) => {
                let child = link.resolve_value(store).await?;
                child.get_value(hashnibbles, store).await
            }
        }
    }

    // It's internal and is only more complex because async_recursion doesn't work here
    #[allow(clippy::type_complexity)]
    fn remove_value<'k, 'v, 'a, B: BlockStore>(
        self: Rc<Self>,
        hashnibbles: &'a mut HashNibbles,
        store: &'a B,
    ) -> LocalBoxFuture<'a, Result<(Rc<Node<K, V, H>>, Option<Pair<K, V>>)>>
    where
        K: DeserializeOwned + Clone + AsRef<[u8]> + 'k,
        V: DeserializeOwned + Clone + 'v,
        'k: 'a,
        'v: 'a,
    {
        Box::pin(async move {
            let bit_index = hashnibbles.try_next()?;

            // If the bit is not set yet, return None.
            if !self.bitmask[bit_index] {
                return Ok((self, None));
            }

            let value_index = self.get_value_index(bit_index);

            let mut node = Rc::try_unwrap(self).unwrap_or_else(|rc| (*rc).clone());

            let removed = match &mut node.pointers[value_index] {
                Pointer::Values(values) => {
                    if values.len() == 1 {
                        // If the key doesn't match, return without removing.
                        if &H::hash(&values[0].key) != hashnibbles.digest {
                            None
                        } else {
                            // If there is only one value, we can remove the entire pointer.
                            node.bitmask.set(bit_index, false);
                            match node.pointers.remove(value_index) {
                                Pointer::Values(mut values) => Some(values.pop().unwrap()),
                                _ => unreachable!(),
                            }
                        }
                    } else {
                        // Otherwise, remove just the value.
                        match values
                            .iter()
                            .position(|p| &H::hash(&p.key) == hashnibbles.digest)
                        {
                            Some(i) => {
                                let value = values.remove(i);
                                // We can take here because we replace the node.pointers here afterwards anyway
                                let values = std::mem::take(values);
                                node.pointers[value_index] = Pointer::Values(values);
                                Some(value)
                            }
                            None => None,
                        }
                    }
                }
                Pointer::Link(link) => {
                    let child = Rc::clone(link.resolve_value(store).await?);
                    let (child, removed) = child.remove_value(hashnibbles, store).await?;
                    if removed.is_some() {
                        // If something has been deleted, we attempt toc canonicalize the pointer.
                        if let Some(pointer) =
                            Pointer::Link(Link::from(child)).canonicalize(store).await?
                        {
                            node.pointers[value_index] = pointer;
                        } else {
                            // This is None if the pointer now points to an empty node.
                            // In that case, we remove it from the parent.
                            node.bitmask.set(bit_index, false);
                            node.pointers.remove(value_index);
                        }
                    } else {
                        node.pointers[value_index] = Pointer::Link(Link::from(child))
                    };
                    removed
                }
            };
            Ok((Rc::new(node), removed))
        })
    }
}

impl<K, V, H: Hasher> Node<K, V, H> {
    /// Returns the count of the values in all the values pointer of a node.
    pub fn count_values(self: &Rc<Self>) -> Result<usize> {
        let mut len = 0;
        for i in self.pointers.iter() {
            if let Pointer::Values(values) = i {
                len += values.len();
            } else {
                bail!(HamtError::ValuesPointerExpected);
            }
        }

        Ok(len)
    }

    // TODO(appcypher): Do we really need this? Why not use PublicDirectorySerializable style instead.
    /// Converts a Node to an IPLD object.
    pub async fn to_ipld<B: BlockStore + ?Sized>(&self, store: &mut B) -> Result<Ipld>
    where
        K: Serialize,
        V: Serialize,
    {
        let bitmask_ipld = ipld_serde::to_ipld(self.bitmask.as_raw_slice())?;
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

impl<K, V, H: Hasher> Default for Node<K, V, H> {
    fn default() -> Self {
        Node {
            bitmask: BitArray::ZERO,
            pointers: Vec::with_capacity(HAMT_BITMASK_BIT_SIZE),
            hasher: PhantomData,
        }
    }
}

#[async_trait(?Send)]
impl<K, V, H> AsyncSerialize for Node<K, V, H>
where
    K: Serialize,
    V: Serialize,
    H: Hasher,
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

impl<'de, K, V, H> Deserialize<'de> for Node<K, V, H>
where
    K: DeserializeOwned,
    V: DeserializeOwned,
    H: Hasher,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let (bitmask, pointers): (BitMaskType, _) = Deserialize::deserialize(deserializer)?;
        Ok(Node {
            bitmask: BitArray::<BitMaskType>::from(bitmask),
            pointers,
            hasher: PhantomData,
        })
    }
}

impl<K, V, H> PartialEq for Node<K, V, H>
where
    K: PartialEq,
    V: PartialEq,
    H: Hasher,
{
    fn eq(&self, other: &Self) -> bool {
        self.bitmask == other.bitmask && self.pointers == other.pointers
    }
}

//--------------------------------------------------------------------------------------------------
// Tests
//--------------------------------------------------------------------------------------------------

#[cfg(test)]
mod hamt_node_unit_tests {
    use super::*;
    use crate::{HashOutput, MemoryBlockStore};
    use lazy_static::lazy_static;
    use test_log::test;

    fn digest(bytes: &[u8]) -> HashOutput {
        let mut nibbles = [0u8; 32];
        nibbles[..bytes.len()].copy_from_slice(bytes);
        nibbles
    }

    lazy_static! {
        static ref HASH_KV_PAIRS: Vec<(HashOutput, &'static str)> = vec![
            (digest(&[0xE0]), "first"),
            (digest(&[0xE1]), "second"),
            (digest(&[0xE2]), "third"),
            (digest(&[0xE3]), "fourth"),
        ];
    }

    #[derive(Debug, Clone)]
    struct MockHasher;
    impl Hasher for MockHasher {
        fn hash<K: AsRef<[u8]>>(key: &K) -> HashOutput {
            let s = std::str::from_utf8(key.as_ref()).unwrap();
            HASH_KV_PAIRS.iter().find(|(_, v)| s == *v).unwrap().0
        }
    }

    #[test(async_std::test)]
    async fn get_value_fetches_deeply_linked_value() {
        let store = &mut MemoryBlockStore::default();

        // Insert 4 values to trigger the creation of a linked node.
        let mut working_node = Rc::new(Node::<String, String, MockHasher>::default());
        for (digest, kv) in HASH_KV_PAIRS.iter() {
            let hashnibbles = &mut HashNibbles::new(digest);
            working_node = working_node
                .set_value(hashnibbles, kv.to_string(), kv.to_string(), store)
                .await
                .unwrap();
        }

        // Get the values.
        for (digest, kv) in HASH_KV_PAIRS.iter() {
            let hashnibbles = &mut HashNibbles::new(digest);
            let value = working_node.get_value(hashnibbles, store).await.unwrap();

            assert_eq!(value, Some(&Pair::new(kv.to_string(), kv.to_string())));
        }
    }

    #[test(async_std::test)]
    async fn remove_value_canonicalizes_linked_node() {
        let store = &mut MemoryBlockStore::default();

        // Insert 4 values to trigger the creation of a linked node.
        let mut working_node = Rc::new(Node::<String, String, MockHasher>::default());
        for (digest, kv) in HASH_KV_PAIRS.iter() {
            let hashnibbles = &mut HashNibbles::new(digest);
            working_node = working_node
                .set_value(hashnibbles, kv.to_string(), kv.to_string(), store)
                .await
                .unwrap();
        }

        assert_eq!(working_node.pointers.len(), 1);

        // Remove the third value.
        let third_hashnibbles = &mut HashNibbles::new(&HASH_KV_PAIRS[2].0);
        working_node = working_node
            .remove_value(third_hashnibbles, store)
            .await
            .unwrap()
            .0;

        // Check that the third value is gone.
        match &working_node.pointers[0] {
            Pointer::Values(values) => {
                assert_eq!(values.len(), 3);
            }
            _ => panic!("Expected values pointer"),
        }

        let value = working_node
            .get_value(third_hashnibbles, store)
            .await
            .unwrap();

        assert!(value.is_none());
    }

    #[test(async_std::test)]
    async fn set_value_splits_when_bucket_threshold_reached() {
        let store = &mut MemoryBlockStore::default();

        // Insert 3 values into the HAMT.
        let mut working_node = Rc::new(Node::<String, String, MockHasher>::default());
        for (idx, (digest, kv)) in HASH_KV_PAIRS.iter().take(3).enumerate() {
            let kv = kv.to_string();
            let hashnibbles = &mut HashNibbles::new(digest);
            working_node = working_node
                .set_value(hashnibbles, kv.clone(), kv.clone(), store)
                .await
                .unwrap();

            match &working_node.pointers[0] {
                Pointer::Values(values) => {
                    assert_eq!(values.len(), idx + 1);
                    assert_eq!(values[idx].key, kv.clone());
                    assert_eq!(values[idx].value, kv.clone());
                }
                _ => panic!("Expected values pointer"),
            }
        }

        // Inserting the fourth value should introduce a link indirection.
        working_node = working_node
            .set_value(
                &mut HashNibbles::new(&HASH_KV_PAIRS[3].0),
                "fourth".to_string(),
                "fourth".to_string(),
                store,
            )
            .await
            .unwrap();

        match &working_node.pointers[0] {
            Pointer::Link(link) => {
                let node = link.get_value().unwrap();
                assert_eq!(node.bitmask.count_ones(), 4);
                assert_eq!(node.pointers.len(), 4);
            }
            _ => panic!("Expected link pointer"),
        }
    }

    #[test(async_std::test)]
    async fn get_value_index_gets_correct_index() {
        let store = &mut MemoryBlockStore::default();
        let hash_expected_idx_samples = [
            (&[0x00], 0),
            (&[0x20], 1),
            (&[0x10], 1),
            (&[0x30], 3),
            (&[0x50], 4),
            (&[0x60], 5),
            (&[0x70], 6),
            (&[0x40], 4),
            (&[0x80], 8),
            (&[0xA0], 9),
            (&[0xB0], 10),
            (&[0xC0], 11),
            (&[0x90], 9),
            (&[0xE0], 13),
            (&[0xD0], 13),
            (&[0xF0], 15),
        ];

        let mut working_node = Rc::new(Node::<String, String>::default());
        for (hash, expected_idx) in hash_expected_idx_samples.into_iter() {
            let bytes = digest(&hash[..]);
            let hashnibbles = &mut HashNibbles::new(&bytes);

            working_node = working_node
                .set_value(
                    hashnibbles,
                    expected_idx.to_string(),
                    expected_idx.to_string(),
                    store,
                )
                .await
                .unwrap();

            assert_eq!(
                working_node.pointers[expected_idx],
                Pointer::Values(vec![Pair::new(
                    expected_idx.to_string(),
                    expected_idx.to_string()
                )])
            );
        }
    }

    #[test(async_std::test)]
    async fn node_can_insert_pair_and_retrieve() {
        let mut store = MemoryBlockStore::default();
        let node = Rc::new(Node::<String, (i32, f64)>::default());

        let node = node
            .set("pill".into(), (10, 0.315), &mut store)
            .await
            .unwrap();

        let value = node.get(&"pill".into(), &store).await.unwrap().unwrap();

        assert_eq!(value, &(10, 0.315));
    }

    #[test(async_std::test)]
    async fn node_is_same_with_irrelevant_remove() {
        // These two keys' hashes have the same first nibble (7)
        let insert_key: String = "GL59 Tg4phDb  bv".into();
        let remove_key: String = "hK i3b4V4152EPOdA".into();

        let store = &mut MemoryBlockStore::default();
        let mut node0: Rc<Node<String, u64>> = Rc::new(Node::default());

        node0 = node0.set(insert_key.clone(), 0, store).await.unwrap();
        (node0, _) = node0.remove(&remove_key, store).await.unwrap();

        assert_eq!(node0.count_values().unwrap(), 1);
    }

    #[test(async_std::test)]
    async fn node_history_independence_regression() {
        let store = &mut MemoryBlockStore::default();

        let mut node1: Rc<Node<String, u64>> = Rc::new(Node::default());
        let mut node2: Rc<Node<String, u64>> = Rc::new(Node::default());

        node1 = node1.set("key 17".into(), 508, store).await.unwrap();
        node1 = node1.set("key 81".into(), 971, store).await.unwrap();
        node1 = node1.set("key 997".into(), 365, store).await.unwrap();
        (node1, _) = node1.remove(&"key 17".into(), store).await.unwrap();
        node1 = node1.set("key 68".into(), 870, store).await.unwrap();
        node1 = node1.set("key 304".into(), 331, store).await.unwrap();

        node2 = node2.set("key 81".into(), 971, store).await.unwrap();
        node2 = node2.set("key 17".into(), 508, store).await.unwrap();
        node2 = node2.set("key 997".into(), 365, store).await.unwrap();
        node2 = node2.set("key 304".into(), 331, store).await.unwrap();
        node2 = node2.set("key 68".into(), 870, store).await.unwrap();
        (node2, _) = node2.remove(&"key 17".into(), store).await.unwrap();

        let cid1 = store.put_async_serializable(&node1).await.unwrap();
        let cid2 = store.put_async_serializable(&node2).await.unwrap();

        assert_eq!(cid1, cid2);
    }
}

#[cfg(test)]
mod hamt_node_prop_tests {

    use crate::private::hamt::strategies::*;
    use proptest::prelude::*;
    use test_strategy::proptest;

    use crate::{dagcbor, MemoryBlockStore};

    use super::*;

    fn small_key() -> impl Strategy<Value = String> {
        (0..1000).prop_map(|i| format!("key {i}"))
    }

    #[proptest(cases = 50)]
    fn test_insert_idempotence(
        #[strategy(operations(small_key(), 0u64..1000, 0..100))] operations: Operations<
            String,
            u64,
        >,
        #[strategy(small_key())] key: String,
        #[strategy(0..1000u64)] value: u64,
    ) {
        async_std::task::block_on(async move {
            let store = &mut MemoryBlockStore::default();
            let node = node_from_operations(operations, store).await.unwrap();

            let node = node.set(key.clone(), value, store).await.unwrap();
            let cid1 = store.put_async_serializable(&node).await.unwrap();

            let node = node.set(key, value, store).await.unwrap();
            let cid2 = store.put_async_serializable(&node).await.unwrap();

            assert_eq!(cid1, cid2);
        })
    }

    #[proptest(cases = 50)]
    fn test_remove_idempotence(
        #[strategy(operations(small_key(), 0u64..1000, 0..100))] operations: Operations<
            String,
            u64,
        >,
        #[strategy(small_key())] key: String,
    ) {
        async_std::task::block_on(async move {
            let store = &mut MemoryBlockStore::default();
            let node = node_from_operations(operations, store).await.unwrap();

            let (node, _) = node.remove(&key, store).await.unwrap();
            let cid1 = store.put_async_serializable(&node).await.unwrap();

            let (node, _) = node.remove(&key, store).await.unwrap();
            let cid2 = store.put_async_serializable(&node).await.unwrap();

            assert_eq!(cid1, cid2);
        })
    }

    #[proptest(cases = 100)]
    fn node_can_encode_decode_as_cbor(
        #[strategy(operations(small_key(), 0u64..1000, 0..1000))] operations: Operations<
            String,
            u64,
        >,
    ) {
        async_std::task::block_on(async move {
            let store = &mut MemoryBlockStore::default();
            let node = node_from_operations(operations, store).await.unwrap();

            let encoded_node = dagcbor::async_encode(&node, store).await.unwrap();
            let decoded_node = dagcbor::decode::<Node<String, u64>>(encoded_node.as_ref()).unwrap();

            assert_eq!(*node, decoded_node);
        })
    }

    #[proptest(cases = 1000, max_shrink_iters = 10_000)]
    fn node_operations_are_history_independent(
        #[strategy(operations_and_shuffled(small_key(), 0u64..1000, 0..100))] pair: (
            Operations<String, u64>,
            Operations<String, u64>,
        ),
    ) {
        async_std::task::block_on(async move {
            let (original, shuffled) = pair;

            let store = &mut MemoryBlockStore::default();

            let node1 = node_from_operations(original, store).await.unwrap();
            let node2 = node_from_operations(shuffled, store).await.unwrap();

            let cid1 = store.put_async_serializable(&node1).await.unwrap();
            let cid2 = store.put_async_serializable(&node2).await.unwrap();

            assert_eq!(cid1, cid2);
        })
    }

    // This is sort of a "control group" for making sure that operations_and_shuffled is correct.
    #[proptest(cases = 200, max_shrink_iters = 10_000)]
    fn hash_map_is_history_independent(
        #[strategy(operations_and_shuffled(small_key(), 0u64..1000, 0..1000))] pair: (
            Operations<String, u64>,
            Operations<String, u64>,
        ),
    ) {
        let (original, shuffled) = pair;

        let map1 = hash_map_from_operations(original);
        let map2 = hash_map_from_operations(shuffled);

        assert_eq!(map1, map2);
    }
}
