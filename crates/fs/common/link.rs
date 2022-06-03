use anyhow::Result;
use async_once_cell::OnceCell;
use libipld::codec::Encode;
use libipld::IpldCodec;
use libipld::{cbor::DagCborCodec, codec::Decode, Cid};

use crate::blockstore;
use crate::BlockStore;

pub(crate) enum IpldLink<T> {
    /// Invariant: the (optional) contents of the cache *must* encode the cid
    Clean {
        cid: Cid,
        cache: OnceCell<T>,
    },
    Dirty(T),
}

impl<T> IpldLink<T>
where
    T: Decode<DagCborCodec> + Encode<DagCborCodec> + OptimizedAwayValue,
{
    fn new(item: T) -> Self {
        Self::Dirty(item)
    }

    async fn resolve<B: BlockStore>(&self, store: &B) -> Result<&T> {
        match self {
            IpldLink::Clean { cid, cache } => {
                cache
                    .get_or_try_init(async { blockstore::load(store, cid).await })
                    .await
            }
            IpldLink::Dirty(node) => Ok(node),
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
            IpldLink::Clean { cid, cache } => match cache.into_inner() {
                Some(cached) => Ok(cached),
                None => blockstore::load(store, &cid).await,
            },
            IpldLink::Dirty(node) => Ok(node),
        }
    }

    async fn seal<B: BlockStore>(&mut self, store: &mut B) -> Result<Cid> {
        match self {
            IpldLink::Clean { cid, .. } => Ok(*cid),
            IpldLink::Dirty(node) => {
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
}

#[cfg(test)]
mod ipld_link_tests {
    use crate::{IpldLink, MemoryBlockStore, OptimizedAwayValue};

    #[async_std::test]
    async fn ipld_link() {
        let link = IpldLink::new(10_u64);
        let mut store = MemoryBlockStore::default();
        let cid = link.seal(&mut store).await.unwrap();
        println!("{}", cid)
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
