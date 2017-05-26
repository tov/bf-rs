# bf-rs
Brainfuck in Rust

`bf-rs` is a Brainfuck interpreter and JIT compiler
based on [Eli Bendersky’s series on JIT
compilation](http://eli.thegreenplace.net/2017/adventures-in-jit-compilation-part-1-an-interpreter/).
It includes a library crate, `bf`, that exports most of the
functionality, and an 
executable `bfi` that provides a command-line interface for executing 
Brainfuck programs.
