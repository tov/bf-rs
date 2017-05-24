use std::io::{Read, Write};

use state::State;
use result::BfResult;
use traits::Interpretable;
use super::*;

impl Interpretable for Program {
    fn interpret_state<R: Read, W: Write>(
        &self, mut state: State, mut input: R, mut output: W) -> BfResult<()>
    {
        interpret(self, &mut state, &mut input, &mut output)
    }
}

fn interpret<R, W>(instructions: &Program, state: &mut State,
                   input: &mut R, output: &mut W)
                   -> BfResult<()>
    where R: Read, W: Write
{
    for instruction in instructions {
        interpret_instruction(instruction, state, input, output)?;
    }

    Ok(())
}

#[inline]
fn interpret_instruction<R, W>(instruction: &Instruction, state: &mut State,
                               input: &mut R, output: &mut W)
                               -> BfResult<()>
    where R: Read, W: Write
{
    match *instruction {
        Instruction::Op(OpCode::Left) => state.left(1)?,
        Instruction::Op(OpCode::Right) => state.right(1)?,
        Instruction::Op(OpCode::Up) => state.up(1),
        Instruction::Op(OpCode::Down) => state.down(1),
        Instruction::Op(OpCode::In) => state.read(input),
        Instruction::Op(OpCode::Out) => state.write(output),
        Instruction::Op(OpCode::Begin) | Instruction::Op(OpCode::End) =>
            panic!("Invalid opcode"),
        Instruction::Loop(ref program) => {
            while state.load() != 0  {
                interpret(&program, state, input, output)?;
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::super::*;
    use test_helpers::*;

    #[test]
    fn assert_no_output() {
        assert_interpret(&[mk_right()] as &Program, &[], &[]);
    }

    #[test]
    fn assert_output_0() {
        assert_interpret(&[mk_right(), mk_out()] as &Program, &[], &[0]);
    }

    #[test]
    fn assert_output_1() {
        assert_interpret(&[mk_up(), mk_out()] as &Program, &[], &[1]);
    }

    #[test]
    fn assert_increment_input() {
        let prog: &Program = &[mk_in(), mk_up(), mk_out()];
        assert_interpret(prog, &[0], &[1]);
        assert_interpret(prog, &[5], &[6]);
        assert_interpret(prog, &[255], &[0]);
    }

    #[test]
    fn assert_increment_loop() {
        let prog: &Program = &[mk_in(), mk_loop(vec![mk_up(), mk_out(), mk_in()])];
        assert_interpret(prog, &[0], &[]);
        assert_interpret(prog, &[1, 0], &[2]);
        assert_interpret(prog, &[1, 4, 0], &[2, 5]);
        assert_interpret(prog, &[8, 255, 18, 0], &[9, 0, 19]);
    }

    #[test]
    fn hello_world() {
        assert_parse_interpret(HELLO_WORLD_SRC, "", "Hello, World!");
    }

    #[test]
    fn factoring() {
        assert_parse_interpret(FACTOR_SRC, "2\n", "2: 2\n");
        assert_parse_interpret(FACTOR_SRC, "3\n", "3: 3\n");
        assert_parse_interpret(FACTOR_SRC, "6\n", "6: 2 3\n");
        assert_parse_interpret(FACTOR_SRC, "100\n", "100: 2 2 5 5\n");
    }

    fn assert_parse_interpret(program: &[u8], input: &str, output: &str) {
        use super::super::parser::parse_program;

        let program = parse_program(program).unwrap();
        assert_interpret(&*program, input.as_bytes(), output.as_bytes());
    }
}
