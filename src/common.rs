//! Definitions common to multiple passes.
//!
//! This includes error handling and the basic definition of Brainfuck commands.

use std::fmt;

/// The result type for Brainfuck operations that can fail.
///
/// This is `Result` specialized to the four kinds of Brainfuck
/// [`Error`](enum.Error.html)s
pub type BfResult<T> = Result<T, Error>;

/// The static and dynamic errors that can happen in Brainfuck.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    /// Unmatched ‘[’ (syntax error)
    UnmatchedBegin,
    /// Unmatched ‘]’ (syntax error)
    UnmatchedEnd,
    /// If execution continues, the pointer will go below 0 (run-time error)
    PointerUnderflow,
    /// If execution continues, the pointer will go beyond the high end of the
    /// memory (run-time error)
    PointerOverflow,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Error::*;

        match *self {
            UnmatchedBegin => write!(f, "unmatched ‘[’"),
            UnmatchedEnd => write!(f, "unmatched ‘]’"),
            PointerUnderflow => write!(f, "pointer underflow"),
            PointerOverflow => write!(f, "pointer overflow"),
        }
    }
}

/// The eight Brainfuck commands.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum Command {
    /// `>`: Increment the data pointer.
    Right,
    /// `<`: Decrement the data pointer.
    Left,
    /// `+`: Increment the byte value at the data pointer.
    Up,
    /// `-`: Decrement the byte value at the data pointer.
    Down,
    /// `,`: Read a byte from the standard input.
    In,
    /// `.`: Write a byte to the standard output.
    Out,
    /// `[`: Begin a loop, which executes if the byte at the pointer is non-zero.
    Begin,
    /// `]`: End a loop, which repeats if the byte at the pointer is non-zero.
    End,
}

#[cfg(not(any(feature = "u16count", feature = "u32count")))]
/// The number of times to repeat a command when run-length encoded.
pub type Count = usize;

#[cfg(feature = "u16count")]
/// The number of times to repeat a command when run-length encoded.
pub type Count = u16;

#[cfg(feature = "u32count")]
/// The number of times to repeat a command when run-length encoded.
pub type Count = u32;

/// Instructions as output by the bytecode flattener.
///
/// These include the result of peephole optimizations that turn sequences of Brainfuck commands
/// into non-Brainfuck instructions that are understood by the appropriate interpreters and the
/// JIT compiler.
///
/// Unlike in the earlier passes, the loop instructions
/// do not include a boxed slice of instructions as a
/// subtree. Note that this type is `Copy`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Instruction {
    /// Decrease the pointer by the specified offset.
    Left(Count),
    /// Increase the pointer by the specified offset.
    Right(Count),
    /// Increase the current byte value by the specified offset.
    Change(u8),
    /// Read a byte of input.
    In,
    /// Write a byte of output.
    Out,
    /// Begin a loop, jumping to the end if the current byte value is 0.
    ///
    /// The `Count` is the address of the matching `JumpNotZero` instruction.
    JumpZero(Count),
    /// End a loop if the current byte value is 0; otherwise repeat the loop.
    ///
    /// The `Count` is the address of the matching `JumpZero` instruction.
    JumpNotZero(Count),
    /// Set the current byte value to 0.
    ///
    /// Equivalent to the concrete Braincode loop `[-]`.
    SetZero,
    /// Add the byte at the pointer to the byte at the specified offset and zero the byte at the
    /// pointer.
    ///
    /// `OffsetAddRight(5)` is equivalent to the concrete Brainfuck loop `[->>>>>+<<<<<]`.
    OffsetAddRight(Count),
    /// Add the byte at the pointer to the byte at the specified offset and zero the byte at the
    /// pointer.
    ///
    /// `OffsetAddRight(5)` is equivalent to the concrete Brainfuck loop `[-<<<<<+>>>>>]`.
    OffsetAddLeft(Count),
    /// Finds the nearest zero to the left that appears offset by a multiple of the given `Count`.
    ///
    /// `FindZeroRight(3)` is equivalent to the concrete Brainfuck loop `[>>>]`.
    FindZeroRight(Count),
    /// Finds the nearest zero to the right that appears offset by a multiple of the given `Count`.
    ///
    /// `FindZeroLeft(3)` is equivalent to the concrete Brainfuck loop `[<<<]`.
    FindZeroLeft(Count),
}

