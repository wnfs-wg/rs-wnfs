use criterion::{
    async_executor::AsyncStdExecutor, black_box, criterion_group, criterion_main, Criterion,
};
use libipld::Cid;
use std::rc::Rc;
use wnfs::{
    dagcbor,
    private::hamt::{Hamt, Node},
    BlockStore, MemoryBlockStore,
};

fn node_set(c: &mut Criterion) {
    c.bench_function("Node set", |b| {
        b.to_async(AsyncStdExecutor).iter(|| async {
            let store = &mut MemoryBlockStore::default();
            let mut node = black_box(Rc::new(<Node<_, _>>::default()));

            for i in 0..50 {
                node = black_box(node.set(i.to_string(), i, store).await.unwrap());
            }
        })
    });
}

fn node_load_get(c: &mut Criterion) {
    let mut store = MemoryBlockStore::default();
    let cid: Cid = async_std::task::block_on(async {
        let mut node = Rc::new(<Node<_, _>>::default());
        for i in 0..50 {
            node = node.set(i.to_string(), i, &mut store).await.unwrap();
        }

        let encoded_hamt = dagcbor::async_encode(&Hamt::with_root(node), &mut store)
            .await
            .unwrap();
        store.put_serializable(&encoded_hamt).await.unwrap()
    });

    c.bench_function("Node load and get", |b| {
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
}

fn node_load_remove(c: &mut Criterion) {
    let mut store = MemoryBlockStore::default();
    let cid: Cid = async_std::task::block_on(async {
        let mut node = Rc::new(<Node<_, _>>::default());
        for i in 0..50 {
            node = node.set(i.to_string(), i, &mut store).await.unwrap();
        }

        let encoded_hamt = dagcbor::async_encode(&Hamt::with_root(node), &mut store)
            .await
            .unwrap();

        store.put_serializable(&encoded_hamt).await.unwrap()
    });

    c.bench_function("Node load and remove", |b| {
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

macro_rules! hamt {
    ($store:expr, { $( ($key:expr, $value:expr) $(, ($key_rest:expr, $value_rest:expr) )* $(,)? )? }) => {{
        let root = Rc::new(Node::default());
        $(
            let root = root.set($key, $value, $store).await.unwrap();
            $(
                let root = root.set($key_rest, $value_rest, $store).await.unwrap();
            )*
        )?
        black_box(Hamt::with_root(root))
    }};
}

fn hamt_load_decode(c: &mut Criterion) {
    let mut store = MemoryBlockStore::default();
    let cid: Cid = async_std::task::block_on(async {
        let hamt: Hamt<String, _> = hamt!(&mut store, {
            ("foo".into(), 1),
            ("bar".into(), 2),
            ("baz".into(), 3),
            ("qux".into(), 4),
            ("quux".into(), 5),
            ("corge".into(), 6),
            ("grault".into(), 7),
            ("garply".into(), 8),
            ("waldo".into(), 9),
            ("fred".into(), 10),
            ("plugh".into(), 11),
            ("xyzzy".into(), 12),
            ("thud".into(), 13),
        });

        let encoded_hamt = dagcbor::async_encode(&hamt, &mut store).await.unwrap();
        store.put_serializable(&encoded_hamt).await.unwrap()
    });

    c.bench_function("HAMT load and decode", |b| {
        b.to_async(AsyncStdExecutor).iter(|| async {
            let encoded_hamt = store.get_deserializable::<Vec<u8>>(&cid).await.unwrap();
            let _: Hamt<String, i32> = black_box(dagcbor::decode(encoded_hamt.as_ref()).unwrap());
        })
    });
}

fn hamt_set_encode(c: &mut Criterion) {
    c.bench_function("HAMT set and encode", |b| {
        b.to_async(AsyncStdExecutor).iter(|| async {
            let store = &mut MemoryBlockStore::default();
            let hamt: Hamt<String, _> = hamt!(store, {
                ("foo".into(), 1),
                ("bar".into(), 2),
                ("baz".into(), 3),
                ("qux".into(), 4),
                ("quux".into(), 5),
                ("corge".into(), 6),
                ("grault".into(), 7),
                ("garply".into(), 8),
                ("waldo".into(), 9),
                ("fred".into(), 10),
                ("plugh".into(), 11),
                ("xyzzy".into(), 12),
                ("thud".into(), 13),
            });

            let _ = black_box(dagcbor::async_encode(&hamt, store).await.unwrap());
        })
    });
}

criterion_group!(
    benches,
    node_set,
    node_load_get,
    node_load_remove,
    hamt_load_decode,
    hamt_set_encode
);

criterion_main!(benches);
