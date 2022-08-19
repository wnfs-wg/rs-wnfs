use std::rc::Rc;

use wasm_bindgen::prelude::wasm_bindgen;
use wnfs::private::PrivateForest as WnfsPrivateForest;

//--------------------------------------------------------------------------------------------------
// Type Definitions
//--------------------------------------------------------------------------------------------------

/// A HAMT forest in a WNFS public file system.
#[wasm_bindgen]
pub struct PrivateForest(pub(crate) Rc<WnfsPrivateForest>);
