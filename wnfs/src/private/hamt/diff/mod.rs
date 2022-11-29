mod key_value;
mod node;

pub use key_value::*;
pub use node::*;

//--------------------------------------------------------------------------------------------------
// Type Definitions
//--------------------------------------------------------------------------------------------------

/// TODO(appcypher): Add docs.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ChangeType {
    Add,
    Remove,
    Modify,
}
