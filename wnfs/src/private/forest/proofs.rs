use super::{hamt::HamtForest, traits::PrivateForest};
use crate::error::{FsError, VerificationError};
use anyhow::Result;
use async_trait::async_trait;
use libipld::Cid;
use std::{
    collections::{BTreeSet, HashMap},
    rc::Rc,
};
use wnfs_common::{BlockStore, HashOutput};
use wnfs_hamt::Pair;
use wnfs_nameaccumulator::{
    AccumulatorSetup, BatchedProofPart, BatchedProofVerification, Name, NameAccumulator,
    UnbatchableProofPart,
};

// TODO(matheus23) add serialization (ideally with capsule)
// Ideally serialization deduplicates the base (first part of the value tuple).
// E.g. by serializing as a nested map (somewhat inverting the way the hash map is laid out in memory in this struct).
#[derive(Debug, Clone)]
pub struct ForestProofs {
    proofs_by_commitment: HashMap<NameAccumulator, (NameAccumulator, UnbatchableProofPart)>,
    batched_proof_part: BatchedProofPart,
}

#[derive(Debug, Clone)]
pub struct ProvingHamtForest {
    forest: Rc<HamtForest>,
    proofs: ForestProofs,
}

impl ForestProofs {
    pub fn new() -> Self {
        Self {
            proofs_by_commitment: HashMap::new(),
            batched_proof_part: BatchedProofPart::new(),
        }
    }

    pub fn add_and_prove_name<'a>(
        &mut self,
        name: &'a Name,
        setup: &AccumulatorSetup,
    ) -> Result<&'a NameAccumulator> {
        let (accumulated, proof) = name.as_proven_accumulator(setup);
        let base = NameAccumulator::from_state(proof.base.clone());
        let commitment = accumulated.clone();

        self.batched_proof_part.add(proof);
        self.proofs_by_commitment
            .insert(commitment, (base, proof.part.clone()));

        Ok(accumulated)
    }

    pub fn verify_proofs(&self, setup: &AccumulatorSetup) -> Result<()> {
        let mut verification = BatchedProofVerification::new(setup);

        for (commitment, (base, proof_part)) in self.proofs_by_commitment.iter() {
            verification.add(base, commitment, proof_part)?;
        }

        verification.verify(&self.batched_proof_part)
    }
}

impl ProvingHamtForest {
    pub fn new(forest: Rc<HamtForest>) -> Self {
        Self {
            forest,
            proofs: ForestProofs::new(),
        }
    }

    pub fn from_proofs(proofs: ForestProofs, forest: Rc<HamtForest>) -> Self {
        Self { forest, proofs }
    }

    pub async fn verify_against_previous_state(
        &self,
        previous: &HamtForest,
        allowed_bases: &BTreeSet<NameAccumulator>,
        store: &mut impl BlockStore,
    ) -> Result<()> {
        let setup = self.forest.get_accumulator_setup();
        if setup != previous.get_accumulator_setup() {
            return Err(FsError::IncompatibleAccumulatorSetups.into());
        }

        self.proofs.verify_proofs(setup)?;

        for change in self.forest.diff(previous, store).await? {
            // Verify that there exists a proof for the changed label & obtain the base that
            // was proven from.
            let Some((base, _)) = self.proofs.proofs_by_commitment.get(&change.key) else {
                return Err(VerificationError::UnverifiedWrite(format!("{:?}", change.key)).into());
            };

            // Verify that the base is allowed to be written to (e.g. has been signed by a party
            // with a signature chain up to the root owner).
            if !allowed_bases.contains(base) {
                return Err(VerificationError::WriteToDisallowedBase(format!("{base:?}")).into());
            }
        }

        Ok(())
    }
}

impl Default for ForestProofs {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait(?Send)]
impl PrivateForest for ProvingHamtForest {
    fn empty_name(&self) -> Name {
        self.forest.empty_name()
    }

    fn get_accumulator_setup(&self) -> &AccumulatorSetup {
        self.forest.get_accumulator_setup()
    }

    async fn has_by_hash(&self, name_hash: &HashOutput, store: &impl BlockStore) -> Result<bool> {
        self.forest.has_by_hash(name_hash, store).await
    }

    async fn has(&self, name: &Name, store: &impl BlockStore) -> Result<bool> {
        self.forest.has(name, store).await
    }

    async fn put_encrypted<'a>(
        &mut self,
        name: &'a Name,
        values: impl IntoIterator<Item = Cid>,
        store: &impl BlockStore,
    ) -> Result<&'a NameAccumulator> {
        let ProvingHamtForest { forest, proofs } = self;

        proofs.add_and_prove_name(name, forest.get_accumulator_setup())?;

        Rc::make_mut(forest)
            .put_encrypted(name, values, store)
            .await
    }

    async fn get_encrypted_by_hash<'b>(
        &'b self,
        name_hash: &HashOutput,
        store: &impl BlockStore,
    ) -> Result<Option<&'b BTreeSet<Cid>>> {
        self.forest.get_encrypted_by_hash(name_hash, store).await
    }

    async fn get_encrypted(
        &self,
        name: &Name,
        store: &impl BlockStore,
    ) -> Result<Option<&BTreeSet<Cid>>> {
        self.forest.get_encrypted(name, store).await
    }

    async fn remove_encrypted(
        &mut self,
        name: &Name,
        store: &impl BlockStore,
    ) -> Result<Option<Pair<NameAccumulator, BTreeSet<Cid>>>> {
        let ProvingHamtForest { forest, proofs } = self;

        proofs.add_and_prove_name(name, forest.get_accumulator_setup())?;

        Rc::make_mut(&mut self.forest)
            .remove_encrypted(name, store)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::{ForestProofs, ProvingHamtForest};
    use crate::private::{
        forest::{hamt::HamtForest, traits::PrivateForest},
        PrivateDirectory, PrivateNode, PrivateRef,
    };
    use anyhow::Result;
    use chrono::Utc;
    use libipld::Cid;
    use rand::thread_rng;
    use std::{collections::BTreeSet, rc::Rc};
    use wnfs_common::{BlockStore, MemoryBlockStore};
    use wnfs_nameaccumulator::NameAccumulator;

    #[async_std::test]
    async fn proving_forest_example() {
        // In between operations, Alice, Bob, and the persistence service would
        // exchange blocks via bitswap, car mirror or some other protocol.
        // Here we're simplifying by sharing a 'global' block store.
        let store = &mut MemoryBlockStore::new();

        // Alice creates a private file system with some data.
        // She shares read access with bob by securely transferring the read_ref.
        // She also publicly announces bob has access to a certain directory at allowed_write_name.
        let (old_forest_cid, read_ref, allowed_write_name) = alice_actions(store).await.unwrap();
        // Bob can take the read_ref and forest and create writes.
        // The output will be a new state of the forest as well as a set of proofs, proving
        // he didn't touch anything in the file system except what he was allowed to.
        let (proofs, new_forest_cid) = bob_actions(old_forest_cid, read_ref, store).await.unwrap();
        // A persistence service can check Bob's changes between the forests via his proofs.
        // The service does *not* need read access (it doesn't get to know the read_ref)
        // and it only gains limited information from the proofs from Bob.
        // The idea is that in practice the persistence service can accept updates from anyone
        // that were indirectly given access by Alice out-of-bounds, and it will store the updated
        // file system.
        persistence_service_actions(
            old_forest_cid,
            new_forest_cid,
            proofs,
            allowed_write_name,
            store,
        )
        .await
        .unwrap();
    }

    /// Alice creates a directory and gives access to it out to someone else.
    /// The returned PrivateRef gives read access and the NameAccumulator is
    /// supposed to be publicly signed for verifyable write access.
    async fn alice_actions(
        store: &mut impl BlockStore,
    ) -> Result<(Cid, PrivateRef, NameAccumulator)> {
        let rng = &mut thread_rng();
        let forest = &mut Rc::new(HamtForest::new_rsa_2048(rng));
        let root_dir = &mut PrivateDirectory::new_and_store(
            &forest.empty_name(),
            Utc::now(),
            forest,
            store,
            rng,
        )
        .await?;

        let private_ref = root_dir.store(forest, store, rng).await?;
        let cid = store.put_async_serializable(forest).await?;
        let setup = forest.get_accumulator_setup();
        let allowed_name = root_dir.header.name.as_accumulator(setup).clone();

        Ok((cid, private_ref, allowed_name))
    }

    /// Bob can take the forest, read data using the private ref
    /// and prove writes.
    async fn bob_actions(
        forest_cid: Cid,
        root_dir_ref: PrivateRef,
        store: &mut impl BlockStore,
    ) -> Result<(ForestProofs, Cid)> {
        let hamt_forest = store.get_deserializable(&forest_cid).await?;
        let mut forest = ProvingHamtForest::new(Rc::new(hamt_forest));
        let rng = &mut thread_rng();

        let mut root_node = PrivateNode::load(&root_dir_ref, &forest, store, None).await?;
        let root_dir = root_node.as_dir_mut()?;

        // Do arbitrary writes in any paths you have access to
        root_dir
            .write(
                &["Some".into(), "file.txt".into()],
                true,
                Utc::now(),
                b"Hello, Alice!".to_vec(),
                &mut forest,
                store,
                rng,
            )
            .await?;

        let ProvingHamtForest { forest, proofs } = forest;

        store.put_async_serializable(&forest).await?;

        Ok((proofs, forest_cid))
    }

    /// A persistence service can verify write proofs relative to a signed
    /// accumulator without read access.
    async fn persistence_service_actions(
        old_forest_cid: Cid,
        new_forest_cid: Cid,
        proofs: ForestProofs,
        allowed_access: NameAccumulator,
        store: &mut impl BlockStore,
    ) -> Result<()> {
        let old_forest = store.get_deserializable(&old_forest_cid).await?;
        let new_forest = store.get_deserializable(&new_forest_cid).await?;

        let forest = ProvingHamtForest::from_proofs(proofs, Rc::new(new_forest));

        forest
            .verify_against_previous_state(&old_forest, &BTreeSet::from([allowed_access]), store)
            .await
    }
}
