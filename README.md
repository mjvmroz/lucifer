# Lucifer

He brings the light.

## Pre-requisites

-   Rust ([install](https://rustup.rs/)) – The Rust programming language and toolchain
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

## The new UI

I'm replacing the UI with a Yew one. It just seems way too convenient to ignore compared to the sketchy interop experience, particularly due to type safety and memory management.

You may need to do stuff like:

```bash
# Tools
cargo install trunk wasm-bindgen-cli
rustup target add wasm32-unknown-unknown

# Start the app
trunk serve
```
