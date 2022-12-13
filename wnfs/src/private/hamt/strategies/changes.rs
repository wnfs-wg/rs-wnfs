#![allow(dead_code)]

use crate::{private::Node, BlockStore};
use anyhow::Result;
use hashbrown::HashMap;
use proptest::{
    prelude::Rng,
    strategy::{NewTree, Strategy, ValueTree},
    test_runner::TestRunner,
};
use rand::{distributions::Standard, prelude::Distribution};
use serde::de::DeserializeOwned;
use std::{fmt::Debug, rc::Rc};

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub(crate) enum Change<K, V> {
    Add(K, V),
    Remove(K),
    Modify(K, V),
}

#[derive(Debug)]
pub(crate) struct ChangeStrategy<'a, K, V> {
    pairs: &'a Vec<(&'a K, &'a V)>,
}

pub(crate) struct ChangeValueTree<K, V> {
    current: Vec<Change<K, V>>,
}

//--------------------------------------------------------------------------------------------------
// Implementations
//--------------------------------------------------------------------------------------------------

impl<'a, K, V> Strategy for ChangeStrategy<'a, K, V>
where
    K: Debug + Clone,
    V: Debug + Clone,
    Standard: Distribution<V>,
{
    type Tree = ChangeValueTree<K, V>;
    type Value = Vec<Change<K, V>>;

    fn new_tree(&self, runner: &mut TestRunner) -> NewTree<Self> {
        let rng = runner.rng();

        let mut current = Vec::new();
        for (k, _) in self.pairs {
            let change: Change<K, V> = match rng.gen_range(0..=3) {
                0 => Change::Add((*k).clone(), rng.gen()),
                1 => Change::Remove((*k).clone()),
                2 => Change::Modify((*k).clone(), rng.gen()),
                3 => continue,
                _ => unreachable!(),
            };
            current.push(change);
        }

        Ok(ChangeValueTree { current })
    }
}

impl<K, V> ValueTree for ChangeValueTree<K, V>
where
    K: Debug + Clone,
    V: Debug + Clone,
{
    type Value = Vec<Change<K, V>>;

    fn current(&self) -> Self::Value {
        self.current.clone()
    }

    fn simplify(&mut self) -> bool {
        false
    }

    fn complicate(&mut self) -> bool {
        false
    }
}

//--------------------------------------------------------------------------------------------------
// Functions
//--------------------------------------------------------------------------------------------------

pub(crate) fn get_changes<'a, K, V>(pairs: &'a Vec<(&'a K, &'a V)>) -> ChangeStrategy<'a, K, V> {
    ChangeStrategy { pairs }
}

pub(crate) fn collect_map_pairs<K, V>(map: &HashMap<K, V>) -> Vec<(&K, &V)> {
    map.iter().map(|(k, v)| (k, v)).collect::<Vec<_>>()
}

pub(crate) async fn prepare_node<K, V, B>(
    mut node: Rc<Node<K, V>>,
    changes: &Vec<Change<K, V>>,
    store: &B,
) -> Result<Rc<Node<K, V>>>
where
    K: Debug + Clone + AsRef<[u8]> + DeserializeOwned,
    V: Debug + Clone + DeserializeOwned,
    B: BlockStore,
{
    for change in changes {
        if let Change::Add(k, _) = change {
            (node, _) = node.remove(k, store).await?;
        }
    }
    Ok(node)
}

pub(crate) async fn apply_changes<K, V, B>(
    mut node: Rc<Node<K, V>>,
    changes: &Vec<Change<K, V>>,
    store: &B,
) -> Result<Rc<Node<K, V>>>
where
    K: Debug + Clone + AsRef<[u8]> + DeserializeOwned,
    V: Debug + Clone + DeserializeOwned,
    B: BlockStore,
{
    for change in changes {
        match change {
            Change::Add(k, v) => {
                node = node.set(k.clone(), v.clone(), store).await?;
            }
            Change::Remove(k) => {
                (node, _) = node.remove(k, store).await?;
            }
            Change::Modify(k, v) => {
                node = node.set(k.clone(), v.clone(), store).await?;
            }
        }
    }
    Ok(node)
}
