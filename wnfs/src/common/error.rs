//! File system errors.

use anyhow::Result;
use skip_ratchet::PreviousErr;
use thiserror::Error;

use crate::NodeType;

/// Core file system errors.
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

    #[error("Cannot compute in-between ratchet {0}")]
    NoIntermediateRatchet(PreviousErr),

    #[error("Cannot find shard for file content")]
    FileShardNotFound,

    #[error("The hashprefix index is out of bounds")]
    InvalidHashPrefixIndex,
}

/// Block store errors.
#[derive(Debug, Error)]
pub enum BlockStoreError {
    #[error("Maximum block size exceeded: Encountered block with {0} bytes")]
    MaximumBlockSizeExceeded(usize),
}

/// HAMT errors.
#[derive(Debug, Error)]
pub enum HamtError {
    #[error("Key does not exist in HAMT")]
    KeyNotFound,
}

/// AES-GCM errors.
#[derive(Debug, Error)]
pub enum AesError {
    #[error("Unable to encrypt data: {0}")]
    UnableToEncrypt(String),

    #[error("Unable to decrypt data: {0}")]
    UnableToDecrypt(String),
}

/// RSA related errors
#[derive(Debug, Error)]
pub enum RsaError {
    #[error("Invalid public key: {0}")]
    InvalidPublicKey(anyhow::Error),

    #[error("Invalid private key: {0}")]
    InvalidPrivateKey(anyhow::Error),

    #[error("Encryption failed: {0}")]
    EncryptionFailed(anyhow::Error),

    #[error("Decryption failed: {0}")]
    DecryptionFailed(anyhow::Error),

    #[error("No private key found for public key")]
    NoPrivateKey,
}

/// Data sharing related errors
#[derive(Debug, Error)]
pub enum ShareError {
    #[error("Unsupported exchange key type: {0}")]
    UnsupportedExchangeKey(String),

    #[error("No sharer or recipients")]
    NoSharerOrRecipients,

    #[error("Share payload not found")]
    SharePayloadNotFound,

    #[error("Unsupported share receipt type")]
    UnsupportedSnapshotShareReceipt,
}

pub fn error<T>(err: impl std::error::Error + Send + Sync + 'static) -> Result<T> {
    Err(err.into())
}
