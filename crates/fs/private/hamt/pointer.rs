use std::rc::Rc;

use libipld::Ipld;
use serde::{
    de::{self, DeserializeOwned},
    ser, Deserialize, Deserializer, Serialize, Serializer,
};

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
            Pointer::NodeLink(link) => match link.cid_cached() {
                Some(cid) => cid.serialize(serializer),
                None => Err(ser::Error::custom("Must flush HAMT before serialization")),
            },
        }
    }
}

impl<K, V> TryFrom<Ipld> for Pointer<K, V>
where
    K: DeserializeOwned,
    V: DeserializeOwned,
{
    type Error = String;

    fn try_from(ipld: Ipld) -> Result<Self, Self::Error> {
        match ipld {
            ipld_list @ Ipld::List(_) => {
                let values: Vec<(K, V)> =
                    Deserialize::deserialize(ipld_list).map_err(|error| error.to_string())?;
                Ok(Self::Values(values))
            }
            Ipld::Link(cid) => Ok(Self::NodeLink(Link::from_cid(cid))),
            other => Err(format!(
                "Expected `Ipld::List` or `Ipld::Link`, got {:#?}",
                other
            )),
        }
    }
}

impl<'de, K, V> Deserialize<'de> for Pointer<K, V>
where
    K: DeserializeOwned,
    V: DeserializeOwned,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ipld::deserialize(deserializer).and_then(|ipld| ipld.try_into().map_err(de::Error::custom))
    }
}

impl<K, V> Default for Pointer<K, V> {
    fn default() -> Self {
        Pointer::Values(Vec::new())
    }
}
