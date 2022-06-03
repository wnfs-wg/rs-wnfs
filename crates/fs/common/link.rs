use anyhow::Result;
use async_once_cell::OnceCell;
use libipld::codec::Encode;
use libipld::IpldCodec;
use libipld::{cbor::DagCborCodec, codec::Decode, Cid};

use crate::blockstore;
use crate::BlockStore;

pub(crate) enum IpldLink<T> {
    Clean { cid: Cid, cache: OnceCell<T> },
    Dirty(T),
}

impl<T> IpldLink<T>
where
    T: Decode<DagCborCodec>,
    T: Encode<DagCborCodec>,
    T: Clone,
{
    pub(crate) fn to(item: T) -> Self {
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
            IpldLink::Clean { cid, cache } => {
                cache
                    .get_or_try_init(async { blockstore::load(store, cid).await })
                    .await
            }
            IpldLink::Dirty(node) => Ok(node),
        }
    }

    pub(crate) async fn get<B: BlockStore>(self, store: &B) -> Result<T> {
        match self {
            IpldLink::Clean { cid, cache } => match cache.into_inner() {
                Some(cached) => Ok(cached),
                None => blockstore::load(store, &cid).await,
            },
            IpldLink::Dirty(node) => Ok(node),
        }
    }

    pub(crate) async fn seal<B: BlockStore>(&mut self, store: &mut B) -> Result<Cid> {
        match self {
            Self::Clean { cid, .. } => Ok(*cid),
            Self::Dirty(node) => {
                let mut bytes: Vec<u8> = Vec::new();
                node.encode(DagCborCodec, &mut bytes)?;
                let cid = store.put_block(bytes, IpldCodec::DagCbor).await?;
                *self = Self::Clean {
                    cid,
                    cache: OnceCell::new_with(Some((*node).clone())),
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
            IpldLink::Clean { cache, .. } => cache.get().is_some(),
            IpldLink::Dirty(_) => true,
        }
    }
}

#[cfg(test)]
mod ipld_link_tests {
    use crate::{IpldLink, MemoryBlockStore};

    #[async_std::test]
    async fn ipld_link() {
        let mut link = IpldLink::to(42_u64);
        let mut store = MemoryBlockStore::default();
        println!("Clean? {}", link.is_clean());
        let cid = link.seal(&mut store).await.unwrap();
        println!("Clean? {}", link.is_clean());
        println!("{}", cid);

        // another link
        let link2: IpldLink<u64> = IpldLink::from_cid(cid);
        println!("Clean? {} Cached? {}", link2.is_clean(), link2.is_cached());
        let num = *link2.resolve(&store).await.unwrap();
        println!("num: {num}");
        // interior mutability makes is_cached suddenly return true :S
        // we may want to just never have that be observable behavior from the outside.
        println!("Clean? {} Cached? {}", link2.is_clean(), link2.is_cached());
    }
}
