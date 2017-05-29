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

pub use self::compiler::{compile, RleCompilable};

use common::{Command, Count};

/// A run-length encoded BF program is a rose tree of run-length encoded statements.
pub type Program = [Statement];

/// A run-length encoded BF instruction.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Statement {
    /// Repeats the given command the given number of times.
    ///
    /// # Invariants
    ///
    /// The `Command` cannot be `Begin` or `End`.
    Cmd(Command, Count),
    /// A loop surrounding a sequence of instructions.
    Loop(Box<[Statement]>),
}

