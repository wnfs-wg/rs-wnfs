mod key_value;
mod node;

pub use key_value::*;
pub use node::*;

//--------------------------------------------------------------------------------------------------
// Type Definitions
//--------------------------------------------------------------------------------------------------

/// This type represents the different kinds of changes to a node.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ChangeType {
    Add,
    Remove,
    Modify,
}
