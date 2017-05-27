//! The peephole optimizer, which replaces common loop forms with single (non-Brainfuck)
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
    /// Decrease the pointer by the specified offset.
    Left(usize),
    /// Increase the pointer by the specified offset.
    Right(usize),
    /// Increase the current byte value by the specified offset.
    ///
    /// Because arithmetic is modulo 256, this implements subtraction as well as addition.
    Change(u8),
    /// Read a byte of input.
    In,
    /// Write a byte of output.
    Out,
    /// Set the current byte value to 0.
    ///
    /// Equivalent to the concrete Braincode loop `[-]`.
    SetZero,
    /// Add the byte at the pointer to the byte at the specified offset and zero the byte at the
    /// pointer.
    ///
    /// `OffsetAddRight(5)` is equivalent to the concrete Brainfuck loop `[->>>>>+<<<<<]`.
    OffsetAddRight(usize),
    /// Add the byte at the pointer to the byte at the specified offset and zero the byte at the
    /// pointer.
    ///
    /// `OffsetAddRight(5)` is equivalent to the concrete Brainfuck loop `[-<<<<<+>>>>>]`.
    OffsetAddLeft(usize),
    /// Finds the nearest zero to the left that appears offset by a multiple of the given `usize`.
    ///
    /// `FindZeroRight(3)` is equivalent to the concrete Brainfuck loop `[>>>]`.
    FindZeroRight(usize),
    /// Finds the nearest zero to the right that appears offset by a multiple of the given `usize`.
    ///
    /// `FindZeroLeft(3)` is equivalent to the concrete Brainfuck loop `[<<<]`.
    FindZeroLeft(usize),
    /// A loop.
    Loop(Box<[Instruction]>),
}