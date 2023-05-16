use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
use rand::{thread_rng, Rng};
use sha3::Digest;
use wnfs_nameaccumulator::{AccumulatorSetup, NameAccumulator, NameSegment};

fn name_segment_from_digest(c: &mut Criterion) {
    c.bench_function("NameSegment::from_digest", |b| {
        b.iter_batched(
            || sha3::Sha3_256::new().chain_update(thread_rng().gen::<[u8; 32]>()),
            NameSegment::from_digest,
            BatchSize::SmallInput,
        );
    });
}

fn name_segment_rng(c: &mut Criterion) {
    c.bench_function("NameSegment::new(rng)", |b| {
        b.iter(|| NameSegment::new(&mut thread_rng()));
    });
}

fn name_accumulator_add(c: &mut Criterion) {
    let setup = &AccumulatorSetup::from_rsa_factoring_challenge(&mut thread_rng());
    c.bench_function("NameAccumulator::add", |b| {
        b.iter_batched(
            || NameSegment::new(&mut thread_rng()),
            |segment| NameAccumulator::empty(setup).add(&segment, setup),
            BatchSize::SmallInput,
        )
    });
}

fn name_accumulator_serialize(c: &mut Criterion) {
    let setup = &AccumulatorSetup::from_rsa_factoring_challenge(&mut thread_rng());
    c.bench_function("NameAccumulator serialization", |b| {
        b.iter_batched(
            || {
                let segment = NameSegment::new(&mut thread_rng());
                let mut name = NameAccumulator::empty(setup);
                name.add(&segment, setup);
                name
            },
            |name| name.into_bytes(),
            BatchSize::SmallInput,
        )
    });
}

criterion_group!(
    benches,
    name_segment_from_digest,
    name_segment_rng,
    name_accumulator_add,
    name_accumulator_serialize,
);

criterion_main!(benches);
