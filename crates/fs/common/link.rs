use anyhow::Result;
use async_once_cell::OnceCell;
use libipld::codec::Encode;
use libipld::IpldCodec;
use libipld::{cbor::DagCborCodec, codec::Decode, Cid};

use crate::blockstore;
use crate::BlockStore;

pub(crate) enum Link<T> {
    /// Invariant: the (optional) contents of the cache *must* encode the cid
    Clean {
        cid: Cid,
        cache: OnceCell<T>,
    },
    Dirty(T),
}

impl<T> Link<T>
where
    T: Decode<DagCborCodec> + Encode<DagCborCodec> + OptimizedAwayValue,
{
    fn new(item: T) -> Self {
        Self::Dirty(item)
    }

    async fn resolve<B: BlockStore>(&self, store: &B) -> Result<&T> {
        match self {
            Self::Clean { cid, cache } => {
                cache
                    .get_or_try_init(async { blockstore::load(store, cid).await })
                    .await
            }
            Self::Dirty(node) => Ok(node),
        }
    }

    // TODO(matheus23) hmmm. Do we even need this?
    // async fn get_mut<B: BlockStore>(&mut self, store: &B) -> Result<&mut T> {
    //     match self {
    //         IpldLink::Clean { cid, cache } => match cache.get_mut() {
    //             Some(node) => Ok(node),
    //             None => {
    //                 let mut node = blockstore::load(store, &cid).await?;
    //                 Ok(&mut node)
    //             }
    //         },
    //         IpldLink::Dirty(node) => Ok(node),
    //     }
    // }

    async fn get<B: BlockStore>(self, store: &B) -> Result<T> {
        match self {
            Self::Clean { cid, cache } => match cache.into_inner() {
                Some(cached) => Ok(cached),
                None => blockstore::load(store, &cid).await,
            },
            Self::Dirty(node) => Ok(node),
        }
    }

    async fn seal<B: BlockStore>(&mut self, store: &mut B) -> Result<Cid> {
        match self {
            Self::Clean { cid, .. } => Ok(*cid),
            Self::Dirty(node) => {
                let mut bytes: Vec<u8> = Vec::new();
                node.encode(DagCborCodec, &mut bytes)?;
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
        let link = IpldLink::new(10_u64);
        let mut link = Link::to(42_u64);
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
