#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use super::{ContentKey, KeyPair, PrivateForest, RevisionKey};
use crate::{public::PublicLink, BlockStore, HashOutput, Namefilter};
use anyhow::Result;
use rand_core::RngCore;
use std::rc::Rc;

//--------------------------------------------------------------------------------------------------
// Type Definitions
//--------------------------------------------------------------------------------------------------

struct Sharer<'a, S: BlockStore> {
    root_did: String,
    store: &'a mut S,
    forest: Rc<PrivateForest>,
}

struct Recipient<'a, S: BlockStore, K: KeyPair = ()> {
    exchange_root: PublicLink,
    store: &'a mut S,
    key: K,
}

struct Share<'a, S: BlockStore> {
    data: &'a [u8],
    count: usize,
    sharer: Option<&'a mut Sharer<'a, S>>,
    recipient: Option<&'a Recipient<'a, S>>,
}

enum SharePayload {
    Temporal {
        label_hash: HashOutput,
        revision_key: RevisionKey,
    },
    Snapshot {
        label_hash: HashOutput,
        content_key: ContentKey,
    },
}

//--------------------------------------------------------------------------------------------------
// Implementations
//--------------------------------------------------------------------------------------------------

impl<'a, S: BlockStore> Sharer<'a, S> {
    fn new(root_did: String, store: &'a mut S, forest: Rc<PrivateForest>) -> Self {
        Self {
            root_did,
            store,
            forest,
        }
    }

    async fn share_with(
        data: impl AsRef<[u8]>,
        recipient: &mut Recipient<'a, S>,
        rng: &mut impl RngCore,
    ) -> Result<Namefilter> {
        // TODO(appcypher): Implement this.
        unimplemented!()
    }

    fn create_share_label(&self, count: usize, exchange_key: &[u8]) -> Namefilter {
        let mut label = Namefilter::default();
        label.add(&self.root_did.as_bytes());
        label.add(&exchange_key);
        label.add(&count.to_le_bytes());
        label.saturate();
        label
    }
}

impl<'a, S: BlockStore, K: KeyPair> Recipient<'a, S, K> {
    fn new(store: &'a mut S, exchange_root: PublicLink, key: K) -> Self {
        Self {
            store,
            exchange_root,
            key,
        }
    }

    async fn receive_share(
        &mut self,
        share_label: Namefilter,
        store: &mut impl BlockStore,
    ) -> Result<Vec<u8>> {
        // TODO(appcypher): Implement this.
        unimplemented!()
    }
}

impl<'a, S: BlockStore> Recipient<'a, S> {
    fn without_key(exchange_root: PublicLink, store: &'a mut S) -> Self {
        Self {
            store,
            exchange_root,
            key: (),
        }
    }
}

impl<'a, S: BlockStore> Share<'a, S> {
    fn new(data: &'a impl AsRef<[u8]>, count: usize) -> Self {
        Self {
            data: data.as_ref(),
            count,
            sharer: None,
            recipient: None,
        }
    }

    fn by(&mut self, sharer: &'a mut Sharer<'a, S>) -> &mut Self {
        self.sharer = Some(sharer);
        self
    }

    fn with(&mut self, recipient: &'a Recipient<'a, S>) -> &mut Self {
        self.recipient = Some(recipient);
        self
    }

    async fn finish(&mut self, rng: &mut impl RngCore) -> Result<Namefilter> {
        // TODO(appcypher): Implement this.
        // if !matches!((self.sharer, self.recipient), (None, None)) { ... }
        unimplemented!()
    }
}

//--------------------------------------------------------------------------------------------------
// Tests
//--------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use libipld::Cid;

    use crate::MemoryBlockStore;

    use super::*;

    #[async_std::test]
    async fn bala_blu_bu_la_ba() {
        let forest = Rc::new(PrivateForest::new());
        let rng = &mut rand::thread_rng();

        let sharer_store = &mut MemoryBlockStore::default();
        let mut sharer = Sharer::new(String::new(), sharer_store, forest);

        let recipient_store = &mut MemoryBlockStore::default();
        let recipient =
            Recipient::without_key(PublicLink::from_cid(Cid::default()), recipient_store);

        let data = b"hello world";

        let label = Share::new(data, 0)
            .by(&mut sharer)
            .with(&recipient)
            .finish(rng)
            .await;
    }
}
