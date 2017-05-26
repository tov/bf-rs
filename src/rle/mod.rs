mod compiler;
mod interpreter;

pub use self::compiler::compile;

use common::Command;

/// A BF program is a sequence of instructions.
pub type Program = [Instruction];

/// The number of times to repeat a command.
pub type Count = usize;

/// A run-length encoded BF instruction.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Instruction {
    Op((Command, Count)),
    Loop(Box<Program>),
}

