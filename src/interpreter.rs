use std::io::{Read, Write};

use super::state::State;
use super::ast::Instruction;

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
        Instruction::Left => state.left(),
        Instruction::Right => state.right(),
        Instruction::Up => state.up(),
        Instruction::Down => state.down(),
        Instruction::In => {
            let mut byte = [0];
            let _ = input.read_exact(&mut byte);
            state.store(byte[0]);
        }
        Instruction::Out => {
            let _ = output.write_all(&[state.load()]);
        }
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
    use super::super::ast::{Instruction, make_loop};
    use super::super::ast::Instruction::*;

    #[test]
    fn assert_no_output() {
        assert_interpret(&[Right], &[], &[]);
    }

    #[test]
    fn assert_output_0() {
        assert_interpret(&[Right, Out], &[], &[0]);
    }

    #[test]
    fn assert_output_1() {
        assert_interpret(&[Up, Out], &[], &[1]);
    }

    #[test]
    fn assert_increment_input() {
        let prog = &[In, Up, Out];
        assert_interpret(prog, &[0], &[1]);
        assert_interpret(prog, &[5], &[6]);
        assert_interpret(prog, &[255], &[0]);
    }

    #[test]
    fn assert_increment_loop() {
        let prog = &[In, make_loop(vec![Up, Out, In])];
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
        use super::super::state::State;
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
