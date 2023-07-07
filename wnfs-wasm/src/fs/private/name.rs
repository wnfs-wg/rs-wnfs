use wasm_bindgen::prelude::wasm_bindgen;
use wnfs_nameaccumulator::{Name as WnfsName, NameAccumulator as WnfsNameAccumulator};

//--------------------------------------------------------------------------------------------------
// Type Definitions
//--------------------------------------------------------------------------------------------------

#[wasm_bindgen]
pub struct Name(pub(crate) WnfsName);

#[wasm_bindgen]
pub struct NameAccumulator(pub(crate) WnfsNameAccumulator);

//--------------------------------------------------------------------------------------------------
// Implementations
//--------------------------------------------------------------------------------------------------
