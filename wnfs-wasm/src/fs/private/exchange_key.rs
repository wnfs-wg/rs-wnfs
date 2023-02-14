use anyhow::Result;
use async_trait::async_trait;
use js_sys::{Promise, Uint8Array};
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen_futures::JsFuture;
use wnfs::private::{ExchangeKey as WnfsExchangeKey, PrivateKey as WnfsPrivateKey};

use crate::fs::utils::anyhow_error;

//--------------------------------------------------------------------------------------------------
// Externs
//--------------------------------------------------------------------------------------------------

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "ExchangeKey")]
    pub type ExchangeKey;

    #[wasm_bindgen(static_method_of = ExchangeKey, js_name = "fromModulus")]
    pub(crate) fn from_modulus(modulus: &[u8]) -> Promise;

    #[wasm_bindgen(method)]
    pub(crate) fn encrypt(key: &ExchangeKey, data: &[u8]) -> Promise;
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "PrivateKey")]
    pub type PrivateKey;

    #[wasm_bindgen(method)]
    pub(crate) fn decrypt(key: &PrivateKey, ciphertext: &[u8]) -> Promise;
}

//--------------------------------------------------------------------------------------------------
// Type Definitions
//--------------------------------------------------------------------------------------------------

#[wasm_bindgen]
pub struct ForeignExchangeKey(pub(crate) ExchangeKey);

#[wasm_bindgen]
pub struct ForeignPrivateKey(pub(crate) PrivateKey);

//--------------------------------------------------------------------------------------------------
// Implementations
//--------------------------------------------------------------------------------------------------

#[async_trait(?Send)]
impl WnfsExchangeKey for ForeignExchangeKey {
    async fn from_modulus(modulus: &[u8]) -> Result<Self>
    where
        Self: Sized,
    {
        let v = JsFuture::from(ExchangeKey::from_modulus(modulus))
            .await
            .map_err(anyhow_error("Cannot create from modulus: {:?}"))?;

        Ok(ForeignExchangeKey(v.into()))
    }

    async fn encrypt(&self, data: &[u8]) -> Result<Vec<u8>> {
        let v = JsFuture::from(self.0.encrypt(data))
            .await
            .map_err(anyhow_error("Cannot encrypt: {:?}"))?;

        Ok(Uint8Array::new(&v).to_vec())
    }
}

#[async_trait(?Send)]
impl WnfsPrivateKey for ForeignPrivateKey {
    async fn decrypt(&self, ciphertext: &[u8]) -> Result<Vec<u8>> {
        let v = JsFuture::from(self.0.decrypt(ciphertext))
            .await
            .map_err(anyhow_error("Cannot create from modukus: {:?}"))?;

        Ok(Uint8Array::new(&v).to_vec())
    }
}
