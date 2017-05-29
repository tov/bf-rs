use super::*;
use peephole;

use common::{Count, Instruction};

/// Program forms that can be compiled to flat bytecode.
pub trait FlatCompilable {
    /// Compile the given program into the peephole AST to prepare for flat bytecode compilation.
    fn with_peephole<F, R>(&self, k: F) -> R
        where F: FnOnce(&peephole::Program) -> R;

    /// Compile the given program to flat bytecode.
    fn flat_compile(&self) -> Box<Program> {
        self.with_peephole(compile)
    }
}

/// Compiles peephole-optimized AST to a flat bytecode program.
pub fn compile(src: &[peephole::Statement]) -> Box<Program> {
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

    pub fn compile(&mut self, src: &[peephole::Statement]) {
        use peephole::Statement as Src;
        use common::Instruction as Obj;

        for instruction in src {
            match *instruction {
                Src::Instr(instruction) => self.issue(instruction),
                Src::Loop(ref body) => {
                    let begin_pc = self.instructions.len();
                    self.issue(Obj::JumpZero(0));
                    self.compile(body);
                    let end_pc = self.instructions.len();
                    self.issue(Obj::JumpNotZero(usize_to_count(begin_pc)));
                    self.instructions[begin_pc] = Obj::JumpZero(usize_to_count(end_pc));
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

/// Converts a `usize` to a `Count`, panicking if the `usize` is out of range.
pub fn usize_to_count(count: usize) -> Count {
    let result: Count = count as Count;
    assert_eq!(result as usize, count);
    result
}

impl FlatCompilable for peephole::Program {
    fn with_peephole<F, R>(&self, k: F) -> R
        where F: FnOnce(&peephole::Program) -> R
    {
        k(self)
    }
}

impl<T: peephole::PeepholeCompilable + ?Sized> FlatCompilable for T {
    fn with_peephole<F, R>(&self, k: F) -> R
        where F: FnOnce(&peephole::Program) -> R
    {
        k(&self.peephole_compile())
    }
}
