use std::{
    hash::{Hash, Hasher},
    ops::Index,
};

use anyhow::Result;
use bitvec::prelude::BitVec;
use twox_hash::XxHash32;

use crate::error;

use super::NameFilterError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BloomFilter<const N: u32> {
    pub(super) bits: BitVec,
    pub len: u32,
    pub params: BloomParams,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BloomParams {
    pub m_bytes: u32,
    pub k_hashes: u32,
    pub n_items: u32,
}

pub struct HashIndexIterator<'a, const N: u32, T: Hash> {
    filter: &'a BloomFilter<N>,
    m_bits: u32,
    item: &'a T,
    index: u32,
}

impl<'a, const N: u32, T: Hash> HashIndexIterator<'a, N, T> {
    pub(super) fn new(bloomfilter: &'a BloomFilter<N>, item: &'a T) -> Self {
        Self {
            filter: bloomfilter,
            m_bits: bloomfilter.params.m_bytes * 8,
            item,
            index: 0,
        }
    }
}

impl<const N: u32, T: Hash> Iterator for HashIndexIterator<'_, N, T> {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.filter.params.n_items {
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

impl<const N: u32> BloomFilter<N> {
    pub fn with_params(params: BloomParams) -> Self {
        Self {
            bits: BitVec::with_capacity(N as usize),
            params,
            len: 0,
        }
    }

    pub fn add<T>(&mut self, item: &T) -> Result<()>
    where
        T: Hash,
    {
        if self.len > self.params.n_items {
            return error(NameFilterError::MaxItemCountReached);
        }

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

impl<const N: u32> Index<u32> for BloomFilter<N> {
    type Output = bool;

    fn index(&self, index: u32) -> &Self::Output {
        &self.bits[index as usize]
    }
}

#[cfg(test)]
mod bloomfilter_tests {}
