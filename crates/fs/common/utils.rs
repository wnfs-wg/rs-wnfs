use anyhow::Result;
use serde::de::Visitor;
use std::fmt;

use crate::{error, FsError};

//--------------------------------------------------------------------------------------------------
// Type Definitions
//--------------------------------------------------------------------------------------------------

pub(crate) struct ByteArrayVisitor<const N: usize>;

#[cfg(test)]
pub(crate) struct Rand;

//--------------------------------------------------------------------------------------------------
// Implementations
//--------------------------------------------------------------------------------------------------

impl<'de, const N: usize> Visitor<'de> for ByteArrayVisitor<N> {
    type Value = [u8; N];

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "a byte array of length {}", N)
    }

    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let bytes: [u8; N] = v.try_into().map_err(E::custom)?;
        Ok(bytes)
    }
}

#[cfg(test)]
impl crate::private::Rng for Rand {
    fn random_bytes<const N: usize>() -> [u8; N] {
        use rand::RngCore;
        let mut bytes = [0u8; N];
        rand::thread_rng().fill_bytes(&mut bytes);
        bytes
    }
}

//--------------------------------------------------------------------------------------------------
// Functions
//--------------------------------------------------------------------------------------------------

pub fn split_last(path_segments: &[String]) -> Result<(&[String], &String)> {
    match path_segments.split_last() {
        Some((last, rest)) => Ok((rest, last)),
        None => error(FsError::InvalidPath),
    }
}

//--------------------------------------------------------------------------------------------------
// Tests
//--------------------------------------------------------------------------------------------------

#[cfg(test)]
mod public_directory_tests {
    use super::*;

    #[test]
    fn split_last_splits_path_segments_into_tail_and_the_rest() {
        let path_segments = ["a".into(), "b".into(), "c".into()];
        let (rest, last) = split_last(&path_segments).unwrap();
        assert_eq!(rest, &["a", "b"]);
        assert_eq!(last, &"c");
    }
}
