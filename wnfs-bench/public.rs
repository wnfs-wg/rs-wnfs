use chrono::Utc;
use criterion::{
    async_executor::AsyncStdExecutor, black_box, criterion_group, criterion_main, BatchSize,
    Criterion,
};
use proptest::{collection::vec, test_runner::TestRunner};
use std::rc::Rc;
use wnfs::{libipld::Cid, public::PublicDirectory};
use wnfs_common::{utils::Sampleable, MemoryBlockStore};

fn write_files(c: &mut Criterion) {
    let mut runner = TestRunner::deterministic();
    let time = Utc::now();

    c.bench_function("write files", |b| {
        b.to_async(AsyncStdExecutor).iter_batched(
            || {
                let store = MemoryBlockStore::default();
                let root = Rc::new(PublicDirectory::new(time));
                let paths = vec(vec("[a-zA-Z0-9_]{1,10}", 1..10), 100).sample(&mut runner);
                (root, store, paths)
            },
            |(root, store, paths)| async move {
                for path_segments in paths.iter() {
                    black_box(
                        Rc::clone(&root)
                            .write(path_segments, Cid::default(), time, &store)
                            .await
                            .unwrap(),
                    );
                }
            },
            BatchSize::SmallInput,
        );
    });
}

criterion_group!(benches, write_files,);

criterion_main!(benches);
