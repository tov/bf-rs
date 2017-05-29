#![feature(test)]

extern crate test;
extern crate bf;

#[cfg(feature = "jit")]
mod jit_only {
    use bf::ast;

    use bf::traits::{Interpretable, JitCompilable};
    use bf::test_helpers;

    use test::Bencher;

    #[bench]
    fn compile_factor(b: &mut Bencher) {
        let program = ast::parse_program(test_helpers::FACTOR_SRC).unwrap();

        b.iter(|| {
            program.jit_compile(true)
        });
    }

    #[bench]
    fn compile_factor_unchecked(b: &mut Bencher) {
        let program = ast::parse_program(test_helpers::FACTOR_SRC).unwrap();

        b.iter(|| {
            program.jit_compile(false)
        });
    }

    #[bench]
    fn run_factor_million(b: &mut Bencher) {
        let program = ast::parse_program(test_helpers::FACTOR_SRC).unwrap();
        let program = program.jit_compile(true);

        b.iter(|| {
            program.interpret_memory(None, b"1000000\n").unwrap()
        });
    }

    #[bench]
    fn run_factor_million_unchecked(b: &mut Bencher) {
        let program = ast::parse_program(test_helpers::FACTOR_SRC).unwrap();
        let program = program.jit_compile(false);

        b.iter(|| {
            program.interpret_memory(None, b"1000000\n").unwrap()
        });
    }
}