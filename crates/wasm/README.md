## The WebAssembly API

## Building the Project

- Install `wasm-pack`

  ```bash
  cargo install wasm-pack
  ```

- Install playwrigth binaries

  ```bash
  npx playwright install
  ```

- Build project

  ```bash
  wasm-pack build --target web
  ```

- Run tests

  ```bash
  yarn playwright test
  ```
