<div align="center">
  <a href="https://github.com/WebNativeFileSystem" target="_blank">
    <img src="https://raw.githubusercontent.com/WebNativeFileSystem/rs-wnfs/main/assets/logo.svg" alt="Fission Logo" width="100" height="100"></img>
  </a>

  <h1 align="center">WebNative FileSystem (WNFS)</h1>

  <p>
    <a href="https://crates.io/crates/wnfs">
      <img src="https://img.shields.io/crates/v/wnfs?label=crates" alt="Concurrency Docs">
    </a>
    <a href="https://codecov.io/gh/WebNativeFileSystem/rs-wnfs">
      <img src="https://codecov.io/gh/WebNativeFileSystem/rs-wnfs/branch/main/graph/badge.svg?token=95YHXFMFF4" alt="Code Coverage"/>
    </a>
    <a href="https://github.com/WebNativeFileSystem/rs-wnfs/actions?query=">
      <img src="https://github.com/WebNativeFileSystem/rs-wnfs/actions/workflows/checks.yaml/badge.svg" alt="Build Status">
    </a>
    <a href="https://github.com/WebNativeFileSystem/rs-wnfs/blob/main/LICENSE">
      <img src="https://img.shields.io/badge/License-Apache%202.0-blue.svg" alt="License">
    </a>
    <a href="https://docs.rs/wnfs">
      <img src="https://img.shields.io/static/v1?label=Docs&message=docs.rs&color=blue" alt="Concurrency Docs">
    </a>
    <a href="https://discord.gg/zAQBDEq">
      <img src="https://img.shields.io/static/v1?label=Discord&message=join%20us!&color=mediumslateblue" alt="Discord">
    </a>
  </p>
</div>

<div align="center"><sub>:warning: Work in progress. Do not use :warning:</sub></div>

##

This project will implement a pure rust crate for creating and manipulating IPLD graphs that encode WNFS.
Its goal is to be as dependency-less as possible in order to be easily compiled to WebAssembly to be used in the browsers or other environments.

## Building the Project

#### REQUIREMENTS

- Rust toolchain

  Rust toolchain can be installed by following the instructions [here](https://doc.rust-lang.org/cargo/getting-started/installation.html)

- WebAssembly toolchain

  We need to build the project in WebAssembly. To do so, we need to install `wasm-pack` and tweak Rust toolchain context.

  <details>
    <summary>Read more</summary>

  - Install `wasm32-unknown-unknown` target

    ```bash
    rustup target add wasm32-unknown-unknown
    ```

  - [rust-analyzer](https://rust-analyzer.github.io/manual.html#installation) is the go-to IDE tool for Rust and if you have it set up, you may want to set the `rust-analyzer.cargo.target` [setting](https://code.visualstudio.com/docs/getstarted/settings#_workspace-settings) to `wasm32-unknown-unknown`

  - Install wasm-pack

    ```bash
    cargo install wasm-pack
    ```

  </details>

- **wnfs** script

  If you are on a Unix platform, you can optionally install the `wnfs` script.

  <details>
    <summary>Read more</summary>

  - Install it using the following command:

    ```bash
    sh script/wnfs.sh setup
    ```

  - This lets you run the `wnfs.sh` script with just the `wnfs` command.

    ```bash
    wnfs help
    ```

  </details>

#### STEPS

- Clone the repository.

  ```bash
  git https://github.com/WebNativeFileSystem/rs-wnfs.git
  ```

- Change directory

  ```bash
  cd rs-wnfs
  ```

- Build the project

  ```bash
  sh scripts/wnfs.sh build
  ```

## Testing the Project

- Run all tests

  ```bash
  sh scripts/wnfs.sh test
  ```

- Show code coverage

  ```bash
  sh scripts/wnfs.sh coverage
  ```
