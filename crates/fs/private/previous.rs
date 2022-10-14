use std::rc::Rc;

use anyhow::{bail, Result};
use skip_ratchet::{ratchet::PreviousIterator, Ratchet};

use super::{PrivateDirectory, PrivateForest, PrivateNode, PrivateNodeHeader};

use crate::{BlockStore, FsError, PathNodes, PathNodesResult};

//--------------------------------------------------------------------------------------------------
// Type Definitions
//--------------------------------------------------------------------------------------------------
pub struct PrviateNodeHistory {
    header: PrivateNodeHeader,
    ratchets: PreviousIterator,
}

pub struct PrivateNodeOnPathHistory {
    // TODO(matheus23) Make triple a struct?
    path: Vec<(Rc<PrivateDirectory>, PrviateNodeHistory, String)>,
    tail: PrviateNodeHistory,
}

impl PrviateNodeHistory {
    pub fn of(
        node: &PrivateNode,
        past_ratchet: &Ratchet,
        discrepancy_budget: usize,
    ) -> Result<Self> {
        Self::from_header(node.get_header().clone(), past_ratchet, discrepancy_budget)
    }

    pub fn from_header(
        header: PrivateNodeHeader,
        past_ratchet: &Ratchet,
        discrepancy_budget: usize,
    ) -> Result<Self> {
        let ratchets = PreviousIterator::new(past_ratchet, &header.ratchet, discrepancy_budget)
            .map_err(|err| FsError::PreviousError(err))?;
        Ok(PrviateNodeHistory { header, ratchets })
    }

    async fn previous_node<B: BlockStore>(
        &mut self,
        forest: &PrivateForest,
        store: &B,
    ) -> Result<Option<PrivateNode>> {
        match self.ratchets.next() {
            None => Ok(None),
            Some(previous_ratchet) => {
                self.header.ratchet = previous_ratchet;
                forest.get(&self.header.get_private_ref()?, store).await
            }
        }
    }

    async fn previous_dir<B: BlockStore>(
        &mut self,
        forest: &PrivateForest,
        store: &B,
    ) -> Result<Option<Rc<PrivateDirectory>>> {
        match self.previous_node(forest, store).await? {
            Some(PrivateNode::Dir(dir)) => Ok(Some(dir)),
            _ => Ok(None),
        }
    }
}

impl PrivateNodeOnPathHistory {
    pub async fn previous_of<B: BlockStore>(
        directory: Rc<PrivateDirectory>,
        path_segments: &[String],
        search_latest: bool,
        forest: &PrivateForest,
        store: &B,
        past_ratchet: &Ratchet,
        discrepancy_budget: usize,
    ) -> Result<PrivateNodeOnPathHistory> {
        let new_ratchet = directory.header.ratchet.clone();

        let (last, path_segments) = match path_segments.split_last() {
            None => {
                return Ok(PrivateNodeOnPathHistory {
                    path: Vec::with_capacity(0),
                    tail: PrviateNodeHistory::of(
                        &PrivateNode::Dir(directory),
                        past_ratchet,
                        discrepancy_budget,
                    )?,
                });
            }
            Some(split) => split,
        };

        let path_nodes = match directory
            .get_path_nodes(path_segments, false, forest, store)
            .await?
        {
            PathNodesResult::Complete(path_nodes) => path_nodes,
            PathNodesResult::MissingLink(_, _) => bail!(FsError::NotFound),
            PathNodesResult::NotADirectory(_, _) => bail!(FsError::NotADirectory),
        };

        let target = match path_nodes
            .tail
            .lookup_node(last, false, forest, store)
            .await?
        {
            Some(target) => target,
            None => bail!(FsError::NotFound),
        };

        let target_clone = target.clone();

        let target_latest = if search_latest {
            target_clone.search_latest(forest, store).await?
        } else {
            target.clone()
        };

        let target_ratchets = PrviateNodeHistory::of(
            &target_latest,
            &target.get_header().ratchet,
            discrepancy_budget,
        )?;

        let mut previous_iter = PrivateNodeOnPathHistory {
            path: Vec::with_capacity(path_nodes.len() + 1),
            tail: target_ratchets,
        };

        let PathNodes { mut path, tail } = path_nodes;

        path.push((tail, last.to_string()));

        for (dir, path_segment) in path {
            previous_iter.path.push((
                Rc::clone(&dir),
                PrviateNodeHistory::of(
                    &PrivateNode::Dir(Rc::clone(&dir)),
                    &dir.header.ratchet,
                    discrepancy_budget,
                )?,
                path_segment,
            ));
        }

        let ratchets = PreviousIterator::new(past_ratchet, &new_ratchet, discrepancy_budget)
            .map_err(|err| FsError::PreviousError(err))?;

        previous_iter.path[0].1.ratchets = ratchets;

        Ok(previous_iter)
    }

    pub async fn previous<B: BlockStore>(
        &mut self,
        forest: &PrivateForest,
        store: &B,
        discrepancy_budget: usize,
    ) -> Result<Option<PrivateNode>> {
        if let Some(node) = self.tail.previous_node(forest, store).await? {
            return Ok(Some(node));
        }

        let mut working_stack: Vec<(Rc<PrivateDirectory>, String)> =
            Vec::with_capacity(self.path.len());

        loop {
            if let Some((dir, mut previous, path_segment)) = self.path.pop() {
                if let Some(prev) = previous.previous_dir(forest, store).await? {
                    self.path.push((prev, previous, path_segment));
                    break;
                }

                working_stack.push((dir, path_segment));
            } else {
                return Ok(None);
            }
        }

        for (directory, path_segment) in working_stack {
            let (_, ancestor_previous, ancestor_segment) = self.path.last_mut().unwrap();

            let ancestor_prev_dir = match ancestor_previous.previous_dir(forest, store).await? {
                Some(dir) => dir,
                None => return Ok(None),
            };

            let older_directory = match ancestor_prev_dir
                .lookup_node(ancestor_segment, false, forest, store)
                .await?
            {
                Some(PrivateNode::Dir(older_directory)) => older_directory,
                _ => return Ok(None),
            };

            self.path.push((
                Rc::clone(&older_directory),
                PrviateNodeHistory::of(
                    &PrivateNode::Dir(directory),
                    &older_directory.header.ratchet,
                    discrepancy_budget,
                )?,
                path_segment,
            ));
        }

        let (ancestor_dir, _, ancestor_segment) = self.path.last().unwrap();

        let older_node = match ancestor_dir
            .lookup_node(ancestor_segment, false, forest, store)
            .await?
        {
            Some(older_node) => older_node,
            None => todo!(),
        };

        self.tail = PrviateNodeHistory::from_header(
            self.tail.header.clone(),
            &older_node.get_header().ratchet,
            discrepancy_budget,
        )?;

        self.tail.previous_node(forest, store).await
    }
}

//--------------------------------------------------------------------------------------------------
// Tests
//--------------------------------------------------------------------------------------------------

#[cfg(test)]
mod private_history_tests {

    use super::*;
    use crate::{
        private::{namefilter::Namefilter, PrivateDirectory, PrivateOpResult},
        MemoryBlockStore,
    };
    use chrono::Utc;
    use proptest::test_runner::{RngAlgorithm, TestRng};

    #[async_std::test]
    async fn previous_of_root_node() {
        let rng = &mut TestRng::deterministic_rng(RngAlgorithm::ChaCha);
        let store = &mut MemoryBlockStore::default();
        let hamt = Rc::new(PrivateForest::new());
        let root_dir = Rc::new(PrivateDirectory::new(
            Namefilter::default(),
            Utc::now(),
            rng,
        ));
        let hamt = hamt
            .set(
                root_dir.header.get_saturated_name(),
                &root_dir.header.get_private_ref().unwrap(),
                &PrivateNode::Dir(Rc::clone(&root_dir)),
                store,
                rng,
            )
            .await
            .unwrap();
        let discrepancy_budget = 1_000_000;
        let past_ratchet = root_dir.header.ratchet.clone();

        let PrivateOpResult { root_dir, hamt, .. } = root_dir
            .write(
                &["file.txt".into()],
                true,
                Utc::now(),
                b"file".to_vec(),
                hamt,
                store,
                rng,
            )
            .await
            .unwrap();

        let PrivateOpResult { root_dir, hamt, .. } = root_dir
            .mkdir(&["docs".into()], true, Utc::now(), hamt, store, rng)
            .await
            .unwrap();

        let mut iterator = PrivateNodeOnPathHistory::previous_of(
            root_dir,
            &[],
            true,
            &*hamt,
            store,
            &past_ratchet,
            discrepancy_budget,
        )
        .await
        .unwrap();

        let prev = iterator
            .previous(&*hamt, store, discrepancy_budget)
            .await
            .unwrap();

        assert!(prev.is_some());

        let prevprev = iterator
            .previous(&*hamt, store, discrepancy_budget)
            .await
            .unwrap();

        assert!(prevprev.is_some());

        let prevprevprev = iterator
            .previous(&*hamt, store, discrepancy_budget)
            .await
            .unwrap();

        assert!(prevprevprev.is_none());
    }

    /// This test will generate the following file system structure:
    ///
    /// (horizontal = time series, vertical = hierarchy)
    /// ```plain
    /// ┌────────────┐              ┌────────────┐              ┌────────────┐
    /// │            │              │            │              │            │
    /// │    Root    ├─────────────►│    Root    ├─────────────►│    Root    │
    /// │            │              │            │              │            │
    /// └────────────┘              └─────┬──────┘              └─────┬──────┘
    ///                                   │                           │
    ///                                   │                           │
    ///                                   ▼                           ▼
    ///                             ┌────────────┐              ┌────────────┐
    ///                             │            │              │            │
    ///                             │    Docs    ├─────────────►│    Docs    │
    ///                             │            │              │            │
    ///                             └─────┬──────┘              └─────┬──────┘
    ///                                   │                           │
    ///                                   │                           │
    ///                                   ▼                           ▼
    ///                             ┌────────────┐              ┌────────────┐
    ///                             │            │              │            │
    ///                             │  Notes.md  ├─────────────►│  Notes.md  │
    ///                             │            │              │            │
    ///                             └────────────┘              └────────────┘
    /// ```
    ///
    /// Then, given the skip ratchet for revision 0 of "Root" and revision 2 of "Root",
    /// it will ask for the backwards-history of the "Root/Docs/Notes.md" file.
    #[async_std::test]
    async fn previous_of_path() {
        let rng = &mut TestRng::deterministic_rng(RngAlgorithm::ChaCha);
        let store = &mut MemoryBlockStore::default();
        let hamt = Rc::new(PrivateForest::new());
        let root_dir = Rc::new(PrivateDirectory::new(
            Namefilter::default(),
            Utc::now(),
            rng,
        ));
        let hamt = hamt
            .set(
                root_dir.header.get_saturated_name(),
                &root_dir.header.get_private_ref().unwrap(),
                &PrivateNode::Dir(Rc::clone(&root_dir)),
                store,
                rng,
            )
            .await
            .unwrap();
        let discrepancy_budget = 1_000_000;
        let past_ratchet = root_dir.header.ratchet.clone();

        let path = ["Docs".into(), "Notes.md".into()];

        let PrivateOpResult { root_dir, hamt, .. } = root_dir
            .write(&path, true, Utc::now(), b"Hi".to_vec(), hamt, store, rng)
            .await
            .unwrap();

        let PrivateOpResult { root_dir, hamt, .. } = root_dir
            .write(&path, true, Utc::now(), b"World".to_vec(), hamt, store, rng)
            .await
            .unwrap();

        let mut iterator = PrivateNodeOnPathHistory::previous_of(
            root_dir,
            &path,
            true,
            &*hamt,
            store,
            &past_ratchet,
            discrepancy_budget,
        )
        .await
        .unwrap();

        assert_eq!(
            iterator
                .previous(&*hamt, store, discrepancy_budget)
                .await
                .unwrap()
                .unwrap()
                .as_file()
                .unwrap()
                .content,
            b"Hi".to_vec()
        );

        assert!(iterator
            .previous(&*hamt, store, discrepancy_budget)
            .await
            .unwrap()
            .is_none());
    }

    /// ```plain
    /// ┌────────────┐              ┌────────────┐
    /// │            │              │            │
    /// │    Root    ├─────────────►│    Root    │
    /// │            │              │            │
    /// └────────────┘              └─────┬──────┘
    ///                                   │
    ///                                   │
    ///                                   ▼
    ///                             ┌────────────┐              ┌────────────┐
    ///                             │            │              │            │
    ///                             │    Docs    ├─────────────►│    Docs    │
    ///                             │            │              │            │
    ///                             └─────┬──────┘              └─────┬──────┘
    ///                                   │                           │
    ///                                   │                           │
    ///                                   ▼                           ▼
    ///                             ┌────────────┐              ┌────────────┐
    ///                             │            │              │            │
    ///                             │  Notes.md  ├─────────────►│  Notes.md  │
    ///                             │            │              │            │
    ///                             └────────────┘              └────────────┘
    /// ```
    #[async_std::test]
    async fn previous_of_seeking() {
        let rng = &mut TestRng::deterministic_rng(RngAlgorithm::ChaCha);
        let store = &mut MemoryBlockStore::default();
        let hamt = Rc::new(PrivateForest::new());
        let root_dir = Rc::new(PrivateDirectory::new(
            Namefilter::default(),
            Utc::now(),
            rng,
        ));
        let hamt = hamt
            .set(
                root_dir.header.get_saturated_name(),
                &root_dir.header.get_private_ref().unwrap(),
                &PrivateNode::Dir(Rc::clone(&root_dir)),
                store,
                rng,
            )
            .await
            .unwrap();
        let discrepancy_budget = 1_000_000;
        let past_ratchet = root_dir.header.ratchet.clone();

        let path = ["Docs".into(), "Notes.md".into()];

        let PrivateOpResult { root_dir, hamt, .. } = root_dir
            .write(&path, true, Utc::now(), b"Hi".to_vec(), hamt, store, rng)
            .await
            .unwrap();

        let PrivateOpResult {
            root_dir,
            hamt,
            result: docs_dir,
            ..
        } = root_dir
            .get_node(&["Docs".into()], true, hamt, store)
            .await
            .unwrap();

        let docs_dir = if let Some(PrivateNode::Dir(docs_dir)) = docs_dir {
            docs_dir
        } else {
            unreachable!()
        };

        let PrivateOpResult { hamt, .. } = docs_dir
            .write(
                &["Notes.md".into()],
                true,
                Utc::now(),
                b"World".to_vec(),
                hamt,
                store,
                rng,
            )
            .await
            .unwrap();

        let mut iterator = PrivateNodeOnPathHistory::previous_of(
            root_dir,
            &path,
            true,
            &*hamt,
            store,
            &past_ratchet,
            discrepancy_budget,
        )
        .await
        .unwrap();

        assert_eq!(
            iterator
                .previous(&*hamt, store, discrepancy_budget)
                .await
                .unwrap()
                .unwrap()
                .as_file()
                .unwrap()
                .content,
            b"Hi".to_vec()
        );

        assert!(iterator
            .previous(&*hamt, store, discrepancy_budget)
            .await
            .unwrap()
            .is_none());
    }
}
