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

pub fn mk_left(count: Count)  -> Instruction { Instruction::Op((Command::Left, count)) }
pub fn mk_right(count: Count) -> Instruction { Instruction::Op((Command::Right, count)) }
pub fn mk_up(count: Count)    -> Instruction { Instruction::Op((Command::Up, count)) }
pub fn mk_down(count: Count)  -> Instruction { Instruction::Op((Command::Down, count)) }
pub fn mk_in(count: Count)    -> Instruction { Instruction::Op((Command::In, count)) }
pub fn mk_out(count: Count)   -> Instruction { Instruction::Op((Command::Out, count)) }

/// Takes a vector of instructions and makes them into a loop.
pub fn mk_loop(instructions: Vec<Instruction>) -> Instruction {
    Instruction::Loop(instructions.into_boxed_slice())
}
