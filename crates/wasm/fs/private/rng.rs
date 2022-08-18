use wasm_bindgen::prelude::wasm_bindgen;
use wnfs::private::Rng as WnfsRng;

//--------------------------------------------------------------------------------------------------
// Externs
//--------------------------------------------------------------------------------------------------

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "Rng")]
    pub type Rng;

    #[wasm_bindgen(method, js_name = "randomBytes")]
    pub fn random_bytes(this: &Rng, count: usize) -> Vec<u8>;
}

//--------------------------------------------------------------------------------------------------
// Implementations
//--------------------------------------------------------------------------------------------------

impl WnfsRng for Rng {
    fn random_bytes<const N: usize>(&self) -> [u8; N] {
        self.random_bytes(N).try_into().unwrap()
    }
}
