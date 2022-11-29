use super::ChangeType;
use crate::{
    private::{HashKey, HashNibbles, Node, Pointer, HAMT_BITMASK_BIT_SIZE},
    BlockStore, Hasher, Link, Pair,
};
use anyhow::Result;
use async_recursion::async_recursion;
use hashbrown::HashMap;
use serde::de::DeserializeOwned;
use std::{fmt, hash::Hash, mem, rc::Rc};

//--------------------------------------------------------------------------------------------------
// Type Definitions
//--------------------------------------------------------------------------------------------------

/// TODO(appcypher): Add docs.
#[derive(Debug, Clone)]
pub struct NodeChange {
    pub r#type: ChangeType,
    pub hashkey: HashKey,
}

//--------------------------------------------------------------------------------------------------
// Functions
//--------------------------------------------------------------------------------------------------

/// TODO(appcypher): Add docs.
/// TODO(appcypher): Maybe add a different entry point to this so we don't have to pass the hashkey.
#[async_recursion(?Send)]
pub async fn node_diff<K, V, H, B>(
    root_main: Link<Rc<Node<K, V, H>>>,
    root_other: Link<Rc<Node<K, V, H>>>,
    depth: Option<u8>,
    hashkey: HashKey,
    store: &mut B,
) -> Result<Vec<NodeChange>>
where
    K: DeserializeOwned + Clone + fmt::Debug + Eq + Hash + AsRef<[u8]>,
    V: DeserializeOwned + Clone + fmt::Debug + Eq,
    H: Hasher + Clone + fmt::Debug + 'static,
    B: BlockStore,
{
    // TODO(appcypher): Remember to decrement depth before recursive calls.
    // Return if depth is 0.
    if matches!(depth, Some(0)) {
        return Ok(vec![]);
    }

    // If Cids are available, check to see if they are equal so we can skip further comparisons.
    if let (Some(cid), Some(cid2)) = (root_main.get_cid(), root_other.get_cid()) {
        if cid == cid2 {
            return Ok(vec![]);
        }
    }

    // Otherwise, get nodes from store.
    let mut main_node = Rc::try_unwrap(root_main.resolve_owned_value(store).await?)
        .unwrap_or_else(|rc| (*rc).clone());

    let mut other_node = Rc::try_unwrap(root_other.resolve_owned_value(store).await?)
        .unwrap_or_else(|rc| (*rc).clone());

    let mut changes = vec![];
    for index in 0..HAMT_BITMASK_BIT_SIZE {
        // Create hashkey for child.
        let mut hashkey = hashkey.clone();
        hashkey.push(index as u8);

        match (main_node.bitmask[index], other_node.bitmask[index]) {
            (true, false) => {
                // Main has a value, other doesn't.
                changes.extend(get_add_or_remove_changes(
                    &main_node.pointers[main_node.get_value_index(index)],
                    ChangeType::Add,
                    hashkey,
                ));
            }
            (false, true) => {
                // Main doesn't have a value, other does.
                changes.extend(get_add_or_remove_changes(
                    &other_node.pointers[other_node.get_value_index(index)],
                    ChangeType::Add,
                    hashkey,
                ));
            }
            (true, true) => {
                // Main and other have a value. They may be the same or different so we check.
                let main_index = main_node.get_value_index(index);
                let main_pointer = mem::take(main_node.pointers.get_mut(main_index).unwrap());

                let other_index = other_node.get_value_index(index);
                let other_pointer = mem::take(other_node.pointers.get_mut(other_index).unwrap());

                changes.extend(
                    get_modified_changes(
                        main_pointer,
                        other_pointer,
                        hashkey,
                        depth.map(|v| v - 1),
                        store,
                    )
                    .await?,
                );
            }
            (false, false) => { /*No change */ }
        }
    }

    Ok(changes)
}

fn get_add_or_remove_changes<K, V, H>(
    node_pointer: &Pointer<K, V, H>,
    r#type: ChangeType,
    hashkey: HashKey,
) -> Vec<NodeChange>
where
    K: AsRef<[u8]>,
    H: Hasher + Clone,
{
    match node_pointer {
        Pointer::Values(values) => values
            .iter()
            .map(|Pair { key, .. }| NodeChange {
                r#type,
                hashkey: HashKey::with_length(H::hash(&key), hashkey.len() as u8 + 1),
            })
            .collect(),
        Pointer::Link(_) => {
            vec![NodeChange { r#type, hashkey }]
        }
    }
}

async fn get_modified_changes<K, V, H, B>(
    main_pointer: Pointer<K, V, H>,
    other_pointer: Pointer<K, V, H>,
    hashkey: HashKey,
    depth: Option<u8>,
    store: &mut B,
) -> Result<Vec<NodeChange>>
where
    K: DeserializeOwned + Clone + fmt::Debug + Eq + Hash + AsRef<[u8]>,
    V: DeserializeOwned + Clone + fmt::Debug + Eq,
    H: Hasher + Clone + fmt::Debug + 'static,
    B: BlockStore,
{
    match (main_pointer, other_pointer) {
        (Pointer::Link(main_link), Pointer::Link(other_link)) => {
            node_diff(main_link, other_link, depth, hashkey, store).await
        }
        (Pointer::Values(main_values), Pointer::Values(other_values)) => {
            let mut changes = vec![];
            let mut main_map = HashMap::<&K, &V>::default();
            let other_map = HashMap::<&K, &V>::from_iter(
                other_values.iter().map(|Pair { key, value }| (key, value)),
            );

            for Pair { key, value } in &main_values {
                match other_map.get(&key) {
                    Some(v) => {
                        if *v != value {
                            changes.push(NodeChange {
                                r#type: ChangeType::Modify,
                                hashkey: HashKey::with_length(
                                    H::hash(&key),
                                    hashkey.len() as u8 + 1,
                                ),
                            });
                        }
                    }
                    None => changes.push(NodeChange {
                        r#type: ChangeType::Add,
                        hashkey: HashKey::with_length(H::hash(&key), hashkey.len() as u8 + 1),
                    }),
                }

                main_map.insert(key, value);
            }

            for Pair { key, value } in &other_values {
                match main_map.get(key) {
                    Some(v) => {
                        if *v != value {
                            changes.push(NodeChange {
                                r#type: ChangeType::Modify,
                                hashkey: HashKey::with_length(
                                    H::hash(&key),
                                    hashkey.len() as u8 + 1,
                                ),
                            });
                        }
                    }
                    None => changes.push(NodeChange {
                        r#type: ChangeType::Remove,
                        hashkey: HashKey::with_length(H::hash(&key), hashkey.len() as u8 + 1),
                    }),
                }
            }

            Ok(changes)
        }
        (Pointer::Values(main_values), Pointer::Link(other_link)) => {
            let main_link = Link::from(
                create_node_from_pairs::<_, _, H, _>(main_values, hashkey.len(), store).await?,
            );

            node_diff(main_link, other_link, depth, hashkey, store).await
        }
        (Pointer::Link(main_link), Pointer::Values(other_values)) => {
            let other_link = Link::from(
                create_node_from_pairs::<_, _, H, _>(other_values, hashkey.len(), store).await?,
            );

            node_diff(main_link, other_link, depth, hashkey, store).await
        }
    }
}

async fn create_node_from_pairs<K, V, H, B: BlockStore>(
    values: Vec<Pair<K, V>>,
    hashkey_length: usize,
    store: &B,
) -> Result<Rc<Node<K, V, H>>>
where
    K: DeserializeOwned + Clone + AsRef<[u8]>,
    V: DeserializeOwned + Clone,
    H: Hasher + Clone + 'static,
{
    let mut node = Rc::new(Node::<_, _, H>::default());
    for Pair { key, value } in values {
        let digest = &H::hash(&key);
        let hashnibbles = &mut HashNibbles::with_cursor(digest, hashkey_length + 1);
        node = node.set_value(hashnibbles, key, value, store).await?;
    }
    Ok(node)
}

//--------------------------------------------------------------------------------------------------
// Tests
//--------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{private::Node, utils::test_setup};
    use std::rc::Rc;

    /// TODO(appcypher): ASCII drawings.
    #[async_std::test]
    async fn can_diff_main_node_with_added_pairs() {
        let store = test_setup::init!(mut store);

        let mut node1 = Rc::new(Node::<[u8; 4], String>::default());
        for i in 0_u32..3 {
            node1 = node1
                .set(i.to_le_bytes(), i.to_string(), store)
                .await
                .unwrap();
        }

        let node2 = Node::<[u8; 4], String>::default();

        let changes = node_diff(
            Link::from(node1),
            Link::from(Rc::new(node2)),
            None,
            HashKey::default(),
            store,
        )
        .await
        .unwrap();

        println!("{:#?}", changes);
    }

    /// TODO(appcypher): ASCII drawings.
    #[async_std::test]
    async fn can_diff_main_node_with_removed_pairs() {
        let store = test_setup::init!(mut store);

        let node1 = Node::<[u8; 4], String>::default();

        let mut node2 = Rc::new(Node::<[u8; 4], String>::default());
        for i in 0_u32..3 {
            node2 = node2
                .set(i.to_le_bytes(), i.to_string(), store)
                .await
                .unwrap();
        }

        let changes = node_diff(
            Link::from(Rc::new(node1)),
            Link::from(node2),
            None,
            HashKey::default(),
            store,
        )
        .await
        .unwrap();

        println!("{:#?}", changes);
    }

    /// TODO(appcypher): ASCII drawings.
    #[async_std::test]
    async fn can_diff_main_node_with_no_changes() {
        let store = test_setup::init!(mut store);

        let mut node1 = Rc::new(Node::<[u8; 4], String>::default());
        for i in 0_u32..3 {
            node1 = node1
                .set(i.to_le_bytes(), i.to_string(), store)
                .await
                .unwrap();
        }

        let mut node2 = Rc::new(Node::<[u8; 4], String>::default());
        for i in 0_u32..3 {
            node2 = node2
                .set(i.to_le_bytes(), i.to_string(), store)
                .await
                .unwrap();
        }

        let changes = node_diff(
            Link::from(node1),
            Link::from(node2),
            None,
            HashKey::default(),
            store,
        )
        .await
        .unwrap();

        println!("{:#?}", changes);
    }

    // - A (Pointer::Values with 3 values) -> B (Pointer::Link with 4 eventual values)
}
