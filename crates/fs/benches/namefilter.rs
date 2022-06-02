use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn add(c: &mut Criterion) {
    let todo = |_n: u64| ();
    c.bench_function("todo 20", |b| b.iter(|| todo(black_box(20))));
}

criterion_group!(benches, add);
criterion_main!(benches);
