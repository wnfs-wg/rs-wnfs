use std::{fmt::Debug, ops::Index};

use anyhow::anyhow;
use bitvec::prelude::BitArray;
use serde::{Deserialize, Serialize};
use xxhash_rust::xxh3;

use wnfs_common::utils::ByteArrayVisitor;

//------------------------------------------------------------------------------
// Type Definitions
//------------------------------------------------------------------------------

/// The bloom filter is a probabilistic data structure that can be used to store a set of hashes.
///
/// `N` is the size of the bloom filter in bytes.
///
/// `K` is the number of bits to be set with each add operation.
///
/// # Examples
///
/// ```
/// use wnfs::private::BloomFilter;
///
/// let mut filter = BloomFilter::<256, 30>::default();
/// filter.add(&[0xF5u8; 32]);
///
/// assert!(filter.contains(&[0xF5u8; 32]));
/// ```
#[derive(Clone, PartialEq, Eq, PartialOrd, Hash)]
pub struct BloomFilter<const N: usize, const K: usize> {
    pub(super) bits: BitArray<[u8; N]>,
}

/// An iterator that generates indices into some bloom filter based on deterministic hashing of specified item.
///
/// `N` is the number of bytes in the bloom filter.
/// This is used to restrict generated value within bloomfilter index space bounds.
///
/// # Examples
///
/// ```
/// use wnfs::private::BloomFilter;
///
/// let filter = BloomFilter::<256, 30>::default();
/// let indices = filter.hash_indices(&[0xF5u8; 32]);
/// let indices = indices.collect::<Vec<_>>();
///
/// assert_eq!(indices.len(), 30);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HashIndexIterator<'a, T: AsRef<[u8]>, const N: usize> {
    item: &'a T,
    index: u64,
}

//------------------------------------------------------------------------------
// Implementations
//------------------------------------------------------------------------------

impl<'a, T: AsRef<[u8]>, const N: usize> HashIndexIterator<'a, T, N> {
    /// Creates a new iterator.
    pub(super) fn new(item: &'a T) -> Self {
        Self { item, index: 0 }
    }

    /// Returns the size of the bloom filter in bits.
    const fn bit_size() -> usize {
        N * 8
    }
}

impl<T: AsRef<[u8]>, const N: usize> Iterator for HashIndexIterator<'_, T, N> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let hash = xxh3::xxh3_64_with_seed(self.item.as_ref(), self.index) as usize;
        let value = hash % Self::bit_size();
        self.index += 1;
        Some(value)
    }
}

impl<const N: usize, const K: usize> BloomFilter<N, K> {
    /// Creates a new bloom filter with all bits unset.
    ///
    /// # Examples
    ///
    /// ```
    /// use wnfs::private::BloomFilter;
    ///
    /// let mut filter = BloomFilter::<256, 30>::new();
    /// filter.add(&[0xF5u8; 32]);
    ///
    /// assert!(filter.contains(&[0xF5u8; 32]));
    /// ```
    pub fn new() -> Self {
        Self {
            bits: Default::default(),
        }
    }

    /// Adds an item to the bloom filter.
    ///
    /// # Examples
    ///
    /// ```
    /// use wnfs::private::BloomFilter;
    ///
    /// let mut filter = BloomFilter::<256, 30>::default();
    /// filter.add(&[0xF5u8; 32]);
    ///
    /// assert!(filter.contains(&[0xF5u8; 32]));
    /// ```
    pub fn add<T>(&mut self, item: &T)
    where
        T: AsRef<[u8]>,
    {
        for i in self.hash_indices(item) {
            self.bits.set(i, true);
        }
    }

    /// Returns the number of hash iterations the bloom filter uses to set bits.
    ///
    /// # Examples
    ///
    /// ```
    /// use wnfs::private::BloomFilter;
    ///
    /// let mut filter = BloomFilter::<256, 30>::default();
    ///
    /// assert_eq!(filter.num_iterations(), 30);
    /// ```
    pub const fn num_iterations(&self) -> usize {
        K
    }

    /// Checks if the item is in the bloom filter.
    ///
    /// # Examples
    ///
    /// ```
    /// use wnfs::private::BloomFilter;
    ///
    /// let mut filter = BloomFilter::<256, 30>::default();
    /// filter.add(&[0xF5u8; 32]);
    ///
    /// assert!(filter.contains(&[0xF5u8; 32]));
    /// ```
    pub fn contains<T>(&self, item: &T) -> bool
    where
        T: AsRef<[u8]>,
    {
        self.hash_indices(item).all(|i| self.bits[i])
    }

    /// Counts the number of bits set in the bloom filter.
    ///
    /// # Examples
    ///
    /// ```
    /// use wnfs::private::BloomFilter;
    ///
    /// let mut filter = BloomFilter::<256, 30>::default();
    /// filter.add(&[0xF5u8; 32]);
    ///
    /// assert_eq!(filter.count_ones(), 30);
    /// ```
    pub fn count_ones(&self) -> usize {
        self.bits.count_ones()
    }

    /// Returns the indices of the bits that would be set if the item was added to the bloom filter.
    ///
    /// # Examples
    ///
    /// ```
    /// use wnfs::private::BloomFilter;
    ///
    /// let filter = BloomFilter::<256, 30>::default();
    /// let indices = filter.hash_indices(&[0xF5u8; 32]);
    /// let indices = indices.collect::<Vec<_>>();
    ///
    /// assert_eq!(indices.len(), 30);
    /// ```
    #[inline]
    pub fn hash_indices<'a, T>(&self, item: &'a T) -> impl Iterator<Item = usize> + 'a
    where
        T: AsRef<[u8]>,
    {
        HashIndexIterator::<_, N>::new(item).take(self.num_iterations())
    }

    /// Get the bytes of the bloom filter.
    ///
    /// # Examples
    ///
    /// ```
    /// use wnfs::private::BloomFilter;
    ///
    /// let mut filter = BloomFilter::<256, 30>::default();
    /// filter.add(&[0xF5u8; 32]);
    ///
    /// let bytes = filter.as_bytes();
    /// assert_eq!(bytes.len(), 256);
    /// ```
    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        self.bits.as_raw_slice()
    }
}

impl<const N: usize, const K: usize> TryFrom<Vec<u8>> for BloomFilter<N, K> {
    type Error = anyhow::Error;

    fn try_from(bytes: Vec<u8>) -> Result<Self, Self::Error> {
        let bits = BitArray::<[u8; N]>::new(bytes.try_into().map_err(|e: Vec<u8>| {
            anyhow!(
                "Cannot convert vector to BloomFilter: Expected length {}",
                e.len()
            )
        })?);
        Ok(Self { bits })
    }
}

impl<const N: usize, const K: usize> Index<usize> for BloomFilter<N, K> {
    type Output = bool;

    fn index(&self, index: usize) -> &Self::Output {
        &self.bits[index]
    }
}

impl<const N: usize, const K: usize> Default for BloomFilter<N, K> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl<const N: usize, const K: usize> Serialize for BloomFilter<N, K> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_bytes(self.bits.as_raw_slice())
    }
}

impl<'de, const N: usize, const K: usize> Deserialize<'de> for BloomFilter<N, K> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(BloomFilter::<N, K> {
            bits: BitArray::<[u8; N]>::new(deserializer.deserialize_bytes(ByteArrayVisitor::<N>)?),
        })
    }
}

impl<const N: usize, const K: usize> Debug for BloomFilter<N, K> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "0x")?;
        for (i, byte) in self.as_bytes().iter().enumerate() {
            if i > 32 {
                write!(f, "..")?;
                break;
            } else {
                write!(f, "{byte:02X}")?;
            }
        }

        Ok(())
    }
}

//------------------------------------------------------------------------------
// Tests
//------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use libipld::serde as ipld_serde;

    #[test]
    fn bloom_filter_can_add_and_validate_item_existence() {
        let mut bloom = BloomFilter::<256, 30>::new();
        let items: Vec<String> = vec!["first".into(), "second".into(), "third".into()];
        items.iter().for_each(|item| {
            bloom.add(item);
        });

        items.iter().for_each(|item| {
            assert!(bloom.contains(item));
        });

        assert!(!bloom.contains(b"irst"));
        assert!(!bloom.contains(b"secnd"));
        assert!(!bloom.contains(b"tird"));
    }

    #[test]
    fn serialized_bloom_filter_can_be_deserialized_correctly() {
        let mut bloom = BloomFilter::<256, 30>::new();
        let items: Vec<String> = vec!["first".into(), "second".into(), "third".into()];
        items.iter().for_each(|item| {
            bloom.add(item);
        });

        let ipld = ipld_serde::to_ipld(&bloom).unwrap();
        let deserialized: BloomFilter<256, 30> = ipld_serde::from_ipld(ipld).unwrap();

        assert_eq!(deserialized, bloom);
    }
}

#[cfg(test)]
mod proptests {
    use proptest::prop_assert_eq;
    use test_strategy::proptest;

    use super::HashIndexIterator;

    #[proptest]
    fn iterator_can_give_unbounded_number_of_indices(#[strategy(0usize..500)] count: usize) {
        let iter = HashIndexIterator::<_, 200>::new(&"hello");

        let indices = (0..20)
            .map(|_| (iter.clone().take(count).collect::<Vec<_>>(), count))
            .collect::<Vec<_>>();

        for (indices, count) in indices {
            prop_assert_eq!(indices.len(), count);
        }
    }
}
