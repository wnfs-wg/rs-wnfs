use proptest::{
    prop_oneof,
    sample::select,
    strategy::{BoxedStrategy, Just, Strategy},
};
use proptest_state_machine::ReferenceStateMachine;
use std::{
    cmp::Ordering,
    collections::{BTreeMap, BTreeSet},
    fmt::Debug,
};

#[derive(Debug, Clone, Copy)]
pub struct ReplicasStateMachine<InnerStateMachine>(InnerStateMachine);

#[derive(Debug, Clone)]
pub struct Replicas<InnerState> {
    /// Non-empty list of concurrently working replicas
    pub replicas: Vec<(CrdtClock, InnerState)>,
}

/// A simple vector clock
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CrdtClock(pub BTreeMap<usize, i32>);

#[derive(Debug, Clone)]
pub enum ReplicaOp<InnerOp> {
    /// Run some state-specific operation on the nth replica
    InnerOp(usize, InnerOp),
    /// Fork from the nth replica state
    Fork(usize),
    /// Merge all heads
    Merge,
}

impl<I: Default> Default for Replicas<I> {
    fn default() -> Self {
        Self::new(I::default())
    }
}

impl<I: Default> Replicas<I> {
    pub fn new(inner: I) -> Self {
        Self {
            replicas: vec![(CrdtClock::new(), inner)],
        }
    }
}

impl<Inner> ReferenceStateMachine for ReplicasStateMachine<Inner>
where
    Inner: ReferenceStateMachine,
    Inner::State: Default + Merge + Debug + Clone + 'static,
    Inner::Transition: Debug + Clone + 'static,
{
    type State = Replicas<Inner::State>;
    type Transition = ReplicaOp<Inner::Transition>;

    fn init_state() -> BoxedStrategy<Self::State> {
        Inner::init_state().prop_map(Replicas::new).boxed()
    }

    fn transitions(state: &Self::State) -> BoxedStrategy<Self::Transition> {
        debug_assert!(!state.replicas.is_empty());
        // Because we can't capture the state ref (lifetime issues)
        let replicas_with_idxes = state
            .replicas
            .iter()
            .cloned()
            .enumerate()
            .collect::<Vec<_>>();

        prop_oneof![
            6 => select(replicas_with_idxes).prop_flat_map(|(replica_idx, (_, replica_state))|
                Inner::transitions(&replica_state)
                    .prop_map(move |op|
                        ReplicaOp::InnerOp(replica_idx, op)
                    )
                ),
            2 => (0usize..state.replicas.len()).prop_map(ReplicaOp::Fork),
            1 => Just(ReplicaOp::Merge),
        ]
        .boxed()
    }

    fn apply(mut state: Self::State, transition: &Self::Transition) -> Self::State {
        match transition {
            ReplicaOp::InnerOp(replica_idx, inner_op) => {
                let replica_state = std::mem::take(&mut state.replicas[*replica_idx].1);
                state.replicas[*replica_idx].0.write_as(*replica_idx);
                state.replicas[*replica_idx].1 = Inner::apply(replica_state, inner_op);
            }
            ReplicaOp::Fork(replica_idx) => {
                state.replicas.push(state.replicas[*replica_idx].clone());
            }
            ReplicaOp::Merge => {
                state.replicas = vec![merge_replicas(state.replicas)];
            }
        }
        state
    }

    fn preconditions(state: &Self::State, transition: &Self::Transition) -> bool {
        match transition {
            ReplicaOp::InnerOp(idx, op) => {
                state.replicas.len() > *idx && Inner::preconditions(&state.replicas[*idx].1, op)
            }
            ReplicaOp::Fork(idx) => state.replicas.len() > *idx,
            ReplicaOp::Merge => true,
        }
    }
}

impl CrdtClock {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn causal_compare(&self, other: &Self) -> Option<Ordering> {
        let mut ordering_so_far = Ordering::Equal;

        let all_replicas = self
            .0
            .keys()
            .chain(other.0.keys())
            .cloned()
            .collect::<BTreeSet<_>>();

        for replica in all_replicas {
            let our_version = self.0.get(&replica).cloned().unwrap_or_default();
            let other_version = other.0.get(&replica).cloned().unwrap_or_default();

            match ordering_so_far {
                Ordering::Equal => {
                    if our_version > other_version {
                        ordering_so_far = Ordering::Greater;
                    }
                    if our_version < other_version {
                        ordering_so_far = Ordering::Less;
                    }
                }
                Ordering::Less => {
                    if our_version > other_version {
                        // diverged
                        return None;
                    }
                }
                Ordering::Greater => {
                    if our_version < other_version {
                        // diverged
                        return None;
                    }
                }
            }
        }

        Some(ordering_so_far)
    }

    pub fn merge(&self, other: &Self) -> Self {
        let mut clock = Self::new();
        let all_replicas = self
            .0
            .keys()
            .chain(other.0.keys())
            .cloned()
            .collect::<BTreeSet<_>>();

        for replica in all_replicas {
            let our_version = self.0.get(&replica).cloned().unwrap_or_default();
            let other_version = other.0.get(&replica).cloned().unwrap_or_default();
            clock
                .0
                .insert(replica, std::cmp::max(our_version, other_version));
        }

        clock
    }

    pub fn max(&self) -> i32 {
        self.0.values().max().cloned().unwrap_or_default()
    }

    pub fn write_as(&mut self, replica_id: usize) {
        let max = self.max();
        *self.0.entry(replica_id).or_default() = max + 1;
    }
}

pub trait Merge: Sized {
    /// Commutative and associative merge function, ideally
    fn merge(items: Vec<Self>) -> Self;
}

fn merge_two_replicas<I: Merge>(
    (clock1, i1): (CrdtClock, I),
    (clock2, i2): (CrdtClock, I),
) -> (CrdtClock, I) {
    (
        clock1.merge(&clock2),
        match clock1.causal_compare(&clock2) {
            Some(Ordering::Equal) => i1,
            Some(Ordering::Greater) => i1,
            Some(Ordering::Less) => i2,
            None => I::merge(vec![i1, i2]),
        },
    )
}

fn merge_replicas<I: Merge>(mut replicas: Vec<(CrdtClock, I)>) -> (CrdtClock, I) {
    let mut replica = replicas.pop().expect("invariant: replicas.len() > 0");
    while let Some(other_replica) = replicas.pop() {
        replica = merge_two_replicas(replica, other_replica);
    }
    replica
}
