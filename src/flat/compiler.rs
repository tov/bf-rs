use super::*;
use ::rle_ast;

pub struct Compiler {
    instructions: Vec<Instruction>,
}

impl Compiler {
    pub fn new() -> Self {
        Compiler {
            instructions: Vec::new(),
        }
    }

    pub fn compile(&mut self, src: &[rle_ast::Instruction]) {
        for instruction in src {
            match *instruction {
                rle_ast::Instruction::Op((op_code, count)) =>
                    self.issue_op(op_code, count),
                rle_ast::Instruction::Loop(ref body) => {
                    let begin_pc = self.instructions.len();
                    self.issue_op(OpCode::Begin, 0);
                    self.compile(&body);
                    let end_pc = self.instructions.len();
                    self.issue_op(OpCode::End, begin_pc);
                    self.instructions[begin_pc] = (OpCode::Begin, end_pc);
                }
            }
        }
    }

    pub fn into_program(self) -> Program {
        self.instructions.into_boxed_slice()
    }

    fn issue_op(&mut self, op_code: OpCode, count: usize) {
        self.instructions.push((op_code, count));
    }
}

pub fn compile(src: &[rle_ast::Instruction]) -> Program {
    let mut compiler = Compiler::new();
    compiler.compile(src);
    compiler.into_program()
}
