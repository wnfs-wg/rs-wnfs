mod directory;
mod file;
mod link;
mod node;

pub use directory::*;
pub use file::*;
pub use link::*;
pub use node::*;

/// Implements getting a unique identifier for a node.
pub trait Id {
    fn get_id(&self) -> String;
}
