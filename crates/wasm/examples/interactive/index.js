import init, {
  MemoryBlockStore,
  PublicDirectory,
} from "../../pkg/wasm_wnfs.js";

import { draw } from "../common/render.js";

//------------------------------------------------------------------------------
// Initialization
//------------------------------------------------------------------------------

await init();

const time = new Date();
const store = new MemoryBlockStore();
const rootDir = new PublicDirectory(time);
const graphRootElement = document.getElementById("graph-canvas");

//------------------------------------------------------------------------------
// Create starting directory
//------------------------------------------------------------------------------

var { rootNode } = await rootDir.mkdir(["pictures", "cats"], new Date(), store);
var tree = await draw(rootNode, store, graphRootElement);

//------------------------------------------------------------------------------
// Keep certain members around for later
//------------------------------------------------------------------------------

globalThis.scope = {
  store,
  graphRootElement,
  tree,
  rootNode,
};
