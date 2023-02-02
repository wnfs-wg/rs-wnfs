use std::io::Cursor;

use anyhow::Result;
use libipld::{cbor::DagCborCodec, codec::Decode, prelude::Encode, Ipld};
use once_cell::sync::OnceCell;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::FsError;

use super::RevisionKey;

/// A wrapper for AES-KWP deterministically encrypted (key-wrapped) data.
///
/// Any data wrapped like this **must not have low entropy**.
///
/// For anything that could potentially have low entropy,
/// please use AES-GCM instead via `ContentKey`.
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
    pub fn from_value(value: T, revision_key: &RevisionKey) -> Result<Self>
    where
        T: Serialize,
    {
        let ipld = value.serialize(libipld::serde::Serializer)?;
        let mut bytes = Vec::new();
        ipld.encode(DagCborCodec, &mut bytes)?;
        let ciphertext = revision_key.key_wrap_encrypt(&bytes)?;

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
    pub fn resolve_value(&self, revision_key: &RevisionKey) -> Result<&T>
    where
        T: DeserializeOwned,
    {
        self.value_cache.get_or_try_init(|| {
            let bytes = revision_key.key_wrap_decrypt(&self.ciphertext)?;
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

// Custom Eq/Ord implementations that bypass the OnceCell:

impl<T: PartialEq> PartialEq for Encrypted<T> {
    fn eq(&self, other: &Self) -> bool {
        self.get_ciphertext() == other.get_ciphertext()
    }
}

impl<T: PartialEq + PartialOrd> PartialOrd for Encrypted<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.ciphertext.partial_cmp(&other.ciphertext)
    }
}

impl<T: Eq + Ord> Ord for Encrypted<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.ciphertext.cmp(&other.ciphertext)
    }
}
