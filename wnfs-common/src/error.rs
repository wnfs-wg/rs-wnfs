//! Errors

use libipld::Cid;
use thiserror::Error;

//--------------------------------------------------------------------------------------------------
// Type Definitions
//--------------------------------------------------------------------------------------------------

/// Block store errors.
#[derive(Debug, Error)]
pub enum BlockStoreError {
    #[error("Maximum block size exceeded: Encountered block with {0} bytes")]
    MaximumBlockSizeExceeded(usize),

    #[error("Cannot find specified CID in block store: {0}")]
    CIDNotFound(Cid),

    #[error("Cannot find handler for block with CID: {0}")]
    BlockHandlerNotFound(Cid),

    #[error("Lock poisoned")]
    LockPoisoned,
}
