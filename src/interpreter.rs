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

