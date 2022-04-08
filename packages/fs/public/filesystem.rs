//! The public file system API.

use super::PublicDirectory;
use crate::common::BlockStore;
use anyhow::Result;

//--------------------------------------------------------------------------------------------------
// Type Definitions
//--------------------------------------------------------------------------------------------------

/// The WNFS public file system.
struct PublicFileSystem<'s, T: BlockStore> {
    blockstore: &'s T,
    root_dir: PublicDirectory,
}

//--------------------------------------------------------------------------------------------------
// Implementations
//--------------------------------------------------------------------------------------------------

impl<'s, T: BlockStore> PublicFileSystem<'s, T> {
    /// Creates a new WNFS public file system.
    pub fn new(blockstore: &'s T, root_dir: PublicDirectory) -> Self {
        Self {
            blockstore,
            root_dir,
        }
    }

    /// Reads file content at the specified path from the file system.
    pub async fn read(path_segments: &[String]) -> Result<()> {
        todo!(
            r#"
            This is entering javascript land.
            We can make writing file blocks to linear memory and return the base address.
            "#
        )
    }
}
