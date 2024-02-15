use super::{Arc, CondSend, CondSync};
use crate::{BlockStore, MemoryBlockStore, CODEC_DAG_CBOR, CODEC_RAW};
use anyhow::Result;
use base64_serde::base64_serde_type;
use bytes::Bytes;
use libipld::{
    cbor::DagCborCodec,
    json::DagJsonCodec,
    prelude::{Decode, Encode, References},
    Cid, Ipld, IpldCodec,
};
use parking_lot::Mutex;
use proptest::{
    strategy::{Strategy, ValueTree},
    test_runner::TestRunner,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{
    collections::{HashMap, HashSet, VecDeque},
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
    pub cid: String,
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
        self.handle_block(cid, &bytes)
    }

    pub fn handle_block(&self, cid: &Cid, bytes: &Bytes) -> Result<BlockSnapshot> {
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
        Ok(BlockSnapshot {
            cid: cid.to_string(),
            value,
            bytes: bytes.clone(),
        })
    }

    pub async fn get_dag_snapshot(&self, root_cid: Cid) -> Result<Vec<BlockSnapshot>> {
        let mut frontier = VecDeque::from([root_cid]);
        let mut visited = HashSet::new();
        let mut snapshots = Vec::new();

        while let Some(cid) = frontier.pop_front() {
            if !visited.insert(cid) {
                continue;
            }

            let snapshot = self.get_block_snapshot(&cid).await?;
            let codec: IpldCodec = cid.codec().try_into()?;
            <Ipld as References<IpldCodec>>::references(
                codec,
                &mut Cursor::new(&snapshot.bytes),
                &mut frontier,
            )?;
            snapshots.push(snapshot);
        }

        Ok(snapshots)
    }

    pub fn add_block_handler(&mut self, cid: Cid, f: BlockHandler) {
        self.block_handlers.lock().insert(cid, f);
    }
}

impl BlockStore for SnapshotBlockStore {
    #[inline]
    async fn get_block(&self, cid: &Cid) -> Result<Bytes> {
        self.inner.get_block(cid).await
    }

    #[inline]
    async fn put_block(&self, bytes: impl Into<Bytes> + CondSend, codec: u64) -> Result<Cid> {
        self.inner.put_block(bytes, codec).await
    }

    #[inline]
    async fn put_block_keyed(&self, cid: Cid, bytes: impl Into<Bytes> + CondSend) -> Result<()> {
        self.inner.put_block_keyed(cid, bytes).await
    }

    #[inline]
    async fn has_block(&self, cid: &Cid) -> Result<bool> {
        self.inner.has_block(cid).await
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
