use crate::private::{
    forest::{hamt::HamtForest, traits::PrivateForest},
    FileContent, PrivateDirectory, PrivateFile, PrivateNode, PrivateRef,
};
use anyhow::Result;
use bytes::Bytes;
use rand_core::CryptoRngCore;
use std::rc::Rc;
use wnfs_common::utils::SnapshotBlockStore;
use wnfs_nameaccumulator::Name;

//--------------------------------------------------------------------------------------------------
// Functions
//--------------------------------------------------------------------------------------------------

pub(crate) async fn walk_dir(
    store: &mut SnapshotBlockStore,
    forest: &mut Rc<HamtForest>,
    root_dir: &Rc<PrivateDirectory>,
    rng: &mut impl CryptoRngCore,
) -> Result<()> {
    let mut stack = vec![root_dir.clone()];
    while let Some(dir) = stack.pop() {
        let private_ref: PrivateRef = dir.store(forest, store, rng).await?;
        let temporal_key = private_ref.temporal_key;
        let snapshot_key = temporal_key.derive_snapshot_key();
        store.add_block_handler(
            private_ref.content_cid,
            Box::new(move |bytes| Ok(Bytes::from(snapshot_key.decrypt(bytes.as_ref())?))),
        );
        store.add_block_handler(
            dir.header
                .store(store, forest.get_accumulator_setup())
                .await?,
            Box::new(move |bytes| Ok(Bytes::from(temporal_key.key_wrap_decrypt(bytes.as_ref())?))),
        );

        let entries = dir.ls(&[], true, forest, store).await?;
        for (name, _) in entries.iter() {
            let node = dir.lookup_node(name, true, forest, store).await?;
            match node.as_ref() {
                Some(PrivateNode::Dir(dir)) => {
                    stack.push(dir.clone());
                }
                Some(PrivateNode::File(file)) => {
                    let private_ref: PrivateRef = file.store(forest, store, rng).await?;
                    let temporal_key = private_ref.temporal_key;
                    let snapshot_key = temporal_key.derive_snapshot_key();
                    store.add_block_handler(
                        private_ref.content_cid,
                        Box::new(move |bytes| {
                            Ok(Bytes::from(snapshot_key.decrypt(bytes.as_ref())?))
                        }),
                    );
                    store.add_block_handler(
                        file.header
                            .store(store, forest.get_accumulator_setup())
                            .await?,
                        Box::new(move |bytes| {
                            Ok(Bytes::from(temporal_key.key_wrap_decrypt(bytes.as_ref())?))
                        }),
                    );
                    if let FileContent::External {
                        key,
                        block_count,
                        base_name,
                        ..
                    } = &file.content.content
                    {
                        for name in PrivateFile::generate_shard_labels(
                            key,
                            0,
                            *block_count,
                            &Name::new(base_name.clone(), []),
                        ) {
                            match forest.get_encrypted(&name, store).await? {
                                Some(cids) => {
                                    let key = key.clone();
                                    store.add_block_handler(
                                        *cids.first().unwrap(),
                                        Box::new(move |bytes| {
                                            Ok(Bytes::from(key.decrypt(bytes.as_ref())?))
                                        }),
                                    )
                                }
                                None => unreachable!(),
                            };
                        }
                    }
                }
                None => unreachable!(),
            }
        }
    }

    Ok(())
}
