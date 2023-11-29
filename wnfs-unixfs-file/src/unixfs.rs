use crate::{
    chunker::DEFAULT_CHUNK_SIZE_LIMIT,
    codecs::Codec,
    protobufs,
    types::{Block, Link, LinkRef, Links, PbLinks},
};
use anyhow::{anyhow, bail, ensure, Result};
use bytes::{Buf, Bytes};
use futures::{future::BoxFuture, FutureExt};
use libipld::{multihash::MultihashDigest, Cid};
use prost::Message;
use std::{
    collections::VecDeque,
    fmt::Debug,
    pin::Pin,
    task::{Context, Poll},
};
use tokio::io::{AsyncRead, AsyncSeek};
use wnfs_common::BlockStore;

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, num_enum::IntoPrimitive, num_enum::TryFromPrimitive,
)]
#[repr(i32)]
pub enum DataType {
    Raw = 0,
    Directory = 1,
    File = 2,
    Metadata = 3,
    Symlink = 4,
    HamtShard = 5,
}

#[derive(Debug, Clone)]
pub struct Unixfs {
    inner: protobufs::Data,
}

impl Unixfs {
    pub fn from_bytes<B: Buf>(bytes: B) -> Result<Self> {
        let proto = protobufs::Data::decode(bytes)?;

        Ok(Unixfs { inner: proto })
    }

    pub fn typ(&self) -> DataType {
        self.inner.r#type.try_into().expect("invalid data type")
    }

    pub fn data(&self) -> Option<&Bytes> {
        self.inner.data.as_ref()
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum UnixfsNode {
    Raw(Bytes),
    RawNode(Node),
    File(Node),
}

#[derive(
    Debug, Copy, Clone, PartialEq, Eq, num_enum::IntoPrimitive, num_enum::TryFromPrimitive, Hash,
)]
#[repr(u64)]
pub enum HamtHashFunction {
    /// Murmur3 6464
    Murmur3 = 0x22,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Node {
    pub(super) outer: protobufs::PbNode,
    pub(super) inner: protobufs::Data,
}

impl Node {
    fn encode(&self) -> Result<Bytes> {
        let bytes = self.outer.encode_to_vec();
        Ok(bytes.into())
    }

    pub fn typ(&self) -> DataType {
        self.inner.r#type.try_into().expect("invalid data type")
    }

    pub fn data(&self) -> Option<Bytes> {
        self.inner.data.clone()
    }

    pub fn filesize(&self) -> Option<u64> {
        self.inner.filesize
    }

    pub fn blocksizes(&self) -> &[u64] {
        &self.inner.blocksizes
    }

    pub fn size(&self) -> Option<usize> {
        if self.outer.links.is_empty() {
            return Some(
                self.inner
                    .data
                    .as_ref()
                    .map(|d| d.len())
                    .unwrap_or_default(),
            );
        }

        None
    }

    pub fn links(&self) -> Links {
        Links::Node(PbLinks::new(&self.outer))
    }

    /// Returns the hash type. Only used for HAMT Shards.
    pub fn hash_type(&self) -> Option<HamtHashFunction> {
        self.inner.hash_type.and_then(|t| t.try_into().ok())
    }

    /// Returns the fanout value. Only used for HAMT Shards.
    pub fn fanout(&self) -> Option<u32> {
        self.inner.fanout.and_then(|f| u32::try_from(f).ok())
    }
}

impl UnixfsNode {
    pub fn decode(cid: &Cid, buf: Bytes) -> Result<Self> {
        match cid.codec() {
            c if c == Codec::Raw as u64 => Ok(UnixfsNode::Raw(buf)),
            _ => {
                let outer = protobufs::PbNode::decode(buf)?;
                let inner_data = outer
                    .data
                    .as_ref()
                    .cloned()
                    .ok_or_else(|| anyhow!("missing data"))?;
                let inner = protobufs::Data::decode(inner_data)?;
                let typ: DataType = inner.r#type.try_into()?;
                let node = Node { outer, inner };

                // ensure correct unixfs type
                match typ {
                    DataType::Raw => todo!(),
                    DataType::File => Ok(UnixfsNode::File(node)),
                    _ => bail!("unixfs data type unsupported: {typ:?}"),
                }
            }
        }
    }

    pub fn encode(&self) -> Result<Block> {
        let res = match self {
            UnixfsNode::Raw(data) => {
                let out = data.clone();
                let links = vec![];
                let cid = Cid::new_v1(
                    Codec::Raw as _,
                    libipld::multihash::Code::Sha2_256.digest(&out),
                );
                Block::new(cid, out, links)
            }
            UnixfsNode::RawNode(node) | UnixfsNode::File(node) => {
                let out = node.encode()?;
                let links = node
                    .links()
                    .map(|x| Ok(x?.cid))
                    .collect::<Result<Vec<_>>>()?;
                let cid = Cid::new_v1(
                    Codec::DagPb as _,
                    libipld::multihash::Code::Sha2_256.digest(&out),
                );
                Block::new(cid, out, links)
            }
        };

        ensure!(
            res.data().len() <= DEFAULT_CHUNK_SIZE_LIMIT,
            "node is too large: {} bytes",
            res.data().len()
        );

        Ok(res)
    }

    pub const fn typ(&self) -> Option<DataType> {
        match self {
            UnixfsNode::Raw(_) => None,
            UnixfsNode::RawNode(_) => Some(DataType::Raw),
            UnixfsNode::File(_) => Some(DataType::File),
        }
    }

    /// Returns the size in bytes of the underlying data.
    /// Available only for `Raw` and `File` which are a single block with no links.
    pub fn size(&self) -> Option<usize> {
        match self {
            UnixfsNode::Raw(data) => Some(data.len()),
            UnixfsNode::RawNode(node) | UnixfsNode::File(node) => node.size(),
        }
    }

    /// Returns the filesize in bytes.
    /// Should only be set for `Raw` and `File`.
    pub fn filesize(&self) -> Option<u64> {
        match self {
            UnixfsNode::Raw(data) => Some(data.len() as u64),
            UnixfsNode::RawNode(node) | UnixfsNode::File(node) => node.filesize(),
        }
    }

    /// Returns the blocksizes of the links
    /// Should only be set for File
    pub fn blocksizes(&self) -> &[u64] {
        match self {
            UnixfsNode::Raw(_) => &[],
            UnixfsNode::RawNode(node) | UnixfsNode::File(node) => node.blocksizes(),
        }
    }

    pub fn links(&self) -> Links<'_> {
        match self {
            UnixfsNode::Raw(_) => Links::Leaf,
            UnixfsNode::RawNode(node) => Links::Node(PbLinks::new(&node.outer)),
            UnixfsNode::File(node) => Links::Node(PbLinks::new(&node.outer)),
        }
    }

    pub fn links_owned(&self) -> Result<VecDeque<Link>> {
        self.links().map(|l| l.map(|l| l.to_owned())).collect()
    }

    pub async fn get_link_by_name<S: AsRef<str>>(
        &self,
        link_name: S,
    ) -> Result<Option<LinkRef<'_>>> {
        let link_name = link_name.as_ref();
        self.links()
            .find(|l| match l {
                Ok(l) => l.name == Some(link_name),
                _ => false,
            })
            .transpose()
    }

    pub fn into_content_reader<B: BlockStore>(
        self,
        store: &B,
        pos_max: Option<usize>,
    ) -> Result<Option<UnixfsFileReader<'_, B>>> {
        match self {
            UnixfsNode::Raw(_) | UnixfsNode::RawNode(_) | UnixfsNode::File(_) => {
                let current_links = vec![self.links_owned()?];

                Ok(Some(UnixfsFileReader {
                    root_node: self,
                    pos: 0,
                    pos_max,
                    current_node: CurrentNodeState::Outer,
                    current_links,
                    store,
                }))
            }
        }
    }
}

#[derive(Debug)]
pub struct UnixfsFileReader<'a, B: BlockStore> {
    root_node: UnixfsNode,
    /// Absolute position in bytes
    pos: usize,
    /// Absolute max position in bytes, only used for clipping responses
    pos_max: Option<usize>,
    /// Current node being operated on, only used for nested nodes (not the root).
    current_node: CurrentNodeState<'a>,
    /// Stack of links left to traverse.
    current_links: Vec<VecDeque<Link>>,
    store: &'a B,
}

impl<'a, B: BlockStore> UnixfsFileReader<'a, B> {
    /// Returns the size in bytes, if known in advance.
    pub fn size(&self) -> Option<u64> {
        self.root_node.filesize()
    }
}

impl<'a, B: BlockStore + Unpin + 'static> AsyncRead for UnixfsFileReader<'a, B> {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut tokio::io::ReadBuf<'_>,
    ) -> Poll<std::io::Result<()>> {
        let UnixfsFileReader {
            root_node,
            pos,
            pos_max,
            current_node,
            current_links,
            store,
        } = &mut *self;

        let typ = root_node.typ();
        // let pos_old = *pos; Unused, see bytes_read below
        match root_node {
            UnixfsNode::Raw(data) => {
                read_data_to_buf(pos, *pos_max, &data[*pos..], buf);
                Poll::Ready(Ok(()))
            }
            UnixfsNode::File(node) => poll_read_file_at(
                cx,
                node,
                *store,
                pos,
                *pos_max,
                buf,
                current_links,
                current_node,
            ),
            _ => Poll::Ready(Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("unsupported Unixfs type for file types: {typ:?} "),
            ))),
        }
        // let bytes_read = *pos - pos_old; // Unused, used to be used for metrics
        // poll_res
    }
}

impl<'a, B: BlockStore + Unpin + Clone + 'static> AsyncSeek for UnixfsFileReader<'a, B> {
    fn start_seek(mut self: Pin<&mut Self>, position: std::io::SeekFrom) -> std::io::Result<()> {
        let UnixfsFileReader {
            root_node,
            pos,
            current_node,
            current_links,
            ..
        } = &mut *self;
        let data_len = root_node.size();
        *current_node = CurrentNodeState::Outer;
        *current_links = vec![root_node.links_owned().unwrap()];
        match position {
            std::io::SeekFrom::Start(offset) => {
                let mut i = offset as usize;
                if let Some(data_len) = data_len {
                    if data_len == 0 {
                        *pos = 0;
                        return Ok(());
                    }
                    i = std::cmp::min(i, data_len - 1);
                }
                *pos = i;
            }
            std::io::SeekFrom::End(offset) => {
                if let Some(data_len) = data_len {
                    if data_len == 0 {
                        *pos = 0;
                        return Ok(());
                    }
                    let mut i = (data_len as i64 + offset) % data_len as i64;
                    if i < 0 {
                        i += data_len as i64;
                    }
                    *pos = i as usize;
                } else {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        "cannot seek from end of unknown length",
                    ));
                }
            }
            std::io::SeekFrom::Current(offset) => {
                let mut i = *pos as i64 + offset;
                i = std::cmp::max(0, i);

                if let Some(data_len) = data_len {
                    if data_len == 0 {
                        *pos = 0;
                        return Ok(());
                    }
                    i = std::cmp::min(i, data_len as i64 - 1);
                }
                *pos = i as usize;
            }
        }
        Ok(())
    }

    fn poll_complete(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<std::io::Result<u64>> {
        Poll::Ready(Ok(self.pos as u64))
    }
}

pub fn read_data_to_buf(
    pos: &mut usize,
    pos_max: Option<usize>,
    data: &[u8],
    buf: &mut tokio::io::ReadBuf<'_>,
) -> usize {
    let data_to_read = pos_max.map(|pos_max| pos_max - *pos).unwrap_or(data.len());
    let amt = std::cmp::min(std::cmp::min(data_to_read, buf.remaining()), data.len());
    buf.put_slice(&data[..amt]);
    *pos += amt;
    amt
}

pub fn find_block(node: &UnixfsNode, pos: u64, node_offset: u64) -> (u64, Option<usize>) {
    let pivots = node
        .blocksizes()
        .iter()
        .scan(node_offset, |state, &x| {
            *state += x;
            Some(*state)
        })
        .collect::<Vec<_>>();
    let block_index = match pivots.binary_search(&pos) {
        Ok(b) => b + 1,
        Err(b) => b,
    };
    if block_index < pivots.len() {
        let next_node_offset = if block_index > 0 {
            pivots[block_index - 1]
        } else {
            node_offset
        };
        (next_node_offset, Some(block_index))
    } else {
        (pivots[pivots.len() - 1], None)
    }
}

#[allow(clippy::large_enum_variant)]
pub enum CurrentNodeState<'a> {
    // Initial state
    Outer,
    // Need to load next node from the list
    NextNodeRequested {
        next_node_offset: usize,
    },
    // Node has been loaded and ready to be processed
    Loaded {
        node_offset: usize,
        node_pos: usize,
        node: UnixfsNode,
    },
    // Ongoing loading of the node
    Loading {
        node_offset: usize,
        fut: BoxFuture<'a, Result<UnixfsNode>>,
    },
}

impl<'a> Debug for CurrentNodeState<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CurrentNodeState::Outer => write!(f, "CurrentNodeState::Outer"),
            CurrentNodeState::NextNodeRequested { next_node_offset } => {
                write!(f, "CurrentNodeState::None ({next_node_offset})")
            }
            CurrentNodeState::Loaded {
                node_offset,
                node_pos,
                node,
            } => {
                write!(
                    f,
                    "CurrentNodeState::Loaded({node_offset:?}, {node_pos:?}, {node:?})"
                )
            }
            CurrentNodeState::Loading { .. } => write!(f, "CurrentNodeState::Loading(Fut)"),
        }
    }
}

fn load_next_node<'a>(
    next_node_offset: usize,
    current_node: &mut CurrentNodeState<'a>,
    current_links: &mut Vec<VecDeque<Link>>,
    store: &'a impl BlockStore,
) -> bool {
    let links = loop {
        if let Some(last_mut) = current_links.last_mut() {
            if last_mut.is_empty() {
                // ignore empty links
                current_links.pop();
            } else {
                // found non empty links
                break last_mut;
            }
        } else {
            // no links left we are done
            return false;
        }
    };

    let link = links.pop_front().unwrap();

    let fut = async move {
        let block = store.get_block(&link.cid).await?;
        let node = UnixfsNode::decode(&link.cid, block)?;

        Ok(node)
    }
    .boxed();
    *current_node = CurrentNodeState::Loading {
        node_offset: next_node_offset,
        fut,
    };
    true
}

#[allow(clippy::too_many_arguments)]
fn poll_read_file_at<'a>(
    cx: &mut Context<'_>,
    root_node: &Node,
    store: &'a impl BlockStore,
    pos: &mut usize,
    pos_max: Option<usize>,
    buf: &mut tokio::io::ReadBuf<'_>,
    current_links: &mut Vec<VecDeque<Link>>,
    current_node: &mut CurrentNodeState<'a>,
) -> Poll<std::io::Result<()>> {
    loop {
        if let Some(pos_max) = pos_max {
            if pos_max <= *pos {
                return Poll::Ready(Ok(()));
            }
        }
        match current_node {
            CurrentNodeState::Outer => {
                // check for links
                if root_node.outer.links.is_empty() {
                    // simplest case just one file
                    let data = root_node.inner.data.as_deref().unwrap_or(&[][..]);
                    read_data_to_buf(pos, pos_max, &data[*pos..], buf);
                    return Poll::Ready(Ok(()));
                }

                // read root local data
                if let Some(ref data) = root_node.inner.data {
                    if *pos < data.len() {
                        read_data_to_buf(pos, pos_max, &data[*pos..], buf);
                        return Poll::Ready(Ok(()));
                    }
                }
                *current_node = CurrentNodeState::NextNodeRequested {
                    next_node_offset: 0,
                };
            }
            CurrentNodeState::NextNodeRequested { next_node_offset } => {
                let loaded_next_node =
                    load_next_node(*next_node_offset, current_node, current_links, store);
                if !loaded_next_node {
                    return Poll::Ready(Ok(()));
                }
            }
            CurrentNodeState::Loading { node_offset, fut } => {
                match fut.poll_unpin(cx) {
                    Poll::Pending => {
                        return Poll::Pending;
                    }
                    Poll::Ready(Ok(node)) => {
                        match node.links_owned() {
                            Ok(links) => {
                                if !links.is_empty() {
                                    let (next_node_offset, block_index) =
                                        find_block(&node, *pos as u64, *node_offset as u64);
                                    if let Some(block_index) = block_index {
                                        let new_links =
                                            links.into_iter().skip(block_index).collect();
                                        current_links.push(new_links);
                                    }
                                    *current_node = CurrentNodeState::NextNodeRequested {
                                        next_node_offset: next_node_offset as usize,
                                    }
                                } else {
                                    *current_node = CurrentNodeState::Loaded {
                                        node_offset: *node_offset,
                                        node_pos: *pos - *node_offset,
                                        node,
                                    }
                                }
                            }
                            Err(e) => {
                                return Poll::Ready(Err(std::io::Error::new(
                                    std::io::ErrorKind::InvalidData,
                                    e.to_string(),
                                )));
                            }
                        }
                        // TODO: do one read
                    }
                    Poll::Ready(Err(e)) => {
                        *current_node = CurrentNodeState::NextNodeRequested {
                            next_node_offset: *node_offset,
                        };
                        return Poll::Ready(Err(std::io::Error::new(
                            std::io::ErrorKind::InvalidData,
                            e.to_string(),
                        )));
                    }
                }
            }
            CurrentNodeState::Loaded {
                ref node_offset,
                ref mut node_pos,
                node: ref mut current_node_inner,
            } => match current_node_inner {
                UnixfsNode::Raw(data) => {
                    if *node_offset + data.len() <= *pos {
                        *current_node = CurrentNodeState::NextNodeRequested {
                            next_node_offset: node_offset + data.len(),
                        };
                        continue;
                    }
                    let bytes_read = read_data_to_buf(pos, pos_max, &data[*node_pos..], buf);
                    *node_pos += bytes_read;
                    return Poll::Ready(Ok(()));
                }
                UnixfsNode::File(node) | UnixfsNode::RawNode(node) => {
                    if let Some(ref data) = node.inner.data {
                        if node_offset + data.len() <= *pos {
                            *current_node = CurrentNodeState::NextNodeRequested {
                                next_node_offset: node_offset + data.len(),
                            };
                            continue;
                        }
                        let bytes_read = read_data_to_buf(pos, pos_max, &data[*node_pos..], buf);
                        *node_pos += bytes_read;
                        return Poll::Ready(Ok(()));
                    }
                    *current_node = CurrentNodeState::NextNodeRequested {
                        next_node_offset: *node_offset,
                    };
                }
            },
        }
    }
}
