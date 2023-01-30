use std::io::Cursor;

use aes_kw::KekAes256;
use anyhow::Result;
use libipld::{cbor::DagCborCodec, codec::Decode, prelude::Encode, Ipld};
use once_cell::sync::OnceCell;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::{AesError, FsError};

use super::AesKey;

/// A wrapper for encrypted data.
///
/// When serialized or deserialized this will only
/// ever emit or consume ciphertexts.
///
/// It can be resolved to the plaintext value `T`, when
/// you call `resolve_value`. Any subsequent calls to
/// `resolve_value` will re-use a cached, decrypted value.
#[derive(Debug, Clone, Eq)]
pub struct Encrypted<T> {
    ciphertext: Vec<u8>,
    value_cache: OnceCell<T>,
}

impl<T> Encrypted<T> {
    /// Constructs an `Encrypted` wrapper from a given plaintext value.
    ///
    /// This will compute a ciphertext by serializing the value and encrypting the
    /// serialized value given the key and randomness.
    ///
    /// To ensure confidentiality, the randomness should be cryptographically secure
    /// randomness.
    pub fn from_value(value: T, key: &AesKey) -> Result<Self>
    where
        T: Serialize,
    {
        let ipld = value.serialize(libipld::serde::Serializer)?;
        let mut bytes = Vec::new();
        ipld.encode(DagCborCodec, &mut bytes)?;
        let ciphertext = KekAes256::from(key.clone().bytes())
            .wrap_with_padding_vec(&bytes)
            .map_err(|e| AesError::UnableToEncrypt(format!("{e}")))?;

        Ok(Self {
            value_cache: OnceCell::from(value),
            ciphertext,
        })
    }

    /// Constructs an `Encrypted` wrapper from some serialized ciphertext.
    ///
    /// This won't compute the decrypted value inside. That has to be lazily
    /// computed via `resolve_value`.
    pub fn from_ciphertext(ciphertext: Vec<u8>) -> Self {
        Self {
            ciphertext,
            value_cache: OnceCell::new(),
        }
    }

    /// Decrypts and deserializes the value inside the `Encrypted` wrapper using
    /// given key.
    ///
    /// This operation may fail if given key doesn't decrypt the ciphertext or
    /// deserializing the value from the encrypted plaintext doesn't work.
    pub fn resolve_value(&self, key: &AesKey) -> Result<&T>
    where
        T: DeserializeOwned,
    {
        self.value_cache.get_or_try_init(|| {
            let bytes = KekAes256::from(key.clone().bytes())
                .unwrap_with_padding_vec(&self.ciphertext)
                .map_err(|e| AesError::UnableToDecrypt(format!("{e}")))?;
            let ipld = Ipld::decode(DagCborCodec, &mut Cursor::new(bytes))?;
            libipld::serde::from_ipld::<T>(ipld)
                .map_err(|e| FsError::InvalidDeserialization(e.to_string()).into())
        })
    }

    /// Gets the ciphertext
    pub fn get_ciphertext(&self) -> &Vec<u8> {
        &self.ciphertext
    }

    /// Consumes the ciphertext
    pub fn take_ciphertext(self) -> Vec<u8> {
        self.ciphertext
    }

    /// Looks up the cached value. If `resolve_value` has never
    /// been called, then the cache will be unpopulated and this will
    /// return `None`.
    pub fn get_value(&self) -> Option<&T> {
        self.value_cache.get()
    }
}

impl<'de, T> Deserialize<'de> for Encrypted<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(Self::from_ciphertext(Vec::<u8>::deserialize(deserializer)?))
    }
}

impl<T> Serialize for Encrypted<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.ciphertext.serialize(serializer)
    }
}

impl<T: PartialEq> PartialEq for Encrypted<T> {
    fn eq(&self, other: &Self) -> bool {
        self.get_ciphertext() == other.get_ciphertext()
    }
}
