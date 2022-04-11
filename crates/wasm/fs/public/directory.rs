//! The bindgen API of PublicDirectory.

use chrono::{DateTime, Utc};
use wasm_bindgen::prelude::wasm_bindgen;
use wnfs::public::PublicDirectory as WnfsPublicDirectory;

#[wasm_bindgen]
pub struct PublicDirectory(WnfsPublicDirectory);

#[wasm_bindgen]
impl PublicDirectory {
    #[wasm_bindgen(constructor)]
    pub fn new(time: &js_sys::Date) -> PublicDirectory {
        // let time = DateTime::<Utc>::from(time); // TODO(appcypher): Fix this.
        let time = Utc::now();

        PublicDirectory(WnfsPublicDirectory::new(time))
    }
}

#[cfg(test)]
mod public_directory_tests {
    use super::*;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    fn new() {
        let time = &js_sys::Date::new_0();
        let public_directory = PublicDirectory::new(time);
    }
}
