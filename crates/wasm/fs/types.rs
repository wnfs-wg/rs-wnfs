use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(typescript_custom_section)]
const TS_BLOCKSTORE: &'static str = r#"
export interface BlockStore {
    putBlock(bytes: Uint8Array, code: number): Promise<Uint8Array>;
    getBlock(cid: Uint8Array): Promise<Uint8Array | undefined>;
}
"#;
