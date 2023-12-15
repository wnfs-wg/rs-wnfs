use crate::fs::{utils::error, JsResult, PrivateForest};
use std::rc::Rc;
use wasm_bindgen::prelude::wasm_bindgen;
use wnfs::private::forest::traits::PrivateForest as WnfsPrivateForest;
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

#[wasm_bindgen]
impl Name {
    #[wasm_bindgen(constructor)]
    pub fn new(accumulator: &NameAccumulator) -> Name {
        Self(WnfsName::new(accumulator.0.clone(), []))
    }

    #[wasm_bindgen(js_name = "toNameAccumulator")]
    pub fn to_name_accumulator(&self, forest: &PrivateForest) -> NameAccumulator {
        let name = &self.0;
        let forest = Rc::clone(&forest.0);
        let accumulator = name.into_accumulator(&forest.get_accumulator_setup());
        return NameAccumulator(accumulator);
    }
}

#[wasm_bindgen]
impl NameAccumulator {
    #[wasm_bindgen(js_name = "fromBytes")]
    pub fn from_bytes(bytes: Vec<u8>) -> JsResult<NameAccumulator> {
        let accumulator = WnfsNameAccumulator::parse_bytes(bytes)
            .map_err(error("Couldn't parse name accumulator"))?;
        Ok(Self(accumulator))
    }

    #[wasm_bindgen(js_name = "toBytes")]
    pub fn to_bytes(&self) -> Vec<u8> {
        self.0.as_bytes().to_vec()
    }
}
