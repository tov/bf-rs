# bf-rs: Brainfuck in Rust

`bf-rs` is a optimizing Brainfuck interpreter and JIT compiler
inspired by Eli Bendersky’s [series on JIT compilation].
It includes a library crate `bf` that exports most of the functionality,
and an executable `bfi` that provides a command-line interface for executing 
Brainfuck programs.

By default, installing `bf` does not enable the JIT compiler, because
that requires nightly Rust. To build and install from crates.io with the JIT 
enabled:

```
$ rustup run nightly cargo install --features=jit bf
```

If you’re interested in how it works, see [the documentation].

[series on JIT compilation]: http://eli.thegreenplace.net/2017/adventures-in-jit-compilation-part-1-an-interpreter/
[the documentation]: https://tov.github.io/bf-rs/bf/

