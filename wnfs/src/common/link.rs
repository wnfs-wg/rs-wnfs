use crate::{AsyncSerialize, BlockStore, IpldEq};
use anyhow::Result;
use async_once_cell::OnceCell;
use async_trait::async_trait;
use libipld::Cid;
use serde::de::DeserializeOwned;
use std::fmt::{self, Debug, Formatter};

//--------------------------------------------------------------------------------------------------
// Type Definitions
//--------------------------------------------------------------------------------------------------

/// A data structure that represents a link in the IPLD graph. Basically it is a "link" to some content addressable value of `T`.
///
/// It supports representing the "link" with a Cid or the deserialized value itself.
///
/// Link needs a `BlockStore` to be able to resolve Cids to corresponding values of `T` and vice versa.
pub enum Link<T> {
    /// A variant of `Link` that started out as a `Cid`.
    /// If the decoded value is resolved using `resolve_value`, then the `value_cache` gets populated and
    /// further calls to `resolve_value` will just return from that cache.
    Encoded { cid: Cid, value_cache: OnceCell<T> },
    /// A variant of `Link` that started out as a deserialized value `T`.
    /// If the cid is resolved using `resolve_cid`, then the `cid_cache` gets populated and further calls
    /// to `resolve_cid` will just return from that cache.
    Decoded { value: T, cid_cache: OnceCell<Cid> },
}

//--------------------------------------------------------------------------------------------------
// Implementations
//--------------------------------------------------------------------------------------------------

impl<T> Link<T> {
    /// Creates a new `Link` that starts out as a Cid.
    pub fn from_cid(cid: Cid) -> Self {
        Self::Encoded {
            cid,
            value_cache: OnceCell::new(),
        }
    }

    /// Gets the Cid stored in type. It attempts to get it from the store if it is not present in type.
    pub async fn resolve_cid<B: BlockStore + ?Sized>(&self, store: &mut B) -> Result<&Cid>
    where
        T: AsyncSerialize,
    {
        match self {
            Self::Encoded { cid, .. } => Ok(cid),
            Self::Decoded { value, cid_cache } => {
                cid_cache
                    .get_or_try_init(async { store.put_async_serializable(value).await })
                    .await
            }
        }
    }

    /// Gets the value stored in link. It attempts to get it from the store if it is not present in link.
    pub async fn resolve_value<B: BlockStore>(&self, store: &B) -> Result<&T>
    where
        T: DeserializeOwned,
    {
        match self {
            Self::Encoded { cid, value_cache } => {
                value_cache
                    .get_or_try_init(async { store.get_deserializable(cid).await })
                    .await
            }
            Self::Decoded { value, .. } => Ok(value),
        }
    }

    /// Gets the cid data stored in type.
    ///
    /// NOTE: This does not attempt to get it from the store if it does not exist..
    pub fn get_cid(&self) -> Option<&Cid> {
        match self {
            Self::Encoded { cid, .. } => Some(cid),
            Self::Decoded { cid_cache, .. } => cid_cache.get(),
        }
    }

    /// Gets the value stored in type.
    ///
    /// NOTE: This does not attempt to get it from the store if it does not exist.
    pub fn get_value(&self) -> Option<&T> {
        match self {
            Self::Encoded { value_cache, .. } => value_cache.get(),
            Self::Decoded { value, .. } => Some(value),
        }
    }

    /// Gets an owned value from type. It attempts to it get from the store if it is not present in type.
    pub async fn resolve_owned_value<B: BlockStore>(self, store: &B) -> Result<T>
    where
        T: DeserializeOwned,
    {
        match self {
            Self::Encoded {
                ref cid,
                value_cache,
            } => match value_cache.into_inner() {
                Some(cached) => Ok(cached),
                None => store.get_deserializable(cid).await,
            },
            Self::Decoded { value, .. } => Ok(value),
        }
    }

    /// Checks if there is a Cid cached in link.
    pub fn has_cid(&self) -> bool {
        match self {
            Self::Decoded { cid_cache, .. } => cid_cache.get().is_some(),
            _ => true,
        }
    }

    /// Checks if there is a value stored in link.
    pub fn has_value(&self) -> bool {
        match self {
            Self::Encoded { value_cache, .. } => value_cache.get().is_some(),
            _ => true,
        }
    }

    /// Compares two links for equality. Attempts to get them from store if they are not already cached.
    pub async fn deep_eq<B: BlockStore>(&self, other: &Link<T>, store: &mut B) -> Result<bool>
    where
        T: PartialEq + AsyncSerialize,
    {
        if self == other {
            return Ok(true);
        }

        Ok(self.resolve_cid(store).await? == other.resolve_cid(store).await?)
    }
}

#[async_trait(?Send)]
impl<T: PartialEq + AsyncSerialize> IpldEq for Link<T> {
    async fn eq<B: BlockStore>(&self, other: &Link<T>, store: &mut B) -> Result<bool> {
        if self == other {
            return Ok(true);
        }

        Ok(self.resolve_cid(store).await? == other.resolve_cid(store).await?)
    }
}

impl<T> From<T> for Link<T> {
    fn from(value: T) -> Self {
        Self::Decoded {
            value,
            cid_cache: OnceCell::new(),
        }
    }
}

impl<T> Clone for Link<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        match self {
            Self::Encoded { cid, value_cache } => Self::Encoded {
                cid: *cid,
                value_cache: OnceCell::new_with(value_cache.get().cloned()),
            },
            Self::Decoded { value, cid_cache } => Self::Decoded {
                value: value.clone(),
                cid_cache: OnceCell::new_with(cid_cache.get().cloned()),
            },
        }
    }
}

impl<T> PartialEq for Link<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Encoded { cid, .. }, Self::Encoded { cid: cid2, .. }) => cid == cid2,
            (Self::Decoded { value, .. }, Self::Decoded { value: value2, .. }) => value == value2,
            (Self::Encoded { cid, .. }, Self::Decoded { value: value2, .. }) => {
                if let Some(cid2) = other.get_cid() {
                    cid == cid2
                } else if let Some(value) = self.get_value() {
                    value == value2
                } else {
                    false
                }
            }
            (Self::Decoded { value, .. }, Self::Encoded { cid: cid2, .. }) => {
                if let Some(cid) = self.get_cid() {
                    cid == cid2
                } else if let Some(value2) = other.get_value() {
                    value == value2
                } else {
                    false
                }
            }
        }
    }
}

impl<T> Debug for Link<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Encoded { cid, .. } => f.debug_tuple("Link::Encoded").field(cid).finish(),
            Self::Decoded { value, .. } => f.debug_tuple("Link::Decoded").field(value).finish(),
        }
    }
}

//--------------------------------------------------------------------------------------------------
// Tests
//--------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use crate::{BlockStore, Link, MemoryBlockStore};

    #[async_std::test]
    async fn link_value_can_be_resolved() {
        let store = &mut MemoryBlockStore::default();
        let cid = store.put_serializable(&256).await.unwrap();
        let link = Link::<u64>::from_cid(cid);

        let value = link.resolve_value(store).await.unwrap();
        assert_eq!(value, &256);
        assert!(link.has_value());
    }

    #[async_std::test]
    async fn link_cid_can_be_resolved() {
        let pair = ("price".into(), 12_000_500);
        let store = &mut MemoryBlockStore::default();
        let link = Link::<(String, u64)>::from(pair.clone());

        let cid = link.resolve_cid(store).await.unwrap();
        let value = store
            .get_deserializable::<(String, u64)>(cid)
            .await
            .unwrap();

        assert_eq!(value, pair);
    }
}
