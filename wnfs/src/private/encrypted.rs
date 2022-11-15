use std::io::Cursor;

use anyhow::Result;
use libipld::{cbor::DagCborCodec, codec::Decode, prelude::Encode, Ipld};
use once_cell::sync::OnceCell;
use rand_core::RngCore;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::FsError;

use super::Key;

#[derive(Debug, Clone, Eq)]
pub struct Encrypted<T> {
    ciphertext: Vec<u8>,
    value_cache: OnceCell<T>,
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
    pub fn from_value(value: T, key: &Key, rng: &mut impl RngCore) -> Result<Self>
    where
        T: Serialize,
    {
        let ipld = value.serialize(libipld::serde::Serializer)?;
        let mut bytes = Vec::new();
        ipld.encode(DagCborCodec, &mut bytes)?;
        let ciphertext = key.encrypt(&Key::generate_nonce(rng), &bytes)?;

        Ok(Self {
            value_cache: OnceCell::from(value),
            ciphertext,
        })
    }

    pub fn from_ciphertext(ciphertext: Vec<u8>) -> Self {
        Self {
            ciphertext,
            value_cache: OnceCell::new(),
        }
    }

    pub fn resolve_value(&self, key: &Key) -> Result<&T>
    where
        T: DeserializeOwned,
    {
        self.value_cache.get_or_try_init(|| {
            let bytes = key.decrypt(&self.ciphertext)?;
            let ipld = Ipld::decode(DagCborCodec, &mut Cursor::new(bytes))?;
            libipld::serde::from_ipld::<T>(ipld)
                .map_err(|e| FsError::InvalidDeserialization(e.to_string()).into())
        })
    }

    pub fn get_ciphertext(&self) -> &Vec<u8> {
        &self.ciphertext
    }

    pub fn to_ciphertext(self) -> Vec<u8> {
        self.ciphertext
    }

    pub fn get_value(&self) -> Option<&T> {
        self.value_cache.get()
    }
}

impl<T: PartialEq> PartialEq for Encrypted<T> {
    fn eq(&self, other: &Self) -> bool {
        self.get_ciphertext() == other.get_ciphertext()
    }
}
