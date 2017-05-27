//! Run-length encodes Brainfuck commands.
//!
//! In `bfi` by default, this pass runs after parsing and before peephole optimization. To run
//! the output from run-length encoding directly and skip peephole optimization, pass the `--rle`
//! flag.
//!
//! This module takes an [unoptimized Brainfuck AST](../ast/index.html) and replaces repeated runs
//! of the same command with a run-length encoded instruction.

mod compiler;
mod interpreter;

pub use self::compiler::compile;

use common::Command;

/// A BF program is a rose tree of instructions.
pub type Program = [Instruction];

#[cfg(not(any(feature = "u16count", feature = "u32count")))]
/// The number of times to repeat a command.
pub type Count = usize;

#[cfg(feature = "u16count")]
/// The number of times to repeat a command.
pub type Count = u16;

#[cfg(feature = "u32count")]
/// The number of times to repeat a command.
pub type Count = u32;

/// A run-length encoded BF instruction.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Instruction {
    /// Repeats the given command the given number of times.
    ///
    /// # Invariants
    ///
    /// Cannot be `Begin` or `End`.
    Cmd(Command, Count),
    /// A loop surrounding a sequence of instructions.
    Loop(Box<[Instruction]>),
}

