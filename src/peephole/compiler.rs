use super::*;
use rle;

/// Peephole-optimizes run-length encoded AST.
///
/// See [`Instruction`](struct.Instruction.html) for descriptions of the peepholes.
pub fn compile(src: &[rle::Statement]) -> Box<Program> {
    let mut compiler = Compiler::new();
    compiler.compile(src);
    compiler.into_program()
}

pub struct Compiler {
    instructions: Vec<Statement>,
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

    pub fn compile(&mut self, src: &[rle::Statement]) {
        use rle::Statement::*;
        use common::Command::*;
        use common::Instruction as Obj;

        for instruction in src {
            match *instruction {
                Cmd(Right, count) =>
                    self.push(Obj::Right(count)),
                Cmd(Left, count) =>
                    self.push(Obj::Left(count)),
                Cmd(Up, count) => {
                    let amount = (count % 256) as u8;
                    self.push(Obj::Add(amount));
                }
                Cmd(Down, count) => {
                    let amount = (256 - count % 256) as u8;
                    self.push(Obj::Add(amount));
                }
                Cmd(In, count) => {
                    for _ in 0 .. count {
                        self.push(Obj::In);
                    }
                }
                Cmd(Out, count) => {
                    for _ in 0 .. count {
                        self.push(Obj::Out);
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
                        self.instructions.push(Statement::Loop(body))
                    }
                }
            }
        }
    }

    pub fn into_program(self) -> Box<Program> {
        self.instructions.into_boxed_slice()
    }

    fn push(&mut self, instr: common::Instruction) {
        self.instructions.push(Statement::Instr(instr));
    }
}

pub fn set_zero_peephole(body: &[Statement]) -> Option<common::Instruction> {
    use self::Statement::*;
    use common::Instruction::*;

    if body.len() == 1 &&
        body[0] == Instr(Add(1)) {
        Some(SetZero)
    } else {
        None
    }
}

pub fn find_zero_peephole(body: &[Statement]) -> Option<common::Instruction> {
    use self::Statement::*;
    use common::Instruction::*;

    if body.len() == 1 {
        match body[0] {
            Instr(Right(count)) =>
                Some(FindZeroRight(count)),
            Instr(Left(count)) =>
                Some(FindZeroLeft(count)),
            _ => None,
        }
    } else {
        None
    }
}

pub fn offset_add_peephole(body: &[Statement]) -> Option<common::Instruction> {
    use self::Statement::*;
    use common::Instruction::*;

    if body.len() == 4 {
        match (&body[0], &body[1], &body[2], &body[3]) {
            (&Instr(Add(255)), &Instr(Right(count_l)), &Instr(Add(1)), &Instr(Left(count_r)))
            if count_l == count_r => {
                Some(OffsetAddRight(count_l))
            }

            (&Instr(Add(255)), &Instr(Left(count_l)), &Instr(Add(1)), &Instr(Right(count_r)))
            if count_l == count_r => {
                Some(OffsetAddLeft(count_l))
            }

            _ => None,
        }
    } else {
        None
    }
}

