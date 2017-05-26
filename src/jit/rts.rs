use std::io::{Read, Write};
use std::mem;

use super::*;
use result::{BfResult, Error};
use state::State;
use traits::Interpretable;

pub const OKAY: u64      = 0;
pub const UNDERFLOW: u64 = 1;
pub const OVERFLOW: u64  = 2;

pub struct RtsState<'a> {
    input:  &'a mut Read,
    output: &'a mut Write,
}

type EntryFunction<'a> = extern "win64" fn(memory: *mut u8,
                                           memory_size: u64,
                                           rts_state: *mut RtsState<'a>) -> u64;

impl Interpretable for Program {
    fn interpret_state<R: Read, W: Write>(&self, mut state: State,
                                          mut input: R, mut output: W)
                                          -> BfResult<()>
    {
        let mut rts = RtsState::new(&mut input, &mut output);

        let f: EntryFunction = unsafe { mem::transmute(self.code.ptr(self.start)) };

        let result = f(state.as_mut_ptr(), state.capacity() as u64, &mut rts);;

        match result {
            OKAY      => Ok(()),
            UNDERFLOW => Err(Error::PointerUnderflow),
            OVERFLOW  => Err(Error::PointerOverflow),
            _ => panic!(format!("Unknown result code: {}", result)),
        }
    }
}

impl<'a> RtsState<'a> {
    fn new<R: Read, W: Write>(input: &'a mut R, output: &'a mut W) -> Self {
        RtsState {
            input: input,
            output: output,
        }
    }

    pub extern "win64" fn read(&mut self) -> u8 {
        let mut buf = [0];
        let _ = self.input.read_exact(&mut buf);
        buf[0]
    }

    pub extern "win64" fn write(&mut self, byte: u8) {
        let _ = self.output.write_all(&[byte]);
    }
}

#[cfg(test)]
mod tests {
    use test_helpers::*;

    #[test]
    fn move_right_once() {
        assert_parse_interpret(b">", "", "");
    }

    #[test]
    fn echo_one_byte() {
        assert_parse_interpret(b",.", "A", "A");
    }

    #[test]
    fn inc_echo_one_byte() {
        assert_parse_interpret(b",+.", "A", "B");
    }


    fn assert_parse_interpret(program: &[u8], input: &str, output: &str) {
        let program = ::ast::parse_program(program).unwrap();
        let program = ::rle_ast::compile(&program);
        let program = ::peephole::compile(&program);
        let program = ::jit::compile(&program);
        assert_interpret(&program, input.as_bytes(), output.as_bytes());
    }
}
