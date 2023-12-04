// use serde::Deserialize;
// use wnfs_hamt::Node;
// use wnfs_nameaccumulator::{BatchedProofPart, NameAccumulator, UnbatchableProofPart};

// #[derive(Deserialize, Debug, Clone)]
// #[serde(rename_all = "camelCase")]
// pub(crate) struct ForestProofsSerializable {
//     proofs_by_commitment: Node<NameAccumulator, Node<NameAccumulator, UnbatchableProofPart>>,
//     batched_proof_part: BatchedProofPart,
// }
