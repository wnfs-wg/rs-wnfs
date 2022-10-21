use std::fmt::Debug;

use aes_gcm::{
    aead::{Aead, NewAead},
    Aes256Gcm, Key as AesKey, Nonce,
};
use anyhow::Result;
use rand_core::RngCore;
use serde::{Deserialize, Serialize};

use crate::{utils, FsError};

//--------------------------------------------------------------------------------------------------
// Contants
//--------------------------------------------------------------------------------------------------

pub(crate) const NONCE_SIZE: usize = 12;
pub const KEY_BYTE_SIZE: usize = 32;

//--------------------------------------------------------------------------------------------------
// Type Definitions
//--------------------------------------------------------------------------------------------------

/// A key used for encryption and decryption. This is a wrapper around a 32 byte AES key.
///
/// # Examples
///
/// ```
/// use wnfs::{private::Key, utils};
/// use rand::thread_rng;
///
/// let rng = &mut thread_rng();
/// let key = Key::new(utils::get_random_bytes(rng));
///
/// println!("Key: {:?}", key);
/// ```
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Key(pub(super) [u8; KEY_BYTE_SIZE]);

//--------------------------------------------------------------------------------------------------
// Implementations
//--------------------------------------------------------------------------------------------------

impl Key {
    /// Creates a new key from [u8; KEY_SIZE].
    ///
    /// # Examples
    ///
    /// ```
    /// use wnfs::{private::Key, utils};
    /// use rand::thread_rng;
    ///
    /// let rng = &mut thread_rng();
    /// let key = Key::new(utils::get_random_bytes(rng));
    ///
    /// println!("Key: {:?}", key);
    /// ```
    pub fn new(bytes: [u8; KEY_BYTE_SIZE]) -> Self {
        Self(bytes)
    }

    /// Encrypts the given plaintext using the key.
    ///
    /// # Examples
    ///
    /// ```
    /// use wnfs::{private::Key, utils};
    /// use rand::thread_rng;
    ///
    /// let rng = &mut thread_rng();
    /// let key = Key::new(utils::get_random_bytes(rng));
    /// let nonce = Key::generate_nonce(rng);
    ///
    /// let plaintext = b"Hello World!";
    /// let ciphertext = key.encrypt(&nonce, plaintext).unwrap();
    /// let decrypted = key.decrypt(&ciphertext).unwrap();
    ///
    /// assert_eq!(plaintext, &decrypted[..]);
    /// ```
    pub fn encrypt(&self, nonce_bytes: &[u8; NONCE_SIZE], data: &[u8]) -> Result<Vec<u8>> {
        let nonce = Nonce::from_slice(nonce_bytes);

        let cipher_text = Aes256Gcm::new(AesKey::from_slice(&self.0))
            .encrypt(nonce, data)
            .map_err(|e| FsError::UnableToEncrypt(format!("{}", e)))?;

        Ok([nonce_bytes.to_vec(), cipher_text].concat())
    }

    /// Decrypts the given ciphertext using the key.
    ///
    /// # Examples
    ///
    /// ```
    /// use wnfs::{private::Key, utils};
    /// use rand::thread_rng;
    ///
    /// let rng = &mut thread_rng();
    /// let key = Key::new(utils::get_random_bytes(rng));
    /// let nonce = Key::generate_nonce(rng);
    ///
    /// let plaintext = b"Hello World!";
    /// let ciphertext = key.encrypt(&nonce, plaintext).unwrap();
    /// let decrypted = key.decrypt(&ciphertext).unwrap();
    ///
    /// assert_eq!(plaintext, &decrypted[..]);
    /// ```
    pub fn decrypt(&self, cipher_text: &[u8]) -> Result<Vec<u8>> {
        let (nonce_bytes, data) = cipher_text.split_at(NONCE_SIZE);

        Ok(Aes256Gcm::new(AesKey::from_slice(&self.0))
            .decrypt(Nonce::from_slice(nonce_bytes), data)
            .map_err(|e| FsError::UnableToDecrypt(format!("{}", e)))?)
    }

    /// Generates a nonce that can be used to encrypt data.
    ///
    /// # Examples
    ///
    /// ```
    /// use wnfs::{private::Key, utils};
    /// use rand::thread_rng;
    ///
    /// let rng = &mut thread_rng();
    /// let nonce = Key::generate_nonce(rng);
    ///
    /// println!("Nonce: {:?}", nonce);
    /// ```
    #[inline]
    pub fn generate_nonce<R>(rng: &mut R) -> [u8; NONCE_SIZE]
    where
        R: RngCore,
    {
        utils::get_random_bytes::<NONCE_SIZE>(rng)
    }

    /// Grabs the bytes of the key.
    pub fn bytes(self) -> [u8; KEY_BYTE_SIZE] {
        self.0
    }

    /// Gets the bytes of the key.
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

impl Debug for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "0x")?;
        for (i, byte) in self.0.iter().enumerate() {
            if i > 6 {
                write!(f, "..")?;
                break;
            } else {
                write!(f, "{:02X}", byte)?;
            }
        }

        Ok(())
    }
}

//--------------------------------------------------------------------------------------------------
// Tests
//--------------------------------------------------------------------------------------------------

#[cfg(test)]
mod key_prop_tests {
    use super::*;
    use proptest::{
        prelude::any,
        test_runner::{RngAlgorithm, TestRng},
    };
    use test_strategy::proptest;

    #[proptest(cases = 100)]
    fn key_can_encrypt_and_decrypt_data(
        #[strategy(any::<Vec<u8>>())] data: Vec<u8>,
        #[strategy(any::<[u8; KEY_BYTE_SIZE]>())] rng_seed: [u8; KEY_BYTE_SIZE],
        key_bytes: [u8; KEY_BYTE_SIZE],
    ) {
        let key = Key::new(key_bytes);
        let rng = &mut TestRng::from_seed(RngAlgorithm::ChaCha, &rng_seed);

        let encrypted = key.encrypt(&Key::generate_nonce(rng), &data).unwrap();
        let decrypted = key.decrypt(&encrypted).unwrap();

        assert_eq!(decrypted, data);
    }
}
