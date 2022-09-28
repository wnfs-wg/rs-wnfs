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

//------------------------------------------------------------------------------
// mkdir -p /videos/dogs
//------------------------------------------------------------------------------

var { rootDir } = await rootDir.mkdir(["videos", "dogs"], time, store);

console.log(`root id after "mkdir -p /videos/dogs": ${rootDir.getId()}`);

var tree = await draw(rootDir, store, tree);
