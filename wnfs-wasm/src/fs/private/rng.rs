use rand_core::{CryptoRng, CryptoRngCore};
use wasm_bindgen::prelude::wasm_bindgen;

//--------------------------------------------------------------------------------------------------
// Externs
//--------------------------------------------------------------------------------------------------

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "Rng")]
    pub type Rng;

    #[wasm_bindgen(method, js_name = "randomBytes")]
    pub fn get_random_bytes(this: &Rng, count: usize) -> Vec<u8>;
}

//--------------------------------------------------------------------------------------------------
// Implementations
//--------------------------------------------------------------------------------------------------

impl CryptoRngCore for Rng {
    fn next_u32(&mut self) -> u32 {
        let bytes = self.get_random_bytes(4);
        u32::from_le_bytes(bytes.try_into().unwrap())
    }

    fn next_u64(&mut self) -> u64 {
        let bytes = self.get_random_bytes(8);
        u64::from_le_bytes(bytes.try_into().unwrap())
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        dest.copy_from_slice(&self.get_random_bytes(dest.len()));
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand_core::Error> {
        self.fill_bytes(dest);
        Ok(())
    }
}

impl CryptoRng for Rng {}
