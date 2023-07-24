use crate::{AsyncSerialize, BlockStore};
use anyhow::Result;
use libipld::{
    codec::{Decode, Encode},
    prelude::Codec,
    serde as ipld_serde, Ipld,
};
use serde::{de::DeserializeOwned, Serialize};
use std::io::Cursor;

/// Encodes a serializable value into DagCbor bytes.
pub fn encode<S, C>(value: &S, codec: C) -> Result<Vec<u8>>
where
    S: Serialize,
    C: Codec,
    Ipld: Encode<C>,
{
    let ipld = ipld_serde::to_ipld(value)?;
    let mut bytes = Vec::new();
    <Ipld as Encode<C>>::encode(&ipld, codec, &mut bytes)?;
    Ok(bytes)
}

/// Encodes an async serializable value into DagCbor bytes.
pub async fn async_encode<V, C>(value: &V, store: &impl BlockStore, codec: C) -> Result<Vec<u8>>
where
    V: AsyncSerialize,
    C: Codec,
    Ipld: Encode<C>,
{
    let ipld = value.async_serialize_ipld(store).await?;
    let mut bytes = Vec::new();
    <Ipld as Encode<C>>::encode(&ipld, codec, &mut bytes)?;
    Ok(bytes)
}

/// Decodes recieved DagCbor bytes into a deserializable value.
pub fn decode<D, C>(bytes: &[u8], codec: C) -> Result<D>
where
    D: DeserializeOwned,
    C: Codec,
    Ipld: Decode<C>,
{
    let ipld = <Ipld as Decode<C>>::decode(codec, &mut Cursor::new(bytes))?;
    Ok(ipld_serde::from_ipld::<_>(ipld)?)
}
