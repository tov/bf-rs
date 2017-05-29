#![feature(test)]

extern crate test;
extern crate bf;

use bf::ast;
use bf::rle;
use bf::peephole::PeepholeCompilable;
use bf::traits::Interpretable;
use bf::test_helpers;

use test::Bencher;

#[bench]
fn compile_factor(b: &mut Bencher) {
    let program = ast::parse_program(test_helpers::FACTOR_SRC).unwrap();

    b.iter(|| {
        let program = rle::compile(&program);
        program.peephole_compile()
    });
}

#[bench]
fn interpret_factor_million(b: &mut Bencher) {
    let program = ast::parse_program(test_helpers::FACTOR_SRC).unwrap();
    let program = program.peephole_compile();

    b.iter(|| {
        program.interpret_memory(None, b"1000000\n").unwrap()
    });
}