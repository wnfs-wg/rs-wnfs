use super::Node;

#[derive(Debug)]
pub struct Hamt<K, V> {
    pub(crate) root: Node<K, V>,
    // phantom: std::marker::PhantomData<(K, V)>,
}

impl<K, V> Hamt<K, V> {
    pub fn new() -> Self {
        todo!()
    }
}
