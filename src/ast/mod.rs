mod parser;
mod interpreter;

pub use self::parser::parse_program;

use op_code::OpCode;

/// A BF program is a sequence of instructions.
pub type Program = Box<[Instruction]>;

/// A BF instruction.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Instruction {
    Op(OpCode),
    Loop(Program),
}

pub fn mk_left()  -> Instruction { Instruction::Op(OpCode::Left) }
pub fn mk_right() -> Instruction { Instruction::Op(OpCode::Right) }
pub fn mk_up()    -> Instruction { Instruction::Op(OpCode::Up) }
pub fn mk_down()  -> Instruction { Instruction::Op(OpCode::Down) }
pub fn mk_in()    -> Instruction { Instruction::Op(OpCode::In) }
pub fn mk_out()   -> Instruction { Instruction::Op(OpCode::Out) }

/// Takes a vector of instructions and makes them into a loop.
///
/// Equivalent to
///
/// ```ignore
/// Instruction::Loop(instructions.into_boxed_slice())
/// ```
pub fn mk_loop(instructions: Vec<Instruction>) -> Instruction {
    Instruction::Loop(instructions.into_boxed_slice())
}