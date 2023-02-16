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
    /// let (name, forest, store, rng) = test_setup::init!(name, mut forest, mut store, mut rng);
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
            wnfs_common::MemoryBlockStore::new()
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
            let (name, mut forest, mut store, mut rng) = test_setup::init!(name, forest, store, rng);
            let file = $crate::private::PrivateFile::with_content(
                name,
                chrono::Utc::now(),
                $content,
                &mut forest,
                &mut store,
                &mut rng,
            )
            .await
            .unwrap();

            (file, (forest, store, rng))
        }};
        [ file ] => {{
            let (name, mut rng) = test_setup::init!(name, rng);
            let file = $crate::private::PrivateFile::new(name, chrono::Utc::now(), &mut rng);

            (file, rng)
        }}
    }

    pub(crate) use init;
    pub(crate) use private;
}
