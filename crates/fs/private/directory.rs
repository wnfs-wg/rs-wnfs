use chrono::{DateTime, Utc};
use field_names::FieldNames;
use libipld::Cid;

use crate::{Metadata, UnixFsNodeKind};

use super::{Hamt, Namefilter};

#[derive(Debug, FieldNames)]
pub struct PrivateDirectory {
    pub(crate) metadata: Metadata,
    pub(crate) name: Namefilter,
    pub(crate) i_number: u64,
    pub(crate) userland: Hamt<Namefilter, Cid>, // TODO(appcypher): use a real types
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
