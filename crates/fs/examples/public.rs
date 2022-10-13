//! This example shows how the different operations you can perform under a public filesystem.
//! More importantly, it shows the immutable nature of the filesystem.

use wnfs::{ipld::Cid, MemoryBlockStore, PublicDirectory, PublicOpResult};

use chrono::Utc;

use std::rc::Rc;

#[async_std::main]
async fn main() {
    // Create a new public directory.
    let dir = Rc::new(PublicDirectory::new(Utc::now()));

    // Create a memory-based blockstore.
    let store = MemoryBlockStore::default();

    // Add a /pictures/cats subdirectory.
    let PublicOpResult { root_dir, .. } = dir
        .mkdir(&["pictures".into(), "cats".into()], Utc::now(), &store)
        .await
        .unwrap();

    // Add a file to /pictures/dogs directory.
    let PublicOpResult { root_dir, .. } = root_dir
        .write(
            &["pictures".into(), "dogs".into(), "billie.jpeg".into()],
            Cid::default(),
            Utc::now(),
            &store,
        )
        .await
        .unwrap();

    // Delete /pictures/cats directory.
    let PublicOpResult { root_dir, .. } = root_dir
        .rm(&["pictures".into(), "cats".into()], &store)
        .await
        .unwrap();

    // List all files in /pictures directory.
    let PublicOpResult { result, .. } = root_dir.ls(&["pictures".into()], &store).await.unwrap();

    println!("Files in /pictures: {:#?}", result);
}
