# simple-wasm

A small attempt to show how WebAssembly works, it's performance improvements and how simple it can be.

I have implemented the `Fibonacci` algorithm both in Javascript and Rust. Here you can test it in both languages and using WebAssembly to see the performance improvement.

# How to it works ?

## Dependencies

To run this project you need:

- [Node.js](https://nodejs.org/en/download/)
- [Rust and Cargo](https://www.rust-lang.org/tools/install)
- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)(to compile the code to WebAssembly)

## Usage

First, run `wasm-pack build --target nodejs` inside the project folder.

This will compile the Rust code to WebAssembly and create a `pkg` folder on the project root ready to be used by a Node.Js application.

To test the Rust function run `cargo run`.

To Test the JS function and the WebAssembly version run `node src/main.js`
