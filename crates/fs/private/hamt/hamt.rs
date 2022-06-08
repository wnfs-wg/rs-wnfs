// TODO(appcypher): Based on ipld_hamt implementation

use super::Node;

#[derive(Debug)]
pub struct Hamt<K, V> {
    root: Node<K, V>,
}

impl<K, V> Hamt<K, V> {
    pub fn new() -> Self {
        todo!()
    }
}

impl<K, V> Default for Hamt<K, V> {
    fn default() -> Self {
        todo!()
    }
}
