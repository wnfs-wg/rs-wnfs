use sha3::{
    digest::{ExtendableOutput, Update, XofReader},
    Shake256,
};

use super::BloomFilter;

//--------------------------------------------------------------------------------------------------
// Constants
//--------------------------------------------------------------------------------------------------

pub const SATURATION_THRESHOLD: usize = 1019;
pub const HASH_BYTE_SIZE: usize = 32;

//--------------------------------------------------------------------------------------------------
// Type Definitions
//--------------------------------------------------------------------------------------------------

/// Namefilters are 2048-bit bloom filters.
///
/// In WNFS they represent the identity key of a file or directory, doubling as a store for checking the ancestor of the file or directory.
///
/// # Examples
///
/// ```
/// use wnfs::Namefilter;
///
/// let mut filter = Namefilter::default();
/// filter.add(&[0xF5u8; 32]);
///
/// assert!(filter.contains(&[0xF5u8; 32]));
/// ```
pub type Namefilter = BloomFilter<256, 30>;

//--------------------------------------------------------------------------------------------------
// Implementations
//--------------------------------------------------------------------------------------------------

impl Namefilter {
    /// Adds hashes to filter until is is over the saturation threshold.
    ///
    /// # Examples
    ///
    /// ```
    /// use wnfs::{private::SATURATION_THRESHOLD, Namefilter};
    ///
    /// let mut filter = Namefilter::default();
    /// filter.add(&[0xF5u8; 32]);
    /// filter.saturate();
    ///
    /// assert!(filter.contains(&[0xF5u8; 32]));
    /// assert!(filter.count_ones() <= SATURATION_THRESHOLD);
    /// ```
    pub fn saturate(&mut self) {
        let mut xof = {
            let mut h = Shake256::default();
            h.update(self.as_bytes());
            h.finalize_xof()
        };

        let hash = &mut [0u8; HASH_BYTE_SIZE];
        loop {
            xof.read(hash);
            let mut clone = self.clone();
            clone.add(hash);
            if clone.count_ones() > SATURATION_THRESHOLD {
                break;
            }
            *self = clone
        }
    }
}

impl AsRef<[u8]> for Namefilter {
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

//--------------------------------------------------------------------------------------------------
// Tests
//--------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn saturate_not_greater_than_threshold() {
        let namefilters = (0..47)
            .map(|i| {
                let mut namefilter = Namefilter::new();
                for i in 0..i {
                    namefilter.add(&[i as u8]);
                }
                namefilter.saturate();
                namefilter
            })
            .collect::<Vec<_>>();

        for namefilter in namefilters {
            assert!(namefilter.count_ones() <= SATURATION_THRESHOLD);
        }
    }
}
