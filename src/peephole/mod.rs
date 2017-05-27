//! The peephole optimizer, which replaces common loop forms with single (non-Brainfuck)
//! instructions.
//!
//! In `bfi`, this is the default final pass if the JIT was not enabled at compile time
//! (with `--features=jit`). If the JIT is present, the peephole optimizer can be selected as
//! the final path with the `--peep` flag.
//!
//! For example, we detect the pattern `[-]`, which decrements the current byte until it reaches
//! zero, and replaces it with the [`SetZero`](../../src/bf/peephole/mod.rs.html#21-22)
//! instruction. See the [`flat::Instruction`](../flat/enum.Instruction.html) enum for a list of
//! the instructions produced by the [peephole compiler](fn.compile.html).

use flat;

mod interpreter;
mod compiler;

pub use self::compiler::compile;
pub use rle::Count;

/// At this level, a program is a rose tree of instructions.
///
/// All instructions are leaves except for the `Loop` instruction, which contains a boxed `Program`.
pub type Program = [Instruction];

/// Instructions as output by the peephole optimizer.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Instruction {
    /// A bytecode instruction, which does not contain any loops.
    ///
    /// # Invariants
    ///
    /// Should not contain a `JumpZero` or `JumpNotZero` instruction.
    Flat(flat::Instruction),
    /// A loop.
    Loop(Box<[Instruction]>),
}