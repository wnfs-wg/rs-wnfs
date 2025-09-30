pub mod balanced_tree;
pub mod builder;
pub mod chunker;
pub mod codecs;
pub mod protobufs;
mod types;
pub mod unixfs;

use crate::codecs::Codec;
use anyhow::{Result, bail};
use ipld_core::ipld::Ipld;
use std::collections::BTreeSet;
use wnfs_common::Cid;

/// Extract links from the given content.
///
/// Links will be returned as a sorted vec
pub fn parse_links(codec: Codec, bytes: &[u8]) -> Result<Vec<Cid>> {
    let mut cids = BTreeSet::new();
    match codec {
        Codec::DagCbor => serde_ipld_dagcbor::from_slice::<Ipld>(bytes)?.references(&mut cids),
        Codec::DagJson => serde_ipld_dagjson::from_slice::<Ipld>(bytes)?.references(&mut cids),
        Codec::DagPb => ipld_dagpb::links(bytes, &mut cids)?,
        Codec::Raw => {}
        _ => bail!("unsupported codec {:?}", codec),
    };
    let links = cids.into_iter().collect();
    Ok(links)
}
