use std::io::{Read, Write};

use state::State;
use common::BfResult;
use traits::Interpretable;
use super::*;

impl Interpretable for Program {
    fn interpret_state<R: Read, W: Write>(
        &self, mut state: State, mut input: R, mut output: W) -> BfResult<()>
    {
        interpret(self, &mut state, &mut input, &mut output)
    }
}

fn interpret<R, W>(instructions: &[Instruction], state: &mut State,
                            input: &mut R, output: &mut W)
                            -> BfResult<()>
    where R: Read, W: Write
{
    for instruction in instructions {
        interpret_instruction(instruction, state, input, output)?;
    }

    Ok(())
}

fn interpret_instruction<R, W>(instructions: &Instruction, state: &mut State,
                               input: &mut R, output: &mut W)
                               -> BfResult<()>
    where R: Read, W: Write
{
    use super::Instruction::*;
    use flat::Instruction::*;

    match *instructions {
        Flat(Left(count)) => state.left(count as usize)?,

        Flat(Right(count)) => state.right(count as usize)?,

        Flat(Change(amount)) => state.up(amount),

        Flat(In) => state.read(input),

        Flat(Out) => state.write(output),

        Flat(SetZero) => state.store(0),

        Flat(OffsetAddRight(offset)) => {
            let value = state.load();
            if value != 0 {
                state.store(0);
                state.up_pos_offset(offset as usize, value)?;
            }
        }

        Flat(OffsetAddLeft(offset)) => {
            let value = state.load();
            if value != 0 {
                state.store(0);
                state.up_neg_offset(offset as usize, value)?;
            }
        }

        Flat(FindZeroRight(skip)) => {
            while state.load() != 0 {
                state.right(skip as usize)?;
            }
        }

        Flat(FindZeroLeft(skip)) => {
            while state.load() != 0 {
                state.left(skip as usize)?;
            }
        }

        Flat(JumpZero(_)) | Flat(JumpNotZero(_)) =>
            panic!("unexpected jump instruction"),

        Loop(ref body) => {
            while state.load() != 0 {
                interpret(body, state, input, output)?;
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
        let program = ::rle::compile(&program);
        let program = ::peephole::compile(&program);
        assert_interpret(&*program, input.as_bytes(), output.as_bytes());
    }
}
