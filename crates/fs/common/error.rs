//! File system errors.

use std::error::Error;

use anyhow::Result;
use thiserror::Error;

/// File system errors.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum FsError {
    #[error("Cannot find a node with the specified CID in block store")]
    CIDNotFoundInBlockstore,
    #[error("Invalid WNFS path")]
    InvalidPath,
    #[error("Expected a file")]
    NotAFile,
    #[error("Expected a directory")]
    NotADirectory,
    #[error("Cannot find file or directory")]
    NotFound,
    #[error("File already exists")]
    FileAlreadyExists,
    #[error("Directory already exists")]
    DirectoryAlreadyExists,
    #[error("Move operation on invalid path")]
    InvalidMoveLocation,
    #[error("Cannot decide cbor data")]
    UndecodableCborData(String),
}

pub fn error<T>(err: impl Error + Send + Sync + 'static) -> Result<T> {
    Err(err.into())
}
