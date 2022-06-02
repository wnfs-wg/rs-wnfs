use chrono::{DateTime, Utc};

use crate::{Metadata, UnixFsNodeKind};

use super::Namefilter;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrivateFile {
    pub(crate) metadata: Metadata,
    pub(crate) name: Namefilter,
    pub(crate) i_number: u64,
    // pub(crate) content: ???, // TODO(appcypher): How to handle content link here?
}

impl PrivateFile {
    pub fn new(time: DateTime<Utc>, i_number: u64) -> Self {
        Self {
            metadata: Metadata::new(time, UnixFsNodeKind::File),
            name: Namefilter::default(),
            i_number,
        }
    }
}
