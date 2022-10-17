use std::rc::Rc;

use anyhow::{bail, Result};
use skip_ratchet::{ratchet::PreviousIterator, Ratchet};

use super::{PrivateDirectory, PrivateForest, PrivateNode, PrivateNodeHeader};

use crate::{BlockStore, FsError, PathNodes, PathNodesResult};

//--------------------------------------------------------------------------------------------------
// Type Definitions
//--------------------------------------------------------------------------------------------------

/// Represents the state of an iterator through the history
/// of a private node on a path relative to a root directory.
pub struct PrivateNodeOnPathHistory {
    /// Keep a reference to the version of the forest used upon construction.
    /// It could *technically* change what's behind a certain key in between
    /// previous node requests, this forces it to be consistent.
    forest: Rc<PrivateForest>,
    /// The history of each path segment leading up to the final node
    path: Vec<PathSegmentHistory>,
    /// The target node's history
    target: PrivateNodeHistory,
}

struct PathSegmentHistory {
    /// The directory that the history was originally created relative to.
    dir: Rc<PrivateDirectory>,
    /// The history of said directory.
    history: PrivateNodeHistory,
    /// The name of the child node to follow for history next.
    path_segment: String,
}

/// This represents the state of an iterator through the history of
/// only a single private node. It can only be constructed when you
/// know the past ratchet state of such a node.
pub struct PrivateNodeHistory {
    /// Keep a reference to the version of the forest used upon construction.
    /// It could *technically* change what's behind a certain key in between
    /// previous node requests, this forces it to be consistent.
    forest: Rc<PrivateForest>,
    /// Keep the original discrepancy budget for consistency & ease of use.
    discrepancy_budget: usize,
    /// The private node header is all we need to look up private nodes in the forest.
    /// This will always be the header of the *next* version after what's retrieved from
    /// the `ratchets` iterator.
    header: PrivateNodeHeader,
    /// The iterator for previous revision ratchets.
    ratchets: PreviousIterator,
}

impl PrivateNodeHistory {
    pub fn of(
        node: &PrivateNode,
        past_ratchet: &Ratchet,
        discrepancy_budget: usize,
        forest: Rc<PrivateForest>,
    ) -> Result<Self> {
        Self::from_header(
            node.get_header().clone(),
            past_ratchet,
            discrepancy_budget,
            forest,
        )
    }

    pub fn from_header(
        header: PrivateNodeHeader,
        past_ratchet: &Ratchet,
        discrepancy_budget: usize,
        forest: Rc<PrivateForest>,
    ) -> Result<Self> {
        let forest = Rc::clone(&forest);
        let ratchets = PreviousIterator::new(past_ratchet, &header.ratchet, discrepancy_budget)
            .map_err(|err| FsError::PreviousError(err))?;
        Ok(PrivateNodeHistory {
            forest,
            discrepancy_budget,
            header,
            ratchets,
        })
    }

    pub async fn previous_node<B: BlockStore>(&mut self, store: &B) -> Result<Option<PrivateNode>> {
        match self.ratchets.next() {
            None => Ok(None),
            Some(previous_ratchet) => {
                self.header.ratchet = previous_ratchet;
                self.forest
                    .get(&self.header.get_private_ref()?, store)
                    .await
            }
        }
    }

    pub async fn previous_dir<B: BlockStore>(
        &mut self,
        store: &B,
    ) -> Result<Option<Rc<PrivateDirectory>>> {
        match self.previous_node(store).await? {
            Some(PrivateNode::Dir(dir)) => Ok(Some(dir)),
            _ => Ok(None),
        }
    }
}

impl PrivateNodeOnPathHistory {
    pub async fn of<B: BlockStore>(
        directory: Rc<PrivateDirectory>,
        past_ratchet: &Ratchet,
        discrepancy_budget: usize,
        path_segments: &[String],
        search_latest: bool,
        forest: Rc<PrivateForest>,
        store: &B,
    ) -> Result<PrivateNodeOnPathHistory> {
        // To get the history on a node on a path from a given directory that we
        // know its newest and oldest ratchet of, we need to generate
        // `PrivateNodeHistory`s for each path segment up to the last node.
        //
        // This is what this function is doing, it constructs the `PrivateNodeOnPathHistory`.
        //
        // Stepping that history forward is then done in `PrivateNodeOnPathHistory#previous`.

        let new_ratchet = directory.header.ratchet.clone();

        let (last, path_segments) = match path_segments.split_last() {
            None => {
                return Ok(PrivateNodeOnPathHistory {
                    forest: Rc::clone(&forest),
                    path: Vec::with_capacity(0),
                    target: PrivateNodeHistory::of(
                        &PrivateNode::Dir(directory),
                        past_ratchet,
                        discrepancy_budget,
                        Rc::clone(&forest),
                    )?,
                });
            }
            Some(split) => split,
        };

        let path_nodes = match directory
            .get_path_nodes(path_segments, false, &*forest, store)
            .await?
        {
            PathNodesResult::Complete(path_nodes) => path_nodes,
            PathNodesResult::MissingLink(_, _) => bail!(FsError::NotFound),
            PathNodesResult::NotADirectory(_, _) => bail!(FsError::NotADirectory),
        };

        // TODO(matheus23) refactor using let-else once rust stable 1.65 released (Nov 3rd)
        let target = match path_nodes
            .tail
            .lookup_node(last, false, &*forest, store)
            .await?
        {
            Some(target) => target,
            None => bail!(FsError::NotFound),
        };

        let target_latest = if search_latest {
            target.search_latest(&*forest, store).await?
        } else {
            target.clone()
        };

        let target_history = PrivateNodeHistory::of(
            &target_latest,
            &target.get_header().ratchet,
            discrepancy_budget,
            Rc::clone(&forest),
        )?;

        let mut previous_iter = PrivateNodeOnPathHistory {
            forest: Rc::clone(&forest),
            path: Vec::with_capacity(path_nodes.len() + 1),
            target: target_history,
        };

        let PathNodes { mut path, tail } = path_nodes;

        path.push((tail, last.to_string()));

        for (dir, path_segment) in path {
            previous_iter.path.push(PathSegmentHistory {
                dir: Rc::clone(&dir),
                history: PrivateNodeHistory::of(
                    &PrivateNode::Dir(Rc::clone(&dir)),
                    &dir.header.ratchet,
                    discrepancy_budget,
                    Rc::clone(&forest),
                )?,
                path_segment,
            });
        }

        // For the first part of the path, we specifically set the history ourselves,
        // because we've had `past_ratchet` passed in from the outside.

        previous_iter.path[0].history.ratchets =
            PreviousIterator::new(past_ratchet, &new_ratchet, discrepancy_budget)
                .map_err(|err| FsError::PreviousError(err))?;

        Ok(previous_iter)
    }

    pub async fn previous<B: BlockStore>(&mut self, store: &B) -> Result<Option<PrivateNode>> {
        // Finding the previous revision of a node works by trying to get
        // the previous revision of the path elements starting on the deepest
        // path node working upwards, in case the history of lower nodes
        // have been exhausted.
        //
        // Once another history entry on the path has been found, we proceed
        // to work back trying to construct new history entries by going downwards
        // on the same path from an older root revision, until we've completed
        // the whole path and found new history entries in every segment.

        if let Some(node) = self.target.previous_node(store).await? {
            return Ok(Some(node));
        }

        let mut working_stack: Vec<(Rc<PrivateDirectory>, String)> =
            Vec::with_capacity(self.path.len());

        loop {
            // Pop elements off the end of the path
            if let Some(mut segment) = self.path.pop() {
                // Try to find a path segment for which we have previous history entries
                if let Some(prev) = segment.history.previous_dir(store).await? {
                    segment.dir = prev;
                    self.path.push(segment);
                    // Once found, we can continue.
                    break;
                }

                working_stack.push((segment.dir, segment.path_segment));
            } else {
                // We have exhausted all histories of all path segments.
                // There's no way we can produce more history entries.
                return Ok(None);
            }
        }

        // Work downwards from the previous history entry of a path segment we found
        for (directory, path_segment) in working_stack {
            let ancestor = self.path.last().unwrap();

            // Go down from the older ancestor directory parallel to the new revision's path
            // TODO(matheus23) refactor using let-else once rust stable 1.65 released (Nov 3rd)
            let older_directory = match ancestor
                .dir
                .lookup_node(&ancestor.path_segment, false, &self.forest, store)
                .await?
            {
                Some(PrivateNode::Dir(older_directory)) => older_directory,
                _ => return Ok(None),
            };

            let mut directory_history = PrivateNodeHistory::of(
                &PrivateNode::Dir(directory),
                &older_directory.header.ratchet,
                discrepancy_budget,
                Rc::clone(&self.forest),
            )?;

            // We need to find the in-between history entry! See the test case `previous_with_multiple_child_changes`.
            // TODO(matheus23) refactor using let-else once rust stable 1.65 released (Nov 3rd)
            let directory_prev = match directory_history.previous_dir(store).await? {
                Some(dir) => dir,
                _ => return Ok(None),
            };

            self.path.push(PathSegmentHistory {
                dir: directory_prev,
                history: directory_history,
                path_segment,
            });
        }

        let ancestor = self.path.last().unwrap();

        // TODO(matheus23) refactor using let-else once rust stable 1.65 released (Nov 3rd)
        let older_node = match ancestor
            .dir
            .lookup_node(&ancestor.path_segment, false, forest, store)
            .await?
        {
            Some(older_node) => older_node,
            None => return Ok(None),
        };

        self.target = PrivateNodeHistory::from_header(
            self.target.header.clone(),
            &older_node.get_header().ratchet,
            discrepancy_budget,
            Rc::clone(&self.forest),
        )?;

        self.target.previous_node(store).await
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

        let mut iterator = PrivateNodeOnPathHistory::of(
            root_dir,
            &past_ratchet,
            discrepancy_budget,
            &[],
            true,
            &*hamt,
            store,
        )
        .await
        .unwrap();

        assert!(iterator.previous(store).await.unwrap().is_some());
        assert!(iterator.previous(store).await.unwrap().is_some());
        assert!(iterator.previous(store).await.unwrap().is_none());
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

        let mut iterator = PrivateNodeOnPathHistory::of(
            root_dir,
            &past_ratchet,
            discrepancy_budget,
            &path,
            true,
            &*hamt,
            store,
        )
        .await
        .unwrap();

        assert_eq!(
            iterator
                .previous(store)
                .await
                .unwrap()
                .unwrap()
                .as_file()
                .unwrap()
                .content,
            b"Hi".to_vec()
        );

        assert!(iterator.previous(store).await.unwrap().is_none());
    }

    /// This test will generate the following file system structure:
    ///
    /// (horizontal = time series, vertical = hierarchy)
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
    ///
    /// This is testing a case where the file system wasn't rooted completely.
    /// Imagine someone wrote the `Notes.md` file with only access up to `Root/Docs`.
    /// The file system diagram looks like this:
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

        let docs_dir = docs_dir.unwrap().as_dir().unwrap();

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

        let mut iterator = PrivateNodeOnPathHistory::of(
            root_dir,
            &past_ratchet,
            discrepancy_budget,
            &path,
            true,
            &*hamt,
            store,
        )
        .await
        .unwrap();

        assert_eq!(
            iterator
                .previous(store)
                .await
                .unwrap()
                .unwrap()
                .as_file()
                .unwrap()
                .content,
            b"Hi".to_vec()
        );

        assert!(iterator.previous(store).await.unwrap().is_none());
    }

    /// This test will generate the following file system structure:
    ///
    /// (horizontal = time series, vertical = hierarchy)
    /// ```plain
    /// ┌────────────┐                              ┌────────────┐
    /// │            │                              │            │
    /// │    Root    ├─────────────────────────────►│    Root    │
    /// │            │                              │            │
    /// └─────┬──────┘                              └─────┬──────┘
    ///       │                                           │
    ///       │                                           │
    ///       ▼                                           ▼
    /// ┌────────────┐        ┌────────────┐        ┌────────────┐
    /// │            │        │            │        │            │
    /// │    Docs    ├───────►│    Docs    ├───────►│    Docs    │
    /// │            │        │            │        │            │
    /// └─────┬──────┘        └─────┬──────┘        └─────┬──────┘
    ///       │                     │                     │
    ///       │                     │                     │
    ///       ▼                     ▼                     ▼
    /// ┌────────────┐        ┌────────────┐        ┌────────────┐
    /// │            │        │            │        │            │
    /// │  Notes.md  ├───────►│  Notes.md  ├───────►│  Notes.md  │
    /// │            │        │            │        │            │
    /// └────────────┘        └────────────┘        └────────────┘
    /// ```
    ///
    /// This case happens when someone who only has access up to
    /// `Root/Docs` writes two revisions of `Notes.md` and
    /// is later rooted by another peer that has full root access.
    #[async_std::test]
    async fn previous_with_multiple_child_changes() {
        let rng = &mut TestRng::deterministic_rng(RngAlgorithm::ChaCha);
        let store = &mut MemoryBlockStore::default();
        let hamt = Rc::new(PrivateForest::new());
        let root_dir = Rc::new(PrivateDirectory::new(
            Namefilter::default(),
            Utc::now(),
            rng,
        ));
        let discrepancy_budget = 1_000_000;
        let path = ["Docs".into(), "Notes.md".into()];

        let PrivateOpResult { root_dir, hamt, .. } = root_dir
            .write(&path, true, Utc::now(), b"rev 0".to_vec(), hamt, store, rng)
            .await
            .unwrap();

        let past_ratchet = root_dir.header.ratchet.clone();

        let PrivateOpResult {
            root_dir,
            hamt,
            result: docs_dir,
            ..
        } = root_dir
            .get_node(&["Docs".into()], true, hamt, store)
            .await
            .unwrap();

        let docs_dir = docs_dir.unwrap().as_dir().unwrap();

        let PrivateOpResult { hamt, .. } = docs_dir
            .write(
                &["Notes.md".into()],
                true,
                Utc::now(),
                b"rev 1".to_vec(),
                hamt,
                store,
                rng,
            )
            .await
            .unwrap();

        let PrivateOpResult { root_dir, hamt, .. } = root_dir
            .write(&path, true, Utc::now(), b"rev 2".to_vec(), hamt, store, rng)
            .await
            .unwrap();

        let mut iterator = PrivateNodeOnPathHistory::of(
            root_dir,
            &past_ratchet,
            discrepancy_budget,
            &path,
            true,
            &*hamt,
            store,
        )
        .await
        .unwrap();

        assert_eq!(
            iterator
                .previous(store)
                .await
                .unwrap()
                .unwrap()
                .as_file()
                .unwrap()
                .content,
            b"rev 1".to_vec()
        );

        assert_eq!(
            iterator
                .previous(store)
                .await
                .unwrap()
                .unwrap()
                .as_file()
                .unwrap()
                .content,
            b"rev 0".to_vec()
        );

        assert!(iterator.previous(store).await.unwrap().is_none());
    }

    /// This test will generate the following file system structure:
    ///
    /// (horizontal = time series, vertical = hierarchy)
    /// ```plain
    /// ┌────────────┐    ┌────────────┐    ┌────────────┐
    /// │            │    │            │    │            │
    /// │    Root    ├───►│    Root    ├───►│    Root    │
    /// │            │    │            │    │            │
    /// └─────┬──────┘    └─────┬──────┘    └─────┬──────┘
    ///       │                 │                 │
    ///       │ ┌───────────────┘                 │
    ///       ▼ ▼                                 ▼
    /// ┌────────────┐                      ┌────────────┐
    /// │            │                      │            │
    /// │    Docs    ├─────────────────────►│    Docs    │
    /// │            │                      │            │
    /// └─────┬──────┘                      └─────┬──────┘
    ///       │                                   │
    ///       │                                   │
    ///       ▼                                   ▼
    /// ┌────────────┐                      ┌────────────┐
    /// │            │                      │            │
    /// │  Notes.md  ├─────────────────────►│  Notes.md  │
    /// │            │                      │            │
    /// └────────────┘                      └────────────┘
    /// ```
    ///
    /// This scenario may happen very commonly when things are
    /// written to the root directory that aren't related to
    /// the path that is looked at for its history.
    #[async_std::test]
    async fn previous_with_unrelated_changes() {
        let rng = &mut TestRng::deterministic_rng(RngAlgorithm::ChaCha);
        let store = &mut MemoryBlockStore::default();
        let hamt = Rc::new(PrivateForest::new());
        let root_dir = Rc::new(PrivateDirectory::new(
            Namefilter::default(),
            Utc::now(),
            rng,
        ));
        let discrepancy_budget = 1_000_000;
        let path = ["Docs".into(), "Notes.md".into()];

        let PrivateOpResult { root_dir, hamt, .. } = root_dir
            .write(&path, true, Utc::now(), b"rev 0".to_vec(), hamt, store, rng)
            .await
            .unwrap();

        let past_ratchet = root_dir.header.ratchet.clone();

        let root_dir = {
            let mut tmp = (*root_dir).clone();
            tmp.advance_ratchet();
            Rc::new(tmp)
        };

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

        let PrivateOpResult { root_dir, hamt, .. } = root_dir
            .write(&path, true, Utc::now(), b"rev 1".to_vec(), hamt, store, rng)
            .await
            .unwrap();

        assert_eq!(
            root_dir.header.ratchet.compare(&past_ratchet, 100).unwrap(),
            2
        );

        let mut iterator = PrivateNodeOnPathHistory::of(
            root_dir,
            &past_ratchet,
            discrepancy_budget,
            &path,
            true,
            &*hamt,
            store,
        )
        .await
        .unwrap();

        assert_eq!(
            iterator
                .previous(store)
                .await
                .unwrap()
                .unwrap()
                .as_file()
                .unwrap()
                .content,
            b"rev 0".to_vec()
        );

        assert!(iterator.previous(store).await.unwrap().is_none());
    }
}
