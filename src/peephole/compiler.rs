use super::*;
use rle;


/// Peephole-optimizes run-length encoded AST.
///
/// See [`Instruction`](struct.Instruction.html) for descriptions of the peepholes.
pub fn compile(src: &[rle::Instruction]) -> Box<Program> {
    let mut compiler = Compiler::new();
    compiler.compile(src);
    compiler.into_program()
}

pub struct Compiler {
    instructions: Vec<Instruction>,
}

macro_rules! or_else {
    ($x:expr) => ($x);
    ($x:expr, $($y:expr),+) => ($x.or_else(|| or_else!($($y),+)))
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
                Cmd(Right, count) =>
                    self.push(Instruction::Right(count)),
                Cmd(Left, count) =>
                    self.push(Instruction::Left(count)),
                Cmd(Up, count) => {
                    let amount = (count % 256) as u8;
                    self.push(Instruction::Change(amount));
                }
                Cmd(Down, count) => {
                    let amount = (256 - count % 256) as u8;
                    self.push(Instruction::Change(amount));
                }
                Cmd(In, count) => {
                    for _ in 0 .. count {
                        self.push(Instruction::In);
                    }
                }
                Cmd(Out, count) => {
                    for _ in 0 .. count {
                        self.push(Instruction::Out);
                    }
                }
                Cmd(Begin, _) | Cmd(End, _) =>
                    panic!("bad opcode"),

                Loop(ref body) => {
                    let body = compile(&*body);

                    let peephole = or_else!(
                        set_zero_peephole(&body),
                        find_zero_peephole(&body),
                        offset_add_peephole(&body)
                    );

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

