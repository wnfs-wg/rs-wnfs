//! Errors

use crate::NodeType;
use anyhow::Result;
use semver::Version;
use skip_ratchet::PreviousErr;
use thiserror::Error;

//--------------------------------------------------------------------------------------------------
// Type Definitions
//--------------------------------------------------------------------------------------------------

#[derive(Debug, Error)]
pub enum HamtError {
    #[error("Hashnibbles cursor has exceeded HashOutput length")]
    CursorOutOfBounds,

    #[error("Cannot canonicalize a link pointer to a node with zero pointer")]
    NonCanonicalizablePointer,

    #[error("Values pointer expected")]
    ValuesPointerExpected,

    #[error("Key does not exist in HAMT")]
    KeyNotFound,
}
