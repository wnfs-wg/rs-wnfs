use anyhow::Result;
use async_once_cell::OnceCell;
use libipld::Cid;
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::BlockStore;

//--------------------------------------------------------------------------------------------------
// Type Definitions
//--------------------------------------------------------------------------------------------------

/// A data structure that represents a link in the IPLD graph. Basically it is "link" to some content addressable value of `T`.
///
/// It supports representing the "link" with a Cid or the deserialized value itself.
///
/// Link needs a `BlockStore` to be able to resolve Cids to corresponding values of `T` and vice versa.
#[derive(Debug)]
pub enum Link<T> {
    /// A variant of `Link` that starts out as a Cid.
    /// It supports converting the Cid to a `T` by caching it only once in `value_cache`.
    Encoded { cid: Cid, value_cache: OnceCell<T> },
    /// A variant of `Link` that starts out as a value of `T`.
    /// It supports converting the value of `T` to a Cid by caching it only once in `cid_cache`.
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

    /// Gets an owned value from link. It attempts to it get from the store if it is not present in link.
    pub async fn get_owned_value<B: BlockStore>(self, store: &B) -> Result<T>
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

    /// Gets the value stored in link.
    ///
    /// NOTE: This does not attempt to get it from the store if it does not exist.
    pub fn get_value(&self) -> Option<&T> {
        match self {
            Self::Encoded { value_cache, .. } => value_cache.get(),
            Self::Decoded { value, .. } => Some(value),
        }
    }

    /// Gets the Cid stored in link.
    ///
    /// NOTE: This does not attempt to get it from the store if it does not exist.
    pub fn get_cid(&self) -> Option<&Cid> {
        match self {
            Self::Encoded { cid, .. } => Some(cid),
            Self::Decoded { cid_cache, .. } => cid_cache.get(),
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

    /// Gets the Cid stored in link. It attempts to get it from the store if it is not present in link.
    pub async fn resolve_cid<B: BlockStore>(&self, store: &mut B) -> Result<&Cid>
    where
        T: Serialize,
    {
        match self {
            Self::Encoded { cid, .. } => Ok(cid),
            Self::Decoded { value, cid_cache } => {
                cid_cache
                    .get_or_try_init(async { store.put_serializable(value).await })
                    .await
            }
        }
    }

    /// Checks if there is a value stored in link.
    pub fn has_value(&self) -> bool {
        match self {
            Self::Encoded { value_cache, .. } => value_cache.get().is_some(),
            _ => true,
        }
    }

    /// Checks if there is a Cid stored in link.
    pub fn has_cid(&self) -> bool {
        match self {
            Self::Decoded { cid_cache, .. } => cid_cache.get().is_some(),
            _ => true,
        }
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
            Link::Encoded { cid, value_cache } => Self::Encoded {
                cid: *cid,
                value_cache: OnceCell::new_with(value_cache.get().cloned()),
            },
            Link::Decoded { value, cid_cache } => Self::Decoded {
                value: value.clone(),
                cid_cache: OnceCell::new_with(cid_cache.get().cloned()),
            },
        }
    }
}

impl<T: PartialEq> PartialEq for Link<T> {
    /// This equality check does not cover cases where one holds a Cid and the other holds a value T.
    /// This is because sealing or resolving the link requires async operation which PartialEq does not expose.
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Encoded { cid, .. }, Self::Encoded { cid: cid2, .. }) => cid == cid2,
            (Self::Decoded { value, .. }, Self::Decoded { value: value2, .. }) => value == value2,
            (Self::Encoded { cid, .. }, Self::Decoded { .. }) => {
                if let Some(cid2) = other.get_cid() {
                    cid == cid2
                } else {
                    false
                }
            }
            (Self::Decoded { .. }, Self::Encoded { cid: cid2, .. }) => {
                if let Some(cid) = self.get_cid() {
                    cid == cid2
                } else {
                    false
                }
            }
            _ => false,
        }
    }
}

//--------------------------------------------------------------------------------------------------
// Tests
//--------------------------------------------------------------------------------------------------

#[cfg(test)]
mod ipld_link_tests {
    use crate::{Link, MemoryBlockStore};

    #[async_std::test]
    async fn ipld_link() {
        let link = Link::from(42_u64);
        let mut store = MemoryBlockStore::default();
        let cid = link.resolve_cid(&mut store).await.unwrap();
        println!("Has Value? {}", link.has_value());
        println!("{}", cid);

        // another link
        let link2: Link<u64> = Link::from_cid(*cid);
        println!(
            "Has Value? {} Has Cid? {}",
            link2.has_value(),
            link2.has_cid()
        );

        let num = *link2.resolve_value(&store).await.unwrap();
        println!("num: {num}");
        // interior mutability makes is_cached suddenly return true :S
        // we may want to just never have that be observable behavior from the outside.
        println!(
            "Has Value? {} Has Cid? {}",
            link2.has_value(),
            link2.has_cid()
        );
    }
}
