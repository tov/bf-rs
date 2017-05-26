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
    use result::{BfResult, Error};

    #[test]
    fn move_right_once() {
        assert_parse_interpret(b">", "", Ok(""));
    }

    #[test]
    fn move_left_once() {
        assert_parse_interpret(b"<", "", Err(Error::PointerUnderflow));
    }

    #[test]
    fn move_right_forever() {
        assert_parse_interpret(b"+[>+]", "", Err(Error::PointerOverflow));
    }

    #[test]
    fn echo_one_byte() {
        assert_parse_interpret(b",.", "A", Ok("A"));
    }

    #[test]
    fn inc_echo_one_byte() {
        assert_parse_interpret(b",+.", "A", Ok("B"));
    }

    #[test]
    fn hello_world() {
        assert_parse_interpret(HELLO_WORLD_SRC, "", Ok("Hello, World!"));
    }

    #[test]
    fn factoring() {
        assert_parse_interpret(FACTOR_SRC, "2\n", Ok("2: 2\n"));
        assert_parse_interpret(FACTOR_SRC, "3\n", Ok("3: 3\n"));
        assert_parse_interpret(FACTOR_SRC, "6\n", Ok("6: 2 3\n"));
        assert_parse_interpret(FACTOR_SRC, "100\n", Ok("100: 2 2 5 5\n"));
    }

    fn assert_parse_interpret(program: &[u8], input: &str, output: BfResult<&str>) {
        let program = ::ast::parse_program(program).unwrap();
        let program = ::rle::compile(&program);
        let program = ::peephole::compile(&program);
        let program = ::jit::compile(&program);
        assert_interpret_result(&program, input.as_bytes(), output.map(|s| s.as_bytes()));
    }
}
