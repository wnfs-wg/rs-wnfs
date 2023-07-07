use num_bigint_dig::BigUint;
use serde::{Deserializer, Serializer};

pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<BigUint, D::Error>
where
    D: Deserializer<'de>,
{
    let bytes: Vec<u8> = serde_bytes::deserialize(deserializer)?;
    Ok(BigUint::from_bytes_be(&bytes))
}

pub(crate) fn serialize<S>(uint: &BigUint, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serde_bytes::serialize(to_bytes_helper::<256>(uint).as_ref(), serializer)
}

pub(crate) fn to_bytes_helper<const N: usize>(state: &BigUint) -> [u8; N] {
    let vec = state.to_bytes_be();
    let mut bytes = [0u8; N];
    let zero_bytes = N - vec.len();
    bytes[zero_bytes..].copy_from_slice(&vec);
    bytes
}
