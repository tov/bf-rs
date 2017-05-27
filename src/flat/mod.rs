//! Brainfuck bytecode, flat instead of an abstract syntax tree.
//!
//! This is closer to
//!
//! Previous passes represent the Brainfuck program as an [abstract syntax
//! tree](../ast.index.html), with
//! child trees for loops. This pass transforms the program to a flat array of
//! [`Instruction`](enum.Instruction.html)s, where loop operations take the
//! address to possibly jump to as a parameter. This representation includes
//! run-length encoding for some instructions, with moving and arithmetic
//! commands taking the count as a parameter. It also includes the
//! instructions produced by the peephole optimizer.

use common;

mod compiler;
mod interpreter;

pub use self::compiler::compile;

/// A program is a flat sequence of instructions.
pub type Program = [common::Instruction];

