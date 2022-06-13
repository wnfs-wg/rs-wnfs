//! Private node link.

use std::rc::Rc;

use crate::Link;

use super::{PrivateDirectory, PrivateFile, PrivateNode};

//--------------------------------------------------------------------------------------------------
// Type Definitions
//--------------------------------------------------------------------------------------------------

pub type PrivateLink = Link<PrivateNode>;

//--------------------------------------------------------------------------------------------------
// Implementations
//--------------------------------------------------------------------------------------------------

impl PrivateLink {
    /// Create a new link to a node.
    pub fn new(node: PrivateNode) -> Self {
        Link::from(node)
    }

    /// Creates a new directory node link.
    pub fn with_dir(dir: Rc<PrivateDirectory>) -> Self {
        Link::from(PrivateNode::Dir(dir))
    }

    /// Creates a new file node link.
    pub fn with_file(file: Rc<PrivateFile>) -> Self {
        Link::from(PrivateNode::File(file))
    }
}
