use crate::Big;
use serde::{Deserializer, Serializer};

pub(crate) fn deserialize<'de, B: Big, D>(deserializer: D) -> Result<B::Num, D::Error>
where
    D: Deserializer<'de>,
{
    let bytes: Vec<u8> = serde_bytes::deserialize(deserializer)?;
    Ok(B::from_bytes_be(&bytes))
}

pub(crate) fn serialize<B: Big, S>(uint: &B::Num, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serde_bytes::serialize(B::to_bytes_be::<256>(uint).as_ref(), serializer)
}
