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
