//! Minimal run-time system, which does I/O.
//!
//! In Bendersky's first JIT, the program uses Linux system calls, but that's
//! insufficiently portable. And maybe I could figure out Darwin system calls, but
//! I’d rather not write retry loops anyway. The technique here is from [the `dynlib-rs`
//! tutorial]. Instead, we store trait objects in [a struct](struct.RtsState.html), pass a pointer
//! to that struct to the generated program, and then have the generated program pass the pointer
//! to that struct to the RTS’s read and write functions.
//!
//! [the `dynlib-rs` tutorial]:(https://censoredusername.github.io/dynasm-rs/language/tutorial.html#advanced-usage)

use std::io::{Read, Write};
use std::mem;

use super::*;
use common::{BfResult, Error};
use state::State;
use traits::Interpretable;

/// The object code terminated successfully.
pub const OKAY: u64      = 0;

/// The pointer would have pointed below the allocated buffer had the program continued.
pub const UNDERFLOW: u64 = 1;

/// The pointer would have pointed above the allocated buffer had the program continued.
pub const OVERFLOW: u64  = 2;

/// Minimal state for our minimal run-time system.
///
/// Trait objects providing channels for standard input and output.
pub struct RtsState<'a> {
    /// Input channel for the `,` operation.
    input:  &'a mut Read,
    /// Output channel for the `.` operation.
    output: &'a mut Write,
}

/// The type of function that we will assemble and then call.
///
/// # Parameters
///
/// `<'a>': the lifetime of the channel references in the run-time system state.
///
/// `memory`: the address of the beginning of memory (also where the pointer starts).
///
/// `memory_size`: the amount of memory allocated, defaults to 30_000 bytes.
///
/// `rts_state`: the state that the run-time system needs to do I/O.
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

