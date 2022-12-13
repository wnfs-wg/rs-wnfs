use crate::{error, FsError, HashOutput};
use anyhow::Result;
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

pub(crate) struct ByteArrayVisitor<const N: usize>;

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

pub(crate) fn split_last(path_segments: &[String]) -> Result<(&[String], &String)> {
    match path_segments.split_last() {
        Some((last, rest)) => Ok((rest, last)),
        None => error(FsError::InvalidPath),
    }
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

/// Creates a [`HashOutput`][HashOutput] ([u8; 32]) from a possibly incomplete slice.
///
/// If the slice is smaller than `HashOutput`, the remaining bytes are filled with zeros.
///
/// # Examples
///
/// ```
/// use wnfs::utils;
///
/// let digest = utils::make_digest(&[0xff, 0x22]);
///
/// assert_eq!(digest.len(), 32);
/// ```
///
/// [HashOutput]: crate::HashOutput
pub fn make_digest(bytes: &[u8]) -> HashOutput {
    let mut nibbles = [0u8; 32];
    nibbles[..bytes.len()].copy_from_slice(bytes);
    nibbles
}

//--------------------------------------------------------------------------------------------------
// Macros
//--------------------------------------------------------------------------------------------------

#[cfg(test)]
pub(crate) mod test_setup {
    #![allow(unused_macros)]
    #![allow(unused_imports)]

    /// This macro is useful for setting up intial states commonly used in tests.
    /// It lets you create a private forest, default namefilters, memory blockstore, etc. in a single line.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::utils::test_setup;
    /// let (name, forest, store, rng) = test_setup::init!(name, forest, mut store, mut rng);
    /// ```
    macro_rules! init {
        [ name ] => {
            $crate::private::Namefilter::default()
        };
        [ forest ] => {
            std::rc::Rc::new($crate::private::PrivateForest::new())
        };
        [ rng ] => {
            proptest::test_runner::TestRng::deterministic_rng(
                proptest::test_runner::RngAlgorithm::ChaCha
            )
        };
        [ runner ] => {
            proptest::test_runner::TestRunner::new(
                proptest::test_runner::Config::default()
            )
        };
        [ store ] => {
            $crate::MemoryBlockStore::new()
        };
        [ mut $name:ident ] => {
            &mut test_setup::init![ $name ]
        };
        [ $a0:ident $( $a1:ident )? $(, $b0:ident $( $b1:ident )? )+ ] => {(
            test_setup::init![ $a0 $( $a1 )? ] $(, test_setup::init![ $b0 $( $b1 )? ] )+
        )};
    }

    /// This macro is useful for creating intial private files and directories in tests.
    /// It lets you create a private directory and private files with content or without content.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::utils::test_setup;
    /// let (dir, _) = test_setup::private!(dir);
    /// let (file, _) = test_setup::private!(file);
    /// let (file, (forest, store, rng)) = test_setup::private!(file, vec![1, 2, 3]);
    /// ```
    macro_rules! private {
        [ dir ] => {{
            let (name, mut rng) = test_setup::init!(name, rng);
            let dir = Rc::new($crate::PrivateDirectory::new(name, chrono::Utc::now(), &mut rng));

            (dir, rng)
        }};
        [ file, $content:expr ] => {{
            let (name, forest, mut store, mut rng) = test_setup::init!(name, forest, store, rng);
            let (file, forest) = $crate::PrivateFile::with_content(
                name,
                chrono::Utc::now(),
                $content,
                forest,
                &mut store,
                &mut rng,
            )
            .await
            .unwrap();

            (file, (forest, store, rng))
        }};
        [ file ] => {{
            let (name, mut rng) = test_setup::init!(name, rng);
            let file = $crate::PrivateFile::new(name, chrono::Utc::now(), &mut rng);

            (file, rng)
        }}
    }

    pub(crate) use init;
    pub(crate) use private;
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
