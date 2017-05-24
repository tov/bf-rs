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
        Instruction::Op((OpCode::Left, count)) => state.left(count),
        Instruction::Op((OpCode::Right, count)) => state.right(count),
        Instruction::Op((OpCode::Up, count)) => state.up(count as u8),
        Instruction::Op((OpCode::Down, count)) => state.down(count as u8),
        Instruction::Op((OpCode::In, count)) => {
            for _ in 0 .. count {
                let mut byte = [0];
                let _ = input.read_exact(&mut byte);
                state.store(byte[0]);
            }
        }
        Instruction::Op((OpCode::Out, count)) => {
            for _ in 0 .. count {
                let _ = output.write_all(&[state.load()]);
            }
        }
        Instruction::Op((OpCode::Begin, _)) | Instruction::Op((OpCode::End, _)) =>
            panic!("Invalid opcode"),
        Instruction::Loop(ref program) => {
            while state.load() != 0  {
                interpret(&program, state, input, output);
            }
        }
    }
}
