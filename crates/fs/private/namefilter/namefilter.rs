use super::{BloomFilter, BloomParams};

pub const SATURATION_THRESHOLD: usize = 1019;

pub type Namefilter = BloomFilter<2048>;

impl Namefilter {
    pub fn new() -> Self {
        Self::with_params(BloomParams {
            m_bytes: 256,
            k_hashes: 30,
        })
    }

    // Add random items in filter until is is over the threhshold.
    pub fn saturate(&mut self) {
        let pop_count = self.count_ones();
        if pop_count >= SATURATION_THRESHOLD {
            return;
        }
    }
}

impl AsRef<[u8]> for Namefilter {
    fn as_ref(&self) -> &[u8] {
        self.bits.as_raw_slice()
    }
}

impl Default for Namefilter {
    fn default() -> Self {
        Self::new()
    }
}
