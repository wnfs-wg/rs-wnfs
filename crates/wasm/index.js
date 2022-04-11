import init, { lookup_node } from "./pkg/wasm_wnfs.js";

init().then(async () => {
  console.log(">>> Getting started 2");

  const result = await lookup_node();

  console.log(result);
});
