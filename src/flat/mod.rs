mod compiler;
mod interpreter;

pub use self::compiler::compile;

use op_code::OpCode;

pub type Program = Box<[Instruction]>;
pub type Instruction = (OpCode, Parameter);
pub type Parameter = usize;

