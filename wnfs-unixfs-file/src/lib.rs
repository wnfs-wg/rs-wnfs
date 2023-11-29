pub mod balanced_tree;
pub mod builder;
pub mod chunker;
pub mod codecs;
pub mod content_loader;
pub mod hamt;
pub mod protobufs;
mod types;
pub mod unixfs;

use crate::codecs::Codec;
pub use crate::types::{Block, Link, LinkRef, Links, LoadedCid, PbLinks, Source};
use anyhow::{bail, Context as _, Result};
use libipld::{prelude::Codec as _, Cid, Ipld, IpldCodec};
use std::collections::BTreeSet;

/// Extract links from the given content.
///
/// Links will be returned as a sorted vec
pub fn parse_links(cid: &Cid, bytes: &[u8]) -> Result<Vec<Cid>> {
    let codec = Codec::try_from(cid.codec()).context("unknown codec")?;
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
