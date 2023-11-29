use crate::{
    balanced_tree::{TreeBuilder, DEFAULT_DEGREE},
    chunker::{self, Chunker, ChunkerConfig, DEFAULT_CHUNK_SIZE_LIMIT},
    protobufs,
    types::Block,
};
use anyhow::{ensure, Result};
use bytes::Bytes;
use futures::{Stream, StreamExt};
use prost::Message;
use std::{fmt::Debug, path::PathBuf, pin::Pin};
use tokio::io::AsyncRead;

enum Content {
    Reader(Pin<Box<dyn AsyncRead + Send>>),
    Path(PathBuf),
}

impl Debug for Content {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Content::Reader(_) => write!(f, "Content::Reader(Pin<Box<dyn AsyncRead + Send>>)"),
            Content::Path(p) => write!(f, "Content::Path({})", p.display()),
        }
    }
}

impl PartialEq for Content {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Content::Reader(_), Content::Reader(_)) => false,
            (Content::Path(self_path), Content::Path(other_path)) => self_path == other_path,
            _ => false,
        }
    }
}

/// Representation of a constructed File.
#[derive(PartialEq)]
pub struct File {
    name: String,
    content: Content,
    tree_builder: TreeBuilder,
    chunker: Chunker,
}

impl Debug for File {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("File")
            .field("name", &self.name)
            .field("content", &self.content)
            .field("tree_builder", &self.tree_builder)
            .field("chunker", &self.chunker)
            .finish()
    }
}

impl File {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub async fn encode_root(self) -> Result<Block> {
        let mut current = None;
        let parts = self.encode().await?;
        tokio::pin!(parts);

        while let Some(part) = parts.next().await {
            current = Some(part);
        }

        current.expect("must not be empty")
    }

    pub async fn encode(self) -> Result<impl Stream<Item = Result<Block>>> {
        let reader = match self.content {
            Content::Path(path) => {
                let f = tokio::fs::File::open(path).await?;
                let buf = tokio::io::BufReader::new(f);
                Box::pin(buf)
            }
            Content::Reader(reader) => reader,
        };
        let chunks = self.chunker.chunks(reader);
        Ok(self.tree_builder.stream_tree(chunks))
    }
}

/// Constructs a UnixFS file.
pub struct FileBuilder {
    name: Option<String>,
    path: Option<PathBuf>,
    reader: Option<Pin<Box<dyn AsyncRead + Send>>>,
    chunker: Chunker,
    degree: usize,
}

impl Default for FileBuilder {
    fn default() -> Self {
        Self {
            name: None,
            path: None,
            reader: None,
            chunker: Chunker::Fixed(chunker::Fixed::default()),
            degree: DEFAULT_DEGREE,
        }
    }
}

impl Debug for FileBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let reader = if self.reader.is_some() {
            "Some(Box<AsyncRead>)"
        } else {
            "None"
        };
        f.debug_struct("FileBuilder")
            .field("path", &self.path)
            .field("name", &self.name)
            .field("chunker", &self.chunker)
            .field("degree", &self.degree)
            .field("reader", &reader)
            .finish()
    }
}

/// FileBuilder separates uses a reader or bytes to chunk the data into raw unixfs nodes
impl FileBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn path<P: Into<PathBuf>>(mut self, path: P) -> Self {
        self.path = Some(path.into());
        self
    }

    pub fn name<N: Into<String>>(mut self, name: N) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn chunker(mut self, chunker: Chunker) -> Self {
        self.chunker = chunker;
        self
    }

    /// Set the chunker to be fixed size.
    pub fn fixed_chunker(mut self, chunk_size: usize) -> Self {
        self.chunker = Chunker::Fixed(chunker::Fixed::new(chunk_size));
        self
    }

    /// Use the rabin chunker.
    pub fn rabin_chunker(mut self) -> Self {
        self.chunker = Chunker::Rabin(Box::default());
        self
    }

    pub fn degree(mut self, degree: usize) -> Self {
        self.degree = degree;
        self
    }

    pub fn content_bytes<B: Into<Bytes>>(mut self, content: B) -> Self {
        let bytes = content.into();
        self.reader = Some(Box::pin(std::io::Cursor::new(bytes)));
        self
    }

    pub fn content_reader<T: AsyncRead + Send + 'static>(mut self, content: T) -> Self {
        self.reader = Some(Box::pin(content));
        self
    }

    pub async fn build(self) -> Result<File> {
        let degree = self.degree;
        let chunker = self.chunker;
        let tree_builder = TreeBuilder::balanced_tree_with_degree(degree);
        if let Some(path) = self.path {
            let name = match self.name {
                Some(n) => n,
                None => path
                    .file_name()
                    .and_then(|s| s.to_str())
                    .unwrap_or_default()
                    .to_string(),
            };
            return Ok(File {
                content: Content::Path(path),
                name,
                chunker,
                tree_builder,
            });
        }

        if let Some(reader) = self.reader {
            let name = self.name.ok_or_else(|| {
                anyhow::anyhow!("must add a name when building a file from a reader or bytes")
            })?;

            return Ok(File {
                content: Content::Reader(reader),
                name,
                chunker,
                tree_builder,
            });
        }
        anyhow::bail!("must have a path to the content or a reader for the content");
    }
}

pub(crate) fn encode_unixfs_pb(
    inner: &protobufs::Data,
    links: Vec<protobufs::PbLink>,
) -> Result<protobufs::PbNode> {
    let data = inner.encode_to_vec();
    ensure!(
        data.len() <= DEFAULT_CHUNK_SIZE_LIMIT,
        "node is too large: {} bytes",
        data.len()
    );

    Ok(protobufs::PbNode {
        links,
        data: Some(data.into()),
    })
}

/// Configuration for adding unixfs content
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    /// Should the outer object be wrapped in a directory?
    pub wrap: bool,
    pub chunker: Option<ChunkerConfig>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chunker::DEFAULT_CHUNKS_SIZE;
    use futures::TryStreamExt;

    #[tokio::test]
    async fn test_builder_stream_small() -> Result<()> {
        // Add a file
        let bar_encoded: Vec<_> = {
            let bar_reader = std::io::Cursor::new(b"bar");
            let bar = FileBuilder::new()
                .name("bar.txt")
                .content_reader(bar_reader)
                .build()
                .await?;
            bar.encode().await?.try_collect().await?
        };
        assert_eq!(bar_encoded.len(), 1);

        // TODO: check content
        Ok(())
    }

    #[tokio::test]
    async fn test_builder_stream_large() -> Result<()> {
        // Add a file
        let bar_encoded: Vec<_> = {
            let bar_reader = std::io::Cursor::new(vec![1u8; 1024 * 1024]);
            let bar = FileBuilder::new()
                .name("bar.txt")
                .content_reader(bar_reader)
                .build()
                .await?;
            bar.encode().await?.try_collect().await?
        };
        assert_eq!(bar_encoded.len(), 5);

        // Add a file
        let mut baz_content = Vec::with_capacity(1024 * 1024 * 2);
        for i in 0..2 {
            for _ in 0..(1024 * 1024) {
                baz_content.push(i);
            }
        }

        let baz_encoded: Vec<_> = {
            let baz_reader = std::io::Cursor::new(baz_content);
            let baz = FileBuilder::new()
                .name("baz.txt")
                .content_reader(baz_reader)
                .build()
                .await?;
            baz.encode().await?.try_collect().await?
        };
        assert_eq!(baz_encoded.len(), 9);

        // TODO: check content
        Ok(())
    }

    #[test]
    fn test_chunk_config_from_str() {
        assert_eq!(
            "fixed".parse::<ChunkerConfig>().unwrap(),
            ChunkerConfig::Fixed(DEFAULT_CHUNKS_SIZE)
        );
        assert_eq!(
            "fixed-123".parse::<ChunkerConfig>().unwrap(),
            ChunkerConfig::Fixed(123)
        );

        assert!("fixed-".parse::<ChunkerConfig>().is_err());
        assert!(format!("fixed-{}", DEFAULT_CHUNK_SIZE_LIMIT + 1)
            .parse::<ChunkerConfig>()
            .is_err());
        assert!("foo-123".parse::<ChunkerConfig>().is_err());
        assert!("foo".parse::<ChunkerConfig>().is_err());

        assert_eq!(
            "rabin".parse::<ChunkerConfig>().unwrap(),
            ChunkerConfig::Rabin
        );
    }
}
