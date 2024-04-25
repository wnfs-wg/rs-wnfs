use proptest::{
    prop_oneof,
    sample::select,
    strategy::{BoxedStrategy, Just, Strategy},
};
use proptest_state_machine::ReferenceStateMachine;
use std::fmt::Debug;

#[derive(Debug, Clone, Copy)]
pub struct ReplicasStateMachine<InnerStateMachine>(InnerStateMachine);

#[derive(Debug, Clone)]
pub struct Replicas<InnerState> {
    /// Non-empty list of concurrently working replicas
    pub replicas: Vec<InnerState>,
}

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
            replicas: vec![inner],
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
            6 => select(replicas_with_idxes).prop_flat_map(|(replica_idx, replica_state)|
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
                let replica_state = std::mem::take(&mut state.replicas[*replica_idx]);
                state.replicas[*replica_idx] = Inner::apply(replica_state, inner_op);
            }
            ReplicaOp::Fork(replica_idx) => {
                state.replicas.push(state.replicas[*replica_idx].clone());
            }
            ReplicaOp::Merge => {
                state.replicas = vec![<Inner as ReferenceStateMachine>::State::merge(
                    state.replicas,
                )];
            }
        }
        state
    }

    fn preconditions(state: &Self::State, transition: &Self::Transition) -> bool {
        match transition {
            ReplicaOp::InnerOp(idx, op) => {
                state.replicas.len() > *idx && Inner::preconditions(&state.replicas[*idx], op)
            }
            ReplicaOp::Fork(idx) => state.replicas.len() > *idx,
            ReplicaOp::Merge => true,
        }
    }
}

pub trait Merge: Sized {
    /// Commutative and associative merge function, ideally
    fn merge(items: Vec<Self>) -> Self;
}
