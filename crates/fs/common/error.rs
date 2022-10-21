//! File system errors.

use anyhow::Result;
use thiserror::Error;

use crate::NodeType;

/// File system errors.
#[derive(Debug, Error)]
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

    #[error("Unable to encrypt data: {0}")]
    UnableToEncrypt(String),

    #[error("Unable to decrypt data: {0}")]
    UnableToDecrypt(String),

    #[error("Invalid deserialization: {0}")]
    InvalidDeserialization(String),

    #[error("Invalid serialization: {0}")]
    InvalidSerialisation(&'static str),

    #[error("Cannot access header data necessary for operation")]
    MissingHeader,

    #[error("Expected encrypted ratchet key")]
    ExpectEncryptedRatchetKey,

    #[error("Expected bare ratchet key")]
    ExpectBareRatchetKey,

    #[error("Missing node type field")]
    MissingNodeType,

    #[error("Found unexpected node type: {0:?}")]
    UnexpectedNodeType(NodeType),
}

pub fn error<T>(err: impl std::error::Error + Send + Sync + 'static) -> Result<T> {
    Err(err.into())
}
