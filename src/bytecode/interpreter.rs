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

fn interpret<R, W>(instructions: &Program, state: &mut State,
                   input: &mut R, output: &mut W)
                       -> BfResult<()>
    where R: Read, W: Write
{
    use common::Instruction::*;

    let mut pc = 0;

    while pc < instructions.len() {
        match instructions[pc] {
            Left(count) => state.left(count)?,
            Right(count) => state.right(count)?,
            Add(count) => state.up(count),
            In => state.read(input),
            Out => state.write(output),

            JumpZero(address) => {
                if state.load() == 0 {
                    pc = address as usize;
                }
            }

            JumpNotZero(address) => {
                if state.load() != 0 {
                    pc = address as usize;
                }
            }

            SetZero => state.store(0),

            OffsetAddRight(offset) => {
                if state.load() != 0 {
                    let value = state.load();
                    state.store(0);
                    state.up_pos_offset(offset, value)?;
                }
            }

            OffsetAddLeft(offset) => {
                if state.load() != 0 {
                    let value = state.load();
                    state.store(0);
                    state.up_neg_offset(offset, value)?;
                }
            }

            FindZeroRight(offset) => {
                while state.load() != 0 {
                    state.right(offset)?;
                }
            }

            FindZeroLeft(offset) => {
                while state.load() != 0 {
                    state.left(offset)?;
                }
            }
        }

        pc += 1;
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
        let program = ::bytecode::compile(&program);
        assert_interpret(&*program, input.as_bytes(), output.as_bytes());
    }
}
