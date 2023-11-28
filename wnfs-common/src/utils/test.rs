use super::{Arc, CondSend, CondSync};
use crate::{BlockStore, MemoryBlockStore, CODEC_DAG_CBOR, CODEC_RAW};
use anyhow::Result;
use async_trait::async_trait;
use base64_serde::base64_serde_type;
use bytes::Bytes;
use libipld::{
    cbor::DagCborCodec,
    json::DagJsonCodec,
    prelude::{Decode, Encode},
    Cid, Ipld,
};
use parking_lot::Mutex;
use proptest::{
    strategy::{Strategy, ValueTree},
    test_runner::TestRunner,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{
    collections::{BTreeMap, HashMap},
    io::Cursor,
};

//--------------------------------------------------------------------------------------------------
// Type Definitions
//--------------------------------------------------------------------------------------------------

pub trait BytesToIpld: CondSync {
    fn convert(&self, bytes: &Bytes) -> Result<Ipld>;
}

type BlockHandler = Arc<dyn BytesToIpld>;

#[derive(Default)]
pub struct SnapshotBlockStore {
    inner: MemoryBlockStore,
    block_handlers: Arc<Mutex<HashMap<Cid, BlockHandler>>>,
}

base64_serde_type!(Base64Standard, base64::engine::general_purpose::STANDARD);

#[derive(Serialize, Deserialize, Debug)]
pub struct BlockSnapshot {
    pub value: Value,
    #[serde(with = "Base64Standard")]
    pub bytes: Bytes,
}

pub trait Sampleable {
    type Value;
    fn sample(&self, runner: &mut TestRunner) -> Self::Value;
}

//--------------------------------------------------------------------------------------------------
// Implementations
//--------------------------------------------------------------------------------------------------

impl SnapshotBlockStore {
    pub async fn get_block_snapshot(&self, cid: &Cid) -> Result<BlockSnapshot> {
        let bytes = self.get_block(cid).await?;
        self.handle_block(cid, &bytes).map(|(_, snapshot)| snapshot)
    }

    pub fn handle_block(&self, cid: &Cid, bytes: &Bytes) -> Result<(String, BlockSnapshot)> {
        let ipld = match cid.codec() {
            CODEC_DAG_CBOR => Ipld::decode(DagCborCodec, &mut Cursor::new(bytes))?,
            CODEC_RAW => match self.block_handlers.lock().get(cid) {
                Some(func) => func.convert(bytes)?,
                None => Ipld::Bytes(bytes.to_vec()),
            },
            _ => unimplemented!(),
        };

        let mut json_bytes = Vec::new();
        ipld.encode(DagJsonCodec, &mut json_bytes)?;

        let value = serde_json::from_slice(&json_bytes)?;
        Ok((
            cid.to_string(),
            BlockSnapshot {
                value,
                bytes: bytes.clone(),
            },
        ))
    }

    pub fn get_all_block_snapshots(&self) -> Result<BTreeMap<String, BlockSnapshot>> {
        self.inner
            .0
            .lock()
            .iter()
            .map(|(cid, bytes)| self.handle_block(cid, bytes))
            .collect()
    }

    pub fn add_block_handler(&mut self, cid: Cid, f: BlockHandler) {
        self.block_handlers.lock().insert(cid, f);
    }
}

#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
impl BlockStore for SnapshotBlockStore {
    #[inline]
    async fn get_block(&self, cid: &Cid) -> Result<Bytes> {
        self.inner.get_block(cid).await
    }

    #[inline]
    async fn put_block(&self, bytes: impl Into<Bytes> + CondSend, codec: u64) -> Result<Cid> {
        self.inner.put_block(bytes, codec).await
    }
}

impl<V, S> Sampleable for S
where
    S: Strategy<Value = V>,
{
    type Value = V;

    fn sample(&self, runner: &mut TestRunner) -> Self::Value {
        self.new_tree(runner)
            .expect("Couldn't generate test value")
            .current()
    }
}
