use crate::{
    private::PrivateForest, utils, BlockStore, HashOutput, Namefilter, PrivateDirectory,
    PrivateNode,
};
use anyhow::Result;
use chrono::Utc;
use rand_core::RngCore;
use std::rc::Rc;

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub struct MutablePrivateDirectory<'b, 'r, B: BlockStore, R: RngCore> {
    pub store: &'b mut B,
    pub rng: &'r mut R,
    pub forest: Rc<PrivateForest>,
    pub root: Rc<PrivateDirectory>,
}

//--------------------------------------------------------------------------------------------------
// Implementations
//--------------------------------------------------------------------------------------------------

impl<'b, 'r, B, R> MutablePrivateDirectory<'b, 'r, B, R>
where
    B: BlockStore,
    R: RngCore,
{
    pub async fn new(
        forest: Rc<PrivateForest>,
        store: &'b mut B,
        rng: &'r mut R,
    ) -> Result<MutablePrivateDirectory<'b, 'r, B, R>> {
        let root = Rc::new(PrivateDirectory::new(
            Namefilter::default(),
            Utc::now(),
            rng,
        ));

        let forest = forest
            .put(
                root.header.get_saturated_name(),
                &root.header.get_private_ref(),
                &PrivateNode::Dir(Rc::clone(&root)),
                store,
                rng,
            )
            .await?;

        Ok(Self {
            store,
            rng,
            forest,
            root,
        })
    }

    pub async fn with_ratchet_seed(
        forest: Rc<PrivateForest>,
        store: &'b mut B,
        rng: &'r mut R,
        ratchet_seed: HashOutput,
    ) -> Result<MutablePrivateDirectory<'b, 'r, B, R>> {
        let root = Rc::new(PrivateDirectory::with_seed(
            Namefilter::default(),
            Utc::now(),
            ratchet_seed,
            utils::get_random_bytes(rng),
        ));

        let forest = forest
            .put(
                root.header.get_saturated_name(),
                &root.header.get_private_ref(),
                &PrivateNode::Dir(Rc::clone(&root)),
                store,
                rng,
            )
            .await?;

        Ok(Self {
            store,
            rng,
            forest,
            root,
        })
    }
}

//--------------------------------------------------------------------------------------------------
// Tests
//--------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use crate::utils::test_setup;

    use super::*;

    #[async_std::test]
    async fn can_create_an_empty_directory() {
        let (forest, store, rng) = test_setup::init!(forest, mut store, mut rng);
        let dir = MutablePrivateDirectory::new(forest, store, rng)
            .await
            .unwrap();

        println!("Directory: {:#?}", dir);
    }
}
