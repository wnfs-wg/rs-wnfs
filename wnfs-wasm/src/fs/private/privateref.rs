use crate::fs::utils::{self, error};
use wasm_bindgen::prelude::wasm_bindgen;
use wnfs::{
    common::HASH_BYTE_SIZE,
    libipld::Cid,
    private::{PrivateRef as WnfsPrivateRef, TemporalKey, KEY_BYTE_SIZE},
};

//--------------------------------------------------------------------------------------------------
// Type Definitions
//--------------------------------------------------------------------------------------------------

#[wasm_bindgen]
pub struct PrivateRef {
    pub(crate) label: Vec<u8>,
    pub(crate) temporal_key: Vec<u8>,
    pub(crate) content_cid: Vec<u8>,
}

//--------------------------------------------------------------------------------------------------
// Implementations
//--------------------------------------------------------------------------------------------------

#[wasm_bindgen]
impl PrivateRef {
    #[wasm_bindgen(constructor)]
    pub fn new(label: Vec<u8>, temporal_key: Vec<u8>, content_cid: Vec<u8>) -> Self {
        Self {
            label,
            temporal_key,
            content_cid,
        }
    }

    #[wasm_bindgen(js_name = "getLabel")]
    pub fn get_label(&self) -> Vec<u8> {
        self.label.clone()
    }

    #[wasm_bindgen(js_name = "getTemporalKey")]
    pub fn get_temporal_key(&self) -> Vec<u8> {
        self.temporal_key.clone()
    }

    #[wasm_bindgen(js_name = "getContentCid")]
    pub fn get_content_cid(&self) -> Vec<u8> {
        self.content_cid.clone()
    }
}

impl TryInto<WnfsPrivateRef> for PrivateRef {
    type Error = js_sys::Error;

    fn try_into(self) -> Result<WnfsPrivateRef, Self::Error> {
        let PrivateRef {
            label,
            temporal_key,
            content_cid,
        } = self;
        let revision_name_hash = utils::expect_bytes::<HASH_BYTE_SIZE>(label)?;

        let key_bytes = utils::expect_bytes::<KEY_BYTE_SIZE>(temporal_key)?;
        let temporal_key = TemporalKey::from(key_bytes);

        let content_cid = Cid::try_from(content_cid).map_err(error("Error parsing CID"))?;
        Ok(WnfsPrivateRef {
            revision_name_hash,
            temporal_key,
            content_cid,
        })
    }
}

impl From<WnfsPrivateRef> for PrivateRef {
    fn from(private_ref: WnfsPrivateRef) -> Self {
        let WnfsPrivateRef {
            revision_name_hash,
            temporal_key,
            content_cid,
        } = private_ref;
        PrivateRef {
            label: Vec::from(revision_name_hash),
            temporal_key: Vec::from(temporal_key.0.bytes()),
            content_cid: content_cid.to_bytes(),
        }
    }
}
