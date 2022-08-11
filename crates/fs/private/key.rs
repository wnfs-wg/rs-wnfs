use std::fmt::Debug;

use aes_gcm::aead::{Aead, NewAead};
use aes_gcm::{Aes256Gcm, Key as AesKey, Nonce};
use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::FsError;

use super::Rng;

//--------------------------------------------------------------------------------------------------
// Contants
//--------------------------------------------------------------------------------------------------

pub(crate) const NONCE_SIZE: usize = 12;

//--------------------------------------------------------------------------------------------------
// Type Definitions
//--------------------------------------------------------------------------------------------------

#[derive(Clone, Serialize, Deserialize)]
pub struct Key(pub(super) Vec<u8>); // TODO(appcypher): Make this [u8; 32];

//--------------------------------------------------------------------------------------------------
// Implementations
//--------------------------------------------------------------------------------------------------

impl Key {
    pub fn encrypt(&self, nonce_bytes: &[u8; NONCE_SIZE], data: &[u8]) -> Result<Vec<u8>> {
        let nonce = Nonce::from_slice(nonce_bytes);

        let cipher_text = Aes256Gcm::new(AesKey::from_slice(&self.0))
            .encrypt(nonce, data)
            .map_err(|e| FsError::UnableToEncrypt(format!("{}", e)))?;

        Ok([cipher_text, nonce_bytes.to_vec()].concat())
    }

    pub fn decrypt(&self, cipher_text: &[u8]) -> Result<Vec<u8>> {
        let (data, nonce_bytes) = cipher_text.split_at(cipher_text.len() - NONCE_SIZE);

        Ok(Aes256Gcm::new(AesKey::from_slice(&self.0))
            .decrypt(Nonce::from_slice(nonce_bytes), data)
            .map_err(|e| FsError::UnableToDecrypt(format!("{}", e)))?)
    }

    #[inline]
    pub fn generate_nonce<R>() -> [u8; NONCE_SIZE]
    where
        R: Rng,
    {
        R::random_bytes::<NONCE_SIZE>()
    }

    pub fn bytes(self) -> Vec<u8> {
        self.0
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

impl<T: AsRef<[u8]>> From<T> for Key {
    fn from(bytes: T) -> Self {
        Self(bytes.as_ref().to_vec())
    }
}

impl Debug for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Key(0x{:02X?})", &self.0[..5])
    }
}
