//! Errors

use thiserror::Error;

//--------------------------------------------------------------------------------------------------
// Type Definitions
//--------------------------------------------------------------------------------------------------

/// HAMT errors.
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

    #[error("The hashprefix index is out of bounds: {0}")]
    HashPrefixIndexOutOfBounds(u8),
}
