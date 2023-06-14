use chrono::{DateTime, Utc};

//--------------------------------------------------------------------------------------------------
// Traits
//--------------------------------------------------------------------------------------------------

/// Implements getting a unique identifier for a node.
pub trait Id {
    /// Gets an identifier for the node.
    fn get_id(&self) -> String;
}

pub trait Time {
    /// Gets the current time.
    fn now() -> DateTime<Utc>;
}

//--------------------------------------------------------------------------------------------------
// Implementations
//--------------------------------------------------------------------------------------------------

impl Time for Utc {
    fn now() -> DateTime<Utc> {
        Utc::now()
    }
}
