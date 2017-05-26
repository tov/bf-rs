use std::io::{Read, Write};
use std::mem;

use super::*;
use result::{BfResult, Error};

const OKAY: u64      = 0;
const UNDERFLOW: u64 = 1;
const OVERFLOW: u64  = 2;

struct RtsState<'a> {
    input:  &'a mut Read,
    output: &'a mut Write,
}

type EntryFunction<'a> = extern "win64" fn(memory: *mut u8,
                                           memory_size: u64,
                                           rts_state: *mut RtsState<'a>) -> u64;

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

    pub fn run(&mut self, memory_size: usize, exe: &Executable) -> BfResult<()> {
        let mut memory = vec![0u8; memory_size];

        let f: EntryFunction = unsafe { mem::transmute(exe.code.ptr(exe.start)) };

        let result = f(memory.as_mut_ptr(), memory_size as u64, self);

        match result {
            0 => Ok(()),
            1 => Err(Error::PointerUnderflow),
            2 => Err(Error::PointerOverflow),
            _ => panic!(format!("Unknown result code: {}", result)),
        }
    }
}

#[cfg(test)]
mod tests {
//    use super::*;
//    use jit::compiler::compile;

//    #[test]
//    fn run_it() {
//        let exe = compile(&[]);
//        assert_eq!(exe.run(3, 4), 7);
//    }
}
