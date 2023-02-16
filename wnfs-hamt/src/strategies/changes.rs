#![cfg(test)]

use super::{operations, Operations};
use crate::Node;
use anyhow::Result;
use proptest::{collection::vec, strategy::Strategy};
use serde::de::DeserializeOwned;
use std::{collections::HashMap, fmt::Debug, rc::Rc};
use wnfs_common::BlockStore;

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub(crate) enum Change<K, V> {
    Add(K, V),
    Remove(K),
    Modify(K, V),
}

//--------------------------------------------------------------------------------------------------
// Functions
//--------------------------------------------------------------------------------------------------

pub(crate) fn generate_changes<K: Debug + Clone, V: Debug + Clone>(
    value_gen: impl Strategy<Value = V>,
    pairs: Vec<(K, V)>,
) -> impl Strategy<Value = Vec<Change<K, V>>> {
    let rngs = vec((0..=3, value_gen), pairs.len());
    rngs.prop_map(move |randoms| {
        pairs
            .clone()
            .into_iter()
            .zip(randoms.into_iter())
            .filter(|(_, (num, _))| *num != 0)
            .map(|((k, _), (num, val))| match num {
                1 => Change::Add(k, val),
                2 => Change::Remove(k),
                3 => Change::Modify(k, val),
                _ => unreachable!(),
            })
            .collect::<Vec<_>>()
    })
}

pub(crate) fn generate_ops_and_changes(
) -> impl Strategy<Value = (Operations<String, u64>, Vec<Change<String, u64>>)> {
    operations("[a-z0-9]{1,3}", 0..1000u64, 1..20).prop_flat_map(|ops| {
        let map = HashMap::from(&ops);
        let pairs = map.into_iter().collect::<Vec<_>>();
        generate_changes(1000..2000_u64, pairs).prop_map(move |changes| (ops.clone(), changes))
    })
}

pub(crate) async fn apply_changes<K, V, B>(
    node: &mut Rc<Node<K, V>>,
    changes: &Vec<Change<K, V>>,
    store: &B,
) -> Result<()>
where
    K: Debug + Clone + AsRef<[u8]> + DeserializeOwned,
    V: Debug + Clone + DeserializeOwned,
    B: BlockStore,
{
    for change in changes {
        match change {
            Change::Add(k, v) => {
                node.set(k.clone(), v.clone(), store).await?;
            }
            Change::Remove(k) => {
                node.remove(k, store).await?;
            }
            Change::Modify(k, v) => {
                node.set(k.clone(), v.clone(), store).await?;
            }
        }
    }

    Ok(())
}

pub(crate) async fn prepare_node<K, V, B>(
    node: &mut Rc<Node<K, V>>,
    changes: &Vec<Change<K, V>>,
    store: &B,
) -> Result<()>
where
    K: Debug + Clone + AsRef<[u8]> + DeserializeOwned,
    V: Debug + Clone + DeserializeOwned,
    B: BlockStore,
{
    for change in changes {
        if let Change::Add(k, _) = change {
            node.remove(k, store).await?;
        }
    }

    Ok(())
}
