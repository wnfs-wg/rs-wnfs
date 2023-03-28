use num_bigint_dig::BigUint;
use serde::{Deserialize, Deserializer, Serializer};

pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<BigUint, D::Error>
where
    D: Deserializer<'de>,
{
    let bytes: Vec<u8> = Vec::deserialize(deserializer)?;
    Ok(BigUint::from_bytes_le(&bytes))
}

pub(crate) fn serialize<S>(uint: &BigUint, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_bytes(&uint.to_bytes_le())
}
