use chrono::{DateTime, Utc};
use field_names::FieldNames;

use crate::{Metadata, UnixFsNodeKind};

use super::{Hamt, Namefilter};

#[derive(Debug, FieldNames)]
pub struct PrivateDirectory {
    metadata: Metadata,
    name: Namefilter,
    i_number: u64,
    userland: Hamt<(), ()>, // TODO(appcypher): use a real types
}

impl PrivateDirectory {
    pub fn new(time: DateTime<Utc>, i_number: u64) -> Self {
        Self {
            metadata: Metadata::new(time, UnixFsNodeKind::Dir),
            userland: Hamt::default(),
            name: Namefilter::default(),
            i_number,
        }
    }
}
