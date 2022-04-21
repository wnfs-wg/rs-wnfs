use wasm_bindgen::prelude::wasm_bindgen;
use wnfs::{
    public::{Id, PublicNode as WnfsPublicNode},
    Shared,
};

use super::PublicDirectory;

/// Wraps a shared<PublicNode>
#[wasm_bindgen]
pub struct SharedNode(pub(crate) Shared<WnfsPublicNode>);

#[wasm_bindgen]
impl SharedNode {
    #[wasm_bindgen(js_name = "asDir")]
    pub fn as_dir(&self) -> PublicDirectory {
        let node = self.0.borrow();
        let dir = node.as_dir();
        PublicDirectory(dir.clone())
    }

    #[wasm_bindgen(js_name = "isDir")]
    pub fn is_dir(&self) -> bool {
        self.0.borrow().is_dir()
    }

    #[wasm_bindgen(js_name = "getId")]
    pub fn get_id(&self) -> String {
        let node = self.0.borrow();
        node.get_id()
    }
}
