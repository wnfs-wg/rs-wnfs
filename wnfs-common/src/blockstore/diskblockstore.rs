use crate::BlockStore;
use anyhow::Result;
use async_trait::async_trait;
use libipld::{Cid, IpldCodec};
use std::{borrow::Cow, path::PathBuf};

/// A disk-based blockstore that you can mutate.
pub struct DiskBlockStore {
    pub path: PathBuf
}

// -------------------------------------------------------------------------------------------------
// Implementations
// -------------------------------------------------------------------------------------------------

impl DiskBlockStore {
    /// Creates a new disk block store.
    pub fn new(path: PathBuf) -> Self {
        // Ensure the directory is empty, if it exists
        if path.exists() {
            // Remove the directory and its contents
            std::fs::remove_dir_all(&path).unwrap();
        }

        // Create the directories required to store the blocks
        std::fs::create_dir_all(&path).unwrap();
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

#[async_trait(?Send)]
impl BlockStore for DiskBlockStore {
    /// Stores an array of bytes in the block store.
    async fn put_block(&self, bytes: Vec<u8>, codec: IpldCodec) -> Result<Cid> {
        // Try to build the CID from the bytes and codec
        let cid = self.create_cid(bytes.clone(), codec)?;
        // Create the file at the specified path
        let mut file = std::fs::File::create(self.0.join(cid.to_string()))?;
        // Write the bytes to disk at the File location
        std::io::Write::write_all(&mut file, &bytes)?;
        // Return Ok status with the generated CID
        Ok(cid)
    }

    /// Retrieves an array of bytes from the block store with given CID.
    async fn get_block(&self, cid: &Cid) -> Result<Cow<Vec<u8>>> {
        // Get the bytes from disk, using the given CID as the filename
        let mut file = std::fs::File::open(self.0.join(cid.to_string()))?;
        // Create a mutable vector of bytes
        let mut bytes: Vec<u8> = Vec::new();
        // Read the bytes into that
        std::io::Read::read_to_end(&mut file, &mut bytes)?;
        // Return Ok status with the bytes
        return Ok(Cow::Owned(bytes));
    }
}
