import init, {
  MemoryBlockStore,
  PublicDirectory,
  PublicFile,
} from "../../pkg/wasm_wnfs.js";

import { Tree } from "../common/tree.js";

await init();
const store = new MemoryBlockStore();
const rootDir = new PublicDirectory(new Date());

//------------------------------------------------------------------------------
// Creating folders
//------------------------------------------------------------------------------

console.log("RootNode id: ", rootDir.getId());

// mkdir -p /pictures/cats
var { rootNode } = await rootDir.mkdir(["pictures", "cats"], new Date(), store);

console.log("RootNode id: ", rootNode.getId());

// mkdir -p /pictures/dogs
var { rootNode } = await rootNode
  .asDir()
  .mkdir(["pictures", "dogs"], new Date(), store);

console.log("RootNode id: ", rootNode.getId());

// mkdir -p /videos/dogs
var { rootNode } = await rootNode
  .asDir()
  .mkdir(["videos", "dogs"], new Date(), store);

console.log("RootNode id: ", rootNode.getId());

//------------------------------------------------------------------------------
// Traversing
//------------------------------------------------------------------------------

let tree = new Tree(rootNode, store);

await tree.traverse();

console.log("Levels", tree.levels);
