mod parser;
mod interpreter;

pub use self::parser::parse_program;

use common::Command;

/// A BF program is a sequence of instructions.
pub type Program = [Instruction];

/// A BF instruction.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Instruction {
    /// A non-loop command.
    Op(Command),
    /// A loop surrounding a sequence of instructions.
    Loop(Box<[Instruction]>),
}

