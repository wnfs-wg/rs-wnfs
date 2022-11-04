use std::io::Cursor;

use anyhow::Result;
use libipld::{
    cbor::DagCborCodec,
    codec::{Decode, Encode},
    Ipld,
};
use once_cell::sync::OnceCell;
use rand_core::RngCore;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::FsError;

use super::Key;

#[derive(Debug, Clone)]
pub enum Encrypted<T> {
    FromCipher {
        ciphertext: Vec<u8>,
        value_cache: OnceCell<T>,
    },
    FromValue {
        value: T,
        ciphertext_cache: OnceCell<Vec<u8>>,
    },
}

impl<'de, T> Deserialize<'de> for Encrypted<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(Self::from_ciphertext(Vec::<u8>::deserialize(deserializer)?))
    }
}

impl<T> Encrypted<T> {
    pub fn from_value(value: T) -> Self
    where
        T: Serialize,
    {
        Self::FromValue {
            value,
            ciphertext_cache: OnceCell::new(),
        }
    }

    pub fn from_ciphertext(ciphertext: Vec<u8>) -> Self {
        Self::FromCipher {
            ciphertext,
            value_cache: OnceCell::new(),
        }
    }

    pub fn resolve_value(&self, key: &Key) -> Result<&T>
    where
        T: DeserializeOwned,
    {
        match self {
            Self::FromCipher {
                ciphertext,
                value_cache,
            } => value_cache.get_or_try_init(|| {
                let bytes = key.decrypt(ciphertext)?;
                let ipld = Ipld::decode(DagCborCodec, &mut Cursor::new(bytes))?;
                libipld::serde::from_ipld::<T>(ipld)
                    .map_err(|e| FsError::InvalidDeserialization(e.to_string()).into())
            }),
            Self::FromValue { value, .. } => Ok(value),
        }
    }

    pub fn resolve_ciphertext(&self, key: &Key, rng: &mut impl RngCore) -> Result<&Vec<u8>>
    where
        T: Serialize,
    {
        match self {
            Self::FromCipher { ciphertext, .. } => Ok(ciphertext),
            Self::FromValue {
                value,
                ciphertext_cache,
            } => ciphertext_cache.get_or_try_init(|| {
                let ipld = value.serialize(libipld::serde::Serializer)?;
                let mut bytes = Vec::new();
                ipld.encode(DagCborCodec, &mut bytes)?;
                key.encrypt(&Key::generate_nonce(rng), &bytes)
            }),
        }
    }

    pub fn get_ciphertext(&self) -> Option<&Vec<u8>> {
        match self {
            Self::FromCipher { ciphertext, .. } => Some(ciphertext),
            Self::FromValue {
                ciphertext_cache, ..
            } => ciphertext_cache.get(),
        }
    }

    pub fn get_value(&self) -> Option<&T> {
        match self {
            Self::FromCipher { value_cache, .. } => value_cache.get(),
            Self::FromValue { value, .. } => Some(value),
        }
    }

    pub fn serialize<S>(
        &self,
        serializer: S,
        key: &Key,
        rng: &mut impl RngCore,
    ) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
        T: Serialize,
    {
        let cipher = self
            .resolve_ciphertext(key, rng)
            .map_err(serde::ser::Error::custom)?;
        cipher.serialize(serializer)
    }
}

impl<T: PartialEq> PartialEq for Encrypted<T> {
    fn eq(&self, other: &Self) -> bool {
        if let (Some(l), Some(r)) = (self.get_ciphertext(), other.get_ciphertext()) {
            return l == r;
        }

        if let (Some(l), Some(r)) = (self.get_value(), other.get_value()) {
            return l == r;
        }

        // We don't know
        false
    }
}
