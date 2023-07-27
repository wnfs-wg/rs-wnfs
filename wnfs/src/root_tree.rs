//! TODO(appcypher): Add private API for now, but remove it later.

#![allow(dead_code)]

use crate::{
    error::FsError,
    private::{
        forest::{hamt::HamtForest, traits::PrivateForest},
        PrivateDirectory,
    },
    public::PublicDirectory,
    VERSION,
};
use anyhow::{bail, Result};
use chrono::{DateTime, Utc};
use libipld_core::cid::Cid;
#[cfg(test)]
use rand::rngs::ThreadRng;
use rand_core::CryptoRngCore;
use semver::Version;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, rc::Rc};
#[cfg(test)]
use wnfs_common::MemoryBlockStore;
use wnfs_common::{BlockStore, Metadata, CODEC_RAW};
#[cfg(test)]
use wnfs_nameaccumulator::AccumulatorSetup;

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub struct RootTree<'a, B: BlockStore, R: CryptoRngCore> {
    pub store: &'a B,
    pub rng: R,
    pub forest: Rc<HamtForest>,
    pub public_root: Rc<PublicDirectory>,
    pub exchange_root: Rc<PublicDirectory>,
    pub private_map: HashMap<Vec<String>, Rc<PrivateDirectory>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RootTreeSerializable {
    pub public: Cid,
    pub exchange: Cid,
    pub forest: Cid,
    pub version: Version,
}

//--------------------------------------------------------------------------------------------------
// Implementations
//--------------------------------------------------------------------------------------------------

impl<'a, B, R> RootTree<'a, B, R>
where
    B: BlockStore,
    R: CryptoRngCore,
{
    pub async fn new(
        forest: Rc<HamtForest>,
        store: &'a B,
        rng: R,
        time: DateTime<Utc>,
        private_map: HashMap<Vec<String>, Rc<PrivateDirectory>>,
    ) -> RootTree<'a, B, R> {
        Self {
            store,
            rng,
            forest,
            public_root: Rc::new(PublicDirectory::new(time)),
            exchange_root: Rc::new(PublicDirectory::new(time)),
            private_map,
        }
    }

    pub async fn create_private_root(&mut self, name: &str) -> Result<()> {
        let root = PrivateDirectory::new_and_store(
            &self.forest.empty_name(),
            Utc::now(),
            &mut self.forest,
            self.store,
            &mut self.rng,
        )
        .await?;

        self.private_map.insert(vec![name.to_string()], root);

        Ok(())
    }

    pub async fn ls(
        &self,
        root_segments: &[String],
        path_segments: &[String],
    ) -> Result<Vec<(String, Metadata)>> {
        let Some(first) = root_segments.first() else {
            bail!(FsError::InvalidPath)
        };

        match first.as_str() {
            "public" => self.public_root.ls(path_segments, self.store).await,
            "exchange" => self.exchange_root.ls(path_segments, self.store).await,
            _ => {
                let root = self
                    .private_map
                    .get(root_segments)
                    .ok_or(FsError::PrivateRefNotFound)?;

                root.ls(path_segments, true, &self.forest, self.store).await
            }
        }
    }

    pub async fn read(
        &self,
        root_segments: &[String],
        path_segments: &[String],
    ) -> Result<Vec<u8>> {
        let Some(first) = root_segments.first() else {
            bail!(FsError::InvalidPath)
        };

        match first.as_str() {
            "public" => {
                let cid = self.public_root.read(path_segments, self.store).await?;
                self.store.get_block(&cid).await.map(|b| b.to_vec())
            }
            "exchange" => {
                let cid = self.exchange_root.read(path_segments, self.store).await?;
                self.store.get_block(&cid).await.map(|b| b.to_vec())
            }
            _ => {
                let root = self
                    .private_map
                    .get(root_segments)
                    .ok_or(FsError::PrivateRefNotFound)?;

                root.read(path_segments, true, &self.forest, self.store)
                    .await
            }
        }
    }

    pub async fn write(
        &mut self,
        root_segments: &[String],
        path_segments: &[String],
        content: Vec<u8>,
        time: DateTime<Utc>,
    ) -> Result<()> {
        let Some(first) = root_segments.first() else {
            bail!(FsError::InvalidPath)
        };

        match first.as_str() {
            "public" => {
                let cid = self.store.put_block(content, CODEC_RAW).await?;
                self.public_root
                    .write(path_segments, cid, time, self.store)
                    .await
            }
            "exchange" => {
                let cid = self.store.put_block(content, CODEC_RAW).await?;
                self.exchange_root
                    .write(path_segments, cid, time, self.store)
                    .await
            }
            _ => {
                let root = self
                    .private_map
                    .get_mut(root_segments)
                    .ok_or(FsError::PrivateRefNotFound)?;

                root.write(
                    path_segments,
                    true,
                    time,
                    content,
                    &mut self.forest,
                    self.store,
                    &mut self.rng,
                )
                .await
            }
        }
    }

    pub async fn mkdir(
        &mut self,
        root_segments: &[String],
        path_segments: &[String],
        time: DateTime<Utc>,
    ) -> Result<()> {
        let Some(first) = root_segments.first() else {
            bail!(FsError::InvalidPath)
        };

        match first.as_str() {
            "public" => {
                self.public_root
                    .mkdir(path_segments, time, self.store)
                    .await
            }
            "exchange" => {
                self.exchange_root
                    .mkdir(path_segments, time, self.store)
                    .await
            }
            _ => {
                let root = self
                    .private_map
                    .get_mut(root_segments)
                    .ok_or(FsError::PrivateRefNotFound)?;

                root.mkdir(
                    path_segments,
                    true,
                    time,
                    &self.forest,
                    self.store,
                    &mut self.rng,
                )
                .await
            }
        }
    }

    pub async fn rm(&mut self, root_segments: &[String], path_segments: &[String]) -> Result<()> {
        let Some(first) = root_segments.first() else {
            bail!(FsError::InvalidPath)
        };

        match first.as_str() {
            "public" => self
                .public_root
                .rm(path_segments, self.store)
                .await
                .map(|_| ()),
            "exchange" => self
                .exchange_root
                .rm(path_segments, self.store)
                .await
                .map(|_| ()),
            _ => {
                let root = self
                    .private_map
                    .get_mut(root_segments)
                    .ok_or(FsError::PrivateRefNotFound)?;

                let _ = root
                    .rm(path_segments, true, &self.forest, self.store)
                    .await?;

                Ok(())
            }
        }
    }

    pub async fn basic_mv(
        &mut self,
        root_segments: &[String],
        path_segments_from: &[String],
        path_segments_to: &[String],
        time: DateTime<Utc>,
    ) -> Result<()> {
        let Some(first) = root_segments.first() else {
            bail!(FsError::InvalidPath)
        };

        match first.as_str() {
            "public" => {
                self.public_root
                    .basic_mv(path_segments_from, path_segments_to, time, self.store)
                    .await
            }
            "exchange" => {
                self.exchange_root
                    .basic_mv(path_segments_from, path_segments_to, time, self.store)
                    .await
            }
            _ => {
                let root = self
                    .private_map
                    .get_mut(root_segments)
                    .ok_or(FsError::PrivateRefNotFound)?;

                root.basic_mv(
                    path_segments_from,
                    path_segments_to,
                    true,
                    time,
                    &mut self.forest,
                    self.store,
                    &mut self.rng,
                )
                .await
            }
        }
    }

    pub async fn store(&self, store: &B) -> Result<Cid> {
        let serializable = RootTreeSerializable {
            public: self.public_root.store(store).await?,
            exchange: self.exchange_root.store(store).await?,
            forest: self.forest.store(store).await?,
            version: VERSION,
        };

        store.put_serializable(&serializable).await
    }

    pub async fn load(
        cid: &Cid,
        store: &'a B,
        rng: R,
        private_map: HashMap<Vec<String>, Rc<PrivateDirectory>>,
    ) -> Result<RootTree<'a, B, R>> {
        let deserialized: RootTreeSerializable = store.get_deserializable(cid).await?;
        let forest = Rc::new(HamtForest::load(&deserialized.forest, store).await?);
        let public_root = Rc::new(store.get_deserializable(&deserialized.public).await?);
        let exchange_root = Rc::new(store.get_deserializable(&deserialized.exchange).await?);

        Ok(Self {
            store,
            rng,
            forest,
            public_root,
            exchange_root,
            private_map,
        })
    }
}

#[cfg(test)]
impl<'a, B: BlockStore> RootTree<'a, B, ThreadRng> {
    pub fn with_store(store: &'a B) -> RootTree<'a, B, ThreadRng> {
        let mut rng = rand::thread_rng();
        let forest = Rc::new(HamtForest::new(AccumulatorSetup::trusted(&mut rng)));
        Self {
            store,
            rng,
            forest,
            public_root: Rc::new(PublicDirectory::new(Utc::now())),
            exchange_root: Rc::new(PublicDirectory::new(Utc::now())),
            private_map: HashMap::default(),
        }
    }
}

//--------------------------------------------------------------------------------------------------
// Tests
//--------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[async_std::test]
    async fn test_roots_read_write() {
        let store = MemoryBlockStore::default();
        let mut root_tree = RootTree::with_store(&store);
        root_tree.create_private_root("private").await.unwrap();

        // Public root

        root_tree
            .write(
                &["public".into()],
                &["test".into(), "file".into()],
                b"hello world".to_vec(),
                Utc::now(),
            )
            .await
            .unwrap();

        let content = root_tree
            .read(&["public".into()], &["test".into(), "file".into()])
            .await
            .unwrap();

        assert_eq!(content, b"hello world".to_vec());

        // Exchange root

        root_tree
            .write(
                &["exchange".into()],
                &["test".into(), "file".into()],
                b"hello world".to_vec(),
                Utc::now(),
            )
            .await
            .unwrap();

        let content = root_tree
            .read(&["exchange".into()], &["test".into(), "file".into()])
            .await
            .unwrap();

        assert_eq!(content, b"hello world".to_vec());

        // Private root

        root_tree
            .write(
                &["private".into()],
                &["test".into(), "file".into()],
                b"hello world".to_vec(),
                Utc::now(),
            )
            .await
            .unwrap();

        let content = root_tree
            .read(&["private".into()], &["test".into(), "file".into()])
            .await
            .unwrap();

        assert_eq!(content, b"hello world".to_vec());
    }
}

#[cfg(test)]
mod snapshot_tests {
    use super::*;
    use fake::{faker::chrono::en::DateTime, Fake};
    use rand_chacha::ChaCha12Rng;
    use rand_core::SeedableRng;
    use wnfs_common::utils::MockStore;

    #[async_std::test]
    async fn root_tree() {
        let rng = &mut ChaCha12Rng::seed_from_u64(0);
        let store = &MockStore::default();
        let paths = [
            (["public".into()], vec!["text.txt".into()]),
            (["exchange".into()], vec!["music".into(), "jazz".into()]),
            (
                ["private".into()],
                vec!["videos".into(), "movies".into(), "anime".into()],
            ),
        ];

        let mut root_tree = RootTree::with_store(store);
        root_tree.create_private_root("private").await.unwrap();

        for (root, path) in paths.iter() {
            root_tree
                .write(
                    root,
                    path,
                    b"hello world".to_vec(),
                    DateTime().fake_with_rng(rng),
                )
                .await
                .unwrap();
        }

        let root_cid = root_tree.store(store).await.unwrap();
        let root = store.get_block_snapshot(&root_cid).await.unwrap();

        insta::assert_json_snapshot!(root);
    }
}
