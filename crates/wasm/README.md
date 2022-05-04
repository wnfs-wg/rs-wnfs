## The WebAssembly API

## Building the Project

- Install `wasm-pack`

  ```bash
  cargo install wasm-pack
  ```

- Build project

  ```bash
  wasm-pack build --target web
  ```

- Run tests

  ```bash
  yarn playwright test
  ```

## Trying the Examples

- Run server

  ```bash
  npx http-server -p 8080
  ```

- Visit the following pages:

  ```bash
  open http://localhost:8080/examples/graph/
  ```

NOTE: Examples will be moved into separate projects in the future.
