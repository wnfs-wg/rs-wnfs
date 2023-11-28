use crate::error::CryptError;
use aes_kw::KekAes256;
use anyhow::{anyhow, Result};
use blake3::traits::digest::Digest;
use chacha20poly1305::{
    aead::{Aead, AeadCore, KeyInit},
    AeadInPlace, Tag, XChaCha20Poly1305, XNonce,
};
use rand_core::CryptoRngCore;
use serde::{Deserialize, Serialize};
use skip_ratchet::Ratchet;
use std::fmt::Debug;
use wnfs_common::utils;

//--------------------------------------------------------------------------------------------------
// Constants
//--------------------------------------------------------------------------------------------------

/// The size of the nonce used when encrypting using snapshot keys.
/// The algorithm used is XChaCha20-Poly1305, i.e. the extended nonce variant,
/// so it's 196 bit.
pub(crate) const NONCE_SIZE: usize = 24;
/// The size of the authentication tag used when encrypting using snapshot keys.
/// The algorithm used is XChaCha20-Poly1305, so it's 128 bit.
pub(crate) const AUTHENTICATION_TAG_SIZE: usize = 16;
/// The general key size used in WNFS: 256-bit
pub const KEY_BYTE_SIZE: usize = 32;

/// The revision segment derivation domain separation info
/// used for salting the hashing function when turning
/// node names into revisioned node names.
pub(crate) const REVISION_SEGMENT_DSI: &str = "wnfs/1.0/revision segment derivation from ratchet";
/// The hiding segment derivation domain separation info
/// used for salting the hashing function when generating
/// the hiding segment from the external file content key,
/// which is added to a file's name to generate the external content base name.
/// This domain separation string is not part of the standard, as the standard leaves
/// the way the base name is derived open to implementations.
pub(crate) const HIDING_SEGMENT_DSI: &str = "wnfs/1.0/hiding segment derivation from content key";
/// The block segment derivation domain separation info
/// used for salting the hashing function when generating
/// the segments for each file's external content blocks.
pub(crate) const BLOCK_SEGMENT_DSI: &str = "wnfs/1.0/segment derivation for file block";
/// The temporal key derivation domain seperation info
/// used for salting the hashing function when deriving
/// symmetric keys from ratchets.
pub(crate) const TEMPORAL_KEY_DSI: &str = "wnfs/1.0/temporal derivation from ratchet";
/// The snapshot key derivation domain separation info
/// used for salting the hashing function when deriving
/// the snapshot key from the temporal key.
pub(crate) const SNAPSHOT_KEY_DSI: &str = "wnfs/1.0/snapshot key derivation from temporal";

//--------------------------------------------------------------------------------------------------
// Type Definitions
//--------------------------------------------------------------------------------------------------

/// The key used to encrypt the content of a node.
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct SnapshotKey(#[serde(with = "serde_byte_array")] pub(crate) [u8; KEY_BYTE_SIZE]);

/// The key used to encrypt the header section of a node.
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct TemporalKey(#[serde(with = "serde_byte_array")] pub(crate) [u8; KEY_BYTE_SIZE]);

//--------------------------------------------------------------------------------------------------
// Implementations
//--------------------------------------------------------------------------------------------------

impl TemporalKey {
    /// Derive a temporal key from the ratchet for use with WNFS.
    pub fn new(ratchet: &Ratchet) -> Self {
        Self(ratchet.derive_key(TEMPORAL_KEY_DSI).finalize().into())
    }

    /// Turn this TemporalKey, which gives read access to the current revision and any future
    /// revisions into a SnapshotKey, which only gives read access to the current revision.
    pub fn derive_snapshot_key(&self) -> SnapshotKey {
        SnapshotKey(blake3::derive_key(SNAPSHOT_KEY_DSI, &self.0))
    }

    /// Encrypt a cleartext with this temporal key.
    ///
    /// Uses authenticated deterministic encryption via AES key wrap with padding (AES-KWP).
    ///
    /// The resulting ciphertext is 8 bytes longer than the next multiple of 8 bytes of the
    /// cleartext input length.
    pub fn key_wrap_encrypt(&self, cleartext: &[u8]) -> Result<Vec<u8>> {
        Ok(KekAes256::from(self.0)
            .wrap_with_padding_vec(cleartext)
            .map_err(|e| CryptError::UnableToEncrypt(anyhow!(e)))?)
    }

    /// Decrypt a ciphertext that was encrypted with this temporal key.
    ///
    /// Uses authenticated deterministic encryption via AES key wrap with padding (AES-KWP).
    ///
    /// The input ciphertext is 8 bytes longer than the next multiple of 8 bytes of the
    /// resulting cleartext length.
    pub fn key_wrap_decrypt(&self, ciphertext: &[u8]) -> Result<Vec<u8>> {
        Ok(KekAes256::from(self.0)
            .unwrap_with_padding_vec(ciphertext)
            .map_err(|e| CryptError::UnableToEncrypt(anyhow!(e)))?)
    }

    /// Return the temporal key's key material.
    pub fn as_bytes(&self) -> &[u8; KEY_BYTE_SIZE] {
        &self.0
    }
}

impl SnapshotKey {
    /// Generate a random snapshot key from given randomness.
    pub fn new(rng: &mut impl CryptoRngCore) -> Self {
        Self(utils::get_random_bytes(rng))
    }

    /// Encrypts the given plaintext using the key.
    ///
    /// # Examples
    ///
    /// ```
    /// use wnfs::private::SnapshotKey;
    /// use wnfs::common::utils;
    /// use rand_chacha::ChaCha12Rng;
    /// use rand_core::SeedableRng;
    ///
    /// let rng = &mut ChaCha12Rng::from_entropy();
    /// let key = SnapshotKey::new(rng);
    ///
    /// let plaintext = b"Hello World!";
    /// let ciphertext = key.encrypt(plaintext, rng).unwrap();
    /// let decrypted = key.decrypt(&ciphertext).unwrap();
    ///
    /// assert_eq!(plaintext, &decrypted[..]);
    /// ```
    pub fn encrypt(&self, data: &[u8], rng: &mut impl CryptoRngCore) -> Result<Vec<u8>> {
        let nonce = Self::generate_nonce(rng);

        let key = self.0.into();
        let cipher_text = XChaCha20Poly1305::new(&key)
            .encrypt(&nonce, data)
            .map_err(|e| CryptError::UnableToEncrypt(anyhow!(e)))?;

        Ok([nonce.to_vec(), cipher_text].concat())
    }

    /// Generates a random 24-byte extended nonce for encryption.
    pub(crate) fn generate_nonce(rng: &mut impl CryptoRngCore) -> XNonce {
        XChaCha20Poly1305::generate_nonce(rng)
    }

    /// Encrypts the cleartext in the given buffer in-place, with given key.
    ///
    /// The nonce is usually pre-pended to the ciphertext.
    ///
    /// The authentication tag is required for decryption and usually appended to the ciphertext.
    pub(crate) fn encrypt_in_place(&self, nonce: &XNonce, buffer: &mut [u8]) -> Result<Tag> {
        let key = self.0.into();
        let tag = XChaCha20Poly1305::new(&key)
            .encrypt_in_place_detached(nonce, &[], buffer)
            .map_err(|e| CryptError::UnableToEncrypt(anyhow!(e)))?;
        Ok(tag)
    }

    /// Decrypts the given ciphertext using the key.
    ///
    /// # Examples
    ///
    /// ```
    /// use wnfs::private::SnapshotKey;
    /// use wnfs::common::utils;
    /// use rand_chacha::ChaCha12Rng;
    /// use rand_core::SeedableRng;
    ///
    /// let rng = &mut ChaCha12Rng::from_entropy();
    /// let key = SnapshotKey::new(rng);
    ///
    /// let plaintext = b"Hello World!";
    /// let ciphertext = key.encrypt(plaintext, rng).unwrap();
    /// let decrypted = key.decrypt(&ciphertext).unwrap();
    ///
    /// assert_eq!(plaintext, &decrypted[..]);
    /// ```
    pub fn decrypt(&self, cipher_text: &[u8]) -> Result<Vec<u8>> {
        let (nonce_bytes, data) = cipher_text.split_at(NONCE_SIZE);
        let key = self.0.into();
        let nonce = XNonce::from_slice(nonce_bytes);

        Ok(XChaCha20Poly1305::new(&key)
            .decrypt(nonce, data)
            .map_err(|e| CryptError::UnableToDecrypt(anyhow!(e)))?)
    }

    /// Decrypts the ciphertext in the given buffer in-place, with given key.
    ///
    /// Usually the nonce is stored as the cipher's prefix and the tag as
    /// the cipher's suffix.
    #[allow(dead_code)] // I figured it makes sense to have this for completeness sake.
    pub(crate) fn decrypt_in_place(
        &self,
        nonce: &XNonce,
        tag: &Tag,
        buffer: &mut [u8],
    ) -> Result<()> {
        let key = self.0.into();
        XChaCha20Poly1305::new(&key)
            .decrypt_in_place_detached(nonce, &[], buffer, tag)
            .map_err(|e| CryptError::UnableToDecrypt(anyhow!(e)))?;
        Ok(())
    }

    /// Return the snapshot key's key material.
    pub fn as_bytes(&self) -> &[u8; KEY_BYTE_SIZE] {
        &self.0
    }
}

//--------------------------------------------------------------------------------------------------
// Proptests
//--------------------------------------------------------------------------------------------------

#[cfg(test)]
mod proptests {
    use super::*;
    use crate::private::KEY_BYTE_SIZE;
    use proptest::{prelude::any, prop_assert_eq, prop_assert_ne};
    use rand_chacha::ChaCha12Rng;
    use rand_core::SeedableRng;
    use test_strategy::proptest;

    #[proptest(cases = 100)]
    fn snapshot_key_can_encrypt_and_decrypt_data(
        #[strategy(any::<Vec<u8>>())] data: Vec<u8>,
        #[strategy(any::<[u8; KEY_BYTE_SIZE]>())] rng_seed: [u8; KEY_BYTE_SIZE],
        key_bytes: [u8; KEY_BYTE_SIZE],
    ) {
        let key = SnapshotKey(key_bytes);
        let rng = &mut ChaCha12Rng::from_seed(rng_seed);

        let encrypted = key.encrypt(&data, rng).unwrap();
        let decrypted = key.decrypt(&encrypted).unwrap();

        if data.len() >= 16 {
            let cipher_part = &encrypted[NONCE_SIZE..NONCE_SIZE + data.len()];
            prop_assert_ne!(cipher_part, &decrypted);
        }

        prop_assert_eq!(&decrypted, &data);
    }

    #[proptest(cases = 100)]
    fn snapshot_key_can_encrypt_and_decrypt_data_in_place(
        data: Vec<u8>,
        key_bytes: [u8; KEY_BYTE_SIZE],
        nonce: [u8; NONCE_SIZE],
    ) {
        let mut buffer = data.clone();
        let nonce = XNonce::from_slice(&nonce);
        let key = SnapshotKey(key_bytes);

        let tag = key.encrypt_in_place(nonce, &mut buffer).unwrap();

        if buffer.len() >= 16 {
            prop_assert_ne!(&buffer, &data);
        }

        key.decrypt_in_place(nonce, &tag, &mut buffer).unwrap();

        prop_assert_eq!(&buffer, &data);
    }
}
