use crate::error::FsError;
use anyhow::Result;
use wnfs_common::utils::{error, ByteArrayVisitor};

//--------------------------------------------------------------------------------------------------
// Functions
//--------------------------------------------------------------------------------------------------

pub(crate) fn split_last(path_segments: &[String]) -> Result<(&[String], &String)> {
    match path_segments.split_last() {
        Some((last, rest)) => Ok((rest, last)),
        None => error(FsError::InvalidPath),
    }
}

/// Deserialize a constant-size slice as a byte array in serde's data model,
/// instead of serde's default, which is an array of integers.
///
/// This function specifically only works for 32-byte slices as they're quite common
/// and can be used with serde's #[serde(deserialize_with = "...")] field parameter.
pub fn deserialize_byte_slice32<'de, D>(deserializer: D) -> Result<[u8; 32], D::Error>
where
    D: serde::Deserializer<'de>,
{
    deserializer.deserialize_bytes(ByteArrayVisitor::<32>)
}

/// Serialize a constant-size slice as a byte array in serde's data model,
/// instead of serde's default, which is an array of integers.
///
/// This function specifically only works for 32-byte slices as they're quite common
/// and can be used with serde's #[serde(serialize_with = "...")] field parameter.
pub fn serialize_byte_slice32<S>(slice: &[u8; 32], serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_bytes(slice)
}

//--------------------------------------------------------------------------------------------------
// Tests
//--------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_last_splits_path_segments_into_tail_and_the_rest() {
        let path_segments = ["a".into(), "b".into(), "c".into()];
        let (rest, last) = split_last(&path_segments).unwrap();
        assert_eq!(rest, &["a", "b"]);
        assert_eq!(last, &"c");
    }
}
