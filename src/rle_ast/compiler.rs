use super::*;
use ::ast;

pub fn compile(program: &[ast::Instruction]) -> Program {
    let mut result = Vec::new();
    let mut buffer = (OpCode::Right, 0);

    let push = |result: &mut Vec<Instruction>, buffer: &mut (OpCode, usize)| {
        if buffer.1 > 0 {
            result.push(Instruction::Op(*buffer));
            *buffer = (OpCode::Right, 0);
        }
    };

    for instruction in program {
        match *instruction {
            ast::Instruction::Op(op_code) => {
                if op_code == buffer.0 {
                    buffer.1 += 1;
                } else {
                    push(&mut result, &mut buffer);
                    buffer = (op_code, 1);
                }
            }

            ast::Instruction::Loop(ref body) => {
                push(&mut result, &mut buffer);
                result.push(Instruction::Loop(compile(&body)));
            }
        }
    }

    push(&mut result, &mut buffer);

    result.into_boxed_slice()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn right_compiles() {
        assert_compile(&[ast::mk_right()], &[mk_right(1)]);
    }

    #[test]
    fn three_rights_compile() {
        assert_compile(&[ast::mk_right(), ast::mk_right(), ast::mk_right()],
                       &[mk_right(3)]);
    }

    #[test]
    fn two_rights_two_ups_compile() {
        assert_compile(&[ast::mk_right(), ast::mk_right(), ast::mk_up(), ast::mk_up()],
                       &[mk_right(2), mk_up(2)]);
    }

    #[test]
    fn loop_compiles() {
        assert_compile(&[ast::mk_in(), ast::mk_loop(vec![ast::mk_right()]), ast::mk_in()],
                       &[mk_in(1), mk_loop(vec![mk_right(1)]), mk_in(1)]);

    }

    fn assert_compile(src: &[ast::Instruction], expected: &[Instruction]) {
        let actual = compile(src);
        assert_eq!(&*actual, expected);
    }
}