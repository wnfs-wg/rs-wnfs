#![allow(clippy::unused_unit)] // To prevent clippy screaming about wasm_bindgen macros.

pub mod fs;

/// Panic hook lets us get better error messages if our Rust code ever panics.
///
/// This function needs to be called at least once during initialisation.
/// https://rustwasm.github.io/docs/wasm-pack/tutorials/npm-browser-packages/template-deep-dive/src-utils-rs.html#2-what-is-console_error_panic_hook
pub fn set_panic_hook() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

// If wee_alloc is enabled, we use the the `wee_alloc` crate to allocate memory which helps us reduce bundle size.
//
// https://rustwasm.github.io/docs/wasm-pack/tutorials/npm-browser-packages/template-deep-dive/wee_alloc.html
cfg_if::cfg_if! {
    if #[cfg(feature = "wee_alloc")] {
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}
