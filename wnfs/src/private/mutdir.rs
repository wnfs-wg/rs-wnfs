use super::{PrivateDirectory, PrivateForest, PrivateNode, PrivateNodeHeader};
use crate::traits::Time;
use anyhow::Result;
use chrono::Utc;
use rand_core::RngCore;
use std::rc::Rc;
use wnfs_common::{BlockStore, HashOutput, Metadata};
use wnfs_namefilter::Namefilter;

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub struct MutablePrivateDirectory<'f, 'b, 'r, B: BlockStore, R: RngCore, T: Time = Utc> {
    pub forest: &'f mut Rc<PrivateForest>,
    pub store: &'b mut B,
    pub rng: &'r mut R,
    pub root: Rc<PrivateDirectory>,
    phantom: std::marker::PhantomData<T>,
}

//--------------------------------------------------------------------------------------------------
// Implementations
//--------------------------------------------------------------------------------------------------

impl<'f, 'b, 'r, B, R, T> MutablePrivateDirectory<'f, 'b, 'r, B, R, T>
where
    B: BlockStore,
    R: RngCore,
    T: Time,
{
    pub async fn new(
        forest: &'f mut Rc<PrivateForest>,
        store: &'b mut B,
        rng: &'r mut R,
    ) -> Result<MutablePrivateDirectory<'f, 'b, 'r, B, R, T>> {
        let root =
            PrivateDirectory::new_and_store(Namefilter::default(), T::now(), forest, store, rng)
                .await?;

        Ok(Self {
            forest,
            store,
            rng,
            root,
            phantom: std::marker::PhantomData,
        })
    }

    pub async fn with_seed(
        forest: &'f mut Rc<PrivateForest>,
        store: &'b mut B,
        rng: &'r mut R,
        ratchet_seed: HashOutput,
        inumber: HashOutput,
    ) -> Result<MutablePrivateDirectory<'f, 'b, 'r, B, R, T>> {
        let root = PrivateDirectory::new_with_seed_and_store(
            Namefilter::default(),
            T::now(),
            ratchet_seed,
            inumber,
            forest,
            store,
            rng,
        )
        .await?;

        Ok(Self {
            forest,
            store,
            rng,
            root,
            phantom: std::marker::PhantomData,
        })
    }

    #[inline]
    pub fn get_header(&self) -> &PrivateNodeHeader {
        &self.root.header
    }

    pub async fn lookup_node(
        &self,
        path_segment: &str,
        search_latest: bool,
    ) -> Result<Option<PrivateNode>> {
        self.root
            .lookup_node(path_segment, search_latest, self.forest, self.store)
            .await
    }

    pub async fn get_node(
        &self,
        path_segments: &[String],
        search_latest: bool,
    ) -> Result<Option<PrivateNode>> {
        self.root
            .get_node(path_segments, search_latest, self.forest, self.store)
            .await
    }

    pub async fn write(
        &mut self,
        path_segments: &[String],
        search_latest: bool,
        content: Vec<u8>,
    ) -> Result<()> {
        self.root
            .write(
                path_segments,
                search_latest,
                T::now(),
                content,
                self.forest,
                self.store,
                self.rng,
            )
            .await
    }

    pub async fn mkdir(&mut self, path_segments: &[String], search_latest: bool) -> Result<()> {
        self.root
            .mkdir(
                path_segments,
                search_latest,
                T::now(),
                self.forest,
                self.store,
                self.rng,
            )
            .await
    }

    pub async fn ls(
        &self,
        path_segments: &[String],
        search_latest: bool,
    ) -> Result<Vec<(String, Metadata)>> {
        self.root
            .ls(path_segments, search_latest, self.forest, self.store)
            .await
    }

    pub async fn rm(
        &mut self,
        path_segments: &[String],
        search_latest: bool,
    ) -> Result<PrivateNode> {
        self.root
            .rm(path_segments, search_latest, self.forest, self.store)
            .await
    }

    pub async fn basic_mv(
        &mut self,
        path_segments_from: &[String],
        path_segments_to: &[String],
        search_latest: bool,
    ) -> Result<()> {
        self.root
            .basic_mv(
                path_segments_from,
                path_segments_to,
                search_latest,
                T::now(),
                self.forest,
                self.store,
                self.rng,
            )
            .await
    }

    pub async fn cp(
        &mut self,
        path_segments_from: &[String],
        path_segments_to: &[String],
        search_latest: bool,
    ) -> Result<()> {
        self.root
            .cp(
                path_segments_from,
                path_segments_to,
                search_latest,
                T::now(),
                self.forest,
                self.store,
                self.rng,
            )
            .await
    }
}

//--------------------------------------------------------------------------------------------------
// Tests
//--------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::test_runner::{RngAlgorithm, TestRng};
    use wnfs_common::{utils, MemoryBlockStore};

    #[async_std::test]
    async fn can_create_directories_deterministically_with_user_provided_seeds() {
        let forest = &mut Rc::new(PrivateForest::new());
        let store = &mut MemoryBlockStore::new();
        let rng = &mut TestRng::deterministic_rng(RngAlgorithm::ChaCha);
        let ratchet_seed = utils::get_random_bytes::<32>(rng);
        let inumber = utils::get_random_bytes::<32>(rng);

        let dir1 = MutablePrivateDirectory::<_, _, Utc>::with_seed(
            forest,
            store,
            rng,
            ratchet_seed,
            inumber,
        )
        .await
        .unwrap();

        let dir1_header = dir1.get_header().clone();

        let dir2 = MutablePrivateDirectory::<_, _, Utc>::with_seed(
            forest,
            store,
            rng,
            ratchet_seed,
            inumber,
        )
        .await
        .unwrap();

        let dir2_header = dir2.get_header();

        assert_eq!(
            dir1_header.derive_temporal_key(),
            dir2_header.derive_temporal_key()
        );

        assert_eq!(
            dir1_header.get_saturated_name(),
            dir2_header.get_saturated_name()
        );
    }
}
