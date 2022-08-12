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

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Key(pub(super) [u8; 32]);

//--------------------------------------------------------------------------------------------------
// Implementations
//--------------------------------------------------------------------------------------------------

impl Key {
    /// Creates a new key from [u8; 32].
    pub fn new(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    /// Encrypts the given plaintext using the key.
    pub fn encrypt(&self, nonce_bytes: &[u8; NONCE_SIZE], data: &[u8]) -> Result<Vec<u8>> {
        let nonce = Nonce::from_slice(nonce_bytes);

        let cipher_text = Aes256Gcm::new(AesKey::from_slice(&self.0))
            .encrypt(nonce, data)
            .map_err(|e| FsError::UnableToEncrypt(format!("{}", e)))?;

        Ok([cipher_text, nonce_bytes.to_vec()].concat())
    }

    /// Decrypts the given ciphertext using the key.
    pub fn decrypt(&self, cipher_text: &[u8]) -> Result<Vec<u8>> {
        let (data, nonce_bytes) = cipher_text.split_at(cipher_text.len() - NONCE_SIZE);

        Ok(Aes256Gcm::new(AesKey::from_slice(&self.0))
            .decrypt(Nonce::from_slice(nonce_bytes), data)
            .map_err(|e| FsError::UnableToDecrypt(format!("{}", e)))?)
    }

    /// Generates a nonce that can be used to encrypt data.
    #[inline]
    pub fn generate_nonce<R>() -> [u8; NONCE_SIZE]
    where
        R: Rng,
    {
        R::random_bytes::<NONCE_SIZE>()
    }

    /// Grabs the bytes of the key.
    pub fn bytes(self) -> [u8; 32] {
        self.0
    }

    /// Gets the bytes of the key.
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

impl Debug for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Key(0x{:02X?})", &self.0[..5])
    }
}

//--------------------------------------------------------------------------------------------------
// Tests
//--------------------------------------------------------------------------------------------------

#[cfg(test)]
mod key_tests {
    use crate::utils::Rand;

    use super::*;
    use test_strategy::proptest;

    #[proptest(cases = 50)]
    fn key_can_encrypt_and_decrypt_data(
        #[strategy("[A-Za-z0-9 ]{1,50}")] data: String,
        key_bytes: [u8; 32],
    ) {
        let key = Key::new(key_bytes);
        let data = data.as_bytes();

        let encrypted = key.encrypt(&Key::generate_nonce::<Rand>(), data).unwrap();
        let decrypted = key.decrypt(&encrypted).unwrap();

        assert_eq!(decrypted, data);
    }
}
