//! Helper definitions for testing both inside and outside (e.g., benches) the crate.

use std::str;

use common::BfResult;
use traits::Interpretable;

/// Source of the factoring program from [`../bf/factor.bf`].
pub const FACTOR_SRC: &'static [u8] = include_bytes!("../bf/factor.bf");

/// Source of a hello program.
pub const HELLO_WORLD_SRC: &'static [u8] =
    b"++++++[>++++++++++++<-]>.\
      >++++++++++[>++++++++++<-]>+.\
      +++++++..+++.>++++[>+++++++++++<-]>.\
      <+++[>----<-]>.<<<<<+++[>+++++<-]>.\
      >>.+++.------.--------.>>+.";

/// Interprets `program`, giving it input `input`, and asserting that its output is `output`.
pub fn assert_interpret<I: Interpretable + ?Sized>(program: &I, input: &[u8], output: &[u8]) {
    assert_interpret_result(program, input, Ok(output));
}

/// Interprets `program`, giving it input `input`, and asserting that its result is `output`.
pub fn assert_interpret_result<I>(program: &I, input: &[u8], output: BfResult<&[u8]>)
    where I: Interpretable + ?Sized
{
    let actual_bytes = program.interpret_memory(None, input);
    let actual = actual_bytes.map(|bytes| str::from_utf8(&bytes).unwrap().to_owned());
    let expected = output.map(|bytes| str::from_utf8(bytes).unwrap().to_owned());

    assert_eq!(actual, expected);

}
