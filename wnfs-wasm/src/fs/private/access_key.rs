use crate::fs::{JsResult, utils::error};
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
    /// Return the label in the forest, used for
    /// accessing the ciphertext that can be decrypted with
    /// this access key.
    #[wasm_bindgen(js_name = "getLabel")]
    pub fn get_label(&self) -> Vec<u8> {
        self.0.get_label().to_vec()
    }

    /// Returns the temporal key or null, in case this
    /// access key only gives access to the shapshot level.
    #[wasm_bindgen(js_name = "getTemporalKey")]
    pub fn get_temporal_key(&self) -> Option<Vec<u8>> {
        self.0
            .get_temporal_key()
            .ok()
            .map(|k| k.as_bytes().to_vec())
    }

    /// Returns the snapshot key.
    /// May derive the key on-the-fly in case this
    /// AccessKey also gives access to the temporal access level.
    #[wasm_bindgen(js_name = "getSnapshotKey")]
    pub fn get_snapshot_key(&self) -> Vec<u8> {
        self.0.get_snapshot_key().as_bytes().to_vec()
    }

    /// Return the CID of what this access key decrypts.
    /// This is mainly used for disambiguation, in case the
    /// label the AccessKey links to has multiple conflicting writes.
    #[wasm_bindgen(js_name = "getContentCid")]
    pub fn get_content_cid(&self) -> Vec<u8> {
        self.0.get_content_cid().to_bytes()
    }

    /// Serialize this AccessKey into bytes.
    /// This will contain secret key material!
    /// Make sure to keep safe or encrypt
    /// (e.g. using the WebCrypto and asymmetrically encrypting these bytes).
    #[wasm_bindgen(js_name = "toBytes")]
    pub fn into_bytes(&self) -> JsResult<Vec<u8>> {
        let bytes = self
            .0
            .to_bytes()
            .map_err(error("Couldn't serialize access key"))?;
        Ok(bytes)
    }

    /// Deserialize an AccessKey previously generated from `into_bytes`.
    #[wasm_bindgen(js_name = "fromBytes")]
    pub fn from_bytes(bytes: &[u8]) -> JsResult<AccessKey> {
        let access_key = WnfsAccessKey::parse(bytes).map_err(error("Couldn't parse access key"))?;
        Ok(Self(access_key))
    }
}
