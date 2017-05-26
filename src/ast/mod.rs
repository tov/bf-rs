mod parser;
mod interpreter;

pub use self::parser::parse_program;

use common::Command;

/// A BF program is a sequence of instructions.
pub type Program = [Instruction];

/// A BF instruction.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Instruction {
    Op(Command),
    Loop(Box<Program>),
}

pub fn mk_left()  -> Instruction { Instruction::Op(Command::Left) }
pub fn mk_right() -> Instruction { Instruction::Op(Command::Right) }
pub fn mk_up()    -> Instruction { Instruction::Op(Command::Up) }
pub fn mk_down()  -> Instruction { Instruction::Op(Command::Down) }
pub fn mk_in()    -> Instruction { Instruction::Op(Command::In) }
pub fn mk_out()   -> Instruction { Instruction::Op(Command::Out) }

/// Takes a vector of instructions and makes them into a loop.
pub fn mk_loop(instructions: Vec<Instruction>) -> Instruction {
    Instruction::Loop(instructions.into_boxed_slice())
}