import init, { MemoryBlockStore, PublicDirectory } from "../pkg/wasm_wnfs.js";

window.wnfs = {
  init,
  MemoryBlockStore,
  PublicDirectory,
};
