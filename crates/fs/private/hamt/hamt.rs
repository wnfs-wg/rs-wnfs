use std::hash::Hash;

#[derive(Debug, PartialEq, Eq, Default)]
pub struct Hamt<K, V> {
    // pub(crate) root: Option<Box<Node<K, V>>>,
    phantom: std::marker::PhantomData<(K, V)>,
}

impl<K, V> Hamt<K, V>
where
    K: Hash + Eq,
{
    pub fn new() -> Self {
        todo!()
    }
}
