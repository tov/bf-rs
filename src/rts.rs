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
    input:  &'a mut dyn Read,
    /// Output channel for the `.` operation.
    output: &'a mut dyn Write,
}

impl<'a> RtsState<'a> {
    pub fn new<R: Read, W: Write>(input: &'a mut R, output: &'a mut W) -> Self {
        RtsState { input, output,
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

    pub extern "C" fn read_c(&mut self) -> u8 {
        let mut buf = [0];
        let _ = self.input.read_exact(&mut buf);
        buf[0]
    }

    pub extern "C" fn write_c(&mut self, byte: u8) {
        let _ = self.output.write_all(&[byte]);
    }
}

