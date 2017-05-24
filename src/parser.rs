use super::ast::{Instruction, Program};

pub fn parse_program(input: &[u8]) -> Result<Program, &'static str> {
    let (program, rest) = parse_instructions(input)?;
    if rest.is_empty() {
        Ok(program)
    } else {
        Err("unmatched ]")
    }
}

type Parser<'a, R> = Result<(R, &'a [u8]), &'static str>;

fn parse_instruction(mut input: &[u8]) -> Parser<Option<Instruction>> {
    loop {
        if input.is_empty() {
            return Ok((None, input));
        } else {
            let c = input[0];
            input = &input[1..];
            match c {
                b'<' => return Ok((Some(Instruction::Left), input)),
                b'>' => return Ok((Some(Instruction::Right), input)),
                b'+' => return Ok((Some(Instruction::Up), input)),
                b'-' => return Ok((Some(Instruction::Down), input)),
                b',' => return Ok((Some(Instruction::In), input)),
                b'.' => return Ok((Some(Instruction::Out), input)),
                b']' => return Err("unmatched ]"),
                b'[' => match parse_instructions(input) {
                    Err(e) => return Err(e),
                    Ok((program, next_input)) => {
                        input = next_input;
                        loop {
                            if input.is_empty() {
                                return Err("unmatched [");
                            } else {
                                let c = input[0];
                                input = &input[1..];
                                match c {
                                    b']' => return Ok((Some(Instruction::Loop(program)), input)),
                                    _    => { /* pass */ }
                                }
                            }
                        }
                    }
                },
                _   => {
                    // pass
                },
            }
        }
    }
}

fn parse_instructions(mut input: &[u8]) -> Parser<Program> {
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

            Err(e @ "unmatched [") => return Err(e),

            _ => break,
        }
    }

    Ok((instructions.into_boxed_slice(), input))
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::ast::*;
    use self::Instruction::*;

    #[test]
    fn single_byte_instructions_parse() {
        assert_parse("<", &[Left]);
        assert_parse(">", &[Right]);
        assert_parse("+", &[Up]);
        assert_parse("-", &[Down]);
        assert_parse(",", &[In]);
        assert_parse(".", &[Out]);
    }

    #[test]
    fn multiple_instructions_parse() {
        assert_parse("<><>+-+-.",
                     &[Left, Right, Left, Right, Up, Down, Up, Down, Out]);
    }

    #[test]
    fn empty_program_parses() {
        assert_parse("", &[]);
    }

    #[test]
    fn empty_loop_parses() {
        assert_parse("[]", &[make_loop(vec![])]);
    }

    #[test]
    fn non_empty_loop_parses() {
        assert_parse("[<]", &[make_loop(vec![Left])]);
        assert_parse("[<.>]", &[make_loop(vec![Left, Out, Right])]);
    }

    #[test]
    fn nested_loops_parse() {
        assert_parse("[<[]]",
                     &[make_loop(vec![Left, make_loop(vec![])])]);
        assert_parse("[<[+],]",
                     &[make_loop(vec![Left, make_loop(vec![Up]), In])]);
    }

    #[test]
    fn comment_is_ignored() {
        assert_parse("hello <", &[Left]);
        assert_parse("h[e<l[l+o] ,world]",
                     &[make_loop(vec![Left, make_loop(vec![Up]), In])]);
    }

    #[test]
    fn trailing_comment_is_ignored() {
        assert_parse("< hello", &[Left]);
    }

    #[test]
    fn all_comment_program_parses() {
        assert_parse("hello", &[]);
    }

    #[test]
    fn left_bracket_without_right_is_error() {
        assert_parse_error("[", "unmatched [");
        assert_parse_error("[<[.]", "unmatched [");
    }

    #[test]
    fn right_bracket_without_left_is_error() {
        assert_parse_error("]", "unmatched ]");
        assert_parse_error(".[.].]", "unmatched ]");
    }

    fn assert_parse(input: &str, program: &[Instruction]) {
        assert_eq!(parse_program(input.as_bytes()), Ok(program.to_vec().into_boxed_slice()));
    }

    fn assert_parse_error(input: &str, message: &str) {
        assert_eq!(parse_program(input.as_bytes()), Err(message));
    }
}
