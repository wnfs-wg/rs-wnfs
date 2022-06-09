// TODO(appcypher): Based on ipld_hamt implementation

use anyhow::Result;

use super::Node;
use crate::BlockStore;

//--------------------------------------------------------------------------------------------------
// Type Definitions
//--------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub struct Hamt<K, V> {
    pub(crate) root: Node<K, V>,
}

//--------------------------------------------------------------------------------------------------
// Implementations
//--------------------------------------------------------------------------------------------------

impl<K, V> Hamt<K, V> {
    pub fn new() -> Self {
        Self {
            root: Node::default(),
        }
    }

    pub fn set<B: BlockStore>(&mut self, _key: K, _value: V, _store: &B) -> Result<Option<V>>
    where
        V: PartialEq,
    {
        // self.root.set(key, value).map(|(r, _)| r)
        // TODO(appcypher)
        todo!()
    }
}

impl<'a, K: PartialEq, V: PartialEq> PartialEq for Hamt<K, V> {
    fn eq(&self, other: &Self) -> bool {
        self.root == other.root
    }
}

impl<K, V> Default for Hamt<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

//--------------------------------------------------------------------------------------------------
// Tests
//--------------------------------------------------------------------------------------------------

#[cfg(test)]
mod hamt_tests {
    use libipld::Cid;

    use super::*;

    #[test]
    fn test_hamt_new() {
        let hamt: Hamt<String, Cid> = Hamt::default();
        assert_eq!(hamt.root, Node::default());
    }
}
