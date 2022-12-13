use super::{diff, ChangeType, Node};
use crate::{BlockStore, FsError, Hasher, Link};
use anyhow::Result;
use serde::de::DeserializeOwned;
use std::{hash::Hash, rc::Rc};

//--------------------------------------------------------------------------------------------------
// Functions
//--------------------------------------------------------------------------------------------------

/// Merges a node with another with the help of a resolver function.
pub async fn merge<K, V, H, F, B: BlockStore>(
    main_link: Link<Rc<Node<K, V, H>>>,
    other_link: Link<Rc<Node<K, V, H>>>,
    f: F,
    store: &mut B,
) -> Result<Rc<Node<K, V, H>>>
where
    F: Fn(&V, &V) -> Result<V>,
    K: DeserializeOwned + Eq + Clone + Hash + AsRef<[u8]>,
    V: DeserializeOwned + Eq + Clone,
    H: Hasher + Clone + 'static,
{
    let kv_changes = diff::kv_diff(main_link.clone(), other_link.clone(), None, store).await?;

    let main_node = main_link.resolve_owned_value(store).await?;
    let other_node = other_link.resolve_owned_value(store).await?;

    let mut merge_node = Rc::clone(&main_node);
    for change in kv_changes {
        match change.r#type {
            ChangeType::Remove => {
                merge_node = merge_node
                    .set(change.key, change.other_value.unwrap(), store)
                    .await?;
            }
            ChangeType::Modify => {
                let main_value = main_node
                    .get(&change.key, store)
                    .await?
                    .ok_or(FsError::KeyNotFoundInHamt)?;

                let other_value = other_node
                    .get(&change.key, store)
                    .await?
                    .ok_or(FsError::KeyNotFoundInHamt)?;

                merge_node = merge_node
                    .set(change.key, f(main_value, other_value)?, store)
                    .await?;
            }
            _ => (),
        }
    }

    Ok(merge_node)
}

//--------------------------------------------------------------------------------------------------
// Tests
//--------------------------------------------------------------------------------------------------

#[cfg(test)]
mod proptests {
    use std::rc::Rc;

    use crate::{
        private::strategies::{self, operations, Operations},
        utils::{test_setup, Sampleable},
        Link,
    };
    use async_std::task;
    use hashbrown::HashMap;
    use test_strategy::proptest;

    #[proptest(cases = 100)]
    fn merge_associativity(
        #[strategy(operations("[a-z0-9]{1,8}", 0..u64::MAX, 1..100))] ops: Operations<String, u64>,
    ) {
        task::block_on(async {
            let (store, runner) = test_setup::init!(mut store, mut runner);

            let map = HashMap::from(&ops);
            let pairs = strategies::collect_map_pairs(&map);
            let strategy_changes_1 = strategies::get_changes(&pairs).sample(runner);
            let strategy_changes_2 = strategies::get_changes(&pairs).sample(runner);

            let node1 = {
                let tmp = strategies::prepare_node(
                    strategies::node_from_operations(&ops, store).await.unwrap(),
                    &strategy_changes_1,
                    store,
                )
                .await
                .unwrap();

                strategies::prepare_node(tmp, &strategy_changes_2, store)
                    .await
                    .unwrap()
            };

            let node2 = strategies::apply_changes(Rc::clone(&node1), &strategy_changes_1, store)
                .await
                .unwrap();

            let node3 = strategies::apply_changes(Rc::clone(&node1), &strategy_changes_2, store)
                .await
                .unwrap();

            let merge_node_1 = {
                let tmp = super::merge(
                    Link::from(Rc::clone(&node1)),
                    Link::from(Rc::clone(&node2)),
                    |a, b| Ok(a.wrapping_add(*b)),
                    store,
                )
                .await
                .unwrap();

                super::merge(
                    Link::from(tmp),
                    Link::from(Rc::clone(&node3)),
                    |a, b| Ok(a.wrapping_add(*b)),
                    store,
                )
                .await
                .unwrap()
            };

            let merge_node_2 = {
                let tmp = super::merge(
                    Link::from(node2),
                    Link::from(node3),
                    |a, b| Ok(a.wrapping_add(*b)),
                    store,
                )
                .await
                .unwrap();

                super::merge(
                    Link::from(tmp),
                    Link::from(node1),
                    |a, b| Ok(a.wrapping_add(*b)),
                    store,
                )
                .await
                .unwrap()
            };

            assert_eq!(merge_node_1, merge_node_2);
        });
    }

    #[proptest(cases = 100)]
    fn merge_commutativity(
        #[strategy(operations("[a-z0-9]{1,8}", 0..u64::MAX, 1..100))] ops: Operations<String, u64>,
    ) {
        task::block_on(async move {
            let (store, runner) = test_setup::init!(mut store, mut runner);

            let map = HashMap::from(&ops);
            let pairs = strategies::collect_map_pairs(&map);
            let strategy_changes = strategies::get_changes(&pairs).sample(runner);

            let node1 = strategies::prepare_node(
                strategies::node_from_operations(&ops, store).await.unwrap(),
                &strategy_changes,
                store,
            )
            .await
            .unwrap();

            let node2 = strategies::apply_changes(Rc::clone(&node1), &strategy_changes, store)
                .await
                .unwrap();

            let merge_node_1 = super::merge(
                Link::from(Rc::clone(&node1)),
                Link::from(Rc::clone(&node2)),
                |a, b| Ok(a.wrapping_add(*b)),
                store,
            )
            .await
            .unwrap();

            let merge_node_2 = super::merge(
                Link::from(node2),
                Link::from(node1),
                |a, b| Ok(a.wrapping_add(*b)),
                store,
            )
            .await
            .unwrap();

            assert_eq!(merge_node_1, merge_node_2);
        })
    }

    #[proptest(cases = 100)]
    fn merge_idempotency(
        #[strategy(operations("[a-z0-9]{1,8}", 0..u64::MAX, 1..100))] ops: Operations<String, u64>,
    ) {
        task::block_on(async move {
            let (store, runner) = test_setup::init!(mut store, mut runner);

            let map = HashMap::from(&ops);
            let pairs = strategies::collect_map_pairs(&map);
            let strategy_changes = strategies::get_changes(&pairs).sample(runner);

            let node1 = strategies::prepare_node(
                strategies::node_from_operations(&ops, store).await.unwrap(),
                &strategy_changes,
                store,
            )
            .await
            .unwrap();

            let node2 = strategies::apply_changes(Rc::clone(&node1), &strategy_changes, store)
                .await
                .unwrap();

            let merge_node_1 = super::merge(
                Link::from(Rc::clone(&node1)),
                Link::from(Rc::clone(&node2)),
                |a, b| Ok(a.wrapping_add(*b)),
                store,
            )
            .await
            .unwrap();

            let merge_node_2 = super::merge(
                Link::from(Rc::clone(&merge_node_1)),
                Link::from(node2),
                |a, b| Ok(a.wrapping_add(*b)),
                store,
            )
            .await
            .unwrap();

            assert_eq!(merge_node_1, merge_node_2);
        })
    }
}
