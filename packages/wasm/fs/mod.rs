use js_sys::Object;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn lookup_node(fs: &Object, path: &str, options: &Object) -> Result<Object, JsValue> {
    todo!("lookup_node should wrap original implementation")
}
