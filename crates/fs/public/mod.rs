mod directory;
mod file;
mod link;
mod node;

pub use directory::*;
pub use file::*;
use link::*;
pub use node::*;

/// Implements getting a unique identifier for a node.
pub trait Id {
    /// Returns a unique identifier for the node.
    fn get_id(&self) -> String;
}
