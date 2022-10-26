use criterion::{
    async_executor::AsyncStdExecutor, black_box, criterion_group, criterion_main, BatchSize,
    Criterion, Throughput,
};
use proptest::{arbitrary::any, test_runner::TestRunner};
use std::{rc::Rc, sync::Arc, time::Duration};
use wnfs::{
    dagcbor,
    private::{
        hamt::{Hamt, Node},
        strategies::{node_from_operations, operations},
    },
    BlockStore, MemoryBlockStore,
};
use wnfs_bench::sampleable::Sampleable;

fn node_set(c: &mut Criterion) {
    let mut runner = TestRunner::deterministic();
    let mut store = MemoryBlockStore::default();
    let operations = operations(any::<[u8; 32]>(), any::<u64>(), 1_000_000).sample(&mut runner);
    let node =
        &*async_std::task::block_on(async { node_from_operations(operations, &mut store).await })
            .expect("Couldn't setup HAMT node from operations");

    let store = Arc::new(store);

    c.bench_function("node set", |b| {
        b.to_async(AsyncStdExecutor).iter_batched(
            || {
                let store = Arc::clone(&store);
                let kv = (any::<[u8; 32]>(), any::<u64>()).sample(&mut runner);
                (store, kv)
            },
            |(store, (key, value))| async move {
                black_box(node.set(key, value, store.as_ref()).await.unwrap());
            },
            BatchSize::SmallInput,
        );
    });
}

fn node_load_get(c: &mut Criterion) {
    let mut store = MemoryBlockStore::default();
    let (cid, bytes) = async_std::task::block_on(async {
        let mut node = Rc::new(<Node<_, _>>::default());
        for i in 0..50 {
            node = node.set(i.to_string(), i, &mut store).await.unwrap();
        }

        let encoded_hamt = dagcbor::async_encode(&Hamt::with_root(node), &mut store)
            .await
            .unwrap();

        let cid = store.put_serializable(&encoded_hamt).await.unwrap();

        (cid, encoded_hamt)
    });

    let mut group = c.benchmark_group("node load and get");
    group.throughput(Throughput::Bytes(bytes.len() as u64));
    group.bench_function("0", |b| {
        b.to_async(AsyncStdExecutor).iter(|| async {
            let encoded_hamt = store.get_deserializable::<Vec<u8>>(&cid).await.unwrap();
            let hamt: Hamt<String, i32> = dagcbor::decode(encoded_hamt.as_ref()).unwrap();

            for i in 0..50 {
                assert!(hamt
                    .root
                    .get(&i.to_string(), &store)
                    .await
                    .unwrap()
                    .is_some());
            }
        })
    });

    group.finish();
}

fn node_load_remove(c: &mut Criterion) {
    let mut store = MemoryBlockStore::default();
    let (cid, bytes) = async_std::task::block_on(async {
        let mut node = Rc::new(<Node<_, _>>::default());
        for i in 0..50 {
            node = node.set(i.to_string(), i, &mut store).await.unwrap();
        }

        let encoded_hamt = dagcbor::async_encode(&Hamt::with_root(node), &mut store)
            .await
            .unwrap();

        let cid = store.put_serializable(&encoded_hamt).await.unwrap();

        (cid, encoded_hamt)
    });

    let mut group = c.benchmark_group("node load and remove");
    group.throughput(Throughput::Bytes(bytes.len() as u64));
    group.bench_function("0", |b| {
        b.to_async(AsyncStdExecutor).iter(|| async {
            let encoded_hamt = store.get_deserializable::<Vec<u8>>(&cid).await.unwrap();
            let mut hamt: Hamt<String, i32> =
                black_box(dagcbor::decode(encoded_hamt.as_ref()).unwrap());

            for i in 0..50 {
                let (root, value) = hamt.root.remove(&i.to_string(), &store).await.unwrap();
                assert!(value.is_some());
                hamt.root = root;
            }
        })
    });
}

fn hamt_load_decode(c: &mut Criterion) {
    let mut store = MemoryBlockStore::default();
    let (cid, bytes) = async_std::task::block_on(async {
        let mut node = Rc::new(<Node<_, _>>::default());
        for i in 0..50 {
            node = node.set(i.to_string(), i, &mut store).await.unwrap();
        }

        let encoded_hamt = dagcbor::async_encode(&Hamt::with_root(node), &mut store)
            .await
            .unwrap();

        let cid = store.put_serializable(&encoded_hamt).await.unwrap();

        (cid, encoded_hamt)
    });

    let mut group = c.benchmark_group("hamt load and decode");
    group.throughput(Throughput::Bytes(bytes.len() as u64));
    group.bench_function("0", |b| {
        b.to_async(AsyncStdExecutor).iter(|| async {
            let encoded_hamt = store.get_deserializable::<Vec<u8>>(&cid).await.unwrap();
            let _: Hamt<String, i32> = black_box(dagcbor::decode(encoded_hamt.as_ref()).unwrap());
        })
    });
}

fn hamt_set_encode(c: &mut Criterion) {
    c.bench_function("hamt set and encode", |b| {
        b.to_async(AsyncStdExecutor).iter(|| async {
            let store = &mut MemoryBlockStore::default();
            let mut node = Rc::new(<Node<_, _>>::default());
            for i in 0..50 {
                node = node.set(i.to_string(), i, store).await.unwrap();
            }

            let hamt = Hamt::with_root(node);

            let _ = black_box(dagcbor::async_encode(&hamt, store).await.unwrap());
        })
    });
}

criterion_group!(
    name = benches;
    config = Criterion::default().measurement_time(Duration::from_secs(10));
    targets =
        node_set,
        node_load_get,
        node_load_remove,
        hamt_load_decode,
        hamt_set_encode
);

criterion_main!(benches);
