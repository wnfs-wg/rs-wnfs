use super::BloomFilter;

//--------------------------------------------------------------------------------------------------
// Constants
//--------------------------------------------------------------------------------------------------

pub const SATURATION_THRESHOLD: usize = 1019;

//--------------------------------------------------------------------------------------------------
// Type Definitions
//--------------------------------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Namefilter(BloomFilter<256>);

//--------------------------------------------------------------------------------------------------
// Implementations
//--------------------------------------------------------------------------------------------------

impl Namefilter {
    pub fn new() -> Self {
        Self(BloomFilter::new(30))
    }

    // Add hashes to filter until is is over the threshold.
    pub fn saturate(&mut self) {
        let pop_count = self.0.count_ones();
        if pop_count >= SATURATION_THRESHOLD {
            return;
        }
    }
}

impl AsRef<[u8]> for Namefilter {
    fn as_ref(&self) -> &[u8] {
        self.0.bits.as_raw_slice()
    }
}

impl Default for Namefilter {
    fn default() -> Self {
        Self::new()
    }
}
