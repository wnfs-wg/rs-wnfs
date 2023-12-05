use crate::constants::HAMT_BITMASK_BYTE_SIZE;
use libipld::{
    cid::serde::{BytesToCidVisitor, CID_SERDE_PRIVATE_IDENTIFIER},
    Cid,
};
use semver::Version;
use serde::{
    de::{SeqAccess, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
};
use serde_byte_array::ByteArray;
use serde_bytes::ByteBuf;
use std::marker::PhantomData;

//--------------------------------------------------------------------------------------------------
// Type Definitions
//--------------------------------------------------------------------------------------------------

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HamtSerializable<K, V> {
    pub(crate) root: NodeSerializable<K, V>,
    pub(crate) version: Version,
    pub(crate) structure: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct NodeSerializable<K, V>(
    pub(crate) ByteArray<HAMT_BITMASK_BYTE_SIZE>,
    pub(crate) Vec<PointerSerializable<K, V>>,
);

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum PointerSerializable<K, V> {
    Values(Vec<(K, V)>),
    Link(Cid),
}

//--------------------------------------------------------------------------------------------------
// Implementations
//--------------------------------------------------------------------------------------------------

impl<K: Serialize, V: Serialize> Serialize for PointerSerializable<K, V> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match *self {
            Self::Values(ref vec) => vec.serialize(serializer),
            Self::Link(ref cid) => {
                let value = ByteBuf::from(cid.to_bytes());
                serializer.serialize_newtype_struct(CID_SERDE_PRIVATE_IDENTIFIER, &value)
            }
        }
    }
}

impl<'de, K: Deserialize<'de>, V: Deserialize<'de>> Deserialize<'de> for PointerSerializable<K, V> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct PointerVisitor<K, V>(PhantomData<(K, V)>);

        impl<'de, K: Deserialize<'de>, V: Deserialize<'de>> Visitor<'de> for PointerVisitor<K, V> {
            type Value = PointerSerializable<K, V>;

            fn expecting(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(
                    fmt,
                    "a valid PointerSerializable represented as CID bytes or as a sequence of tuples of keys and values"
                )
            }

            fn visit_newtype_struct<D: Deserializer<'de>>(
                self,
                deserializer: D,
            ) -> Result<Self::Value, D::Error> {
                let cid = deserializer.deserialize_bytes(BytesToCidVisitor)?;
                Ok(PointerSerializable::Link(cid))
            }

            fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
                let mut values = Vec::new();
                while let Some(elem) = seq.next_element::<(K, V)>()? {
                    values.push(elem);
                }
                Ok(PointerSerializable::Values(values))
            }
        }

        let visitor = PointerVisitor(PhantomData);
        deserializer.deserialize_any(visitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use testresult::TestResult;

    #[test]
    fn test_pointer_link_roundtrip() -> TestResult {
        let pointers = PointerSerializable::<String, String>::Link(Cid::default());
        let bytes = serde_ipld_dagcbor::to_vec(&pointers)?;

        let pointers_back: PointerSerializable<String, String> =
            serde_ipld_dagcbor::from_slice(&bytes)?;

        assert_eq!(pointers, pointers_back);

        Ok(())
    }

    #[test]
    fn test_pointer_values_roundtrip() -> TestResult {
        let pointers = PointerSerializable::Values(vec![(1, 10), (2, 20), (3, 30)]);
        let bytes = serde_ipld_dagcbor::to_vec(&pointers)?;

        let pointers_back: PointerSerializable<u32, u32> = serde_ipld_dagcbor::from_slice(&bytes)?;

        assert_eq!(pointers, pointers_back);

        Ok(())
    }
}
