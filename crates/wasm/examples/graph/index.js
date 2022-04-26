import init, { MemoryBlockStore, PublicDirectory } from "./wasm_wnfs.js";

import { draw } from "./render.js";

//------------------------------------------------------------------------------
// Init
//------------------------------------------------------------------------------

await init();

const time = new Date();
const store = new MemoryBlockStore();
const rootDir = new PublicDirectory(time);

console.log(`root id at creation: ${rootDir.getId()}`);

//------------------------------------------------------------------------------
// Globals
//------------------------------------------------------------------------------

globalThis.store = store;

//------------------------------------------------------------------------------
// mkdir -p /pictures/cats
//------------------------------------------------------------------------------

var { rootNode } = await rootDir.mkdir(["pictures", "cats"], time, store);
console.log(`root id after "mkdir -p /pictures/cats": ${rootNode.getId()}`);

var tree = await draw(rootNode, store);

// //------------------------------------------------------------------------------
// // mkdir -p /pictures/dogs
// //------------------------------------------------------------------------------

// var { rootNode } = await rootNode
//   .asDir()
//   .mkdir(["pictures", "dogs"], time, store);

// console.log(`root id after "mkdir -p /pictures/dogs": ${rootNode.getId()}`);

// var tree = await draw(rootNode, store, tree);

// //------------------------------------------------------------------------------
// // mkdir -p /videos/dogs
// //------------------------------------------------------------------------------

// var { rootNode } = await rootNode
//   .asDir()
//   .mkdir(["videos", "dogs"], time, store);

// console.log(`root id after "mkdir -p /videos/dogs": ${rootNode.getId()}`);

// var tree = await draw(rootNode, store, tree);

// //------------------------------------------------------------------------------
// // echo '...' >> /videos/dogs/puppy.mp4
// //------------------------------------------------------------------------------

// var { rootNode } = await rootNode
//   .asDir()
//   .write(
//     ["videos", "dogs", "puppy.mp4"],
//     "bafybeigdyrzt5sfp7udm7hu76uh7y26nf3efuylqabf3oclgtqy55fbzdi",
//     time,
//     store
//   );

// console.log(
//   `root id after "echo '...' >> /videos/dogs/puppy.mp4": ${rootNode.getId()}`
// );

// var tree = await draw(rootNode, store, tree);

// //------------------------------------------------------------------------------
// // echo '...' >> /pictures/cats/kitten.png
// //------------------------------------------------------------------------------

// var { rootNode } = await rootNode
//   .asDir()
//   .write(
//     ["pictures", "cats", "kitten.png"],
//     "bafybeigdyrzt5sfp7udm7hu76uh7y26nf3efuylqabf3oclgtqy55fbzdi",
//     time,
//     store
//   );

// console.log(
//   `root id after "echo '...' >> /pictures/cats/kitten.png": ${rootNode.getId()}`
// );

// var tree = await draw(rootNode, store, tree);

// //------------------------------------------------------------------------------
// // mkdir -p /music/rock
// //------------------------------------------------------------------------------

// var { rootNode } = await rootNode.asDir().mkdir(["music", "rock"], time, store);

// console.log(`root id after "mkdir -p /music/rock": ${rootNode.getId()}`);

// var tree = await draw(rootNode, store, tree);

// //------------------------------------------------------------------------------
// // echo '...' >>  /music/rock/toxicity.mp3
// //------------------------------------------------------------------------------

// var { rootNode } = await rootNode
//   .asDir()
//   .write(
//     ["music", "rock", "toxicity.mp3"],
//     "bafybeigdyrzt5sfp7udm7hu76uh7y26nf3efuylqabf3oclgtqy55fbzdi",
//     time,
//     store
//   );

// console.log(
//   `root id after "echo '...' >> /music/rock/toxicity.mp3": ${rootNode.getId()}`
// );

// var tree = await draw(rootNode, store, tree);

// //------------------------------------------------------------------------------
// // rm  /pictures/cats/kitten.png
// //------------------------------------------------------------------------------

// var { rootNode } = await rootNode
//   .asDir()
//   .rm(["pictures", "cats", "kitten.png"], store);

// console.log(
//   `root id after "rm /pictures/cats/kitten.png": ${rootNode.getId()}`
// );

// var tree = await draw(rootNode, store, tree);

// //------------------------------------------------------------------------------
// // echo '...' >>  /movies/anime/ghibli/ponyo.mov
// //------------------------------------------------------------------------------

// var { rootNode } = await rootNode
//   .asDir()
//   .write(
//     ["movies", "anime", "ghibli", "ponyo.mov"],
//     "bafybeigdyrzt5sfp7udm7hu76uh7y26nf3efuylqabf3oclgtqy55fbzdi",
//     time,
//     store
//   );

// console.log(
//   `root id after "echo '...' >> /movies/anime/ghibli/ponyo.mov": ${rootNode.getId()}`
// );

// var tree = await draw(rootNode, store, tree);
