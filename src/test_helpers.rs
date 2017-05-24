use std::str;

use traits::Interpretable;

pub const FACTOR_SRC: &'static [u8] = include_bytes!("../bf/factor.bf");

pub const HELLO_WORLD_SRC: &'static [u8] =
    b"++++++[>++++++++++++<-]>.\
      >++++++++++[>++++++++++<-]>+.\
      +++++++..+++.>++++[>+++++++++++<-]>.\
      <+++[>----<-]>.<<<<<+++[>+++++<-]>.\
      >>.+++.------.--------.>>+.";

pub fn assert_interpret<I: Interpretable + ?Sized>(program: &I, input: &[u8], output: &[u8]) {
    let actual_bytes = program.interpret_memory(None, input).unwrap();
    let actual = str::from_utf8(&actual_bytes).unwrap();
    let expected = str::from_utf8(output).unwrap();

    assert_eq!(actual, expected);
}
