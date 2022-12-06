use wasm_bindgen::prelude::wasm_bindgen;
use wnfs::private::Namefilter as WnfsNamefilter;

//--------------------------------------------------------------------------------------------------
// Type Definitions
//--------------------------------------------------------------------------------------------------

#[wasm_bindgen]
pub struct Namefilter(pub(crate) WnfsNamefilter);

//--------------------------------------------------------------------------------------------------
// Implementations
//--------------------------------------------------------------------------------------------------

#[wasm_bindgen]
impl Namefilter {
    /// Creates a new namefilter.
    #[wasm_bindgen(constructor)]
    #[allow(clippy::new_without_default)]
    pub fn new() -> Namefilter {
        Self(WnfsNamefilter::default())
    }
}
