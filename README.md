# WASM Rust CLI
Experimentation with wasmtime as a means for developing WASM-powered cli apps using [wasmtime](https://docs.wasmtime.dev)

this assumes `rustup` has already been installed.


## Instructions
```
# get and install wasmtime
# you'll then have to source your environment
# or restart your terminal
curl https://wasmtime.dev/install.sh -sSf | bash

# add the wasm target to rustup
rustup target add wasm32-wasi

# compile to WASM
rustc gcd.rs --target wasm32-wasi

# execute the WASM code  you just compiled
wasmtime gcd.wasm 1288822 12472
```
