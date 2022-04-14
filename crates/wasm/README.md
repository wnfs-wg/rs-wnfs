## The WebAssembly API

### Building the Project

- Install `wasm-pack`

```bash
cargo install wasm-pack
```

- Build project

```bash
wasm-pack build --target web
```

### Testing the Project

- Start the test

```bash
wasm-pack test --chrome
```

- Run tests in the browser

```bash
open http://127.0.0.1:8000
```
