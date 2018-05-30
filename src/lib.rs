#![doc(html_root_url = "http://tov.github.io/bf-rs")]
//!
//! `bf-rs` is a optimizing Brainfuck interpreter and JIT compiler
//! inspired by Eli Bendersky’s [series on JIT compilation].
//! It includes a library crate `bf` that exports most of the functionality,
//! and an executable `bfi` that provides a command-line interface for executing
//! Brainfuck programs.
//!
//! This crate supports Rust version 1.20 and later. However,
//! by default, installing `bf` does not enable the JIT compiler because
//! that requires nightly Rust. To build and install from crates.io with the native x86-64
//! JIT enabled:
//!
//! ```shell
//! $ cargo +nightly install bf --features=jit
//! ```
//!
//! [series on JIT compilation]: http://eli.thegreenplace.net/2017/adventures-in-jit-compilation-part-1-an-interpreter/
//!
//! This library implements a number of compilation passes:
//!
//!  - First, Brainfuck concrete syntax is parsed into
//! [an abstract syntax tree](ast/index.html).
//!
//!  - Then, repeated sequences of the same command are
//! [run-length encoded](rle/index.html).
//!
//!  - Then, common loop forms are converted to new (non-Brainfuck)
//! instructions by the [peephole optimizer](peephole/index.html).
//!
//!  - The peephole output can be [flattened to bytecode](bytecode/index.html),
//! which is then interpreted.
//!
//!  - Or, if the `jit` feature is enabled (nightly only), the peephole output
//! can be [just-in-time compiled to x64 machine code](jit/index.html).
//!
//!  - Or, if the `llvm` feature is enabled (LLVM ≥ 3.8 must be in the PATH to build),
//! the peephole output can be [JIT compiled using LLVM](llvm/index.html).
//! (This is quite slow right now.)
//!
//! Interpreters are provided for the intermediate forms as well. In particular,
//! all representations of Brainfuck programs implement the
//! [`Interpretable`](traits/trait.Interpretable.html) trait.

#![cfg_attr(feature = "jit", feature(plugin))]
#![cfg_attr(feature = "jit", plugin(dynasm))]

#[cfg(feature = "jit")]
extern crate dynasmrt;

#[cfg(feature = "llvm")]
extern crate llvm_sys;

pub mod common;
pub mod state;
pub mod traits;
pub mod rts;

pub mod ast;
pub mod rle;
pub mod bytecode;
pub mod peephole;

#[cfg(feature = "jit")]
pub mod jit;

#[cfg(feature = "llvm")]
pub mod llvm;

pub mod test_helpers;
