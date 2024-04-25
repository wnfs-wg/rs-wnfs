use super::{simple_string, Merge};
use proptest::{
    collection::vec,
    prop_oneof,
    sample::select,
    strategy::{BoxedStrategy, Just, Strategy},
};
use proptest_state_machine::strategy::ReferenceStateMachine;
use std::collections::{btree_map::Entry, BTreeMap, BTreeSet};

pub type MockMetadata = String;

pub type Path = Vec<String>;

#[derive(Debug, Clone, Default)]
pub struct FileSystemState {
    /// Represents a file multimap. Multiple values indicate concurrent versions
    pub files: BTreeMap<Path, BTreeSet<(MockMetadata, String)>>,
}

#[derive(Debug, Clone)]
pub enum FileSystemOp {
    Write(Path, (MockMetadata, String)),
    Remove(Path),
}

impl FileSystemState {
    pub fn all_directories<E: Extend<Path>>(&self, mut all_directories: E) -> E {
        for path in self.files.keys() {
            for i in 1..(path.len() - 1) {
                let mut sub_path = path.clone();
                let _ = sub_path.split_off(i);
                all_directories.extend(Some(sub_path));
            }
        }
        all_directories
    }

    pub fn all_paths<E: Extend<Path>>(&self, mut all_paths: E) -> E {
        for path in self.files.keys() {
            for i in 1..path.len() {
                let mut sub_path = path.clone();
                let _ = sub_path.split_off(i);
                all_paths.extend(Some(sub_path));
            }
        }
        all_paths
    }

    pub fn merge_with(&mut self, other: &Self) {
        // Lots of opportunities for algorithmic improvement
        self.files.retain(|file_path, _| {
            !other
                .files
                .keys()
                .any(|other_path| other_path.starts_with(&file_path))
        });

        let our_paths = self.files.keys().cloned().collect::<Vec<_>>();
        let paths_to_copy = other.files.iter().filter(|(file_path, _)| {
            !our_paths
                .iter()
                .any(|our_path| our_path.starts_with(&file_path))
        });

        for (other_path, other_file) in paths_to_copy {
            match self.files.entry(other_path.clone()) {
                Entry::Vacant(vacant) => {
                    vacant.insert(other_file.clone());
                }
                Entry::Occupied(mut occupied) => {
                    occupied.get_mut().extend(other_file.iter().cloned());
                }
            }
        }
    }
}

impl Merge for FileSystemState {
    fn merge(mut items: Vec<Self>) -> Self {
        let mut state = Self::default();
        while let Some(other) = items.pop() {
            state.merge_with(&other);
        }
        state
    }
}

impl ReferenceStateMachine for FileSystemState {
    type State = Self;
    type Transition = FileSystemOp;

    fn init_state() -> BoxedStrategy<Self::State> {
        Just(FileSystemState::default()).boxed()
    }

    fn transitions(state: &Self::State) -> BoxedStrategy<Self::Transition> {
        let all_paths = state.all_paths(Vec::new());
        let all_dirs = state.all_directories(BTreeSet::new());
        let all_files = state.files.keys().cloned().collect::<Vec<_>>();

        let file_path_strategy = simple_path()
            .prop_filter("doesn't overwrite a directory", move |path| {
                !all_dirs.contains(path)
            })
            .prop_filter(
                "doesn't try overwriting a file with a directory",
                move |path| {
                    !all_files
                        .iter()
                        .any(|file_path| path != file_path && path.starts_with(&file_path))
                },
            );

        let write_strategy = (file_path_strategy, simple_file())
            .prop_map(|(path, file)| FileSystemOp::Write(path, file));

        if all_paths.is_empty() {
            write_strategy.boxed()
        } else {
            prop_oneof![
                4 => write_strategy,
                1 => select(all_paths).prop_map(FileSystemOp::Remove)
            ]
            .boxed()
        }
    }

    fn apply(mut state: Self::State, transition: &Self::Transition) -> Self::State {
        match transition {
            FileSystemOp::Write(path, content) => {
                state
                    .files
                    .insert(path.clone(), BTreeSet::from([content.clone()]));
            }
            FileSystemOp::Remove(path) => {
                state.files = state
                    .files
                    .into_iter()
                    .filter(|(file_path, _)| !file_path.starts_with(&path))
                    .collect();
            }
        }
        state
    }

    fn preconditions(state: &Self::State, transition: &Self::Transition) -> bool {
        match transition {
            FileSystemOp::Write(write_path, _) => {
                // we can't overwrite a directory with a file with the same name
                !state.all_directories(BTreeSet::new()).contains(write_path)
                // we don't try overwriting a file with a directory
                    && !state
                        .files
                        .keys()
                        .any(|file_path| write_path != file_path && file_path.starts_with(write_path))
            }
            FileSystemOp::Remove(path) => {
                // we can't remove paths that don't exist
                state
                    .files
                    .keys()
                    .any(|file_path| file_path.starts_with(&path))
            }
        }
    }
}

pub fn simple_file() -> impl Strategy<Value = (MockMetadata, String)> {
    (simple_string(), simple_string())
}

pub fn simple_path() -> impl Strategy<Value = Path> {
    vec(simple_string(), 1..5)
}
