use super::ChangeType;
use crate::{private::Node, BlockStore, Hasher, Link, Pair};
use anyhow::{Ok, Result};
use either::Either::{self, *};
use serde::de::DeserializeOwned;
use std::{hash::Hash, rc::Rc};

//--------------------------------------------------------------------------------------------------
// Type Definitions
//--------------------------------------------------------------------------------------------------

/// Represents a change to some key-value pair of a HAMT node.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyValueChange<K, V> {
    pub r#type: ChangeType,
    pub key: K,
    pub main_value: Option<V>,
    pub other_value: Option<V>,
}

type EitherPairOrNode<'a, K, V, H> = Option<Either<&'a Pair<K, V>, &'a Rc<Node<K, V, H>>>>;

//--------------------------------------------------------------------------------------------------
// Functions
//--------------------------------------------------------------------------------------------------

/// Compare two nodes and get the key-value changes made to the main node.
///
/// This implementation gets all the changes to main node at the leaf node level.
///
/// This is a more expensive operation because it gathers the key value pairs under a node has
/// been added or removed even though we can simply return a reference to the node itself.
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
///     let changes = diff::kv_diff(
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
pub async fn kv_diff<K, V, H, B>(
    main_link: Link<Rc<Node<K, V, H>>>,
    other_link: Link<Rc<Node<K, V, H>>>,
    store: &mut B,
) -> Result<Vec<KeyValueChange<K, V>>>
where
    K: DeserializeOwned + Clone + Eq + Hash + AsRef<[u8]>,
    V: DeserializeOwned + Clone + Eq,
    H: Hasher + Clone + 'static,
    B: BlockStore,
{
    let node_changes = super::node_diff(main_link.clone(), other_link.clone(), store).await?;

    let main_node = main_link.resolve_value(store).await?;
    let other_node = other_link.resolve_value(store).await?;

    let mut kv_changes = Vec::new();
    for change in node_changes {
        match change.r#type {
            ChangeType::Add => {
                let result = main_node.get_node_at(&change.hashkey, store).await?;
                kv_changes
                    .extend(generate_add_or_remove_changes(result, ChangeType::Add, store).await?);
            }
            ChangeType::Remove => {
                let result = other_node.get_node_at(&change.hashkey, store).await?;
                kv_changes.extend(
                    generate_add_or_remove_changes(result, ChangeType::Remove, store).await?,
                );
            }
            ChangeType::Modify => match (
                main_node.get_node_at(&change.hashkey, store).await?,
                other_node.get_node_at(&change.hashkey, store).await?,
            ) {
                (Some(Left(main_pair)), Some(Left(other_pair))) => {
                    kv_changes.push(KeyValueChange {
                        r#type: ChangeType::Modify,
                        key: main_pair.key.clone(),
                        main_value: Some(main_pair.value.clone()),
                        other_value: Some(other_pair.value.clone()),
                    });
                }
                _ => unreachable!("Node change type is Modify but nodes not found or not pairs."),
            },
        }
    }

    Ok(kv_changes)
}

async fn generate_add_or_remove_changes<'a, K, V, H, B>(
    node: EitherPairOrNode<'a, K, V, H>,
    r#type: ChangeType,
    store: &B,
) -> Result<Vec<KeyValueChange<K, V>>>
where
    B: BlockStore,
    K: DeserializeOwned + Clone,
    V: DeserializeOwned + Clone,
    H: Hasher + Clone + 'static,
{
    match node {
        Some(Left(Pair { key, value })) => Ok(vec![KeyValueChange {
            r#type,
            key: key.clone(),
            main_value: if r#type == ChangeType::Add {
                Some(value.clone())
            } else {
                None
            },
            other_value: if r#type == ChangeType::Remove {
                Some(value.clone())
            } else {
                None
            },
        }]),
        Some(Right(node)) => {
            node.flat_map(
                &|Pair { key, value }| {
                    Ok(KeyValueChange {
                        r#type,
                        key: key.clone(),
                        main_value: if r#type == ChangeType::Add {
                            Some(value.clone())
                        } else {
                            None
                        },
                        other_value: if r#type == ChangeType::Remove {
                            Some(value.clone())
                        } else {
                            None
                        },
                    })
                },
                store,
            )
            .await
        }
        _ => unreachable!("Node change type is Remove but node is not found."),
    }
}

//--------------------------------------------------------------------------------------------------
// Tests
//--------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::{ChangeType::*, *};
    use crate::{
        private::{HashNibbles, Node},
        utils::test_setup,
    };
    use helper::*;
    use std::rc::Rc;

    mod helper {
        use once_cell::sync::Lazy;

        use crate::{utils, HashOutput, Hasher};

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
        pub(crate) struct MockHasher;
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

        let changes = kv_diff(
            Link::from(Rc::clone(&main_node)),
            Link::from(Rc::clone(&other_node)),
            store,
        )
        .await
        .unwrap();

        assert_eq!(
            changes,
            vec![
                KeyValueChange {
                    r#type: Add,
                    key: [2, 0, 0, 0,],
                    main_value: Some(String::from("2")),
                    other_value: None,
                },
                KeyValueChange {
                    r#type: Add,
                    key: [1, 0, 0, 0,],
                    main_value: Some(String::from("1")),
                    other_value: None,
                },
            ]
        );

        let changes = kv_diff(Link::from(other_node), Link::from(main_node), store)
            .await
            .unwrap();

        assert_eq!(
            changes,
            vec![
                KeyValueChange {
                    r#type: Remove,
                    key: [2, 0, 0, 0,],
                    main_value: None,
                    other_value: Some(String::from("2")),
                },
                KeyValueChange {
                    r#type: Remove,
                    key: [1, 0, 0, 0,],
                    main_value: None,
                    other_value: Some(String::from("1")),
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

        let changes = kv_diff(Link::from(main_node), Link::from(other_node), store)
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

        let changes = kv_diff(
            Link::from(Rc::clone(&main_node)),
            Link::from(Rc::clone(&other_node)),
            store,
        )
        .await
        .unwrap();

        assert_eq!(
            changes,
            vec![
                KeyValueChange {
                    r#type: Modify,
                    key: "second".to_string(),
                    main_value: Some("second_modified".to_string()),
                    other_value: Some("second".to_string()),
                },
                KeyValueChange {
                    r#type: Remove,
                    key: "third".to_string(),
                    main_value: None,
                    other_value: Some("third".to_string()),
                },
                KeyValueChange {
                    r#type: Add,
                    key: "fourth".to_string(),
                    main_value: Some("fourth".to_string()),
                    other_value: None,
                },
                KeyValueChange {
                    r#type: Add,
                    key: "fifth".to_string(),
                    main_value: Some("fifth".to_string()),
                    other_value: None,
                },
            ]
        );

        let changes = kv_diff(Link::from(other_node), Link::from(main_node), store)
            .await
            .unwrap();

        assert_eq!(
            changes,
            vec![
                KeyValueChange {
                    r#type: Modify,
                    key: "second".to_string(),
                    main_value: Some("second".to_string()),
                    other_value: Some("second_modified".to_string()),
                },
                KeyValueChange {
                    r#type: Add,
                    key: "third".to_string(),
                    main_value: Some("third".to_string()),
                    other_value: None,
                },
                KeyValueChange {
                    r#type: Remove,
                    key: "fourth".to_string(),
                    main_value: None,
                    other_value: Some("fourth".to_string()),
                },
                KeyValueChange {
                    r#type: Remove,
                    key: "fifth".to_string(),
                    main_value: None,
                    other_value: Some("fifth".to_string()),
                },
            ]
        );
    }
}

#[cfg(test)]
mod proptests {
    use crate::{
        private::{
            strategies::{self, generate_kvs, generate_ops_and_changes, Change, Operations},
            ChangeType,
        },
        utils::test_setup,
        Link,
    };
    use async_std::task;
    use std::{collections::HashSet, rc::Rc};
    use test_strategy::proptest;

    #[proptest(cases = 100, max_shrink_iters = 4000)]
    fn diff_correspondence(
        #[strategy(generate_ops_and_changes())] ops_changes: (
            Operations<String, u64>,
            Vec<Change<String, u64>>,
        ),
    ) {
        task::block_on(async {
            let store = test_setup::init!(mut store);
            let (ops, strategy_changes) = ops_changes;

            let other_node = strategies::prepare_node(
                strategies::node_from_operations(&ops, store).await.unwrap(),
                &strategy_changes,
                store,
            )
            .await
            .unwrap();

            let main_node =
                strategies::apply_changes(Rc::clone(&other_node), &strategy_changes, store)
                    .await
                    .unwrap();

            let changes = super::kv_diff(
                Link::from(Rc::clone(&main_node)),
                Link::from(Rc::clone(&other_node)),
                store,
            )
            .await
            .unwrap();

            assert_eq!(strategy_changes.len(), changes.len());
            for strategy_change in strategy_changes {
                assert!(changes.iter().any(|c| match &strategy_change {
                    Change::Add(k, _) => c.r#type == ChangeType::Add && &c.key == k,
                    Change::Modify(k, _) => {
                        c.r#type == ChangeType::Modify && &c.key == k
                    }
                    Change::Remove(k) => {
                        c.r#type == ChangeType::Remove && &c.key == k
                    }
                }));
            }
        });
    }

    #[proptest(cases = 1000, max_shrink_iters = 40000)]
    fn diff_unique_keys(
        #[strategy(generate_kvs("[a-z0-9]{1,3}", 0u64..1000, 0..100))] kvs1: Vec<(String, u64)>,
        #[strategy(generate_kvs("[a-z0-9]{1,3}", 0u64..1000, 0..100))] kvs2: Vec<(String, u64)>,
    ) {
        task::block_on(async {
            let store = test_setup::init!(mut store);

            let node1 = strategies::node_from_kvs(kvs1, store).await.unwrap();
            let node2 = strategies::node_from_kvs(kvs2, store).await.unwrap();

            let changes = super::kv_diff(Link::from(node1), Link::from(node2), store)
                .await
                .unwrap();

            let change_set = changes
                .iter()
                .map(|c| c.key.clone())
                .collect::<HashSet<_>>();

            assert_eq!(change_set.len(), changes.len());
        });
    }

    #[proptest(cases = 100)]
    fn add_remove_flip(
        #[strategy(generate_kvs("[a-z0-9]{1,3}", 0u64..1000, 0..100))] kvs1: Vec<(String, u64)>,
        #[strategy(generate_kvs("[a-z0-9]{1,3}", 0u64..1000, 0..100))] kvs2: Vec<(String, u64)>,
    ) {
        task::block_on(async {
            let store = test_setup::init!(mut store);

            let node1 = strategies::node_from_kvs(kvs1, store).await.unwrap();
            let node2 = strategies::node_from_kvs(kvs2, store).await.unwrap();

            let changes = super::kv_diff(
                Link::from(Rc::clone(&node1)),
                Link::from(Rc::clone(&node2)),
                store,
            )
            .await
            .unwrap();

            let flipped_changes = super::kv_diff(Link::from(node2), Link::from(node1), store)
                .await
                .unwrap();

            assert_eq!(changes.len(), flipped_changes.len());
            for change in changes {
                assert!(flipped_changes.iter().any(|c| match change.r#type {
                    ChangeType::Add => c.r#type == ChangeType::Remove && c.key == change.key,
                    ChangeType::Remove => c.r#type == ChangeType::Add && c.key == change.key,
                    ChangeType::Modify => c.r#type == ChangeType::Modify && c.key == change.key,
                }));
            }
        });
    }
}
