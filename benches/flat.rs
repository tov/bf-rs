#![feature(test)]

extern crate test;
extern crate bf;

use bf::ast;
use bf::traits::{Interpretable, FlatCompilable};
use bf::test_helpers;

use test::Bencher;

#[bench]
fn compile_factor(b: &mut Bencher) {
    let program = ast::parse_program(test_helpers::FACTOR_SRC).unwrap();

    b.iter(|| {
        program.flat_compile()
    });
}

#[bench]
fn interpret_factor_million(b: &mut Bencher) {
    let program = ast::parse_program(test_helpers::FACTOR_SRC).unwrap();
    let program = program.flat_compile();

    b.iter(|| {
        program.interpret_memory(None, b"1000000\n").unwrap()
    });
}