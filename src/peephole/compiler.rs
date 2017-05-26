use super::*;
use rle;

pub struct Compiler {
    instructions: Vec<Instruction>,
}

impl Compiler {
    pub fn new() -> Self {
        Compiler {
            instructions: Vec::new(),
        }
    }

    pub fn compile(&mut self, src: &[rle::Instruction]) {
        use rle::Instruction::*;
        use common::Command::*;

        for instruction in src {
            match *instruction {
                Op((Right, count)) =>
                    self.push(Instruction::Right(count)),
                Op((Left, count)) =>
                    self.push(Instruction::Left(count)),
                Op((Up, count)) => {
                    let amount = (count % 256) as u8;
                    self.push(Instruction::Change(amount));
                }
                Op((Down, count)) => {
                    let amount = (256 - count % 256) as u8;
                    self.push(Instruction::Change(amount));
                }
                Op((In, count)) => {
                    for _ in 0 .. count {
                        self.push(Instruction::In);
                    }
                }
                Op((Out, count)) => {
                    for _ in 0 .. count {
                        self.push(Instruction::Out);
                    }
                }
                Op((Begin, _)) | Op((End, _)) =>
                    panic!("bad opcode"),

                Loop(ref body) => {
                    let body = compile(&*body);

                    let peephole = set_zero_peephole(&body)
                        .or_else(|| find_zero_peephole(&body))
                        .or_else(|| offset_add_peephole(&body));

                    if let Some(instr) = peephole {
                        self.push(instr);
                    } else {
                        self.push(Instruction::Loop(body))
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

pub fn set_zero_peephole(body: &[Instruction]) -> Option<Instruction> {
    if body.len() == 1 &&
        body[0] == Instruction::Change(1) {
        Some(Instruction::SetZero)
    } else {
        None
    }
}

pub fn find_zero_peephole(body: &[Instruction]) -> Option<Instruction> {
    if body.len() == 1 {
        match body[0] {
            Instruction::Right(count) =>
                Some(Instruction::FindZeroRight(count)),
            Instruction::Left(count) =>
                Some(Instruction::FindZeroLeft(count)),
            _ => None,
        }
    } else {
        None
    }
}

pub fn offset_add_peephole(body: &[Instruction]) -> Option<Instruction> {
    use self::Instruction::*;

    if body.len() == 4 {
        match (&body[0], &body[1], &body[2], &body[3]) {
            (&Change(255), &Right(count_l), &Change(1), &Left(count_r))
            if count_l == count_r => {
                Some(Instruction::OffsetAddRight(count_l))
            }

            (&Change(255), &Left(count_l), &Change(1), &Right(count_r))
            if count_l == count_r => {
                Some(Instruction::OffsetAddLeft(count_l))
            }

            _ => None,
        }
    } else {
        None
    }
}

pub fn compile(src: &[rle::Instruction]) -> Box<Program> {
    let mut compiler = Compiler::new();
    compiler.compile(src);
    compiler.into_program()
}
