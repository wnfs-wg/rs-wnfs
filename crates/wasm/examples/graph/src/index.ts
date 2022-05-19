import "./styles/index.css";

import { MemoryBlockStore } from "./blockstore";
import { draw } from "./render";
import { sampleCID } from "../../../tests/mock";

//------------------------------------------------------------------------------
// Init
//------------------------------------------------------------------------------

const { PublicDirectory } = await import("../../../pkg/index");

const time = new Date();
const store = new MemoryBlockStore();
const root = new PublicDirectory(time);

console.log(`root id at creation: ${root.getId()}`);

//------------------------------------------------------------------------------
// Globals
//------------------------------------------------------------------------------

globalThis.store = store;

//------------------------------------------------------------------------------
// mkdir -p /pictures/cats
//------------------------------------------------------------------------------

var { rootDir } = await root.mkdir(["pictures", "cats"], time, store);
console.log(`root id after "mkdir -p /pictures/cats": ${rootDir.getId()}`);

var tree = await draw(rootDir, store);

// ------------------------------------------------------------------------------
// mkdir -p /pictures/dogs
// ------------------------------------------------------------------------------

var { rootDir } = await rootDir.mkdir(["pictures", "dogs"], time, store);

console.log(`root id after "mkdir -p /pictures/dogs": ${rootDir.getId()}`);

var tree = await draw(rootDir, store, tree);

//------------------------------------------------------------------------------
// mkdir -p /videos/dogs
//------------------------------------------------------------------------------

var { rootDir } = await rootDir.mkdir(["videos", "dogs"], time, store);

console.log(`root id after "mkdir -p /videos/dogs": ${rootDir.getId()}`);

var tree = await draw(rootDir, store, tree);

//------------------------------------------------------------------------------
// echo '...' >> /videos/dogs/puppy.mp4
//------------------------------------------------------------------------------

var { rootDir } = await rootDir.write(
  ["videos", "dogs", "puppy.mp4"],
  sampleCID,
  time,
  store
);

console.log(
  `root id after "echo '...' >> /videos/dogs/puppy.mp4": ${rootDir.getId()}`
);

var tree = await draw(rootDir, store, tree);

//------------------------------------------------------------------------------
// echo '...' >> /pictures/cats/kitten.png
//------------------------------------------------------------------------------

var { rootDir } = await rootDir.write(
  ["pictures", "cats", "kitten.png"],
  sampleCID,
  time,
  store
);

console.log(
  `root id after "echo '...' >> /pictures/cats/kitten.png": ${rootDir.getId()}`
);

var tree = await draw(rootDir, store, tree);

//------------------------------------------------------------------------------
// mkdir -p /music/rock
//------------------------------------------------------------------------------

var { rootDir } = await rootDir.mkdir(["music", "rock"], time, store);

console.log(`root id after "mkdir -p /music/rock": ${rootDir.getId()}`);

var tree = await draw(rootDir, store, tree);

//------------------------------------------------------------------------------
// echo '...' >>  /music/rock/toxicity.mp3
//------------------------------------------------------------------------------

var { rootDir } = await rootDir.write(
  ["music", "rock", "toxicity.mp3"],
  sampleCID,
  time,
  store
);

console.log(
  `root id after "echo '...' >> /music/rock/toxicity.mp3": ${rootDir.getId()}`
);

var tree = await draw(rootDir, store, tree);

//------------------------------------------------------------------------------
// rm  /pictures/cats/kitten.png
//------------------------------------------------------------------------------

var { rootDir } = await rootDir.rm(["pictures", "cats", "kitten.png"], store);

console.log(`root id after "rm /pictures/cats/kitten.png": ${rootDir.getId()}`);

var tree = await draw(rootDir, store, tree);

//------------------------------------------------------------------------------
// echo '...' >>  /movies/anime/ghibli/ponyo.mov
//------------------------------------------------------------------------------

var { rootDir } = await rootDir.write(
  ["movies", "anime", "ghibli", "ponyo.mov"],
  sampleCID,
  time,
  store
);

console.log(
  `root id after "echo '...' >> /movies/anime/ghibli/ponyo.mov": ${rootDir.getId()}`
);

var tree = await draw(rootDir, store, tree);
