//! File system errors.

use std::{
    error::Error,
    fmt::{Debug, Display},
};

use anyhow::Result;

/// File system errors.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FsError {
    CIDNotFoundInBlockstore,
    InvalidPath,
    NotAFile,
    NotADirectory,
    NotFound,
    FileAlreadyExists,
    DirectoryAlreadyExists,
    UndecodableCborData(String),
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
