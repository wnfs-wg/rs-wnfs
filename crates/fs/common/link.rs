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
pub(crate) enum Link<T> {
    /// Invariant: the (optional) contents of the cache *must* encode the cid
    Clean {
        cid: Cid,
        cache: OnceCell<T>,
    },
    Dirty(T),
}

impl<T> Clone for Link<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        match self {
            Link::Clean { cid, cache } => Self::Clean {
                cid: cid.clone(),
                cache: OnceCell::new_with(cache.get().cloned()),
            },
            Link::Dirty(node) => Self::Dirty(node.clone()),
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

impl<T> Link<T>
where
    T: DeserializeOwned + OptimizedAwayValue,
{
    pub(crate) fn new(item: T) -> Self {
        Self::Dirty(item)
    }

    pub(crate) fn from_cid(cid: Cid) -> Self {
        Self::Clean {
            cid,
            cache: OnceCell::new(),
        }
    }

    pub(crate) async fn resolve<B: BlockStore>(&self, store: &B) -> Result<&T> {
        match self {
            Self::Clean { cid, cache } => {
                cache
                    .get_or_try_init(async { load(cid, store).await })
                    .await
            }
            Self::Dirty(node) => Ok(node),
        }
    }

    pub(crate) async fn get<B: BlockStore>(self, store: &B) -> Result<T> {
        match self {
            Self::Clean { cid, cache } => match cache.into_inner() {
                Some(cached) => Ok(cached),
                None => load(&cid, store).await,
            },
            Self::Dirty(node) => Ok(node),
        }
    }

    pub(crate) async fn seal<B: BlockStore, Fut>(
        &mut self,
        store: &mut B,
        serialize: impl Fn(&T) -> Fut,
    ) -> Result<Cid>
    where
        Fut: Future<Output = Result<Vec<u8>>>,
    {
        match self {
            Self::Clean { cid, .. } => Ok(*cid),
            Self::Dirty(node) => {
                let bytes = serialize(node).await?;
                let cid = store.put_block(bytes, IpldCodec::DagCbor).await?;
                let node = std::mem::replace(node, T::bogus_value());
                *self = Self::Clean {
                    cid,
                    cache: OnceCell::new_with(Some(node)),
                };
                Ok(cid)
            }
        }
    }

    fn is_clean(&self) -> bool {
        matches!(self, Self::Clean { .. })
    }

    fn is_cached(&self) -> bool {
        match self {
            Self::Clean { cache, .. } => cache.get().is_some(),
            Self::Dirty(_) => true,
        }
    }
}

#[cfg(test)]
mod ipld_link_tests {
    use crate::{Link, MemoryBlockStore, OptimizedAwayValue};

    #[async_std::test]
    async fn ipld_link() {
        let mut link = Link::new(42_u64);
        let mut store = MemoryBlockStore::default();
        let cid = link.seal(&mut store).await.unwrap();
        println!("Clean? {}", link.is_clean());
        println!("{}", cid);

        // another link
        let link2: Link<u64> = Link::from_cid(cid);
        println!("Clean? {} Cached? {}", link2.is_clean(), link2.is_cached());
        let num = *link2.resolve(&store).await.unwrap();
        println!("num: {num}");
        // interior mutability makes is_cached suddenly return true :S
        // we may want to just never have that be observable behavior from the outside.
        println!("Clean? {} Cached? {}", link2.is_clean(), link2.is_cached());
    }

    impl OptimizedAwayValue for u64 {
        fn bogus_value() -> Self {
            0
        }
    }
}

pub(crate) trait OptimizedAwayValue {
    fn bogus_value() -> Self;
}
