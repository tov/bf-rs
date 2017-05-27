//! Parsing and interpretation for unoptimized Brainfuck abstract syntax trees.
//!
//! In `bfi` by default, this pass runs before run-length encoding and peephole optimization. To
//! run the unoptimized AST directly and skip all optimization, pass `--ast` flag.
//!
//! In this module, BF programs are represented by the [`Program`](type.Program.html)
//! type, which is an array of [`Instruction`](enum.Instruction.html)s. `Instruction`s
//! correspond directly to Brainfuck commands, except that loops are represented as subtrees
//! rather than with begin and end markers.

mod parser;
mod interpreter;

pub use self::parser::parse_program;

use common::Command;

/// A BF program is represented as an array of instructions. The array will
/// typically be boxed.
pub type Program = [Instruction];

/// An unoptimized BF instruction.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Instruction {
    /// A non-loop command.
    ///
    /// # Invariants
    ///
    /// The `Command` cannot be `Begin` or `End`.
    Cmd(Command),
    /// A loop surrounding a sequence of instructions.
    Loop(Box<[Instruction]>),
}

