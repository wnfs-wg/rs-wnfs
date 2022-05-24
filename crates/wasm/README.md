## Wasm WNFS

This package implements the primitives for creating and manipulating IPLD graphs that encode WNFS.

The core of this project is a WebAssembly binary compiled from the [Rust source code](https://github.com/WebNativeFileSystem/rs-wnfs/tree/main/crates/fs).

## Outline

- [Usage](#usage)
- [Setting up the project](#setting-up-the-project)
- [Testing the Project](#testing-the-project)
- [Publishing Package](#publishing-package)

## Usage

Creating a new public directory.

```js
import { PublicDirectory } from "wnfs";

const time = new Date();
const dir = new PublicDirectory(time);
```

The in-memory files and directories you create with `wnfs` will need to be sealed and stored somewhere. For that, an type that implements the BlockStore interface like [this one](https://github.com/WebNativeFileSystem/rs-wnfs/blob/8bb0fbb457051295f1ed4a4707dc230c04612658/crates/wasm/examples/graph/src/blockstore.ts#L9-L29) can be used.

```js
import { MemoryBlockStore } from "./store";
import { PublicDirectory } from "wnfs";

const time = new Date();
const dir = new PublicDirectory(time);
const store = new MemoryBlockStore();

// ...
```

The WNFS API is immutable, therefore, we need to keep track of the updated root directory after every change.

Each fs operation returns a possibly updated root directory that subsequent changes can be applied on.

```js
// ...

// Create a /pictures/cats directory.
var { rootDir } = await dir.mkdir(["pictures", "cats"], time, store);

// Get a sample CIDv1.
const cid = Uint8Array.from([
  1, 112, 18, 32, 195, 196, 115, 62, 200, 175, 253, 6, 207, 158, 159, 245, 15,
  252, 107, 205, 46, 200, 90, 97, 112, 0, 75, 183, 9, 102, 156, 49, 222, 148,
  57, 26,
]);

// Add a file to /pictures/cats.
var { rootDir } = await rootDir.write(
  ["pictures", "cats", "tabby.png"],
  cid,
  time,
  store
);

// Create and add a file to /pictures/dogs directory.
var { rootDir } = await rootDir.write(
  ["pictures", "dogs", "billie.jpeg"],
  cid,
  time,
  store
);

// Delete /pictures/cats directory.
var { rootDir } = await rootDir.rm(["pictures", "cats"], store);

// List all files in /pictures directory.
var { result } = await rootDir.ls(["pictures"], store);

console.log("Files in /pictures directory:", result);
```

## Setting up the Project

- Install `wasm-pack`

  ```bash
  cargo install wasm-pack
  ```

- Install dependencies

  ```bash
  yarn
  ```

- Install playwright binaries

  ```bash
  npx playwright install
  ```

- Build project

  ```bash
  wasm-pack build
  ```

## Testing the Project

- Run tests

  ```bash
  yarn playwright test
  ```

## Publishing Package

- Build the project

  ```bash
  rs-wnfs build --wasm
  ```

- Publish from the `pkg` directory

  ```bash
  cd pkg
  ```

  ```bash
  npm publish
  ```
