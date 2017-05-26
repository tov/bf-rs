mod compiler;
mod interpreter;

pub use self::compiler::compile;

use common::Command;

/// A BF program is a sequence of instructions.
pub type Program = [Instruction];

pub type Count = usize;

/// A BF instruction.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Instruction {
    Op((Command, Count)),
    Loop(Box<Program>),
}

