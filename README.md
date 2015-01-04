# cargo-emscripten

Builds your Cargo projects for emscripten.

**Warning: everything here is very experimental. There are tons of warnings when compiling, tons of things are that just wrong, and the code is very ugly.**

## Pre-LLVM-3.5 warning

[Emscripten will soon upgrade to LLVM 3.5](https://github.com/kripken/emscripten-fastcomp/issues/51). In the meanwhile, **some code will produce compilation errors in emscripten**. If you see an assertion that failed with `some i64 things that we can't allow yet`, it's because of this.

## Pre-Rust-1.0 warning

This project is in stand-by before Rust becomes a bit more stable.

## Prerequisites

- You have to install [the `incoming` version of emscripten](http://kripken.github.io/emscripten-site/docs/tools_reference/emsdk.html#how-do-i-track-the-latest-emscripten-development-with-the-sdk).
- You can't use the std for now or it won't compile (but you don't need to put `#![no_std]` in your code). [Here is an example of a code that works](https://gist.github.com/tomaka/24c058db5ae31dfafb3f) if you just want to try it.
- In order to build `cargo-emscripten`, you need to use the same nightlies as Cargo. Run the `.travis.install.deps.sh` file from rust-lang/cargo.

## How to use it

Start by compiling `cargo-emscripten` (this project).

Running `cargo-emscripten` is the same as running `cargo build`, except that the project will be compiled for emscripten. For the moment only HTML output is supported.

If you don't have `emcc` in your PATH, you can pass the `--emcc` command line option to `cargo-emscripten` in order to indicate its location.
