use super::ChangeType;
use crate::{
    private::{HashKey, Node},
    BlockStore, Hasher, Link, Pair,
};
use anyhow::{Ok, Result};
use either::Either::{self, *};
use serde::de::DeserializeOwned;
use std::{fmt, hash::Hash, rc::Rc};

//--------------------------------------------------------------------------------------------------
// Type Definitions
//--------------------------------------------------------------------------------------------------

/// The change between two nodes at a given key.
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

/// TODO(appcypher): Add docs.
/// TODO(appcypher): Add tests.
pub async fn kv_diff<K, V, H, B>(
    root_main: Link<Rc<Node<K, V, H>>>,
    root_other: Link<Rc<Node<K, V, H>>>,
    depth: Option<u8>,
    store: &mut B,
) -> Result<Vec<KeyValueChange<K, V>>>
where
    K: DeserializeOwned + Clone + fmt::Debug + Eq + Hash + AsRef<[u8]>,
    V: DeserializeOwned + Clone + fmt::Debug + Eq,
    H: Hasher + Clone + fmt::Debug + 'static,
    B: BlockStore,
{
    let node_changes = super::node_diff(
        root_main.clone(),
        root_other.clone(),
        depth,
        HashKey::default(),
        store,
    )
    .await?;

    let main_node = root_main.resolve_value(store).await?;
    let other_node = root_main.resolve_value(store).await?;

    let mut kv_changes = Vec::new();
    for change in node_changes {
        match change.r#type {
            ChangeType::Add => {
                let result = main_node.get_node_at(&change.hashkey, 0, store).await?;
                kv_changes
                    .extend(create_add_or_remove_changes(result, ChangeType::Add, store).await?);
            }
            ChangeType::Remove => {
                let result = other_node.get_node_at(&change.hashkey, 0, store).await?;
                kv_changes
                    .extend(create_add_or_remove_changes(result, ChangeType::Remove, store).await?);
            }
            ChangeType::Modify => match (
                main_node.get_node_at(&change.hashkey, 0, store).await?,
                other_node.get_node_at(&change.hashkey, 0, store).await?,
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

async fn create_add_or_remove_changes<'a, K, V, H, B>(
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
    // Depending on change type, main_value and other_value will be None or Some.
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
mod tests {}
