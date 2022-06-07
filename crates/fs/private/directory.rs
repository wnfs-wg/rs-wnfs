use chrono::{DateTime, Utc};
use field_names::FieldNames;

use crate::{Metadata, UnixFsNodeKind};

use super::Namefilter;

#[derive(Debug, FieldNames)]
pub struct PrivateDirectory {
    pub(crate) metadata: Metadata,
    pub(crate) name: Namefilter,
    pub(crate) i_number: u64,
    // pub(crate) userland: Hamt<String, Link>,
}

impl PrivateDirectory {
    pub fn new(time: DateTime<Utc>, i_number: u64) -> Self {
        Self {
            metadata: Metadata::new(time, UnixFsNodeKind::Dir),
            // userland: Hamt::new(),
            name: Namefilter::default(),
            i_number,
        }
    }
}
