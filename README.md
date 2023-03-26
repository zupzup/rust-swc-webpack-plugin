# rust-swc-webpack-plugin

Example of a custom SWC plugin for webpack

* Compile with `nightly-2022-09-23-x86_64-unknown-linux-gnu`
    * If you don't have it installed - use `rustup install nightly-2022-09-23` and `rustup default nightly-2022-09-23-x86_64-unknown-linux-gnu`
* Add the WASM target - `rustup target add wasm32-wasi`
* Build for WASM release - `cargo build-wasi --release`
* In `./jsproject`, run `npm install`
* Then, run `./node_modules/.bin/webpack`
* In `./jsproject/build/bundle.js`, check the output file

