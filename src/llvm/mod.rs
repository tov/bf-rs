//! JIT compiler for Brainfuck based on LLVM.

mod compiler;

pub use self::compiler::compile;
