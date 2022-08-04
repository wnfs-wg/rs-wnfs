use anyhow::Result;
use async_once_cell::OnceCell;
use libipld::Cid;
use serde::{de::DeserializeOwned, Serialize};

use crate::{BlockStore, Link};

use super::{Key, Rng};

//--------------------------------------------------------------------------------------------------
// Type Definitions
//--------------------------------------------------------------------------------------------------

/// A data structure that represents a link in the IPLD graph. Basically it is a "link" to some content addressable value of `T`.
///
/// It supports representing the "link" with a Cid or the deserialized value itself.
///
/// Link needs a `BlockStore` to be able to resolve Cids to corresponding values of `T` and vice versa.
#[derive(Debug, Clone)]
pub struct PrivateLink<T>(Link<T>);

//--------------------------------------------------------------------------------------------------
// Implementations
//--------------------------------------------------------------------------------------------------

impl<V> PrivateLink<V> {
    /// Creates a new `Referenceable` that starts out as a value of `R`.
    pub fn from_reference(cid: Cid) -> Self {
        Self(Link::Encoded {
            reference: cid,
            value_cache: OnceCell::new(),
        })
    }

    /// Gets an owned value from type. It attempts to it get from the store if it is not present in type.
    pub async fn get_owned_value<'a, B, R>(self, store: &B, key: &Key) -> Result<V>
    where
        B: BlockStore,
        R: Rng,
        V: DeserializeOwned,
    {
        match self.0 {
            Link::Encoded {
                ref reference,
                value_cache,
            } => match value_cache.into_inner() {
                Some(cached) => Ok(cached),
                None => store.get_private_deserializable(reference, key).await,
            },
            Link::Decoded { value, .. } => Ok(value),
        }
    }

    /// Gets the value stored in type.
    ///
    /// NOTE: This does not attempt to get it from the store if it does not exist.
    #[inline]
    pub fn get_value(&self) -> Option<&V> {
        self.0.get_value()
    }

    /// Gets the cid data stored in type.
    ///
    /// NOTE: This does not attempt to get it from the store if it does not exist.
    #[inline]
    pub fn get_cid<R: Rng>(&self) -> Option<&Cid> {
        self.0.get_cid()
    }

    /// Gets the value stored in link. It attempts to get it from the store if it is not present in link.
    pub async fn resolve_value<'a, 'b, B, R>(&'a self, store: &B, key: &Key) -> Result<&'a V>
    where
        B: BlockStore,
        R: Rng,
        V: DeserializeOwned,
    {
        match &self.0 {
            Link::Encoded {
                reference,
                value_cache,
            } => {
                value_cache
                    .get_or_try_init(async {
                        store.get_private_deserializable(reference, key).await
                    })
                    .await
            }
            Link::Decoded { value, .. } => Ok(value),
        }
    }

    /// Gets the cid data stored in type. It attempts to get it from the store if it is not present in type.
    pub async fn resolve_cid<'a, 'b, B, R>(&'a self, store: &mut B, key: &Key) -> Result<&'a Cid>
    where
        B: BlockStore,
        R: Rng,
        V: Serialize,
    {
        match &self.0 {
            Link::Encoded { reference, .. } => Ok(reference),
            Link::Decoded {
                value,
                reference_cache,
            } => {
                reference_cache
                    .get_or_try_init(async {
                        store.put_private_serializable::<_, R>(value, key).await
                    })
                    .await
            }
        }
    }

    /// Checks if there is a value stored in link.
    #[inline]
    pub fn has_value(&self) -> bool {
        self.0.has_value()
    }

    /// Checks if there is a Cid stored in link.
    #[inline]
    pub fn has_cid(&self) -> bool {
        self.0.has_cid()
    }
}

impl<V> From<V> for PrivateLink<V> {
    fn from(value: V) -> Self {
        Self(Link::Decoded {
            value,
            reference_cache: OnceCell::new(),
        })
    }
}
