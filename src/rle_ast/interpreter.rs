use std::io::{Read, Write};

use state::State;
use result::BfResult;
use interpreter::Interpretable;
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
        Instruction::Op((OpCode::Left, count)) => state.left(count)?,
        Instruction::Op((OpCode::Right, count)) => state.right(count)?,
        Instruction::Op((OpCode::Up, count)) => state.up(count as u8),
        Instruction::Op((OpCode::Down, count)) => state.down(count as u8),
        Instruction::Op((OpCode::In, count)) => {
            for _ in 0 .. count {
                state.read(input);
            }
        }
        Instruction::Op((OpCode::Out, count)) => {
            for _ in 0 .. count {
                state.write(output);
            }
        }
        Instruction::Op((OpCode::Begin, _)) | Instruction::Op((OpCode::End, _)) =>
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
    use test_helpers::*;

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
        let program = ::ast::parse_program(program).unwrap();
        let program = ::rle_ast::compile(&program);
        assert_interpret(&*program, input.as_bytes(), output.as_bytes());
    }
}
