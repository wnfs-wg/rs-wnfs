use std::fmt::Debug;
use wnfs_common::utils::Arc;

pub struct Node<K, V> {
    dirty: bool,
    level: u16,
    keys: Vec<K>,
    values: Vec<V>,
    links: Vec<Arc<Node<K, V>>>,
}

impl<K: Debug, V: Debug> Debug for Node<K, V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Node")
            .field("dirty", &self.dirty)
            .field("level", &self.level)
            .field("keys", &DebugString(&self.keys))
            .field("values", &DebugString(&self.values))
            .field("links", &self.links)
            .finish()
    }
}

struct DebugString<T: Debug>(T);

impl<T: Debug> Debug for DebugString<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{:?}", self.0))
    }
}

// Implementations

impl<K, V> Node<K, V>
where
    K: Clone + Debug + Ord + AsRef<[u8]>,
    V: Clone + Debug,
{
    pub fn empty(level: u16) -> Self {
        Self {
            dirty: true,
            level,
            keys: Vec::new(),
            links: Vec::new(),
            values: Vec::new(),
        }
    }

    pub fn key(&self) -> &K {
        self.keys.first().expect("Invalid node") // TODO fix this?
    }

    pub fn insert(&mut self, key: K, value: V) {
        self.dirty = true;

        if self.level == 0 {
            self.insert_leaf(key, value);
        } else {
            self.insert_node(key, value);
        }
    }

    fn insert_leaf(&mut self, key: K, value: V) {
        debug_assert_eq!(self.keys.len(), self.values.len());

        match self.keys.binary_search(&key) {
            Ok(index) => {
                println!("Set index {index} to {key:?}");
                self.keys[index] = key;
                self.values[index] = value;
            }
            Err(index) => {
                println!("Inserted at index {index}: {key:?}");
                self.keys.insert(index, key);
                self.values.insert(index, value);
            }
        };
    }

    fn insert_node(&mut self, key: K, value: V) {
        debug_assert_eq!(self.keys.len(), self.links.len());

        let index = match self.keys.binary_search(&key) {
            Ok(exact_match) => exact_match,
            Err(0) => {
                // We have a new smallest key
                self.keys[0] = key.clone();
                0
            }
            Err(insert_position) => insert_position - 1,
        };

        Arc::make_mut(&mut self.links[index]).insert(key, value);
    }

    pub fn is_canonical(&self) -> bool {
        // empty nodes are non-canonical. They shouldn't exist in practice
        let Some(key) = self.keys.last() else {
            return false;
        };

        should_split(self.keys.len(), self.level, key)
    }

    pub fn canonicalize(self: &mut Arc<Self>) {
        if self.level == 0 {
            debug_assert_eq!(self.keys.len(), self.values.len());
        } else {
            debug_assert_eq!(self.keys.len(), self.links.len());
        }

        let nodes = Arc::make_mut(self).canonicalize_below();
        if nodes.is_empty() {
            return;
        }

        let level = self.level + 1;
        let node = Arc::clone(self);
        let mut links = Vec::with_capacity(nodes.len() + 1);
        links.push(node);
        links.extend(nodes.into_iter().map(Arc::new));

        let keys = links
            .iter()
            .map(|l| l.as_ref().key().clone())
            .collect::<Vec<_>>();

        *self = Arc::new(Self {
            dirty: false,
            level,
            keys,
            links,
            values: Vec::new(),
        });

        if self.level == 0 {
            debug_assert_eq!(self.keys.len(), self.values.len());
        } else {
            debug_assert_eq!(self.keys.len(), self.links.len());
        }
    }

    // TODO: return smallvec?
    fn canonicalize_below(&mut self) -> Vec<Self> {
        // No need to canonicalize if there were not changes since last time
        if !self.dirty {
            return Vec::new();
        }

        self.dirty = false;

        if self.level == 0 {
            self.chunk_leaf()
        } else {
            self.canonicalize_children();
            self.chunk_node()
        }
    }

    fn canonicalize_children(&mut self) {
        debug_assert_ne!(self.level, 0);

        let mut merge_node = None;
        let mut i = 0;
        while i < self.links.len() {
            let node = &mut self.links[i];
            if !node.dirty {
                i += 1;
                continue;
            }

            let node = Arc::make_mut(node);

            if let Some(merge) = merge_node {
                node.prepend(merge);
                self.keys[i] = node.key().clone();
            }

            let mut right_siblings = node.canonicalize_below();
            let last_right_sibling = right_siblings.pop();

            i += right_siblings.len();

            let keys = right_siblings.iter().map(Self::key);

            self.keys.splice(i..i, keys.cloned());
            self.links
                .splice(i..i, right_siblings.into_iter().map(Arc::new));

            merge_node = last_right_sibling;
            i += 1;
        }

        if let Some(last_node) = merge_node {
            self.keys.push(last_node.key().clone());
            self.links.push(Arc::new(last_node));
        }
    }

    fn chunk_leaf(&mut self) -> Vec<Self> {
        debug_assert_eq!(self.keys.len(), self.values.len());

        let Some((partition_idx, _)) = self
            .keys
            .iter()
            .enumerate()
            .find(|(idx, key)| should_split(idx + 1, self.level, key))
        else {
            // No need to split yet - node has space
            return Vec::new();
        };

        let keys = self.keys.drain(partition_idx + 1..);
        let values = self.values.drain(partition_idx + 1..);

        let mut nodes = Vec::with_capacity(2);
        let mut current_node = Self::empty(0);
        for (key, value) in keys.into_iter().zip(values.into_iter()) {
            current_node.keys.push(key);
            current_node.values.push(value);

            // determine whether to split
            if current_node.is_canonical() {
                current_node.dirty = false;
                debug_assert_eq!(current_node.keys.len(), current_node.values.len());
                nodes.push(current_node);
                current_node = Self::empty(0);
            }
        }

        if !current_node.keys.is_empty() {
            debug_assert_eq!(current_node.keys.len(), current_node.values.len());
            nodes.push(current_node);
        }

        nodes
    }

    fn chunk_node(&mut self) -> Vec<Self> {
        debug_assert!(self.keys.len() == self.links.len());

        let Some((partition_idx, _)) = self
            .keys
            .iter()
            .enumerate()
            .find(|(idx, key)| should_split(idx + 1, self.level, key))
        else {
            // No need to split yet - node has space
            return Vec::new();
        };

        let keys = self.keys.drain(partition_idx + 1..);
        let links = self.links.drain(partition_idx + 1..);

        let mut nodes = Vec::with_capacity(2);
        let mut current_node = Self::empty(self.level);
        for (key, link) in keys.into_iter().zip(links.into_iter()) {
            current_node.keys.push(key);
            current_node.links.push(link);

            // determine whether to split
            if current_node.is_canonical() {
                current_node.dirty = false;
                nodes.push(current_node);
                current_node = Self::empty(self.level);
            }
        }

        if !current_node.keys.is_empty() {
            nodes.push(current_node);
        }

        nodes
    }

    fn prepend(&mut self, merge_node: Node<K, V>) {
        debug_assert_eq!(self.level, merge_node.level);
        if self.level == 0 {
            debug_assert_eq!(self.keys.len(), self.values.len());
            debug_assert_eq!(merge_node.keys.len(), merge_node.values.len());
        } else {
            debug_assert_eq!(self.keys.len(), self.links.len());
            debug_assert_eq!(merge_node.keys.len(), merge_node.links.len());
        }

        let Node {
            mut keys,
            mut values,
            mut links,
            ..
        } = merge_node;

        std::mem::swap(&mut self.keys, &mut keys);
        std::mem::swap(&mut self.values, &mut values);
        std::mem::swap(&mut self.links, &mut links);

        self.keys.extend(keys);
        self.values.extend(values);
        self.links.extend(links);
    }
}

pub fn should_split(size: usize, level: u16, key: impl AsRef<[u8]>) -> bool {
    // we force size 2 at minimum for internal nodes
    if level > 0 && size < 2 {
        return false;
    }

    let hash: [u8; 32] = blake3::hash(key.as_ref()).into();
    let hash = u64::from_le_bytes(hash[..8].try_into().unwrap());

    weibull_check(size as u64, 256, hash)
}

/// https://github.com/dolthub/dolt/blob/main/go/store/prolly/tree/node_splitter.go#L247
pub fn weibull_check(size: u64, target: u64, hash: u64) -> bool {
    let fullness = size as f64 / target as f64;
    let p = hash as f64 / u64::MAX as f64;
    weibull_cdf(fullness) > p
}

// https://en.wikipedia.org/wiki/Weibull_distribution#First_alternative
pub fn weibull_cdf(x: f64) -> f64 {
    let exponent = -x.powi(4);
    -exponent.exp_m1()
}

// Boring instances

impl<K: Clone, V: Clone> Clone for Node<K, V> {
    fn clone(&self) -> Self {
        Self {
            // persisted_as: self
            //     .persisted_as
            //     .get()
            //     .cloned()
            //     .map_or_else(OnceCell::new, OnceCell::new_with),
            dirty: self.dirty,
            level: self.level,
            keys: self.keys.clone(),
            values: self.values.clone(),
            links: self.links.clone(),
        }
    }
}

// Serialization stuff

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct NodeSerializable<K, V> {
//     level: u16,
//     keys: Vec<K>,
//     values: Vec<V>,
//     nodes: Vec<Cid>,
// }

// impl<K, V> Storable for Node<K, V>
// where
//     K: Clone + Serialize + DeserializeOwned + CondSync,
//     V: Storable + CondSync,
//     V::Serializable: Serialize + DeserializeOwned,
// {
//     type Serializable = NodeSerializable<K, V::Serializable>;

//     async fn to_serializable(&self, store: &impl BlockStore) -> Result<Self::Serializable> {
//         let level = self.level;

//         let mut values = Vec::with_capacity(self.values.len());
//         for value in self.values.iter() {
//             values.push(value.to_serializable(store).await?);
//         }

//         let mut nodes = Vec::with_capacity(self.links.len());
//         for node in self.links.iter() {
//             nodes.push(boxed_fut(node.resolve_cid(store)).await?);
//         }

//         Ok(NodeSerializable {
//             level,
//             keys: self.keys.clone(),
//             values,
//             nodes,
//         })
//     }

//     async fn from_serializable(
//         cid: Option<&Cid>,
//         serializable: Self::Serializable,
//     ) -> Result<Self> {
//         let mut values = Vec::with_capacity(serializable.values.len());
//         for ser_value in serializable.values {
//             values.push(V::from_serializable(None, ser_value).await?);
//         }

//         Ok(Self {
//             persisted_as: cid.cloned().map_or_else(OnceCell::new, OnceCell::new_with),
//             level: serializable.level,
//             keys: serializable.keys.clone(),
//             values,
//             links: serializable.nodes.into_iter().map(Link::from_cid).collect(),
//         })
//     }

//     fn persisted_as(&self) -> Option<&OnceCell<Cid>> {
//         Some(&self.persisted_as)
//     }
// }

#[cfg(test)]
mod tests {
    use super::{weibull_cdf, weibull_check, Node};
    use wnfs_common::utils::Arc;

    #[test]
    fn test_weibull_check_chunking() {
        let target = 16u64;
        let mut size = 0u64;

        for i in 0..1000u64 {
            println!("{i}");
            let hash: [u8; 32] = blake3::hash(&i.to_le_bytes()).into();
            let hash = u64::from_le_bytes(hash[..8].try_into().unwrap());
            if weibull_check(size, target, hash) {
                println!("-----------");
                size = 0;
            } else {
                size += 1;
            }
        }
    }

    #[test]
    fn test_weibull_cdf() {
        for i in 0..=20u64 {
            let x = i as f64 / 10f64;
            println!("{}", weibull_cdf(x))
        }
    }

    #[test]
    fn test_stuff() {
        let tree = &mut Arc::new(Node::empty(0));
        for i in 0..10000 {
            Arc::make_mut(tree).insert(format!("{i}"), "Hi");
            tree.canonicalize();
        }

        let mut nodes = vec![Arc::clone(tree)];
        let mut num_nodes = 0f64;
        let mut total_degrees = 0f64;
        let mut nodes_over_10 = 0f64;
        let mut total_degrees_over_10 = 0f64;
        while let Some(node) = nodes.pop() {
            println!("Node level {}, degree: {}", node.level, node.keys.len());
            num_nodes += 1.0;
            total_degrees += node.keys.len() as f64;

            if node.keys.len() > 10 {
                nodes_over_10 += 1.0;
                total_degrees_over_10 += node.keys.len() as f64;
            }

            if node.level != 0 {
                nodes.extend(node.links.iter().cloned());
            }
        }

        println!("Num nodes: {num_nodes}");
        println!("Average degree: {}", total_degrees / num_nodes);

        println!("Num nodes (> 10 keys): {nodes_over_10}");
        println!(
            "Average degree (> 10 keys): {}",
            total_degrees_over_10 / nodes_over_10
        );
    }
}
