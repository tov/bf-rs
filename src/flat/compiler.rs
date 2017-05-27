use super::*;
use peephole;

/// Compiles peephole-optimized AST to a flat bytecode program.
pub fn compile(src: &[peephole::Instruction]) -> Box<Program> {
    let mut compiler = Compiler::new();
    compiler.compile(src);
    compiler.into_program()
}

pub struct Compiler {
    instructions: Vec<Instruction>,
}

impl Compiler {
    pub fn new() -> Self {
        Compiler {
            instructions: Vec::new(),
        }
    }

    pub fn compile(&mut self, src: &[peephole::Instruction]) {
        use peephole::Instruction as Src;
        use super::Instruction as Obj;

        for instruction in src {
            match *instruction {
                Src::Right(count) => self.issue(Obj::Right(count)),
                Src::Left(count) => self.issue(Obj::Left(count)),
                Src::Change(count) => self.issue(Obj::Change(count)),
                Src::In => self.issue(Obj::In),
                Src::Out => self.issue(Obj::Out),
                Src::SetZero => self.issue(Obj::SetZero),
                Src::OffsetAddRight(offset) => self.issue(Obj::OffsetAddRight(offset)),
                Src::OffsetAddLeft(offset) => self.issue(Obj::OffsetAddLeft(offset)),
                Src::FindZeroRight(offset) => self.issue(Obj::FindZeroRight(offset)),
                Src::FindZeroLeft(offset) => self.issue(Obj::FindZeroLeft(offset)),

                Src::Loop(ref body) => {
                    let begin_pc = self.instructions.len();
                    self.issue(Obj::JumpZero(0));
                    self.compile(body);
                    let end_pc = self.instructions.len();
                    self.issue(Obj::JumpNotZero(begin_pc));
                    self.instructions[begin_pc] = Obj::JumpZero(end_pc);
                }
            }
        }
    }

    pub fn into_program(self) -> Box<Program> {
        self.instructions.into_boxed_slice()
    }

    fn issue(&mut self, instruction: Instruction) {
        self.instructions.push(instruction);
    }
}

