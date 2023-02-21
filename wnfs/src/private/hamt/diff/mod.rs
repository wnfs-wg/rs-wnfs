mod key_value;
mod node;

pub use key_value::*;
pub use node::*;

use super::{HashNibbles, Node};
use crate::{BlockStore, Hasher, Pair};
use anyhow::Result;
use serde::de::DeserializeOwned;
use std::rc::Rc;

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

//--------------------------------------------------------------------------------------------------
// Functions
//--------------------------------------------------------------------------------------------------

async fn create_node_from_pairs<K, V, H>(
    values: Vec<Pair<K, V>>,
    depth: usize,
    store: &impl BlockStore,
) -> Result<Rc<Node<K, V, H>>>
where
    K: DeserializeOwned + Clone + AsRef<[u8]>,
    V: DeserializeOwned + Clone,
    H: Hasher + Clone + 'static,
{
    let mut node = Rc::new(Node::<_, _, H>::default());
    for Pair { key, value } in values {
        let digest = &H::hash(&key);
        let hashnibbles = &mut HashNibbles::with_cursor(digest, depth);
        node.set_value(hashnibbles, key, value, store).await?;
    }
    Ok(node)
}
