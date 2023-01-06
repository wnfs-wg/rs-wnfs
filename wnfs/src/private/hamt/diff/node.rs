use super::ChangeType;
use crate::{
    private::{HashNibbles, HashPrefix, Node, Pointer, HAMT_BITMASK_BIT_SIZE},
    utils::UnwrapOrClone,
    BlockStore, Hasher, Link, Pair,
};
use anyhow::Result;
use async_recursion::async_recursion;
use serde::de::DeserializeOwned;
use std::{collections::HashMap, hash::Hash, mem, rc::Rc};

//--------------------------------------------------------------------------------------------------
// Type Definitions
//--------------------------------------------------------------------------------------------------

/// Represents a change to some node or key-value pair of a HAMT.
#[derive(Debug, Clone, PartialEq)]
pub struct NodeChange {
    pub r#type: ChangeType,
    pub hashprefix: HashPrefix,
}

//--------------------------------------------------------------------------------------------------
// Functions
//--------------------------------------------------------------------------------------------------

/// Compare two nodes and get differences in keys relative to the main node.
///
/// This implementation returns early once it detects differences between intermediate nodes of the
/// HAMT. In such cases, it returns the hash prefix for the set of keys that were added, removed
/// or modified.
///
/// When a node has been added or removed, this implementation does not visit the children, instead
/// it returns the hashprefix representing the node. This leads a more efficient implementation that does
/// not contain keys and values and stops at node level if the node itself has been added or removed.
///
/// # Examples
///
/// ```
/// use std::rc::Rc;
/// use wnfs::{private::{Node, diff}, Link, Pair, MemoryBlockStore};
///
/// #[async_std::main]
/// async fn main() {
///     let store = &mut MemoryBlockStore::new();
///     let mut main_node = Rc::new(Node::<[u8; 4], String>::default());
///     for i in 0u32..3 {
///         main_node = main_node
///             .set(i.to_le_bytes(), i.to_string(), store)
///             .await
///             .unwrap();
///     }
///
///     let mut other_node = Rc::new(Node::<[u8; 4], String>::default());
///     other_node = other_node
///         .set(0_u32.to_le_bytes(), 0_u32.to_string(), store)
///         .await
///         .unwrap();
///
///     let changes = diff::node_diff(
///         Link::from(Rc::clone(&main_node)),
///         Link::from(Rc::clone(&other_node)),
///         store,
///     )
///     .await
///     .unwrap();
///
///
///    println!("Changes {:#?}", changes);
/// }
/// ```
#[async_recursion(?Send)]
pub async fn node_diff<K, V, H, B>(
    main_link: Link<Rc<Node<K, V, H>>>,
    other_link: Link<Rc<Node<K, V, H>>>,
    store: &mut B,
) -> Result<Vec<NodeChange>>
where
    K: DeserializeOwned + Clone + Eq + Hash + AsRef<[u8]>,
    V: DeserializeOwned + Clone + Eq,
    H: Hasher + Clone + 'static,
    B: BlockStore,
{
    node_diff_helper(main_link, other_link, HashPrefix::default(), store).await
}

#[async_recursion(?Send)]
pub async fn node_diff_helper<K, V, H, B>(
    main_link: Link<Rc<Node<K, V, H>>>,
    other_link: Link<Rc<Node<K, V, H>>>,
    hashprefix: HashPrefix,
    store: &mut B,
) -> Result<Vec<NodeChange>>
where
    K: DeserializeOwned + Clone + Eq + Hash + AsRef<[u8]>,
    V: DeserializeOwned + Clone + Eq,
    H: Hasher + Clone + 'static,
    B: BlockStore,
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
        // Create hashprefix for child.
        let mut hashprefix = hashprefix.clone();
        hashprefix.push(index as u8);

        match (main_node.bitmask[index], other_node.bitmask[index]) {
            (true, false) => {
                // Main has a value, other doesn't.
                changes.extend(generate_add_or_remove_changes(
                    &main_node.pointers[main_node.get_value_index(index)],
                    ChangeType::Add,
                    hashprefix,
                ));
            }
            (false, true) => {
                // Main doesn't have a value, other does.
                changes.extend(generate_add_or_remove_changes(
                    &other_node.pointers[other_node.get_value_index(index)],
                    ChangeType::Remove,
                    hashprefix,
                ));
            }
            (true, true) => {
                // Main and other have a value. They may be the same or different so we check.
                let main_index = main_node.get_value_index(index);
                let main_pointer = mem::take(main_node.pointers.get_mut(main_index).unwrap());

                let other_index = other_node.get_value_index(index);
                let other_pointer = mem::take(other_node.pointers.get_mut(other_index).unwrap());

                changes.extend(
                    generate_modify_changes(main_pointer, other_pointer, hashprefix, store).await?,
                );
            }
            (false, false) => { /*No change */ }
        }
    }

    Ok(changes)
}

fn generate_add_or_remove_changes<K, V, H>(
    node_pointer: &Pointer<K, V, H>,
    r#type: ChangeType,
    hashprefix: HashPrefix,
) -> Vec<NodeChange>
where
    K: AsRef<[u8]>,
    H: Hasher + Clone,
{
    match node_pointer {
        Pointer::Values(values) => values
            .iter()
            .map(|Pair { key, .. }| {
                let digest = H::hash(&key);
                let hashprefix = HashPrefix::with_length(digest, digest.len() as u8 * 2);
                NodeChange { r#type, hashprefix }
            })
            .collect(),
        Pointer::Link(_) => {
            vec![NodeChange { r#type, hashprefix }]
        }
    }
}

async fn generate_modify_changes<K, V, H, B>(
    main_pointer: Pointer<K, V, H>,
    other_pointer: Pointer<K, V, H>,
    hashprefix: HashPrefix,
    store: &mut B,
) -> Result<Vec<NodeChange>>
where
    K: DeserializeOwned + Clone + Eq + Hash + AsRef<[u8]>,
    V: DeserializeOwned + Clone + Eq,
    H: Hasher + Clone + 'static,
    B: BlockStore,
{
    match (main_pointer, other_pointer) {
        (Pointer::Link(main_link), Pointer::Link(other_link)) => {
            node_diff_helper(main_link, other_link, hashprefix, store).await
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
                            let digest = H::hash(&key);
                            let hashprefix =
                                HashPrefix::with_length(digest, digest.len() as u8 * 2);
                            changes.push(NodeChange {
                                r#type: ChangeType::Modify,
                                hashprefix,
                            });
                        }
                    }
                    None => {
                        let digest = H::hash(&key);
                        let hashprefix = HashPrefix::with_length(digest, digest.len() as u8 * 2);
                        changes.push(NodeChange {
                            r#type: ChangeType::Add,
                            hashprefix,
                        })
                    }
                }

                main_map.insert(key, value);
            }

            for Pair { key, .. } in &other_values {
                if matches!(main_map.get(key), None) {
                    let digest = H::hash(&key);
                    let hashprefix = HashPrefix::with_length(digest, digest.len() as u8 * 2);
                    changes.push(NodeChange {
                        r#type: ChangeType::Remove,
                        hashprefix,
                    })
                }
            }

            Ok(changes)
        }
        (Pointer::Values(main_values), Pointer::Link(other_link)) => {
            let main_link = Link::from(
                create_node_from_pairs::<_, _, H, _>(main_values, hashprefix.len(), store).await?,
            );

            node_diff_helper(main_link, other_link, hashprefix, store).await
        }
        (Pointer::Link(main_link), Pointer::Values(other_values)) => {
            let other_link = Link::from(
                create_node_from_pairs::<_, _, H, _>(other_values, hashprefix.len(), store).await?,
            );

            node_diff_helper(main_link, other_link, hashprefix, store).await
        }
    }
}

async fn create_node_from_pairs<K, V, H, B: BlockStore>(
    values: Vec<Pair<K, V>>,
    hashprefix_length: usize,
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
        let hashnibbles = &mut HashNibbles::with_cursor(digest, hashprefix_length);
        node = node.set_value(hashnibbles, key, value, store).await?;
    }
    Ok(node)
}

//--------------------------------------------------------------------------------------------------
// Tests
//--------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::{ChangeType::*, *};
    use crate::{
        private::{Node, MAX_HASH_NIBBLE_LENGTH},
        utils::{self, test_setup},
    };
    use helper::*;
    use sha3::Sha3_256;
    use std::rc::Rc;

    mod helper {
        use crate::{utils, HashOutput, Hasher};
        use once_cell::sync::Lazy;

        pub(super) static HASH_KV_PAIRS: Lazy<Vec<(HashOutput, &'static str)>> = Lazy::new(|| {
            vec![
                (utils::make_digest(&[0xA0]), "first"),
                (utils::make_digest(&[0xA3]), "second"),
                (utils::make_digest(&[0xA7]), "third"),
                (utils::make_digest(&[0xAC]), "fourth"),
                (utils::make_digest(&[0xAE]), "fifth"),
            ]
        });

        #[derive(Debug, Clone)]
        pub(super) struct MockHasher;
        impl Hasher for MockHasher {
            fn hash<K: AsRef<[u8]>>(key: &K) -> HashOutput {
                HASH_KV_PAIRS
                    .iter()
                    .find(|(_, v)| key.as_ref() == <dyn AsRef<[u8]>>::as_ref(v))
                    .unwrap()
                    .0
            }
        }
    }

    #[async_std::test]
    async fn can_diff_main_node_with_added_removed_pairs() {
        let store = test_setup::init!(mut store);

        let mut main_node = Rc::new(Node::<[u8; 4], String>::default());
        for i in 0u32..3 {
            main_node = main_node
                .set(i.to_le_bytes(), i.to_string(), store)
                .await
                .unwrap();
        }

        let mut other_node = Rc::new(Node::<[u8; 4], String>::default());
        other_node = other_node
            .set(0_u32.to_le_bytes(), 0_u32.to_string(), store)
            .await
            .unwrap();

        let changes = node_diff(
            Link::from(Rc::clone(&main_node)),
            Link::from(Rc::clone(&other_node)),
            store,
        )
        .await
        .unwrap();

        let hashprefix_of_1 = HashPrefix::with_length(
            Sha3_256::hash(&1_u32.to_le_bytes()),
            MAX_HASH_NIBBLE_LENGTH as u8,
        );

        let hashprefix_of_2 = HashPrefix::with_length(
            Sha3_256::hash(&2_u32.to_le_bytes()),
            MAX_HASH_NIBBLE_LENGTH as u8,
        );

        assert_eq!(
            changes,
            vec![
                NodeChange {
                    r#type: Add,
                    hashprefix: hashprefix_of_2.clone()
                },
                NodeChange {
                    r#type: Add,
                    hashprefix: hashprefix_of_1.clone(),
                },
            ]
        );

        let changes = node_diff(Link::from(other_node), Link::from(main_node), store)
            .await
            .unwrap();

        assert_eq!(
            changes,
            vec![
                NodeChange {
                    r#type: Remove,
                    hashprefix: hashprefix_of_2
                },
                NodeChange {
                    r#type: Remove,
                    hashprefix: hashprefix_of_1,
                },
            ]
        );
    }

    #[async_std::test]
    async fn can_diff_main_node_with_no_changes() {
        let store = test_setup::init!(mut store);

        let mut main_node = Rc::new(Node::<_, _>::default());
        for i in 0_u32..3 {
            main_node = main_node
                .set(i.to_le_bytes(), i.to_string(), store)
                .await
                .unwrap();
        }

        let mut other_node = Rc::new(Node::<_, _>::default());
        for i in 0_u32..3 {
            other_node = other_node
                .set(i.to_le_bytes(), i.to_string(), store)
                .await
                .unwrap();
        }

        let changes = node_diff(Link::from(main_node), Link::from(other_node), store)
            .await
            .unwrap();

        assert!(changes.is_empty());
    }

    #[async_std::test]
    async fn can_diff_nodes_with_different_structure_and_modified_changes() {
        let store = test_setup::init!(mut store);

        // A node that adds the first 3 pairs of HASH_KV_PAIRS.
        let mut other_node = Rc::new(Node::<_, _, MockHasher>::default());
        for (digest, kv) in HASH_KV_PAIRS.iter().take(3) {
            other_node = other_node
                .set_value(
                    &mut HashNibbles::new(digest),
                    kv.to_string(),
                    kv.to_string(),
                    store,
                )
                .await
                .unwrap();
        }

        // Another node that keeps the first pair, modify the second pair, removes the third pair, and adds the fourth and fifth pair.
        let mut main_node = Rc::new(Node::<_, _, MockHasher>::default());
        main_node = main_node
            .set_value(
                &mut HashNibbles::new(&HASH_KV_PAIRS[0].0),
                HASH_KV_PAIRS[0].1.to_string(),
                HASH_KV_PAIRS[0].1.to_string(),
                store,
            )
            .await
            .unwrap();

        main_node = main_node
            .set_value(
                &mut HashNibbles::new(&HASH_KV_PAIRS[1].0),
                HASH_KV_PAIRS[1].1.to_string(),
                String::from("second_modified"),
                store,
            )
            .await
            .unwrap();

        for (digest, kv) in HASH_KV_PAIRS.iter().skip(3).take(2) {
            main_node = main_node
                .set_value(
                    &mut HashNibbles::new(digest),
                    kv.to_string(),
                    kv.to_string(),
                    store,
                )
                .await
                .unwrap();
        }

        let changes = node_diff(
            Link::from(Rc::clone(&main_node)),
            Link::from(Rc::clone(&other_node)),
            store,
        )
        .await
        .unwrap();

        assert_eq!(
            changes,
            vec![
                NodeChange {
                    r#type: Modify,
                    hashprefix: HashPrefix::with_length(
                        utils::make_digest(&[0xA3, 0x00]),
                        MAX_HASH_NIBBLE_LENGTH as u8
                    ),
                },
                NodeChange {
                    r#type: Remove,
                    hashprefix: HashPrefix::with_length(
                        utils::make_digest(&[0xA7, 0x00]),
                        MAX_HASH_NIBBLE_LENGTH as u8
                    ),
                },
                NodeChange {
                    r#type: Add,
                    hashprefix: HashPrefix::with_length(
                        utils::make_digest(&[0xAC, 0x00]),
                        MAX_HASH_NIBBLE_LENGTH as u8
                    ),
                },
                NodeChange {
                    r#type: Add,
                    hashprefix: HashPrefix::with_length(
                        utils::make_digest(&[0xAE, 0x00]),
                        MAX_HASH_NIBBLE_LENGTH as u8
                    ),
                },
            ]
        );

        let changes = node_diff(Link::from(other_node), Link::from(main_node), store)
            .await
            .unwrap();

        assert_eq!(
            changes,
            vec![
                NodeChange {
                    r#type: Modify,
                    hashprefix: HashPrefix::with_length(
                        utils::make_digest(&[0xA3, 0x00]),
                        MAX_HASH_NIBBLE_LENGTH as u8
                    ),
                },
                NodeChange {
                    r#type: Add,
                    hashprefix: HashPrefix::with_length(
                        utils::make_digest(&[0xA7, 0x00]),
                        MAX_HASH_NIBBLE_LENGTH as u8
                    ),
                },
                NodeChange {
                    r#type: Remove,
                    hashprefix: HashPrefix::with_length(
                        utils::make_digest(&[0xAC, 0x00]),
                        MAX_HASH_NIBBLE_LENGTH as u8
                    ),
                },
                NodeChange {
                    r#type: Remove,
                    hashprefix: HashPrefix::with_length(
                        utils::make_digest(&[0xAE, 0x00]),
                        MAX_HASH_NIBBLE_LENGTH as u8
                    ),
                },
            ]
        );
    }
}

#[cfg(test)]
mod proptests {
    use super::*;
    use crate::{
        private::strategies::{self, generate_kvs},
        utils::test_setup,
    };
    use async_std::task;
    use test_strategy::proptest;

    #[proptest]
    fn add_remove_flip(
        #[strategy(generate_kvs("[a-z0-9]{1,3}", 0u64..1000, 0..100))] kvs1: Vec<(String, u64)>,
        #[strategy(generate_kvs("[a-z0-9]{1,3}", 0u64..1000, 0..100))] kvs2: Vec<(String, u64)>,
    ) {
        task::block_on(async {
            let store = test_setup::init!(mut store);

            let node1 = strategies::node_from_kvs(kvs1, store).await.unwrap();
            let node2 = strategies::node_from_kvs(kvs2, store).await.unwrap();

            let changes = node_diff(
                Link::from(Rc::clone(&node1)),
                Link::from(Rc::clone(&node2)),
                store,
            )
            .await
            .unwrap();

            let flipped_changes = node_diff(Link::from(node2), Link::from(node1), store)
                .await
                .unwrap();

            assert_eq!(changes.len(), flipped_changes.len());
            for change in changes {
                assert!(flipped_changes.iter().any(|c| match change.r#type {
                    ChangeType::Add =>
                        c.r#type == ChangeType::Remove && c.hashprefix == change.hashprefix,
                    ChangeType::Remove =>
                        c.r#type == ChangeType::Add && c.hashprefix == change.hashprefix,
                    ChangeType::Modify =>
                        c.r#type == ChangeType::Modify && c.hashprefix == change.hashprefix,
                }));
            }
        });
    }
}
