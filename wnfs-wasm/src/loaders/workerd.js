// @ts-nocheck
// This file is inserted into ./web by the build script

import WASM from './wasm_bg.wasm'
import { initSync } from './wasm.js'
initSync(WASM)
export * from './wasm.js'
