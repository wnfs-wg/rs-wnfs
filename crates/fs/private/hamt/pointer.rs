use std::rc::Rc;

use serde::{ser, Serialize, Serializer};

use crate::Link;

use super::Node;

#[derive(Debug, Clone)]
pub enum Pointer<K, V> {
    Values(Vec<(K, V)>),
    NodeLink(Link<Rc<Node<K, V>>>),
}

/// Serialize the Pointer like an untagged enum.
impl<K, V> Serialize for Pointer<K, V>
where
    K: Serialize,
    V: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Pointer::Values(vals) => vals.serialize(serializer),
            Pointer::NodeLink(Link::Clean { cid, .. }) => cid.serialize(serializer),
            Pointer::NodeLink(Link::Dirty(_)) => {
                Err(ser::Error::custom("Cannot serialize cached values"))
            }
        }
    }
}
