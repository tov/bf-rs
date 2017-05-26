mod compiler;
mod interpreter;

pub use self::compiler::compile;

use op_code::OpCode;

/// A BF program is a sequence of instructions.
pub type Program = [Instruction];

pub type Count = usize;

/// A BF instruction.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Instruction {
    Op((OpCode, Count)),
    Loop(Box<Program>),
}

pub fn mk_left(count: Count)  -> Instruction { Instruction::Op((OpCode::Left, count)) }
pub fn mk_right(count: Count) -> Instruction { Instruction::Op((OpCode::Right, count)) }
pub fn mk_up(count: Count)    -> Instruction { Instruction::Op((OpCode::Up, count)) }
pub fn mk_down(count: Count)  -> Instruction { Instruction::Op((OpCode::Down, count)) }
pub fn mk_in(count: Count)    -> Instruction { Instruction::Op((OpCode::In, count)) }
pub fn mk_out(count: Count)   -> Instruction { Instruction::Op((OpCode::Out, count)) }

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
