# bf-rs: Brainfuck in Rust

[![Build Status](https://travis-ci.org/tov/libffi-rs.svg?branch=master)](https://travis-ci.org/tov/bf-rs)
[![Crates.io](https://img.shields.io/crates/v/bf.svg?maxAge=2592000)](https://crates.io/crates/bf)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

`bf-rs` is a optimizing Brainfuck interpreter and JIT compiler
inspired by Eli Bendersky’s [series on JIT compilation].
It includes a library crate `bf` that exports most of the functionality,
and an executable `bfi` that provides a command-line interface for executing 
Brainfuck programs.

By default, installing `bf` does not enable the JIT compiler, because
that requires nightly Rust. To build and install from crates.io with the JIT 
enabled:

```
$ cargo +nightly install --features=jit bf
```

If you’re interested in how it works, see [the documentation].

[series on JIT compilation]: http://eli.thegreenplace.net/2017/adventures-in-jit-compilation-part-1-an-interpreter/
[the documentation]: https://tov.github.io/bf-rs/bf/

