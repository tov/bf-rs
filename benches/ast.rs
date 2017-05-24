#![feature(test)]

extern crate test;
extern crate bf;

use bf::ast;
use bf::interpreter::Interpretable;
use bf::test_helpers;

use test::Bencher;

#[bench]
fn parse_empty_program(b: &mut Bencher) {
    b.iter(|| {
        ast::parse_program(b"").unwrap();
    })
}

#[bench]
fn parse_factoring(b: &mut Bencher) {
    b.iter(|| {
        ast::parse_program(test_helpers::FACTOR_SRC).unwrap();
    })
}

#[bench]
fn interpret_factoring_million(b: &mut Bencher) {
    let program = ast::parse_program(test_helpers::FACTOR_SRC).unwrap();

    b.iter(|| {
        program.interpret_memory(None, b"1000000\n").unwrap()
    });
}