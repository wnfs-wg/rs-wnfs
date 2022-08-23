use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{HashOutput, Id, Metadata, UnixFsNodeKind};

use super::{namefilter::Namefilter, INumber, PrivateNodeHeader};

//--------------------------------------------------------------------------------------------------
// Type Definitions
//--------------------------------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PrivateFileContent {
    pub(crate) metadata: Metadata,
    pub(crate) content: Vec<u8>, // Inlined file content. // TODO(appcypher): Support linked file content.
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrivateFile {
    pub(crate) header: PrivateNodeHeader,
    pub(crate) content: PrivateFileContent,
}

//--------------------------------------------------------------------------------------------------
// Implementations
//--------------------------------------------------------------------------------------------------

impl PrivateFile {
    pub fn new(
        parent_bare_name: Namefilter,
        inumber: INumber,
        ratchet_seed: HashOutput,
        time: DateTime<Utc>,
        content: Vec<u8>,
    ) -> Self {
        Self {
            header: PrivateNodeHeader::new(parent_bare_name, inumber, ratchet_seed),
            content: PrivateFileContent {
                metadata: Metadata::new(time, UnixFsNodeKind::File),
                content,
            },
        }
    }
}

impl Id for PrivateFile {
    fn get_id(&self) -> String {
        format!("{:p}", &self.header)
    }
}
