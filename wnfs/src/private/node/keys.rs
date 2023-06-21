use crate::{
    error::AesError,
    private::{AesKey, KEY_BYTE_SIZE, NONCE_SIZE},
};
use aes_gcm::{
    aead::{consts::U12, Aead},
    AeadInPlace, Aes256Gcm, KeyInit, Nonce, Tag,
};
use aes_kw::KekAes256;
use anyhow::Result;
use rand_core::RngCore;
use serde::{Deserialize, Serialize};
use sha3::{Digest, Sha3_256};
use skip_ratchet::Ratchet;
use std::fmt::Debug;
use wnfs_hamt::Hasher;
use wnfs_nameaccumulator::NameSegment;

//--------------------------------------------------------------------------------------------------
// Type Definitions
//--------------------------------------------------------------------------------------------------

/// The key used to encrypt the content of a node.
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct SnapshotKey(pub AesKey);

/// The key used to encrypt the header section of a node.
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct TemporalKey(pub AesKey);

//--------------------------------------------------------------------------------------------------
// Implementations
//--------------------------------------------------------------------------------------------------

impl TemporalKey {
    /// Turn this TemporalKey, which gives read access to the current revision and any future
    /// revisions into a SnapshotKey, which only gives read access to the current revision.
    pub fn derive_snapshot_key(&self) -> SnapshotKey {
        let TemporalKey(key) = self;
        SnapshotKey::from(Sha3_256::hash(&key.as_bytes()))
    }

    /// Encrypt a cleartext with this temporal key.
    ///
    /// Uses authenticated deterministic encryption via AES key wrap with padding (AES-KWP).
    ///
    /// The resulting ciphertext is 8 bytes longer than the next multiple of 8 bytes of the
    /// cleartext input length.
    pub fn key_wrap_encrypt(&self, cleartext: &[u8]) -> Result<Vec<u8>> {
        Ok(KekAes256::from(self.0.clone().bytes())
            .wrap_with_padding_vec(cleartext)
            .map_err(|e| AesError::UnableToEncrypt(format!("{e}")))?)
    }

    /// Decrypt a ciphertext that was encrypted with this temporal key.
    ///
    /// Uses authenticated deterministic encryption via AES key wrap with padding (AES-KWP).
    ///
    /// The input ciphertext is 8 bytes longer than the next multiple of 8 bytes of the
    /// resulting cleartext length.
    pub fn key_wrap_decrypt(&self, ciphertext: &[u8]) -> Result<Vec<u8>> {
        Ok(KekAes256::from(self.0.clone().bytes())
            .unwrap_with_padding_vec(ciphertext)
            .map_err(|e| AesError::UnableToEncrypt(format!("{e}")))?)
    }

    /// TODO(matheus23)
    pub(crate) fn to_revision_segment(&self) -> NameSegment {
        let mut hasher = Sha3_256::new();
        // TODO(matheus23): Specification?
        hasher.update("Revision name acc element");
        hasher.update(self.0.as_bytes());
        NameSegment::from_digest(hasher)
    }
}

impl SnapshotKey {
    /// Encrypts the given plaintext using the key.
    ///
    /// # Examples
    ///
    /// ```
    /// use wnfs::private::{AesKey, SnapshotKey};
    /// use wnfs::common::utils;
    /// use rand::thread_rng;
    ///
    /// let rng = &mut thread_rng();
    /// let key = SnapshotKey::from(utils::get_random_bytes(rng));
    ///
    /// let plaintext = b"Hello World!";
    /// let ciphertext = key.encrypt(plaintext, rng).unwrap();
    /// let decrypted = key.decrypt(&ciphertext).unwrap();
    ///
    /// assert_eq!(plaintext, &decrypted[..]);
    /// ```
    pub fn encrypt(&self, data: &[u8], rng: &mut impl RngCore) -> Result<Vec<u8>> {
        let nonce = Self::generate_nonce(rng);

        let cipher_text = Aes256Gcm::new(&self.0.clone().bytes().into())
            .encrypt(&nonce, data)
            .map_err(|e| AesError::UnableToEncrypt(format!("{e}")))?;

        Ok([nonce.to_vec(), cipher_text].concat())
    }

    /// Generates a random 12-byte nonce for encryption.
    pub(crate) fn generate_nonce(rng: &mut impl RngCore) -> Nonce<U12> {
        let mut nonce = Nonce::default();
        rng.fill_bytes(&mut nonce);
        nonce
    }

    /// Encrypts the cleartext in the given buffer in-place, with given key.
    ///
    /// The nonce is usually pre-pended to the ciphertext.
    ///
    /// The authentication tag is required for decryption and usually appended to the ciphertext.
    pub(crate) fn encrypt_in_place(&self, nonce: &Nonce<U12>, buffer: &mut [u8]) -> Result<Tag> {
        let tag = Aes256Gcm::new(&self.0.clone().bytes().into())
            .encrypt_in_place_detached(nonce, &[], buffer)
            .map_err(|e| AesError::UnableToEncrypt(format!("{e}")))?;
        Ok(tag)
    }

    /// Decrypts the given ciphertext using the key.
    ///
    /// # Examples
    ///
    /// ```
    /// use wnfs::private::{AesKey, SnapshotKey};
    /// use wnfs::common::utils;
    /// use rand::thread_rng;
    ///
    /// let rng = &mut thread_rng();
    /// let key = SnapshotKey::from(utils::get_random_bytes(rng));
    ///
    /// let plaintext = b"Hello World!";
    /// let ciphertext = key.encrypt(plaintext, rng).unwrap();
    /// let decrypted = key.decrypt(&ciphertext).unwrap();
    ///
    /// assert_eq!(plaintext, &decrypted[..]);
    /// ```
    pub fn decrypt(&self, cipher_text: &[u8]) -> Result<Vec<u8>> {
        let (nonce_bytes, data) = cipher_text.split_at(NONCE_SIZE);

        Ok(Aes256Gcm::new(&self.0.clone().bytes().into())
            .decrypt(Nonce::from_slice(nonce_bytes), data)
            .map_err(|e| AesError::UnableToDecrypt(format!("{e}")))?)
    }

    /// Decrypts the ciphertext in the given buffer in-place, with given key.
    ///
    /// Usually the nonce is stored as the cipher's prefix and the tag as
    /// the cipher's suffix.
    #[allow(dead_code)] // I figured it makes sense to have this for completeness sake.
    pub(crate) fn decrypt_in_place(
        &self,
        nonce: &Nonce<U12>,
        tag: &Tag,
        buffer: &mut [u8],
    ) -> Result<()> {
        Aes256Gcm::new(&self.0.clone().bytes().into())
            .decrypt_in_place_detached(nonce, &[], buffer, tag)
            .map_err(|e| AesError::UnableToDecrypt(format!("{e}")))?;
        Ok(())
    }
}

impl From<AesKey> for TemporalKey {
    fn from(key: AesKey) -> Self {
        Self(key)
    }
}

impl From<[u8; KEY_BYTE_SIZE]> for TemporalKey {
    fn from(key: [u8; KEY_BYTE_SIZE]) -> Self {
        Self(AesKey::new(key))
    }
}

impl From<&Ratchet> for TemporalKey {
    fn from(ratchet: &Ratchet) -> Self {
        Self::from(AesKey::new(
            ratchet
                .derive_key("WNFS temporal key derivation")
                .finalize()
                .into(),
        ))
    }
}

impl From<AesKey> for SnapshotKey {
    fn from(key: AesKey) -> Self {
        Self(key)
    }
}

impl From<[u8; KEY_BYTE_SIZE]> for SnapshotKey {
    fn from(key: [u8; KEY_BYTE_SIZE]) -> Self {
        Self(AesKey::new(key))
    }
}

impl From<SnapshotKey> for AesKey {
    fn from(key: SnapshotKey) -> Self {
        key.0
    }
}

//--------------------------------------------------------------------------------------------------
// Proptests
//--------------------------------------------------------------------------------------------------

#[cfg(test)]
mod proptests {
    use super::*;
    use crate::private::KEY_BYTE_SIZE;
    use proptest::{
        prelude::any,
        prop_assert_eq, prop_assert_ne,
        test_runner::{RngAlgorithm, TestRng},
    };
    use test_strategy::proptest;

    #[proptest(cases = 100)]
    fn snapshot_key_can_encrypt_and_decrypt_data(
        #[strategy(any::<Vec<u8>>())] data: Vec<u8>,
        #[strategy(any::<[u8; KEY_BYTE_SIZE]>())] rng_seed: [u8; KEY_BYTE_SIZE],
        key_bytes: [u8; KEY_BYTE_SIZE],
    ) {
        let key = SnapshotKey::from(key_bytes);
        let rng = &mut TestRng::from_seed(RngAlgorithm::ChaCha, &rng_seed);

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
        let nonce = Nonce::from_slice(&nonce);
        let key = SnapshotKey::from(key_bytes);

        let tag = key.encrypt_in_place(nonce, &mut buffer).unwrap();

        if buffer.len() >= 16 {
            prop_assert_ne!(&buffer, &data);
        }

        key.decrypt_in_place(nonce, &tag, &mut buffer).unwrap();

        prop_assert_eq!(&buffer, &data);
    }
}
