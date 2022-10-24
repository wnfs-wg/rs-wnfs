use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};
use wnfs::{dagcbor, private::namefilter::Namefilter};

fn add(c: &mut Criterion) {
    c.bench_function("namefilter add", |b| {
        b.iter(|| {
            let mut namefilter = black_box(Namefilter::default());
            for i in 0..50 {
                black_box(namefilter.add(&i.to_string()));
            }
        })
    });
}

fn contains(c: &mut Criterion) {
    let mut namefilter = Namefilter::default();
    for i in 0..50 {
        namefilter.add(&i.to_string());
    }

    c.bench_function("namefilter contains", |b| {
        b.iter(|| {
            for i in 0..50 {
                assert!(namefilter.contains(&i.to_string()));
            }
        })
    });
}

fn saturate(c: &mut Criterion) {
    let mut namefilter = Namefilter::default();
    for i in 0..50 {
        namefilter.add(&i.to_string());
    }

    c.bench_function("namefilter saturate", |b| {
        b.iter(|| {
            black_box(namefilter.saturate());
        })
    });
}

fn encode(c: &mut Criterion) {
    let mut namefilter = Namefilter::default();
    for i in 0..50 {
        namefilter.add(&i.to_string());
    }

    c.bench_function("namefilter encode", |b| {
        b.iter(|| {
            let _ = black_box(dagcbor::encode(&namefilter).unwrap());
        })
    });
}

fn decode(c: &mut Criterion) {
    let mut namefilter = Namefilter::default();
    for i in 0..50 {
        namefilter.add(&i.to_string());
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
