# Lucifer

He brings the light.

Rust raytracer implementation based on [raytracer in a weekend](https://raytracing.github.io/books/RayTracingInOneWeekend.html).

Implemented monolithically with WASM in mind as a compilation target, not because it makes sense but because it provided me with the opportunity to learn more things at once.

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
