mod sampleable;

use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion, Throughput};
use proptest::{arbitrary::any, collection::vec, test_runner::TestRunner};
use sampleable::Sampleable;
use wnfs::{dagcbor, private::namefilter::Namefilter};

const FILTER_CAPACITY: usize = 47;

fn add(c: &mut Criterion) {
    let mut runner = TestRunner::deterministic();
    c.bench_function("namefilter add", |b| {
        b.iter_batched(
            || {
                (
                    Namefilter::default(),
                    vec(any::<[u8; 32]>(), FILTER_CAPACITY).sample(&mut runner),
                )
            },
            |(mut namefilter, elements)| {
                for element in elements {
                    black_box(namefilter.add(&element));
                }
            },
            BatchSize::SmallInput,
        )
    });
}

fn contains(c: &mut Criterion) {
    let mut runner = TestRunner::deterministic();
    c.bench_function("namefilter contains", |b| {
        b.iter_batched(
            || {
                let mut namefilter = Namefilter::default();
                for _ in 0..FILTER_CAPACITY {
                    namefilter.add(&any::<[u8; 32]>().sample(&mut runner));
                }
                namefilter
            },
            |namefilter| {
                for i in 0..FILTER_CAPACITY {
                    assert!(namefilter.contains(&i.to_string()));
                }
            },
            BatchSize::SmallInput,
        )
    });
}

fn saturate(c: &mut Criterion) {
    let mut runner = TestRunner::deterministic();
    c.bench_function("namefilter saturate", |b| {
        b.iter_batched(
            || {
                let mut namefilter = black_box(Namefilter::default());
                namefilter.add(&any::<[u8; 32]>().sample(&mut runner));
                namefilter
            },
            |mut namefilter| {
                black_box(namefilter.saturate());
            },
            BatchSize::SmallInput,
        )
    });
}

fn encode(c: &mut Criterion) {
    let mut runner = TestRunner::deterministic();
    c.bench_function("namefilter encode", |b| {
        b.iter_batched(
            || {
                let mut namefilter = Namefilter::default();
                for _ in 0..FILTER_CAPACITY {
                    namefilter.add(&any::<[u8; 32]>().sample(&mut runner));
                }
                namefilter
            },
            |namefilter| {
                let _ = black_box(dagcbor::encode(black_box(&namefilter)).unwrap());
            },
            BatchSize::SmallInput,
        )
    });
}

fn decode(c: &mut Criterion) {
    let mut runner = TestRunner::deterministic();
    let mut namefilter = Namefilter::default();
    for _ in 0..FILTER_CAPACITY {
        namefilter.add(&any::<[u8; 32]>().sample(&mut runner));
    }
    let encoded_namefilter = dagcbor::encode(&namefilter).unwrap();

    let mut group = c.benchmark_group("namefilter decode");
    group.throughput(Throughput::Bytes(encoded_namefilter.len() as u64));
    group.bench_function("0", |b| {
        b.iter(|| async {
            let _ = black_box(dagcbor::decode::<Namefilter>(encoded_namefilter.as_ref()).unwrap());
        })
    });
}

criterion_group!(benches, add, contains, saturate, encode, decode);

criterion_main!(benches);
