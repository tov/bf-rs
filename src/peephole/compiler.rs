use super::*;
use op_code::OpCode;
use rle_ast;

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
        use rle_ast::Instruction::*;

        for instruction in src {
            match *instruction {
                Op((OpCode::Right, count)) =>
                    self.push(Instruction::Right(count)),
                Op((OpCode::Left, count)) =>
                    self.push(Instruction::Left(count)),
                Op((OpCode::Up, count)) => {
                    let amount = (count % 256) as u8;
                    self.push(Instruction::Change(amount));
                }
                Op((OpCode::Down, count)) => {
                    let amount = (256 - count % 256) as u8;
                    self.push(Instruction::Change(amount));
                }
                Op((OpCode::In, count)) => {
                    for _ in 0 .. count {
                        self.push(Instruction::In);
                    }
                }
                Op((OpCode::Out, count)) => {
                    for _ in 0 .. count {
                        self.push(Instruction::Out);
                    }
                }
                Op((OpCode::Begin, _)) | Op((OpCode::End, _)) =>
                    panic!("bad opcode"),

                Loop(ref body) => {
                    let peephole = set_zero_peephole(&body)
                        .or_else(|| find_zero_peephole(&body))
                        .or_else(|| offset_add_peephole(&body));

                    if let Some(instr) = peephole {
                        self.push(instr);
                    } else {
                        let begin_pc = self.instructions.len();
                        self.push(Instruction::JumpZero(0));
                        self.compile(&body);
                        let end_pc = self.instructions.len();
                        self.push(Instruction::JumpNotZero(begin_pc));
                        self.instructions[begin_pc] = Instruction::JumpZero(end_pc);
                    }
                }
            }
        }
    }

    pub fn into_program(self) -> Box<Program> {
        self.instructions.into_boxed_slice()
    }

    fn push(&mut self, instr: Instruction) {
        self.instructions.push(instr);
    }
}

pub fn set_zero_peephole(body: &[rle_ast::Instruction]) -> Option<Instruction> {
    if body.len() == 1 &&
        (body[0] == rle_ast::Instruction::Op((OpCode::Up, 1)) ||
         body[0] == rle_ast::Instruction::Op((OpCode::Down, 1))) {
        Some(Instruction::SetZero)
    } else {
        None
    }
}

pub fn find_zero_peephole(body: &[rle_ast::Instruction]) -> Option<Instruction> {
    if body.len() == 1 {
        match body[0] {
            rle_ast::Instruction::Op((OpCode::Right, count)) =>
                Some(Instruction::FindZeroRight(count)),
            rle_ast::Instruction::Op((OpCode::Left, count)) =>
                Some(Instruction::FindZeroLeft(count)),
            _ => None,
        }
    } else {
        None
    }
}

pub fn offset_add_peephole(body: &[rle_ast::Instruction]) -> Option<Instruction> {
    use rle_ast::Instruction::*;

    if body.len() == 4 {
        match (&body[0], &body[1], &body[2], &body[3]) {
            (&Op((OpCode::Down, 1)),
             &Op((OpCode::Right, count_l)),
             &Op((OpCode::Up, 1)),
             &Op((OpCode::Left, count_r))) if count_l == count_r => {
                Some(Instruction::OffsetAddRight(count_l))
            }

            (&Op((OpCode::Down, 1)),
             &Op((OpCode::Left, count_l)),
             &Op((OpCode::Up, 1)),
             &Op((OpCode::Right, count_r))) if count_l == count_r => {
                Some(Instruction::OffsetAddLeft(count_l))
            }

            _ => None,
        }
    } else {
        None
    }
}

pub fn compile(src: &[rle_ast::Instruction]) -> Box<Program> {
    let mut compiler = Compiler::new();
    compiler.compile(src);
    compiler.into_program()
}
