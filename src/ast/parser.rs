use super::*;
use common::{BfResult, Error};

pub fn parse_program(input: &[u8]) -> BfResult<Box<Program>> {
    let (program, rest) = parse_instructions(input)?;
    if rest.is_empty() {
        Ok(program)
    } else {
        Err(Error::UnmatchedEnd)
    }
}

type Parser<'a, R> = BfResult<(R, &'a [u8])>;

fn parse_instruction(mut input: &[u8]) -> Parser<Option<Instruction>> {
    loop {
        if let Some((&c, next_input)) = input.split_first() {
            input = next_input;
            match c {
                b'<' => return Ok((Some(mk_left()),  input)),
                b'>' => return Ok((Some(mk_right()), input)),
                b'+' => return Ok((Some(mk_up()),    input)),
                b'-' => return Ok((Some(mk_down()),  input)),
                b',' => return Ok((Some(mk_in()),    input)),
                b'.' => return Ok((Some(mk_out()),   input)),
                b']' => return Err(Error::UnmatchedEnd),
                b'[' => match parse_instructions(input) {
                    Err(e) => return Err(e),
                    Ok((program, next_input)) => {
                        input = next_input;
                        loop {
                            match input.split_first() {
                                Some((&b']', next_input)) =>
                                    return Ok((Some((Instruction::Loop(program))), next_input)),
                                Some((_, next_input)) =>
                                    input = next_input,
                                None =>
                                    return Err(Error::UnmatchedBegin),
                            }
                        }
                    }
                },
                _   => {
                    // pass
                },
            }
        } else {
            return Ok((None, input));
        }
    }
}

fn parse_instructions(mut input: &[u8]) -> Parser<Box<Program>> {
    let mut instructions = Vec::new();

    loop {
        match parse_instruction(input) {
            Ok((Some(instruction), next_input)) => {
                instructions.push(instruction);
                input = next_input;
            }

            Ok((None, next_input)) => {
                input = next_input;
                break;
            }

            Err(e @ Error::UnmatchedBegin) => return Err(e),

            _ => break,
        }
    }

    Ok((instructions.into_boxed_slice(), input))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_byte_instructions_parse() {
        assert_parse("<", &[mk_left()]);
        assert_parse(">", &[mk_right()]);
        assert_parse("+", &[mk_up()]);
        assert_parse("-", &[mk_down()]);
        assert_parse(",", &[mk_in()]);
        assert_parse(".", &[mk_out()]);
    }

    #[test]
    fn multiple_instructions_parse() {
        assert_parse("<><>+-+-.",
                     &[mk_left(), mk_right(), mk_left(), mk_right(),
                         mk_up(), mk_down(), mk_up(), mk_down(), mk_out()]);
    }

    #[test]
    fn empty_program_parses() {
        assert_parse("", &[]);
    }

    #[test]
    fn empty_loop_parses() {
        assert_parse("[]", &[mk_loop(vec![])]);
    }

    #[test]
    fn non_empty_loop_parses() {
        assert_parse("[<]", &[mk_loop(vec![mk_left()])]);
        assert_parse("[<.>]", &[mk_loop(vec![mk_left(), mk_out(), mk_right()])]);
    }

    #[test]
    fn nested_loops_parse() {
        assert_parse("[<[]]",
                     &[mk_loop(vec![mk_left(), mk_loop(vec![])])]);
        assert_parse("[<[+],]",
                     &[mk_loop(vec![mk_left(), mk_loop(vec![mk_up()]), mk_in()])]);
    }

    #[test]
    fn comment_is_ignored() {
        assert_parse("hello <", &[mk_left()]);
        assert_parse("h[e<l[l+o] ,world]",
                     &[mk_loop(vec![mk_left(), mk_loop(vec![mk_up()]), mk_in()])]);
    }

    #[test]
    fn trailing_comment_is_ignored() {
        assert_parse("< hello", &[mk_left()]);
    }

    #[test]
    fn all_comment_program_parses() {
        assert_parse("hello", &[]);
    }

    #[test]
    fn left_bracket_without_right_is_error() {
        assert_parse_error("[", Error::UnmatchedBegin);
        assert_parse_error("[<[.]", Error::UnmatchedBegin);
    }

    #[test]
    fn right_bracket_without_left_is_error() {
        assert_parse_error("]", Error::UnmatchedEnd);
        assert_parse_error(".[.].]", Error::UnmatchedEnd);
    }

    fn assert_parse(input: &str, program: &[Instruction]) {
        assert_eq!(parse_program(input.as_bytes()), Ok(program.to_vec().into_boxed_slice()));
    }

    fn assert_parse_error(input: &str, message: Error) {
        assert_eq!(parse_program(input.as_bytes()), Err(message));
    }
}
