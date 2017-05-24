mod interpreter;
mod compiler;

pub use self::compiler::*;

pub type Program = [Instruction];

/// Instructions as output by the peephole optimizer.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
    /// Begin a loop whose end is at the given address
    Begin(usize),
    /// End a loop whose beginning is at the given address
    End(usize),
    /// Set the current byte to 0
    SetZero,
    /// Add the byte at the pointer to the byte at the specified offset and zero the byte at the
    /// pointer.
    MoveAddRight(usize),
    /// Add the byte at the pointer to the byte at the specified offset and zero the byte at the
    /// pointer.
    MoveAddLeft(usize),
    /// Move the pointer to a zero, skipping the offset at a time.
    FindZeroRight(usize),
    /// Move the pointer to a zero, skipping the offset at a time.
    FindZeroLeft(usize),
}