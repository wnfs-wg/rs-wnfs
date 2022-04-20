import init, {
  MemoryBlockStore,
  PublicDirectory,
} from "../../pkg/wasm_wnfs.js";

import { Tree } from "../common/tree.js";

await init();
const time = new Date();
const store = new MemoryBlockStore();
const rootDir = new PublicDirectory(new Date());

//------------------------------------------------------------------------------
// Creating folders
//------------------------------------------------------------------------------

console.log(`root id at creation: ${rootDir.getId()}`);

// mkdir -p /pictures/cats
var { rootNode } = await rootDir.mkdir(["pictures", "cats"], new Date(), store);

console.log(`root id after "mkdir -p /pictures/cats": ${rootDir.getId()}`);

// mkdir -p /pictures/dogs
var { rootNode } = await rootNode
  .asDir()
  .mkdir(["pictures", "dogs"], new Date(), store);

console.log(`root id after "mkdir -p /pictures/dogs": ${rootDir.getId()}`);

// mkdir -p /videos/dogs
var { rootNode } = await rootNode
  .asDir()
  .mkdir(["videos", "dogs"], new Date(), store);

console.log(`root id after "mkdir -p /videos/dogs": ${rootDir.getId()}`);

// echo '...' >> /videos/dogs/puppy.png
var { rootNode } = await rootNode
  .asDir()
  .write(
    ["videos", "dogs", "puppy.png"],
    "bafybeigdyrzt5sfp7udm7hu76uh7y26nf3efuylqabf3oclgtqy55fbzdi",
    time,
    store
  );

console.log(
  `root id after "echo '...' >> /videos/dogs/puppy.png": ${rootDir.getId()}`
);

//------------------------------------------------------------------------------
// Traversing
//------------------------------------------------------------------------------

let tree = new Tree(rootNode, store);

await tree.traverse();

console.log("tree levels", tree.levels);
