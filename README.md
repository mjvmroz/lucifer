# Lucifer

He brings the light.

## Pre-requisites

-   Rust ([install](https://rustup.rs/)) – The Rust programming language and toolchain
-   (And then get some delicious nightly goodness)
    ```bash
    rustup component add rust-src --toolchain nightly-2021-07-29-x86_64-unknown-linux-gnu
    ```
-   `wasm-pack` ([install](https://rustwasm.github.io/wasm-pack/installer/)) – Tools to build WASM binaries and bindings
-   NodeJS ([install](https://nodejs.org/en/download/package-manager/)) – The Only Webscale™ runtime and package manager
-   Yarn ([install](https://yarnpkg.com/getting-started/install)) – A Better™ package manager for NodeJS

## Install NPM dependencies

```bash
cd client
yarn
```

## Build and run the app

Out of the box, it will start on http://0.0.0.0:9090

```bash
cd client
yarn start
```
