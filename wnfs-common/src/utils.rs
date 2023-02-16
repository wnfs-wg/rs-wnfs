use crate::{error, FsError};
use anyhow::Result;
use futures::{AsyncRead, AsyncReadExt};
#[cfg(any(test, feature = "test_strategies"))]
use proptest::{
    strategy::{Strategy, ValueTree},
    test_runner::TestRunner,
};
use rand_core::RngCore;
use serde::de::Visitor;
use std::fmt;

//--------------------------------------------------------------------------------------------------
// Type Definitions
//--------------------------------------------------------------------------------------------------

pub struct ByteArrayVisitor<const N: usize>;

#[cfg(any(test, feature = "test_strategies"))]
pub trait Sampleable {
    type Value;
    fn sample(&self, runner: &mut TestRunner) -> Self::Value;
}

//--------------------------------------------------------------------------------------------------
// Implementations
//--------------------------------------------------------------------------------------------------

impl<'de, const N: usize> Visitor<'de> for ByteArrayVisitor<N> {
    type Value = [u8; N];

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "a byte array of length {N}")
    }

    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let bytes: [u8; N] = v.try_into().map_err(E::custom)?;
        Ok(bytes)
    }
}

#[cfg(any(test, feature = "test_strategies"))]
impl<V, S> Sampleable for S
where
    S: Strategy<Value = V>,
{
    type Value = V;

    fn sample(&self, runner: &mut TestRunner) -> Self::Value {
        self.new_tree(runner)
            .expect("Couldn't generate test value")
            .current()
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

pub async fn read_fully(
    stream: &mut (impl AsyncRead + Unpin),
    buffer: &mut [u8],
) -> Result<(usize, bool)> {
    let mut bytes_read = 0;
    let mut done = false;
    loop {
        let bytes_read_in_iteration = stream.read(&mut buffer[bytes_read..]).await?;

        bytes_read += bytes_read_in_iteration;

        if bytes_read_in_iteration == 0 {
            done = true;
            break;
        }

        if bytes_read == buffer.len() {
            break;
        }
    }
    Ok((bytes_read, done))
}

/// Generates a random byte array of the given length.
///
/// # Examples
///
/// ```
/// use rand::thread_rng;
/// use wnfs::utils;
///
/// let rng = &mut thread_rng();
/// let bytes = utils::get_random_bytes::<32>(rng);
///
/// assert_eq!(bytes.len(), 32);
/// ```
pub fn get_random_bytes<const N: usize>(rng: &mut impl RngCore) -> [u8; N] {
    let mut bytes = [0u8; N];
    rng.fill_bytes(&mut bytes);
    bytes
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
