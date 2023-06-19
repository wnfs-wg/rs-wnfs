use num_bigint_dig::BigUint;
use serde::{Deserialize, Deserializer, Serializer};

pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<BigUint, D::Error>
where
    D: Deserializer<'de>,
{
    // TODO(matheus23): Pad to 256 bytes
    let bytes: Vec<u8> = Vec::deserialize(deserializer)?;
    Ok(BigUint::from_bytes_le(&bytes))
}

pub(crate) fn serialize<S>(uint: &BigUint, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_bytes(&to_bytes_helper(uint))
}

pub(crate) fn to_bytes_helper(state: &BigUint) -> [u8; 256] {
    let vec = state.to_bytes_le();
    let mut bytes = [0u8; 256];
    bytes[..vec.len()].copy_from_slice(&vec);
    bytes
}
