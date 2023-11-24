//! Errors

use thiserror::Error;

//--------------------------------------------------------------------------------------------------
// Type Definitions
//--------------------------------------------------------------------------------------------------

#[derive(Debug, PartialEq, Eq, Error)]
pub enum VerificationError {
    #[error("Hash-to-prime didn't end up prime")]
    LHashNonPrime,

    #[error("Residue outside range")]
    ResidueOutsideRange,

    #[error("NameAccumulator batched proof validation failed")]
    ValidationFailed,

    #[error("Couldn't invert base accumulator state")]
    NoInverse,
}
