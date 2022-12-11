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
#[derive(Debug, Clone, PartialEq)]
pub struct NodeChange {
    pub r#type: ChangeType,
    pub hashkey: HashKey,
}

//--------------------------------------------------------------------------------------------------
// Functions
//--------------------------------------------------------------------------------------------------

/// Compares two HAMT nodes and returns the changes between them.
///
/// TODO(appcypher): Add docs.
#[async_recursion(?Send)]
pub async fn node_diff<K, V, H, B>(
    main_link: Link<Rc<Node<K, V, H>>>,
    other_link: Link<Rc<Node<K, V, H>>>,
    depth: Option<u8>,
    store: &mut B,
) -> Result<Vec<NodeChange>>
where
    K: DeserializeOwned + Clone + fmt::Debug + Eq + Hash + AsRef<[u8]>,
    V: DeserializeOwned + Clone + fmt::Debug + Eq,
    H: Hasher + Clone + fmt::Debug + 'static,
    B: BlockStore,
{
    node_diff_helper(main_link, other_link, depth, HashKey::default(), store).await
}

#[async_recursion(?Send)]
pub async fn node_diff_helper<K, V, H, B>(
    main_link: Link<Rc<Node<K, V, H>>>,
    other_link: Link<Rc<Node<K, V, H>>>,
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
    // Return if depth is 0.
    if matches!(depth, Some(0)) {
        return Ok(vec![]);
    }

    // If Cids are available, check to see if they are equal so we can skip further comparisons.
    if let (Some(cid), Some(cid2)) = (main_link.get_cid(), other_link.get_cid()) {
        if cid == cid2 {
            return Ok(vec![]);
        }
    }

    // Otherwise, get nodes from store.
    let mut main_node = Rc::try_unwrap(main_link.resolve_owned_value(store).await?)
        .unwrap_or_else(|rc| (*rc).clone());

    let mut other_node = Rc::try_unwrap(other_link.resolve_owned_value(store).await?)
        .unwrap_or_else(|rc| (*rc).clone());

    let mut changes = vec![];
    for index in 0..HAMT_BITMASK_BIT_SIZE {
        // Create hashkey for child.
        let mut hashkey = hashkey.clone();
        hashkey.push(index as u8);

        match (main_node.bitmask[index], other_node.bitmask[index]) {
            (true, false) => {
                // Main has a value, other doesn't.
                changes.extend(generate_add_or_remove_changes(
                    &main_node.pointers[main_node.get_value_index(index)],
                    ChangeType::Add,
                    hashkey,
                ));
            }
            (false, true) => {
                // Main doesn't have a value, other does.
                changes.extend(generate_add_or_remove_changes(
                    &other_node.pointers[other_node.get_value_index(index)],
                    ChangeType::Remove,
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
                    generate_modified_changes(
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

fn generate_add_or_remove_changes<K, V, H>(
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

async fn generate_modified_changes<K, V, H, B>(
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
            node_diff_helper(main_link, other_link, depth, hashkey, store).await
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

            for Pair { key, .. } in &other_values {
                if matches!(main_map.get(key), None) {
                    changes.push(NodeChange {
                        r#type: ChangeType::Remove,
                        hashkey: HashKey::with_length(H::hash(&key), hashkey.len() as u8 + 1),
                    })
                }
            }

            Ok(changes)
        }
        (Pointer::Values(main_values), Pointer::Link(other_link)) => {
            let main_link = Link::from(
                create_node_from_pairs::<_, _, H, _>(main_values, hashkey.len(), store).await?,
            );

            node_diff_helper(main_link, other_link, depth, hashkey, store).await
        }
        (Pointer::Link(main_link), Pointer::Values(other_values)) => {
            let other_link = Link::from(
                create_node_from_pairs::<_, _, H, _>(other_values, hashkey.len(), store).await?,
            );

            node_diff_helper(main_link, other_link, depth, hashkey, store).await
        }
    }
}

async fn create_node_from_pairs<K, V, H, B: BlockStore>(
    values: Vec<Pair<K, V>>,
    hashkey_length: usize,
    store: &B,
) -> Result<Rc<Node<K, V, H>>>
where
    K: DeserializeOwned + Clone + AsRef<[u8]> + fmt::Debug,
    V: DeserializeOwned + Clone + fmt::Debug,
    H: Hasher + Clone + 'static,
{
    let mut node = Rc::new(Node::<_, _, H>::default());
    for Pair { key, value } in values {
        let digest = &H::hash(&key);
        let hashnibbles = &mut HashNibbles::with_cursor(digest, hashkey_length);
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
        private::Node,
        utils::{self, test_setup},
    };
    use helper::*;
    use std::rc::Rc;

    mod helper {
        use crate::{utils, HashOutput, Hasher};
        use lazy_static::lazy_static;

        lazy_static! {
            pub(super) static ref HASH_KV_PAIRS: Vec<(HashOutput, &'static str)> = vec![
                (utils::make_digest(&[0xA0]), "first"),
                (utils::make_digest(&[0xA3]), "second"),
                (utils::make_digest(&[0xA7]), "third"),
                (utils::make_digest(&[0xAC]), "fourth"),
                (utils::make_digest(&[0xAE]), "fifth"),
            ];
        }

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

    /// TODO(appcypher): ASCII drawings.
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
            None,
            store,
        )
        .await
        .unwrap();

        assert_eq!(
            changes,
            vec![
                NodeChange {
                    r#type: Add,
                    hashkey: HashKey::with_length(utils::make_digest(&[0x1F]), 2)
                },
                NodeChange {
                    r#type: Add,
                    hashkey: HashKey::with_length(utils::make_digest(&[0x29]), 2),
                },
            ]
        );

        let changes = node_diff(Link::from(other_node), Link::from(main_node), None, store)
            .await
            .unwrap();

        assert_eq!(
            changes,
            vec![
                NodeChange {
                    r#type: Remove,
                    hashkey: HashKey::with_length(utils::make_digest(&[0x1F]), 2)
                },
                NodeChange {
                    r#type: Remove,
                    hashkey: HashKey::with_length(utils::make_digest(&[0x29]), 2),
                },
            ]
        );
    }

    /// TODO(appcypher): ASCII drawings.
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

        let changes = node_diff(Link::from(main_node), Link::from(other_node), None, store)
            .await
            .unwrap();

        assert!(changes.is_empty());
    }

    /// TODO(appcypher): ASCII drawings.
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
            None,
            store,
        )
        .await
        .unwrap();

        assert_eq!(
            changes,
            vec![
                NodeChange {
                    r#type: Modify,
                    hashkey: HashKey::with_length(utils::make_digest(&[0xA3, 0x00]), 3),
                },
                NodeChange {
                    r#type: Remove,
                    hashkey: HashKey::with_length(utils::make_digest(&[0xA7, 0x00]), 3),
                },
                NodeChange {
                    r#type: Add,
                    hashkey: HashKey::with_length(utils::make_digest(&[0xAC, 0x00]), 3),
                },
                NodeChange {
                    r#type: Add,
                    hashkey: HashKey::with_length(utils::make_digest(&[0xAE, 0x00]), 3),
                },
            ]
        );

        let changes = node_diff(Link::from(other_node), Link::from(main_node), None, store)
            .await
            .unwrap();

        assert_eq!(
            changes,
            vec![
                NodeChange {
                    r#type: Modify,
                    hashkey: HashKey::with_length(utils::make_digest(&[0xA3, 0x00]), 3),
                },
                NodeChange {
                    r#type: Add,
                    hashkey: HashKey::with_length(utils::make_digest(&[0xA7, 0x00]), 3),
                },
                NodeChange {
                    r#type: Remove,
                    hashkey: HashKey::with_length(utils::make_digest(&[0xAC, 0x00]), 3),
                },
                NodeChange {
                    r#type: Remove,
                    hashkey: HashKey::with_length(utils::make_digest(&[0xAE, 0x00]), 3),
                },
            ]
        );
    }
}

#[cfg(test)]
mod proptests {
    use super::*;

    use crate::{
        private::strategies::{self, operations, Operations},
        utils::test_setup,
        Link,
    };
    use async_std::task;
    use sha3::Sha3_256;
    use test_strategy::proptest;

    #[proptest(cases = 100)]
    fn add_hashmap_correspondence(
        #[strategy(operations("[a-z0-9]{1,8}", 0..u64::MAX, 1..100))] mut ops: Operations<
            String,
            u64,
        >,
    ) {
        task::block_on(async {
            let (store, runner) = test_setup::init!(mut store, mut runner);

            // Other node.
            let (k, v) = ops.prepare_insert(("[a-z0-9]{1,8}", 0..u64::MAX), runner);
            let other_node = strategies::node_from_operations(&ops, store).await.unwrap();
            let other_map = other_node.to_hashmap(store).await.unwrap();

            // Main node with an added kv pair.
            let main_node = Rc::clone(&other_node).set(k, v, store).await.unwrap();
            let main_map = main_node.to_hashmap(store).await.unwrap();

            let changes = node_diff(Link::from(main_node), Link::from(other_node), None, store)
                .await
                .unwrap();

            assert_eq!(changes.len(), 1);
            assert_eq!(changes[0].r#type, ChangeType::Add);

            for k in main_map.keys() {
                if other_map.get(k).is_none() {
                    assert_eq!(changes[0].hashkey.digest, Sha3_256::hash(k));
                }
            }
        });
    }

    #[proptest(cases = 100)]
    fn remove_hashmap_correspondence(
        #[strategy(operations("[a-z0-9]{1,8}", 0..u64::MAX, 1..100))] mut ops: Operations<
            String,
            u64,
        >,
    ) {
        task::block_on(async {
            let (store, runner) = test_setup::init!(mut store, mut runner);

            // Other node.
            let (ref k, _) = ops.prepare_remove(("[a-z0-9]{1,8}", 0..u64::MAX), runner);
            let other_node = strategies::node_from_operations(&ops, store).await.unwrap();
            let other_map = other_node.to_hashmap(store).await.unwrap();

            // Main node with a removed kv pair.
            let (main_node, _) = Rc::clone(&other_node).remove(k, store).await.unwrap();
            let main_map = main_node.to_hashmap(store).await.unwrap();

            let changes = node_diff(Link::from(main_node), Link::from(other_node), None, store)
                .await
                .unwrap();

            assert_eq!(changes.len(), 1);
            assert_eq!(changes[0].r#type, ChangeType::Remove);

            for k in other_map.keys() {
                if main_map.get(k).is_none() {
                    assert_eq!(changes[0].hashkey.digest, Sha3_256::hash(k));
                }
            }
        });
    }

    #[proptest(cases = 100)]
    fn modify_hashmap_correspondence(
        #[strategy(operations("[a-z0-9]{1,8}", 0..u64::MAX, 1..100))] mut ops: Operations<
            String,
            u64,
        >,
    ) {
        task::block_on(async {
            let (store, runner) = test_setup::init!(mut store, mut runner);

            // Other node.
            let (k, _, v) = ops.prepare_modify(("[a-z0-9]{1,8}", 0..u64::MAX, 0..u64::MAX), runner);
            let other_node = strategies::node_from_operations(&ops, store).await.unwrap();
            let other_map = other_node.to_hashmap(store).await.unwrap();

            // Main node with a modified kv pair.
            let main_node = Rc::clone(&other_node).set(k, v, store).await.unwrap();
            let main_map = main_node.to_hashmap(store).await.unwrap();

            let changes = node_diff(Link::from(main_node), Link::from(other_node), None, store)
                .await
                .unwrap();

            assert_eq!(changes.len(), 1);
            assert_eq!(changes[0].r#type, ChangeType::Modify);

            for k in other_map.keys() {
                if main_map.get(k).is_none() {
                    assert_eq!(changes[0].hashkey.digest, Sha3_256::hash(k));
                }
            }
        });
    }

    #[proptest]
    fn no_diff() {}

    #[proptest]
    fn add_remove_flip() {}
}
