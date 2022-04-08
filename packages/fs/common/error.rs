//! File system errors.

use anyhow::Result;
use std::{
    error::Error,
    fmt::{Debug, Display},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FsError {
    CIDNotFoundInBlockstore,
    InvalidPath,
    NodeNotFound,
}

impl std::error::Error for FsError {}

impl Display for FsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub fn error<T>(err: impl Error + Send + Sync + 'static) -> Result<T> {
    Err(err.into())
}
