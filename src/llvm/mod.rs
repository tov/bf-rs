//! JIT compiler for Brainfuck based on LLVM.
//!
//! Enabled with `--features=llvm`. This is actually quite slow, because LLVM takes a long time
//! optimizing. However, the actual running of the optimized code appears to be quite fast.

mod wrapper;
mod compiler;

pub use self::compiler::{LlvmCompilable, compile_and_run};
