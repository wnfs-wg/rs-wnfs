use std::io::Cursor;

use anyhow::Result;
use async_once_cell::OnceCell;
use futures::Future;
use libipld::cbor::DagCborCodec;
use libipld::codec::Decode;
use libipld::codec::Encode;
use libipld::serde::from_ipld;
use libipld::serde::to_ipld;
use libipld::Cid;
use libipld::Ipld;
use libipld::IpldCodec;
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::BlockStore;

// TODO(matheus23) we should have a custom impl of PartialEq and Eq
#[derive(Debug)]
pub enum Link<T> {
    /// Invariant: the (optional) contents of the cache *must* encode the cid
    Encoded {
        cid: Cid,
        value_cache: OnceCell<T>,
    },
    Decoded {
        value: T,
        cid_cache: OnceCell<Cid>,
    },
}

impl<T> Clone for Link<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        match self {
            Link::Encoded { cid, value_cache } => Self::Encoded {
                cid: cid.clone(),
                value_cache: OnceCell::new_with(value_cache.get().cloned()),
            },
            Link::Decoded { value, cid_cache } => Self::Decoded {
                value: value.clone(),
                cid_cache: OnceCell::new_with(cid_cache.get().cloned()),
            },
        }
    }
}

// TODO(appcypher): Move to blockstore
async fn load<D: DeserializeOwned, B: BlockStore>(cid: &Cid, store: &B) -> Result<D> {
    // TODO(appcypher): Abstract the codec.
    let bytes = store.get_block(cid).await?;
    let ipld = Ipld::decode(DagCborCodec, &mut Cursor::new(bytes.as_ref()))?;
    Ok(from_ipld::<D>(ipld)?)
}

// TODO(appcypher): Move to blockstore
async fn store_put<S: Serialize, B: BlockStore>(value: &S, store: &mut B) -> Result<Cid> {
    let ipld = to_ipld(value)?;
    let mut bytes = Vec::new();
    ipld.encode(DagCborCodec, &mut bytes)?;
    store.put_block(bytes, IpldCodec::DagCbor).await
}

impl<T> Link<T> {
    pub(crate) fn from_cid(cid: Cid) -> Self {
        Self::Encoded {
            cid,
            value_cache: OnceCell::new(),
        }
    }

    pub(crate) async fn resolve<B: BlockStore>(&self, store: &B) -> Result<&T>
    where
        T: DeserializeOwned,
    {
        match self {
            Self::Encoded { cid, value_cache } => {
                value_cache
                    .get_or_try_init(async { load(cid, store).await })
                    .await
            }
            Self::Decoded { value, .. } => Ok(value),
        }
    }

    pub(crate) async fn get<B: BlockStore>(self, store: &B) -> Result<T>
    where
        T: DeserializeOwned,
    {
        match self {
            Self::Encoded { cid, value_cache } => match value_cache.into_inner() {
                Some(cached) => Ok(cached),
                None => load(&cid, store).await,
            },
            Self::Decoded { value, .. } => Ok(value),
        }
    }

    pub(crate) fn get_cached(&self) -> Option<&T> {
        match self {
            Self::Encoded { value_cache, .. } => value_cache.get(),
            Self::Decoded { value, .. } => Some(value),
        }
    }

    pub(crate) fn cid_cached(&self) -> Option<&Cid> {
        match self {
            Self::Encoded { cid, .. } => Some(cid),
            Self::Decoded { cid_cache, .. } => cid_cache.get(),
        }
    }

    pub(crate) async fn seal<B: BlockStore>(&self, store: &mut B) -> Result<&Cid>
    where
        T: Serialize,
    {
        match self {
            Self::Encoded { cid, .. } => Ok(cid),
            Self::Decoded { value, cid_cache } => {
                cid_cache
                    .get_or_try_init(async { store_put(value, store).await })
                    .await
            }
        }
    }

    fn is_value_cached(&self) -> bool {
        match self {
            Self::Encoded { value_cache, .. } => value_cache.get().is_some(),
            Self::Decoded { .. } => true,
        }
    }

    fn is_cid_cached(&self) -> bool {
        match self {
            Self::Encoded { .. } => true,
            Self::Decoded { cid_cache, .. } => cid_cache.get().is_some(),
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

// impl<T> From<Cid> for Link<T> {
//     fn from(cid: Cid) -> Self {
//         Self::Encoded {
//             cid,
//             value_cache: OnceCell::new(),
//         }
//     }
// }

#[cfg(test)]
mod ipld_link_tests {
    use crate::{Link, MemoryBlockStore};

    #[async_std::test]
    async fn ipld_link() {
        let link = Link::from(42_u64);
        let mut store = MemoryBlockStore::default();
        let cid = link.seal(&mut store).await.unwrap();
        println!("Value Cached? {}", link.is_value_cached());
        println!("{}", cid);

        // another link
        let link2: Link<u64> = Link::from_cid(*cid);
        println!(
            "Value Cached? {} Cid Cached? {}",
            link2.is_value_cached(),
            link2.is_cid_cached()
        );
        let num = *link2.resolve(&store).await.unwrap();
        println!("num: {num}");
        // interior mutability makes is_cached suddenly return true :S
        // we may want to just never have that be observable behavior from the outside.
        println!(
            "Value Cached? {} Cid Cached? {}",
            link2.is_value_cached(),
            link2.is_cid_cached()
        );
    }
}
