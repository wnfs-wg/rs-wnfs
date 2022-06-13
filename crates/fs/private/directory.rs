use std::collections::BTreeMap;

use chrono::{DateTime, Utc};
use field_names::FieldNames;

use crate::{Metadata, UnixFsNodeKind};

use super::{Namefilter, PrivateLink};

#[derive(Debug, FieldNames)]
pub struct PrivateDirectory {
    pub(crate) metadata: Metadata,
    pub(crate) name: Namefilter,
    pub(crate) nonce: u64,
    pub(crate) userland: BTreeMap<String, PrivateLink>,
}

impl PrivateDirectory {
    pub fn new(time: DateTime<Utc>, nonce: u64) -> Self {
        Self {
            metadata: Metadata::new(time, UnixFsNodeKind::Dir),
            userland: BTreeMap::default(),
            name: Namefilter::default(),
            nonce,
        }
    }
}
