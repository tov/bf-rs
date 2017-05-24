pub mod compiler;
pub mod interpreter;

use ::op_code::OpCode;

pub type Program = Box<[Instruction]>;
pub type Instruction = (OpCode, Parameter);
pub type Parameter = usize;

