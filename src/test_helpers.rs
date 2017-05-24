use std::str;

use interpreter::Interpretable;
use result::BfResult;

pub const FACTOR_SRC: &'static [u8] = include_bytes!("../bf/factor.bf");

pub fn assert_interpret<I: Interpretable + ?Sized>(program: &I, input: &[u8], output: &[u8]) {
    let actual: BfResult<String> = program.interpret_memory(None, input)
        .map(|output| str::from_utf8(&output).unwrap().to_owned());
    let expected: BfResult<String> = Ok(str::from_utf8(output).unwrap().to_owned());

    assert_eq!(actual, expected);
}
