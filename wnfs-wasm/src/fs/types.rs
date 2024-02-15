use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(typescript_custom_section)]
const TS_BLOCKSTORE: &'static str = r#"
export interface BlockStore {
    putBlockKeyed(cid: Uint8Array, bytes: Uint8Array): Promise<void>;
    getBlock(cid: Uint8Array): Promise<Uint8Array | undefined>;
    hasBlock(cid: Uint8Array): Promise<boolean>;
}
"#;
