use std::str;

use result::BfResult;
use traits::Interpretable;

pub const FACTOR_SRC: &'static [u8] = include_bytes!("../bf/factor.bf");

pub const HELLO_WORLD_SRC: &'static [u8] =
    b"++++++[>++++++++++++<-]>.\
      >++++++++++[>++++++++++<-]>+.\
      +++++++..+++.>++++[>+++++++++++<-]>.\
      <+++[>----<-]>.<<<<<+++[>+++++<-]>.\
      >>.+++.------.--------.>>+.";

pub fn assert_interpret<I: Interpretable + ?Sized>(program: &I, input: &[u8], output: &[u8]) {
    assert_interpret_result(program, input, Ok(output));
}

pub fn assert_interpret_result<I>(program: &I, input: &[u8], output: BfResult<&[u8]>)
    where I: Interpretable + ?Sized
{
    let actual_bytes = program.interpret_memory(None, input);
    let actual = actual_bytes.map(|bytes| str::from_utf8(&bytes).unwrap().to_owned());
    let expected = output.map(|bytes| str::from_utf8(bytes).unwrap().to_owned());

    assert_eq!(actual, expected);

}
