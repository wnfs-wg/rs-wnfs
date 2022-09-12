use rand_core::RngCore;

pub trait Rng: RngCore {
    fn random_bytes<const N: usize>(&mut self) -> [u8; N] {
        let mut bytes = [0u8; N];
        self.fill_bytes(&mut bytes);
        bytes
    }
}

#[cfg(test)]
impl crate::private::Rng for proptest::test_runner::TestRng {}
