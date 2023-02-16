mod key_value;
mod node;

use futures::Future;
pub use key_value::*;
pub use node::*;

use super::{HashNibbles, Node, HAMT_BITMASK_BIT_SIZE};
use crate::{utils::UnwrapOrClone, BlockStore, Hasher, Link, Pair};
use anyhow::Result;
use serde::de::DeserializeOwned;
use std::{hash::Hash, rc::Rc};

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

pub async fn diff_helper<K, V, H, A, Fut>(
    main_link: Link<Rc<Node<K, V, H>>>,
    other_link: Link<Rc<Node<K, V, H>>>,
    store: &mut impl BlockStore,
    f: impl Fn(usize, &mut Node<K, V, H>, &mut Node<K, V, H>) -> Fut,
) -> Result<Vec<A>>
where
    K: DeserializeOwned + Clone + Eq + Hash + AsRef<[u8]>,
    V: DeserializeOwned + Clone + Eq,
    H: Hasher + Clone + 'static,
    Fut: Future<Output = Result<Vec<A>>>,
{
    // If Cids are available, check to see if they are equal so we can skip further comparisons.
    if let (Some(cid), Some(cid2)) = (main_link.get_cid(), other_link.get_cid()) {
        if cid == cid2 {
            return Ok(vec![]);
        }
    }

    // Otherwise, get nodes from store.
    let mut main_node = main_link
        .resolve_owned_value(store)
        .await?
        .unwrap_or_clone()?;

    let mut other_node = other_link
        .resolve_owned_value(store)
        .await?
        .unwrap_or_clone()?;

    let mut changes = vec![];
    for index in 0..HAMT_BITMASK_BIT_SIZE {
        changes.extend(f(index, &mut main_node, &mut other_node).await?);
    }

    Ok(changes)
}

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
        node = node.set_value(hashnibbles, key, value, store).await?;
    }
    Ok(node)
}
