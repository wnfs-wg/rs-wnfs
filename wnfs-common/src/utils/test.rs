use crate::{decode, encode, AsyncSerialize, BlockStore, MemoryBlockStore, CODEC_DAG_JSON};
use anyhow::Result;
use base64_serde::base64_serde_type;
use bytes::Bytes;
use libipld::{cbor::DagCborCodec, json::DagJsonCodec, serde as ipld_serde, Cid, Ipld};
use proptest::{
    strategy::{Strategy, ValueTree},
    test_runner::TestRunner,
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

//--------------------------------------------------------------------------------------------------
// Type Definitions
//--------------------------------------------------------------------------------------------------

#[derive(Default)]
pub struct MockStore(MemoryBlockStore);

base64_serde_type!(Base64Standard, base64::engine::general_purpose::STANDARD);

#[derive(Serialize, Deserialize, Debug)]
pub struct MockData<V>
where
    V: Serialize,
{
    pub value: V,
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

#[async_trait::async_trait(?Send)]
impl BlockStore for MockStore {
    #[inline]
    async fn get_block(&self, cid: &Cid) -> Result<Bytes> {
        self.0.get_block(cid).await
    }

    #[inline]
    async fn put_block(&self, bytes: impl Into<Bytes>, codec: u64) -> Result<Cid> {
        self.0.put_block(bytes, codec).await
    }

    async fn get_deserializable<V: DeserializeOwned>(&self, cid: &Cid) -> Result<V> {
        let bytes = self.get_block(cid).await?;
        Ok(serde_json::from_slice(&bytes)?)
    }

    async fn put_serializable<V: Serialize>(&self, value: &V) -> Result<Cid> {
        // We serialize the value to raw CBOR bytes first.
        let cbor = encode(&ipld_serde::to_ipld(value)?, DagCborCodec)?;

        // Then we deserialize it back to `Ipld`. We can be sure value based on the generated cbor.
        let value: Ipld = decode(&cbor, DagCborCodec)?;

        let json = encode(
            &ipld_serde::to_ipld(MockData { value, cbor })?,
            DagJsonCodec,
        )?;

        self.put_block(json, CODEC_DAG_JSON).await
    }

    async fn put_async_serializable<V: AsyncSerialize>(&self, value: &V) -> Result<Cid> {
        // We serialize the value to raw CBOR bytes first.
        let cbor = encode(&value.async_serialize_ipld(self).await?, DagCborCodec)?;

        // Then we deserialize it back to `Ipld`. We can be sure value based on the generated cbor.
        let value: Ipld = decode(&cbor, DagCborCodec)?;

        let json = encode(
            &ipld_serde::to_ipld(MockData { value, cbor })?,
            DagJsonCodec,
        )?;

        self.put_block(json, CODEC_DAG_JSON).await
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
