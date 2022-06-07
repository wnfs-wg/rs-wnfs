use anyhow::Result;
use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum HamtError {
    CursorOutOfBounds,
    // InvalidHashBitLen,
    // MaxDepth,
}

impl std::error::Error for HamtError {}

impl Display for HamtError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub fn error<T>(err: impl Error + Send + Sync + 'static) -> Result<T> {
    Err(err.into())
}
