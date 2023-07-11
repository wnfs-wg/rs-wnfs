use wasm_bindgen::prelude::wasm_bindgen;
use wnfs::private::AccessKey as WnfsAccessKey;

//--------------------------------------------------------------------------------------------------
// Type Definitions
//--------------------------------------------------------------------------------------------------

#[wasm_bindgen]
pub struct AccessKey(pub(crate) WnfsAccessKey);

//--------------------------------------------------------------------------------------------------
// Implementations
//--------------------------------------------------------------------------------------------------

#[wasm_bindgen]
impl AccessKey {
    #[wasm_bindgen(js_name = "getLabel")]
    pub fn get_label(&self) -> Vec<u8> {
        self.0.get_label().to_vec()
    }

    #[wasm_bindgen(js_name = "getTemporalKey")]
    pub fn get_temporal_key(&self) -> Vec<u8> {
        self.0.get_temporal_key().unwrap().0.to_vec()
    }

    #[wasm_bindgen(js_name = "getContentCid")]
    pub fn get_content_cid(&self) -> Vec<u8> {
        self.0.get_content_cid().to_bytes()
    }

    #[wasm_bindgen(js_name = "toBytes")]
    pub fn into_bytes(&self) -> Vec<u8> {
        Vec::<u8>::from(&self.0)
    }

    #[wasm_bindgen(js_name = "fromBytes")]
    pub fn from_bytes(bytes: &[u8]) -> Self {
        Self(WnfsAccessKey::from(bytes).into())
    }
}
