use crate::BlockStore;
use anyhow::Result;
use async_trait::async_trait;
use libipld::{Cid, IpldCodec};
use serde::{Deserialize, Serialize, Serializer};
use std::{borrow::Cow, path::PathBuf};

/// A disk-based blockstore that you can mutate.
#[derive(Debug)]
pub struct DiskBlockStore {
    pub path: PathBuf,
}

// -------------------------------------------------------------------------------------------------
// Implementations
// -------------------------------------------------------------------------------------------------

impl DiskBlockStore {
    /// Creates a new disk block store.
    pub fn new(path: PathBuf) -> Self {
        // Return the new DiskBlockStore
        Self { path }
    }

    pub fn erase(&self) -> Result<()> {
        // Remove the directory
        std::fs::remove_dir_all(&self.path)?;
        // Return Ok status
        Ok(())
    }
}

impl Clone for DiskBlockStore {
    fn clone(&self) -> Self {
        Self::new(self.path.clone())
    }
}

impl Serialize for DiskBlockStore {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Serialize the path
        self.path.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for DiskBlockStore {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Deserialize the path
        let path = PathBuf::deserialize(deserializer)?;
        // Return Ok status with the new DiskBlockStore
        Ok(Self::new(path))
    }
}

#[async_trait(?Send)]
impl BlockStore for DiskBlockStore {
    /// Stores an array of bytes in the block store.
    async fn put_block(&self, bytes: Vec<u8>, codec: IpldCodec) -> Result<Cid> {
        // If the parent directory doesn't already exist
        if !self.path.exists() {
            // Create the directories required to store the blocks
            std::fs::create_dir_all(&self.path).unwrap();
        }

        // Try to build the CID from the bytes and codec
        let cid = self.create_cid(&bytes, codec)?;
        let file_path = self.path.join(cid.to_string());

        // If this file has not already been written to disk
        if !file_path.exists() {
            // Create the file at the specified path
            let mut file = std::fs::File::create(file_path)?;
            // Write the bytes to disk at the File location
            std::io::Write::write_all(&mut file, &bytes)?;
        }

        // Return Ok status with the generated CID
        Ok(cid)
    }

    /// Retrieves an array of bytes from the block store with given CID.
    async fn get_block(&self, cid: &Cid) -> Result<Cow<Vec<u8>>> {
        // Get the bytes from disk, using the given CID as the filename
        let mut file = std::fs::File::open(self.path.join(cid.to_string()))?;
        // Create a mutable vector of bytes
        let mut bytes: Vec<u8> = Vec::new();
        // Read the bytes into that
        std::io::Read::read_to_end(&mut file, &mut bytes)?;
        // Return Ok status with the bytes
        return Ok(Cow::Owned(bytes));
    }
}
