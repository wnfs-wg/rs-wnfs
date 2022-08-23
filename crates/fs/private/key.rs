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

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
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

        Ok([nonce_bytes.to_vec(), cipher_text].concat())
    }

    /// Decrypts the given ciphertext using the key.
    pub fn decrypt(&self, cipher_text: &[u8]) -> Result<Vec<u8>> {
        let (nonce_bytes, data) = cipher_text.split_at(NONCE_SIZE);

        Ok(Aes256Gcm::new(AesKey::from_slice(&self.0))
            .decrypt(Nonce::from_slice(nonce_bytes), data)
            .map_err(|e| FsError::UnableToDecrypt(format!("{}", e)))?)
    }

    /// Generates a nonce that can be used to encrypt data.
    #[inline]
    pub fn generate_nonce<R>(rng: &mut R) -> [u8; NONCE_SIZE]
    where
        R: Rng,
    {
        rng.random_bytes::<NONCE_SIZE>()
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
mod key_prop_tests {
    use crate::utils::ProptestRng;

    use super::*;
    use proptest::prelude::any;
    use proptest::test_runner::RngAlgorithm;
    use test_strategy::proptest;

    #[proptest(cases = 50)]
    fn key_can_encrypt_and_decrypt_data(
        #[strategy(any::<Vec<u8>>())] data: Vec<u8>,
        #[strategy(any::<[u8; 32]>())] rng_seed: [u8; 32],
        key_bytes: [u8; 32],
    ) {
        let key = Key::new(key_bytes);
        let rng = &mut ProptestRng::from_seed(RngAlgorithm::ChaCha, &rng_seed);

        let encrypted = key.encrypt(&Key::generate_nonce(rng), &data).unwrap();
        let decrypted = key.decrypt(&encrypted).unwrap();

        assert_eq!(decrypted, data);
    }
}
