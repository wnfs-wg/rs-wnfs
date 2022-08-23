use std::rc::Rc;

use wasm_bindgen::prelude::wasm_bindgen;
use wnfs::private::PrivateForest as WnfsPrivateForest;

//--------------------------------------------------------------------------------------------------
// Type Definitions
//--------------------------------------------------------------------------------------------------

/// A HAMT forest in a WNFS public file system.
#[wasm_bindgen]
pub struct PrivateForest(pub(crate) Rc<WnfsPrivateForest>);

//--------------------------------------------------------------------------------------------------
// Implementations
//--------------------------------------------------------------------------------------------------

#[wasm_bindgen]
impl PrivateForest {
    /// Creates a new HAMT forest.
    #[wasm_bindgen(constructor)]
    #[allow(clippy::new_without_default)]
    pub fn new() -> PrivateForest {
        Self(Rc::new(WnfsPrivateForest::default()))
    }
}
