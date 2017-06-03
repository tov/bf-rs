//! JIT compiler for Brainfuck based on LLVM.

mod wrapper;
mod compiler;

pub use self::compiler::{LlvmCompilable, compile_and_run};
