use super::{PublicDirectory, PublicNode};
use crate::traits::Time;
use anyhow::Result;
use chrono::Utc;
use libipld::Cid;
use std::rc::Rc;
use wnfs_common::{BlockStore, Metadata};

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub struct MutablePublicDirectory<'b, B: BlockStore, T: Time = Utc> {
    pub store: &'b mut B,
    pub root: Rc<PublicDirectory>,
    phantom: std::marker::PhantomData<T>,
}

//--------------------------------------------------------------------------------------------------
// Implementations
//--------------------------------------------------------------------------------------------------

impl<'b, B, T> MutablePublicDirectory<'b, B, T>
where
    B: BlockStore,
    T: Time,
{
    pub async fn new(store: &'b mut B) -> Result<MutablePublicDirectory<'b, B, T>> {
        let root = Rc::new(PublicDirectory::new(T::now()));

        Ok(Self {
            store,
            root,
            phantom: std::marker::PhantomData,
        })
    }

    pub async fn lookup_node<'a>(&'a self, path_segment: &str) -> Result<Option<&'a PublicNode>> {
        self.root.lookup_node(path_segment, self.store).await
    }

    pub async fn get_node<'a>(
        &'a self,
        path_segments: &[String],
    ) -> Result<Option<&'a PublicNode>> {
        self.root.get_node(path_segments, self.store).await
    }

    pub async fn write(&mut self, path_segments: &[String], content_cid: Cid) -> Result<()> {
        self.root
            .write(path_segments, content_cid, T::now(), self.store)
            .await
    }

    pub async fn mkdir(&mut self, path_segments: &[String]) -> Result<()> {
        self.root.mkdir(path_segments, T::now(), self.store).await
    }

    pub async fn ls(&self, path_segments: &[String]) -> Result<Vec<(String, Metadata)>> {
        self.root.ls(path_segments, self.store).await
    }

    pub async fn rm(&mut self, path_segments: &[String]) -> Result<PublicNode> {
        self.root.rm(path_segments, self.store).await
    }

    pub async fn basic_mv(
        &mut self,
        path_segments_from: &[String],
        path_segments_to: &[String],
    ) -> Result<()> {
        self.root
            .basic_mv(path_segments_from, path_segments_to, T::now(), self.store)
            .await
    }
}

//--------------------------------------------------------------------------------------------------
// Tests
//--------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use wnfs_common::MemoryBlockStore;

    #[async_std::test]
    async fn can_create_directory() {
        let store = &mut MemoryBlockStore::new();

        let _ = MutablePublicDirectory::<_, Utc>::new(store).await.unwrap();
    }
}
