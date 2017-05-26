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
