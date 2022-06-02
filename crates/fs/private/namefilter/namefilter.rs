use super::{BloomFilter, BloomParams};

// TODO(appcypher): What is the magic number? Should this be 1024 if we want half-filled bloom filters?
pub const SATURATION_THRESHOLD: usize = 1019;

pub type Namefilter = BloomFilter<256>;

impl Namefilter {
    pub fn new() -> Self {
        Self::with_params(BloomParams {
            m_bytes: 256,
            k_hashes: 30,
            n_items: 1024,
        })
    }

    // Add random items in filter until is is over the threhshold.
    pub fn saturate(&mut self) {
        let pop_count = self.count_ones();
        if pop_count >= SATURATION_THRESHOLD {
            return;
        }

        // TODO(appcypher): Implement saturation.
        todo!();
    }
}

impl Default for Namefilter {
    fn default() -> Self {
        Self::new()
    }
}
