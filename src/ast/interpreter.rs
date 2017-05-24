use std::io::{Read, Write};

use ::state::State;
use super::*;

pub fn interpret<R, W>(instructions: &[Instruction], state: &mut State,
                   input: &mut R, output: &mut W)
    where R: Read, W: Write
{
    for instruction in instructions {
        interpret_instruction(instruction, state, input, output);
    }
}

#[inline]
fn interpret_instruction<R, W>(instruction: &Instruction, state: &mut State,
                       input: &mut R, output: &mut W)
    where R: Read, W: Write
{
    match *instruction {
        Instruction::Op(OpCode::Left) => state.left(1),
        Instruction::Op(OpCode::Right) => state.right(1),
        Instruction::Op(OpCode::Up) => state.up(1),
        Instruction::Op(OpCode::Down) => state.down(1),
        Instruction::Op(OpCode::In) => state.read(input),
        Instruction::Op(OpCode::Out) => state.write(output),
        Instruction::Op(OpCode::Begin) | Instruction::Op(OpCode::End) =>
            panic!("Invalid opcode"),
        Instruction::Loop(ref program) => {
            while state.load() != 0  {
                interpret(&program, state, input, output);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::interpret;
    use super::super::*;

    #[test]
    fn assert_no_output() {
        assert_interpret(&[mk_right()], &[], &[]);
    }

    #[test]
    fn assert_output_0() {
        assert_interpret(&[mk_right(), mk_out()], &[], &[0]);
    }

    #[test]
    fn assert_output_1() {
        assert_interpret(&[mk_up(), mk_out()], &[], &[1]);
    }

    #[test]
    fn assert_increment_input() {
        let prog = &[mk_in(), mk_up(), mk_out()];
        assert_interpret(prog, &[0], &[1]);
        assert_interpret(prog, &[5], &[6]);
        assert_interpret(prog, &[255], &[0]);
    }

    #[test]
    fn assert_increment_loop() {
        let prog = &[mk_in(), mk_loop(vec![mk_up(), mk_out(), mk_in()])];
        assert_interpret(prog, &[0], &[]);
        assert_interpret(prog, &[1, 0], &[2]);
        assert_interpret(prog, &[1, 4, 0], &[2, 5]);
        assert_interpret(prog, &[8, 255, 18, 0], &[9, 0, 19]);
    }

    #[test]
    fn hello_world() {
        assert_parse_interpret("++++++[>++++++++++++<-]>.\
                                >++++++++++[>++++++++++<-]>+.\
                                +++++++..+++.>++++[>+++++++++++<-]>.\
                                <+++[>----<-]>.<<<<<+++[>+++++<-]>.\
                                >>.+++.------.--------.>>+.",
            "",
            "Hello, World!");
    }

    fn assert_interpret(program: &[Instruction], input: &[u8], output: &[u8]) {
        use ::state::State;
        use std::io::Cursor;
        use std::str;

        let mut reader = Cursor::new(input);
        let mut writer = Cursor::new(Vec::<u8>::new());
        let mut state  = State::new();

        interpret(program, &mut state, &mut reader, &mut writer);
        assert_eq!(str::from_utf8(writer.into_inner().as_slice()),
                   str::from_utf8(output));
    }

    fn assert_parse_interpret(program: &str, input: &str, output: &str) {
        use super::super::parser::parse_program;

        let program = parse_program(program.as_bytes()).unwrap();
        assert_interpret(&program, input.as_bytes(), output.as_bytes());
    }
}
