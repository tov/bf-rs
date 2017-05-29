//! The peephole optimizer, which replaces common loop forms with single (non-Brainfuck)
//! instructions.
//!
//! In `bfi`, this is the default final pass if the JIT was not enabled at compile time
//! (with `--features=jit`). If the JIT is present, the peephole optimizer can be selected as
//! the final path with the `--peep` flag.
//!
//! For example, we detect the pattern `[-]`, which decrements the current byte until it reaches
//! zero, and replaces it with the [`SetZero`](../../src/bf/peephole/mod.rs.html#21-22)
//! instruction. See the [`bytecode::Instruction`](../bytecode/enum.Instruction.html) enum for a list of
//! the instructions produced by the [peephole compiler](fn.compile.html).

use common;

mod interpreter;
mod compiler;

pub use self::compiler::{compile, PeepholeCompilable};

/// At this level, a program is a rose tree of statements.
///
/// All instructions are leaves except for the `Loop` instruction, which contains a boxed `Program`.
pub type Program = [Statement];

/// Instructions as output by the peephole optimizer.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Statement {
    /// A bytecode instruction, which does not contain any loops.
    ///
    /// # Invariants
    ///
    /// Should not contain a `JumpZero` or `JumpNotZero` instruction.
    Instr(common::Instruction),
    /// A loop.
    Loop(Box<[Statement]>),
}
