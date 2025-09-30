use anyhow::Result;
use libipld::{
    Ipld,
    codec::{Decode, Encode},
    prelude::Codec,
    serde as ipld_serde,
};
use serde::{Serialize, de::DeserializeOwned};
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
