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
        use flat::Instruction as Flat;

        for instruction in src {
            match *instruction {
                Cmd(Right, count) =>
                    self.push(Flat::Right(count)),
                Cmd(Left, count) =>
                    self.push(Flat::Left(count)),
                Cmd(Up, count) => {
                    let amount = (count % 256) as u8;
                    self.push(Flat::Change(amount));
                }
                Cmd(Down, count) => {
                    let amount = (256 - count % 256) as u8;
                    self.push(Flat::Change(amount));
                }
                Cmd(In, count) => {
                    for _ in 0 .. count {
                        self.push(Flat::In);
                    }
                }
                Cmd(Out, count) => {
                    for _ in 0 .. count {
                        self.push(Flat::Out);
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
                        self.instructions.push(Instruction::Loop(body))
                    }
                }
            }
        }
    }

    pub fn into_program(self) -> Box<Program> {
        self.instructions.into_boxed_slice()
    }

    fn push(&mut self, instr: flat::Instruction) {
        self.instructions.push(Instruction::Flat(instr));
    }
}

pub fn set_zero_peephole(body: &[Instruction]) -> Option<flat::Instruction> {
    use self::Instruction::*;
    use flat::Instruction::*;

    if body.len() == 1 &&
        body[0] == Flat(Change(1)) {
        Some(SetZero)
    } else {
        None
    }
}

pub fn find_zero_peephole(body: &[Instruction]) -> Option<flat::Instruction> {
    use self::Instruction::*;
    use flat::Instruction::*;

    if body.len() == 1 {
        match body[0] {
            Flat(Right(count)) =>
                Some(FindZeroRight(count)),
            Flat(Left(count)) =>
                Some(FindZeroLeft(count)),
            _ => None,
        }
    } else {
        None
    }
}

pub fn offset_add_peephole(body: &[Instruction]) -> Option<flat::Instruction> {
    use self::Instruction::*;
    use flat::Instruction::*;

    if body.len() == 4 {
        match (&body[0], &body[1], &body[2], &body[3]) {
            (&Flat(Change(255)), &Flat(Right(count_l)), &Flat(Change(1)), &Flat(Left(count_r)))
            if count_l == count_r => {
                Some(OffsetAddRight(count_l))
            }

            (&Flat(Change(255)), &Flat(Left(count_l)), &Flat(Change(1)), &Flat(Right(count_r)))
            if count_l == count_r => {
                Some(OffsetAddLeft(count_l))
            }

            _ => None,
        }
    } else {
        None
    }
}

