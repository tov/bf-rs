//! The peephole optimizir, which replaces common loop forms with single (non-Brainfuck)
//! instructions.
//!
//! For example, we detect the pattern `[-]`, which decrements the current byte until it reaches
//! zero, and replaces it with the [`SetZero`](../../src/bf/peephole/mod.rs.html#21-22)
//! instruction. See the [`Instruction`](enum.Instruction.html) enum for a list of the
//! instructions produced by the [peephole compiler](fn.compile.html).

mod interpreter;
mod compiler;

pub use self::compiler::compile;

/// At this level, a program is a rose tree of instructions.
///
/// All instructions are leaves except for the `Loop` instruction, which contains a boxed `Program`.
pub type Program = [Instruction];

/// Instructions as output by the peephole optimizer.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Instruction {
    /// Move the pointer left by the specified offset.
    Left(usize),
    /// Move the pointer right by the specified offset.
    Right(usize),
    /// Change the value at the pointer by the specified offset.
    Change(u8),
    /// Read input.
    In,
    /// Write output.
    Out,
    /// Set the current byte to 0
    SetZero,
    /// Add the byte at the pointer to the byte at the specified offset and zero the byte at the
    /// pointer.
    OffsetAddRight(usize),
    /// Add the byte at the pointer to the byte at the specified offset and zero the byte at the
    /// pointer.
    OffsetAddLeft(usize),
    /// Move the pointer to a zero, skipping the offset at a time.
    FindZeroRight(usize),
    /// Move the pointer to a zero, skipping the offset at a time.
    FindZeroLeft(usize),
    /// A loop
    Loop(Box<Program>),
}