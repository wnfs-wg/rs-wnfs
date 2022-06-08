use std::{
    hash::{Hash, Hasher},
    ops::Index,
};

use anyhow::Result;
use bitvec::prelude::BitArray;
use twox_hash::XxHash32;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BloomFilter<const N: usize> {
    pub(super) bits: BitArray<[u8; N]>,
    pub len: u32,
    pub params: BloomParams,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BloomParams {
    pub m_bytes: u32,
    pub k_hashes: u32,
}

pub struct HashIndexIterator<'a, const N: usize, T: Hash> {
    filter: &'a BloomFilter<N>,
    m_bits: u32,
    item: &'a T,
    index: u32,
}

impl<'a, const N: usize, T: Hash> HashIndexIterator<'a, N, T> {
    pub(super) fn new(bloomfilter: &'a BloomFilter<N>, item: &'a T) -> Self {
        Self {
            filter: bloomfilter,
            m_bits: bloomfilter.params.m_bytes * 8,
            item,
            index: 0,
        }
    }
}

impl<const N: usize, T: Hash> Iterator for HashIndexIterator<'_, N, T> {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.filter.params.k_hashes {
            let hash = {
                let mut h = XxHash32::with_seed(self.index);
                self.item.hash(&mut h);
                h.finish()
            };

            let value = (hash % self.m_bits as u64) as u32;
            self.index += 1;
            Some(value)
        } else {
            None
        }
    }
}

impl<const N: usize> BloomFilter<N> {
    pub fn with_params(params: BloomParams) -> Self {
        Self {
            bits: BitArray::<[u8; N], _>::ZERO,
            params,
            len: 0,
        }
    }

    pub fn add<T>(&mut self, item: &T) -> Result<()>
    where
        T: Hash,
    {
        let indices = self.hash_indices(item).collect::<Vec<_>>();
        for i in indices {
            self.bits.set(i as usize, true);
        }

        Ok(())
    }

    pub fn contains<T>(&self, item: &T) -> bool
    where
        T: Hash,
    {
        for i in self.hash_indices(item) {
            if !self.bits[i as usize] {
                return false;
            }
        }

        true
    }

    pub fn count_ones(&self) -> usize {
        self.bits.count_ones()
    }

    #[inline(always)]
    pub fn hash_indices<'a, T>(&'a self, item: &'a T) -> HashIndexIterator<'a, N, impl Hash>
    where
        T: Hash,
    {
        HashIndexIterator::new(self, item)
    }

    #[inline(always)]
    pub fn len(&self) -> u32 {
        self.len
    }

    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}

impl<const N: usize> Index<u32> for BloomFilter<N> {
    type Output = bool;

    fn index(&self, index: u32) -> &Self::Output {
        &self.bits[index as usize]
    }
}

#[cfg(test)]
mod bloomfilter_tests {}
