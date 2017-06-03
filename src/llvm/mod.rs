//! JIT compiler for Brainfuck based on LLVM.
//!
//! Enabled with `--features=llvm`. This currently doesn't link on some platforms due to trouble
//! with libffi. And when it does link, it appears to use the LLVM bitcode interpreter rather
//! than MCJIT, and consequently is very slow.

mod wrapper;
mod compiler;

pub use self::compiler::{LlvmCompilable, compile_and_run};
