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
    use crate::{
        private::strategies::{self, operations, Operations},
        utils::test_setup,
        Link,
    };
    use async_std::task;
    use std::{cmp, rc::Rc};
    use test_strategy::proptest;

    #[proptest(cases = 100)]
    fn merge_associativity(
        #[strategy(operations("[a-z0-9]{1,4}", 0u64..1000, 0..100))] ops1: Operations<String, u64>,
        #[strategy(operations("[a-z0-9]{1,4}", 0u64..1000, 0..100))] ops2: Operations<String, u64>,
        #[strategy(operations("[a-z0-9]{1,4}", 0u64..1000, 0..100))] ops3: Operations<String, u64>,
    ) {
        task::block_on(async {
            let store = test_setup::init!(mut store);

            let node1 = strategies::node_from_operations(&ops1, store)
                .await
                .unwrap();

            let node2 = strategies::node_from_operations(&ops2, store)
                .await
                .unwrap();

            let node3 = strategies::node_from_operations(&ops3, store)
                .await
                .unwrap();

            let merge_node_left_assoc = {
                let tmp = super::merge(
                    Link::from(Rc::clone(&node1)),
                    Link::from(Rc::clone(&node2)),
                    |a, b| Ok(cmp::min(*a, *b)),
                    store,
                )
                .await
                .unwrap();

                super::merge(
                    Link::from(tmp),
                    Link::from(Rc::clone(&node3)),
                    |a, b| Ok(cmp::min(*a, *b)),
                    store,
                )
                .await
                .unwrap()
            };

            let merge_node_right_assoc = {
                let tmp = super::merge(
                    Link::from(node2),
                    Link::from(node3),
                    |a, b| Ok(cmp::min(*a, *b)),
                    store,
                )
                .await
                .unwrap();

                super::merge(
                    Link::from(node1),
                    Link::from(tmp),
                    |a, b| Ok(cmp::min(*a, *b)),
                    store,
                )
                .await
                .unwrap()
            };

            assert_eq!(merge_node_left_assoc, merge_node_right_assoc);
        });
    }

    #[proptest(cases = 100)]
    fn merge_commutativity(
        #[strategy(operations("[a-z0-9]{1,4}", 0u64..1000, 0..100))] ops1: Operations<String, u64>,
        #[strategy(operations("[a-z0-9]{1,4}", 0u64..1000, 0..100))] ops2: Operations<String, u64>,
    ) {
        task::block_on(async {
            let store = test_setup::init!(mut store);

            let node1 = strategies::node_from_operations(&ops1, store)
                .await
                .unwrap();

            let node2 = strategies::node_from_operations(&ops2, store)
                .await
                .unwrap();

            let merge_node_1 = super::merge(
                Link::from(Rc::clone(&node1)),
                Link::from(Rc::clone(&node2)),
                |a, b| Ok(cmp::min(*a, *b)),
                store,
            )
            .await
            .unwrap();

            let merge_node_2 = super::merge(
                Link::from(node2),
                Link::from(node1),
                |a, b| Ok(cmp::min(*a, *b)),
                store,
            )
            .await
            .unwrap();

            assert_eq!(merge_node_1, merge_node_2);
        })
    }

    #[proptest(cases = 100)]
    fn merge_idempotency(
        #[strategy(operations("[a-z0-9]{1,4}", 0u64..1000, 0..100))] ops1: Operations<String, u64>,
        #[strategy(operations("[a-z0-9]{1,4}", 0u64..1000, 0..100))] ops2: Operations<String, u64>,
    ) {
        task::block_on(async {
            let store = test_setup::init!(mut store);

            let node1 = strategies::node_from_operations(&ops1, store)
                .await
                .unwrap();

            let node2 = strategies::node_from_operations(&ops2, store)
                .await
                .unwrap();

            let merge_node_1 = super::merge(
                Link::from(Rc::clone(&node1)),
                Link::from(Rc::clone(&node2)),
                |a, b| Ok(cmp::min(*a, *b)),
                store,
            )
            .await
            .unwrap();

            let merge_node_2 = super::merge(
                Link::from(Rc::clone(&merge_node_1)),
                Link::from(node2),
                |a, b| Ok(cmp::min(*a, *b)),
                store,
            )
            .await
            .unwrap();

            assert_eq!(merge_node_1, merge_node_2);
        })
    }
}
