use criterion::{BatchSize, Criterion, criterion_group, criterion_main};
use rand::{Rng, thread_rng};
use wnfs_nameaccumulator::{AccumulatorSetup, BigNumDig, BigNumRug, NameAccumulator, NameSegment};

fn name_segment_from_digest(c: &mut Criterion) {
    c.bench_function("NameSegment::<BigNumDig>::new_hashed", |b| {
        b.iter_batched(
            || thread_rng().r#gen::<[u8; 32]>(),
            |sth| NameSegment::<BigNumDig>::new_hashed("wnfs benchmarks", sth),
            BatchSize::SmallInput,
        );
    });
    c.bench_function("NameSegment::<BigNumRug>::new_hashed", |b| {
        b.iter_batched(
            || thread_rng().r#gen::<[u8; 32]>(),
            |sth| NameSegment::<BigNumRug>::new_hashed("wnfs benchmarks", sth),
            BatchSize::SmallInput,
        );
    });
}

fn name_segment_rng(c: &mut Criterion) {
    c.bench_function("NameSegment::<BigNumDig>::new(rng)", |b| {
        b.iter(|| NameSegment::<BigNumDig>::new(&mut thread_rng()));
    });
    c.bench_function("NameSegment::<BigNumRug>::new(rng)", |b| {
        b.iter(|| NameSegment::<BigNumRug>::new(&mut thread_rng()));
    });
}

fn name_accumulator_add(c: &mut Criterion) {
    let setup = &AccumulatorSetup::<BigNumDig>::from_rsa_2048(&mut thread_rng());
    c.bench_function("NameAccumulator::<BigNumDig>::add", |b| {
        b.iter_batched(
            || NameSegment::<BigNumDig>::new(&mut thread_rng()),
            |segment| NameAccumulator::<BigNumDig>::empty(setup).add(Some(&segment), setup),
            BatchSize::SmallInput,
        )
    });
    let setup = &AccumulatorSetup::<BigNumRug>::from_rsa_2048(&mut thread_rng());
    c.bench_function("NameAccumulator::<BigNumRug>::add", |b| {
        b.iter_batched(
            || NameSegment::<BigNumRug>::new(&mut thread_rng()),
            |segment| NameAccumulator::<BigNumRug>::empty(setup).add(Some(&segment), setup),
            BatchSize::SmallInput,
        )
    });
}

fn name_accumulator_serialize(c: &mut Criterion) {
    let setup = &AccumulatorSetup::<BigNumDig>::from_rsa_2048(&mut thread_rng());
    c.bench_function("NameAccumulator::<BigNumDig> serialization", |b| {
        b.iter_batched(
            || {
                let segment = NameSegment::<BigNumDig>::new(&mut thread_rng());
                let mut name = NameAccumulator::<BigNumDig>::empty(setup);
                name.add(Some(&segment), setup);
                name
            },
            |name| name.into_bytes(),
            BatchSize::SmallInput,
        )
    });
    let setup = &AccumulatorSetup::<BigNumRug>::from_rsa_2048(&mut thread_rng());
    c.bench_function("NameAccumulator::<BigNumRug> serialization", |b| {
        b.iter_batched(
            || {
                let segment = NameSegment::<BigNumRug>::new(&mut thread_rng());
                let mut name = NameAccumulator::<BigNumRug>::empty(setup);
                name.add(Some(&segment), setup);
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
