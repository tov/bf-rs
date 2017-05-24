use super::*;
use ::ast;

/// Represents the state of an RLE compiler from `ast::Instruction` to `Instruction`.
pub struct Compiler {
    instructions: Vec<Instruction>,
    buffer: (OpCode, usize),
}

impl Compiler {
    /// Creates a new RLE compiler.
    pub fn new() -> Self {
        Compiler {
            instructions: Vec::new(),
            buffer: (OpCode::Right, 0),
        }
    }

    /// Compiles the given sequence of instructions.
    pub fn compile(&mut self, program: &[ast::Instruction]) {
        for instruction in program {
            match *instruction {
                ast::Instruction::Op(op_code) => self.issue_op(op_code),
                ast::Instruction::Loop(ref body) => self.issue_loop(compile(body)),
            }
        }
    }

    /// Extracts the compiled program.
    pub fn into_program(mut self) -> Program {
        self.push_op();
        self.instructions.into_boxed_slice()
    }

    fn push_op(&mut self) {
        if self.buffer.1 > 0 {
            self.instructions.push(Instruction::Op(self.buffer));
            self.buffer = (OpCode::Right, 0);
        }
    }

    fn issue_op(&mut self, op_code: OpCode) {
        if op_code == self.buffer.0 {
            self.buffer.1 += 1;
        } else {
            self.push_op();
            self.buffer = (op_code, 1)
        }
    }

    fn issue_loop(&mut self, body: Program) {
        self.push_op();
        self.instructions.push(Instruction::Loop(body));
    }
}

/// Compiles the given sequence of instructions into a program.
pub fn compile(program: &[ast::Instruction]) -> Program {
    let mut compiler = Compiler::new();
    compiler.compile(program);
    compiler.into_program()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn right_compiles() {
        assert_compile(&[ast::mk_right()], &[mk_right(1)]);
    }

    #[test]
    fn three_rights_compile() {
        assert_compile(&[ast::mk_right(), ast::mk_right(), ast::mk_right()],
                       &[mk_right(3)]);
    }

    #[test]
    fn two_rights_two_ups_compile() {
        assert_compile(&[ast::mk_right(), ast::mk_right(), ast::mk_up(), ast::mk_up()],
                       &[mk_right(2), mk_up(2)]);
    }

    #[test]
    fn loop_compiles() {
        assert_compile(&[ast::mk_in(), ast::mk_loop(vec![ast::mk_right()]), ast::mk_in()],
                       &[mk_in(1), mk_loop(vec![mk_right(1)]), mk_in(1)]);

    }

    fn assert_compile(src: &[ast::Instruction], expected: &[Instruction]) {
        let actual = compile(src);
        assert_eq!(&*actual, expected);
    }
}
