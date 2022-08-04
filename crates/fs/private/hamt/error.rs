use thiserror::Error;

#[derive(Debug, Error)]
pub enum HamtError {
    #[error("Hashnibbles cursor has exceeded HashOutput length")]
    CursorOutOfBounds,

    #[error("Cannot canonicalize a link pointer to a node with zero pointer")]
    NonCanonicalizablePointer,
    
    #[error("Values pointer expected")]
    ValuesPointerExpected,
}
