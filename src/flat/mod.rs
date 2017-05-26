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

mod compiler;
mod interpreter;

pub use self::compiler::compile;

/// A program is a flat sequence of instructions.
pub type Program = [Instruction];

/// Instructions as output by the bytecode flattener.
///
/// Unlike in the earlier passes, the loop instructions
/// do not include a boxed slice of instructions as a
/// subtree. Note that this type is `Copy`.
///
/// This is not necessary for interpretation, but it might
/// perform better because of the cache. So far, it appears
/// to perform worse than the peephole-optimized AST, but
/// I’m going to try making the operations fit in a word
/// instead of two to see if that goes faster.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Instruction {
    /// Decrease the pointer left by the specified offset.
    Left(usize),
    /// Increase the pointer right by the specified offset.
    Right(usize),
    /// Increase the current byte value by the specified offset.
    Change(u8),
    /// Read a byte of input.
    In,
    /// Write a byte of output.
    Out,
    /// Begin a loop, jumping to the end if the current byte value.
    JumpZero(usize),
    /// End a loop if the current byte value is 0; otherwise repeat the loop.
    JumpNotZero(usize),
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
}

