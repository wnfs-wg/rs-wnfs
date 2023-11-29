pub mod balanced_tree;
pub mod builder;
pub mod chunker;
pub mod codecs;
pub mod protobufs;
mod types;
pub mod unixfs;

use crate::codecs::Codec;
use anyhow::{bail, Result};
use libipld::{prelude::Codec as _, Cid, Ipld, IpldCodec};
use std::collections::BTreeSet;

/// Extract links from the given content.
///
/// Links will be returned as a sorted vec
pub fn parse_links(codec: Codec, bytes: &[u8]) -> Result<Vec<Cid>> {
    let mut cids = BTreeSet::new();
    let codec = match codec {
        Codec::DagCbor => IpldCodec::DagCbor,
        Codec::DagPb => IpldCodec::DagPb,
        Codec::DagJson => IpldCodec::DagJson,
        Codec::Raw => IpldCodec::Raw,
        _ => bail!("unsupported codec {:?}", codec),
    };
    codec.references::<Ipld, _>(bytes, &mut cids)?;
    let links = cids.into_iter().collect();
    Ok(links)
}
