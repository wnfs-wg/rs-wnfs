use wasm_bindgen::prelude::wasm_bindgen;
use wnfs::private::PrivateForest as WnfsPrivateForest;

//--------------------------------------------------------------------------------------------------
// Type Definitions
//--------------------------------------------------------------------------------------------------

/// A directory in a WNFS public file system.
#[wasm_bindgen]
pub struct PrivateForest(WnfsPrivateForest);

//--------------------------------------------------------------------------------------------------
// Implementations
//--------------------------------------------------------------------------------------------------
