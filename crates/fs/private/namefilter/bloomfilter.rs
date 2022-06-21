use std::{
    hash::{Hash, Hasher},
    ops::Index,
};

use bitvec::prelude::BitArray;
use twox_hash::XxHash32;

//------------------------------------------------------------------------------
// Type Definitions
//------------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BloomFilter<const N: usize> {
    pub(super) bits: BitArray<[u8; N]>,
    pub k_hashes: usize,
}

pub struct HashIndexIterator<'a, T: Hash> {
    m_bits: usize,
    item: &'a T,
    index: usize,
}

//------------------------------------------------------------------------------
// Implementations
//------------------------------------------------------------------------------

impl<'a, T: Hash> HashIndexIterator<'a, T> {
    pub(super) fn new(m_bits: usize, item: &'a T) -> Self {
        Self {
            m_bits,
            item,
            index: 0,
        }
    }
}

impl<T: Hash> Iterator for HashIndexIterator<'_, T> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let hash = {
            let mut h = XxHash32::with_seed(self.index as u32);
            self.item.hash(&mut h);
            h.finish()
        };

        let value = (hash % self.m_bits as u64) as usize;
        self.index += 1;
        Some(value)
    }
}

impl<const N: usize> BloomFilter<N> {
    pub fn new(k_hashes: usize) -> Self {
        Self {
            bits: BitArray::<[u8; N], _>::ZERO,
            k_hashes,
        }
    }

    pub fn add<T>(&mut self, item: &T)
    where
        T: Hash,
    {
        for i in self.hash_indices(item) {
            self.bits.set(i, true);
        }
    }

    pub fn contains<T>(&self, item: &T) -> bool
    where
        T: Hash,
    {
        for i in self.hash_indices(item) {
            if !self.bits[i] {
                return false;
            }
        }

        true
    }

    pub fn count_ones(&self) -> usize {
        self.bits.count_ones()
    }

    #[inline(always)]
    pub fn hash_indices<'a, T>(&self, item: &'a T) -> impl Iterator<Item = usize> + 'a
    where
        T: Hash,
    {
        HashIndexIterator::new(N * 8, item).take(self.k_hashes as usize)
    }
}

impl<const N: usize> Index<usize> for BloomFilter<N> {
    type Output = bool;

    fn index(&self, index: usize) -> &Self::Output {
        &self.bits[index]
    }
}

//------------------------------------------------------------------------------
// Tests
//------------------------------------------------------------------------------

#[cfg(test)]
mod bloomfilter_tests {
    use super::*;

    #[test]
    fn test_bloomfilter_() {
        let mut bloom = BloomFilter::<256>::new(30);
        bloom.add(&"hello");
    }
}
