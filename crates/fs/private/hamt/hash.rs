// TODO(appcypher): Based on ipld_hamt implementation

use anyhow::Result;
use sha3::{Digest, Sha3_256};

use crate::error;

use super::error::HamtError;

//--------------------------------------------------------------------------------------------------
// Constants
//--------------------------------------------------------------------------------------------------

const HASH_BYTES: usize = 32;
const MAX_CURSOR_DEPTH: usize = HASH_BYTES * 2;

//--------------------------------------------------------------------------------------------------
// Type Definition
//--------------------------------------------------------------------------------------------------

pub type HashOutput = [u8; HASH_BYTES];

pub trait GenerateHash {
    fn generate_hash<K: AsRef<[u8]>>(key: &K) -> HashOutput; // &[u8]
}

pub struct HashQuartets<'a> {
    digest: &'a HashOutput,
    cursor: usize,
}

//--------------------------------------------------------------------------------------------------
// Implementation
//--------------------------------------------------------------------------------------------------

impl<'a> HashQuartets<'a> {
    pub fn new(digest: &'a HashOutput) -> HashQuartets<'a> {
        Self::with_cursor(digest, 0)
    }

    /// Constructs hash quartets with custom cursor index.
    pub fn with_cursor(digest: &'a HashOutput, cursor: usize) -> HashQuartets<'a> {
        Self { digest, cursor }
    }

    pub fn next(&mut self) -> Result<u8> {
        if self.cursor >= MAX_CURSOR_DEPTH {
            return error(HamtError::CursorOutOfBounds);
        }

        let byte = self.digest[self.cursor / 2];
        let byte = if self.cursor % 2 == 0 {
            byte & 0b0000_1111
        } else {
            byte >> 4
        };

        self.cursor += 1;
        Ok(byte)
    }
}

impl GenerateHash for Sha3_256 {
    fn generate_hash<K: AsRef<[u8]>>(key: &K) -> HashOutput {
        let mut hasher = Self::default();
        hasher.update(key.as_ref());
        hasher.finalize().into()
    }
}

//--------------------------------------------------------------------------------------------------
// Tests
//--------------------------------------------------------------------------------------------------

#[cfg(test)]
mod hash_quartets_tests {
    use super::*;

    #[test]
    fn hash_quartets_can_cursor_over_digest() {
        let mut key = [0u8; HASH_BYTES];
        key[0] = 0b10001000;
        key[1] = 0b10101010;
        key[2] = 0b10111111;
        key[3] = 0b11111111;
        let mut hb = HashQuartets::new(&key);
        assert_eq!(hb.next().unwrap(), 0b1000);
        assert_eq!(hb.next().unwrap(), 0b1000);
        assert_eq!(hb.next().unwrap(), 0b1010);
        assert_eq!(hb.next().unwrap(), 0b1010);
        assert_eq!(hb.next().unwrap(), 0b1111);
        assert_eq!(hb.next().unwrap(), 0b1011);
        assert_eq!(hb.next().unwrap(), 0b1111);
        assert_eq!(hb.next().unwrap(), 0b1111);
        for _ in 0..(MAX_CURSOR_DEPTH - 8) {
            assert!(hb.next().is_ok());
        }
        assert!(hb.next().is_err());
    }
}
