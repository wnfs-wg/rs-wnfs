use super::{hamt::HamtForest, traits::PrivateForest};
use crate::error::{FsError, VerificationError};
use anyhow::Result;
use async_trait::async_trait;
use libipld::Cid;
use sha3::Sha3_256;
use std::{
    collections::{BTreeSet, HashMap},
    rc::Rc,
};
use wnfs_common::{BlockStore, HashOutput};
use wnfs_hamt::{Hasher, Pair};
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
        self: &mut Self,
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

    pub async fn verify_against_previous_state(
        &self,
        previous: &HamtForest,
        store: &mut impl BlockStore,
    ) -> Result<()> {
        let setup = self.forest.get_accumulator_setup();
        if setup != previous.get_accumulator_setup() {
            return Err(FsError::IncompatibleAccumulatorSetups.into());
        }

        self.proofs.verify_proofs(setup)?;

        for change in self.forest.diff(previous, store).await? {
            if !self.proofs.proofs_by_commitment.contains_key(&change.key) {
                return Err(VerificationError::UnverifiedWrite(Sha3_256::hash(&change.key)).into());
            }
        }

        Ok(())
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
        self: &mut Self,
        name: &'a Name,
        values: impl IntoIterator<Item = Cid>,
        store: &mut impl BlockStore,
    ) -> Result<&'a NameAccumulator> {
        let ProvingHamtForest { forest, proofs } = self;

        proofs.add_and_prove_name(&name, forest.get_accumulator_setup())?;

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
        self: &mut Self,
        name_hash: &HashOutput,
        store: &mut impl BlockStore,
    ) -> Result<Option<Pair<NameAccumulator, BTreeSet<Cid>>>> {
        // TODO(matheus23) implement proofs for removal
        // Would need a refactor though:
        // name_hash: &HashOutput is not enough info, we need the &Name.
        Rc::make_mut(&mut self.forest)
            .remove_encrypted(name_hash, store)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::ProvingHamtForest;
    use crate::private::{
        forest::{hamt::HamtForest, traits::PrivateForest},
        PrivateDirectory,
    };
    use chrono::Utc;
    use rand::thread_rng;
    use std::rc::Rc;
    use wnfs_common::MemoryBlockStore;

    #[async_std::test]
    async fn proving_forest_example() {
        let store = &mut MemoryBlockStore::new();
        let rng = &mut thread_rng();
        let forest = &mut Rc::new(HamtForest::new_rsa_2048(rng));
        let root_dir = &mut PrivateDirectory::new_and_store(
            &forest.empty_name(),
            Utc::now(),
            forest,
            store,
            rng,
        )
        .await
        .unwrap();

        root_dir
            .write(
                &["Docs".into(), "test.txt".into()],
                true,
                Utc::now(),
                b"Hello, World!".to_vec(),
                forest,
                store,
                rng,
            )
            .await
            .unwrap();

        root_dir.store(forest, store, rng).await.unwrap();

        let old_forest = Rc::clone(forest);

        let forest = &mut ProvingHamtForest::new(Rc::clone(forest));

        root_dir
            .write(
                &["Docs".into(), "test.txt".into()],
                true,
                Utc::now(),
                b"Something else".to_vec(),
                forest,
                store,
                rng,
            )
            .await
            .unwrap();

        root_dir.store(forest, store, rng).await.unwrap();

        assert!(forest
            .verify_against_previous_state(&old_forest, store)
            .await
            .is_ok());
    }
}
