use proptest::strategy::Strategy;

mod file_system;
mod replicas;

pub use file_system::*;
use proptest_state_machine::ReferenceStateMachine;
pub use replicas::*;
use test_strategy::proptest;

fn simple_string() -> impl Strategy<Value = String> {
    (0..6u32).prop_map(|c| char::from_u32('a' as u32 + c).unwrap().to_string())
}

#[proptest]
fn test_replicas_state_machine_doesnt_panic(
    #[strategy(ReplicasStateMachine::<FileSystemState>::sequential_strategy(1..100).prop_map(|(state, transitions, _)| (state, transitions)))]
    generated: (Replicas<FileSystemState>, Vec<ReplicaOp<FileSystemOp>>),
) {
    let (mut state, transitions) = generated;

    for transition in transitions {
        println!("{transition:?}");
        state = ReplicasStateMachine::<FileSystemState>::apply(state, &transition);
    }

    println!("{state:#?}");
}
