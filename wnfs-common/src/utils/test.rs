use crate::{encode, BlockStore, MemoryBlockStore, CODEC_DAG_CBOR, CODEC_RAW};
use anyhow::Result;
use base64_serde::base64_serde_type;
use bytes::Bytes;
use libipld::{
    cbor::DagCborCodec,
    json::DagJsonCodec,
    prelude::{Decode, Encode},
    Cid, Ipld,
};
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

type BlockHandler = Box<dyn Fn(&Bytes) -> Result<Bytes>>;

#[derive(Default)]
pub struct SnapshotBlockStore {
    inner: MemoryBlockStore,
    block_handlers: HashMap<Cid, BlockHandler>,
}

base64_serde_type!(Base64Standard, base64::engine::general_purpose::STANDARD);

#[derive(Serialize, Deserialize, Debug)]
pub struct BlockSnapshot {
    pub value: Value,
    #[serde(with = "Base64Standard")]
    pub cbor: Vec<u8>,
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
        let cbor_bytes = self.get_block(cid).await?;

        let ipld = Ipld::decode(DagCborCodec, &mut Cursor::new(cbor_bytes.clone()))?;
        let mut json_bytes = Vec::new();
        ipld.encode(DagJsonCodec, &mut json_bytes)?;

        let value = serde_json::from_slice(&json_bytes)?;
        Ok(BlockSnapshot {
            value,
            cbor: cbor_bytes.to_vec(),
        })
    }

    pub fn get_all_block_snapshots(&self) -> Result<BTreeMap<String, BlockSnapshot>> {
        self.inner
            .0
            .borrow()
            .iter()
            .map(|(cid, v)| {
                let cbor_bytes = match cid.codec() {
                    CODEC_DAG_CBOR => v.clone(),
                    CODEC_RAW => match self.block_handlers.get(cid) {
                        Some(func) => func(v)?,
                        None => Bytes::from(encode(
                            &Ipld::List(vec![Ipld::Bytes(v.to_vec())]),
                            DagCborCodec,
                        )?),
                    },
                    _ => unimplemented!(),
                };

                let ipld = Ipld::decode(DagCborCodec, &mut Cursor::new(cbor_bytes.clone()))?;

                let mut json_bytes = Vec::new();
                ipld.encode(DagJsonCodec, &mut json_bytes)?;

                let value = serde_json::from_slice(&json_bytes)?;
                Ok((
                    cid.to_string(),
                    BlockSnapshot {
                        value,
                        cbor: cbor_bytes.to_vec(),
                    },
                ))
            })
            .collect()
    }

    pub fn add_block_handler(&mut self, cid: Cid, f: BlockHandler) {
        self.block_handlers.insert(cid, f);
    }
}

#[async_trait::async_trait(?Send)]
impl BlockStore for SnapshotBlockStore {
    #[inline]
    async fn get_block(&self, cid: &Cid) -> Result<Bytes> {
        self.inner.get_block(cid).await
    }

    #[inline]
    async fn put_block(&self, bytes: impl Into<Bytes>, codec: u64) -> Result<Cid> {
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
