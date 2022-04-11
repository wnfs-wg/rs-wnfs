//! Public fs file node.

use anyhow::Result;
use chrono::{DateTime, Utc};
use libipld::{cbor::DagCborCodec, prelude::Encode, Cid, DagCbor, IpldCodec};

use crate::{BlockStore, Metadata, UnixFsNodeKind};

/// A file in a WNFS public file system.
#[derive(Debug, Clone, PartialEq, Eq, DagCbor)]
pub struct PublicFile {
    pub(crate) metadata: Metadata,
    pub(crate) userland: Cid,
    pub(crate) previous: Option<Cid>,
}

impl PublicFile {
    /// Creates a new file using the given metadata and CID.
    pub fn new(time: DateTime<Utc>, userland: Cid) -> Self {
        Self {
            metadata: Metadata::new(time, UnixFsNodeKind::File),
            userland,
            previous: None,
        }
    }

    /// Stores WNFS block(s) in chosen block store.
    pub async fn store<B: BlockStore>(&self, store: &mut B) -> Result<Cid> {
        let bytes = {
            let mut tmp = vec![];
            self.encode(DagCborCodec, &mut tmp)?;
            tmp
        };
        store.put_block(bytes, IpldCodec::DagCbor).await
    }
}

#[cfg(test)]
mod public_file_tests {
    use std::io::Cursor;

    use chrono::Utc;
    use libipld::prelude::Decode;

    use super::*;

    #[async_std::test]
    async fn directory_encode_decode_successful() {
        let file = PublicFile::new(Utc::now(), Cid::default());

        let mut encoded_bytes = vec![];

        file.encode(DagCborCodec, &mut encoded_bytes).unwrap();

        let mut cursor = Cursor::new(encoded_bytes);

        let decoded_file = PublicFile::decode(DagCborCodec, &mut cursor).unwrap();

        assert_eq!(file, decoded_file);
    }
}
