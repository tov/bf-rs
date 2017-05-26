#![feature(test)]

extern crate test;
extern crate bf;

#[cfg(feature = "jit")]
mod jit_only {
    use bf::ast;
    use bf::rle_ast;
    use bf::peephole;
    use bf::jit;

    use bf::traits::Interpretable;
    use bf::test_helpers;

    use test::Bencher;

    #[bench]
    fn compile_factor(b: &mut Bencher) {
        let program = ast::parse_program(test_helpers::FACTOR_SRC).unwrap();

        b.iter(|| {
            let program = rle_ast::compile(&program);
            let program = peephole::compile(&program);
            jit::compile(&program)
        });
    }

    #[bench]
    fn interpret_factor_million(b: &mut Bencher) {
        let program = ast::parse_program(test_helpers::FACTOR_SRC).unwrap();
        let program = rle_ast::compile(&program);
        let program = peephole::compile(&program);
        let program = jit::compile(&program);

        b.iter(|| {
            program.interpret_memory(None, b"1000000\n").unwrap()
        });
    }
}