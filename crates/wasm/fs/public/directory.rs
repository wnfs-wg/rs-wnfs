//! The bindgen API for PublicDirectory.

use chrono::{DateTime, Utc};
use wasm_bindgen::prelude::wasm_bindgen;
use wnfs::public::PublicDirectory as WnfsPublicDirectory;

/// A directory in a WNFS public file system.
#[wasm_bindgen]
pub struct PublicDirectory(WnfsPublicDirectory);

#[wasm_bindgen]
impl PublicDirectory {
    /// Creates a new directory using the given metadata.
    #[wasm_bindgen(constructor)]
    pub fn new(time: &js_sys::Date) -> PublicDirectory {
        let time = DateTime::<Utc>::from(time);
        PublicDirectory(WnfsPublicDirectory::new(time))
    }
}

#[cfg(test)]
mod public_directory_tests {
    use super::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn create_directory_successfully() {
        let time = &js_sys::Date::new_0();
        let _public_directory = PublicDirectory::new(time);
    }
}
