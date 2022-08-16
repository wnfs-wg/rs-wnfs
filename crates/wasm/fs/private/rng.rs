use wasm_bindgen::prelude::wasm_bindgen;
use wnfs::private::Rng as WnfsRng;

//--------------------------------------------------------------------------------------------------
// Externs
//--------------------------------------------------------------------------------------------------

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "ExternRng")]
    pub type ExternRng;

    #[wasm_bindgen(static_method_of = ExternRng, js_name = "randomBytes")]
    pub fn extern_random_bytes(count: usize) -> Vec<u8>;
}

//--------------------------------------------------------------------------------------------------
// Implementations
//--------------------------------------------------------------------------------------------------

impl WnfsRng for ExternRng {
    fn random_bytes<const N: usize>() -> [u8; N] {
        ExternRng::extern_random_bytes(N).try_into().unwrap()
    }
}
