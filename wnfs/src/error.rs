//! Errors

use semver::Version;
use skip_ratchet::PreviousErr;
use thiserror::Error;
use wnfs_common::NodeType;

//--------------------------------------------------------------------------------------------------
// Type Definitions
//--------------------------------------------------------------------------------------------------

/// Core file system errors.
#[derive(Debug, Error)]
pub enum FsError {
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

    #[error("Invalid deserialization: {0}")]
    InvalidDeserialization(String),

    #[error("Missing node type field")]
    MissingNodeType,

    #[error("Found unexpected node type, expected {0:?}")]
    UnexpectedNodeType(NodeType),

    #[error("Found unexpected version: {0:?}")]
    UnexpectedVersion(Version),

    #[error("Cannot compute in-between ratchet {0}")]
    NoIntermediateRatchet(PreviousErr),

    #[error("Cannot find shard for file content")]
    FileShardNotFound,

    #[error("Cannot merge or compare forests, incompatible accumulator setups")]
    IncompatibleAccumulatorSetups,

    #[error("Mismatch between PrivateNode name {0} and its mountpoint {0}")]
    MountPointAndDeserializedNameMismatch(String, String),

    #[error("Cannot find private ref with specified root path")]
    PrivateRefNotFound,
}

/// Data sharing related errors
#[derive(Debug, Error)]
pub enum ShareError {
    #[error("No sharer or recipients")]
    NoSharerOrRecipients,

    #[error("Share payload not found")]
    SharePayloadNotFound,

    #[error("Unsupported share receipt type")]
    UnsupportedSnapshotShareReceipt,
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
#[cfg(test)]
#[derive(Debug, Error)]
pub enum RsaError {
    #[error("Invalid public key: {0}")]
    InvalidPublicKey(anyhow::Error),

    #[error("Encryption failed: {0}")]
    EncryptionFailed(anyhow::Error),

    #[error("Decryption failed: {0}")]
    DecryptionFailed(anyhow::Error),
}

#[derive(Debug, Error)]
pub enum VerificationError {
    #[error("Couldn't verify write for label {0}")]
    UnverifiedWrite(String),

    #[error("Write to disallowed base {0}")]
    WriteToDisallowedBase(String),
}
