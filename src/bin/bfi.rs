extern crate bf;
extern crate clap;

use std::io::{self, Read, Write};
use std::fs::File;
use std::process::exit;

use clap::{Arg, App};

use bf::ast::parser;
use bf::rle_ast;
use bf::flat;
use bf::interpreter::Interpretable;

fn main() {
    let program = match parser::parse_program(&get_program()) {
        Ok(program) => program,
        Err(e) => error_exit(2, format!("Syntax error: {:?}.", e)),
    };

    let program = rle_ast::compiler::compile(&program);
    let program = flat::compiler::compile(&program);

    match program.interpret_stdin(None) {
        Ok(()) => (),
        Err(e) => error_exit(3, format!("Runtime error: {:?}.", e)),
    }
}

fn get_program() -> Vec<u8> {
    let mut input = Vec::new();

    let matches = build_clap_app().get_matches();
    if let Some(exprs) = matches.values_of("expr") {
        for e in exprs {
            input.extend(e.as_bytes());
        }
    } else if let Some(files) = matches.values_of("INPUT") {
        for f in files {
            let mut file = File::open(f).unwrap();
            file.read_to_end(&mut input).unwrap();
        }
    } else {
        error_exit(1, "No program given.".to_owned());
    }

    input
}

fn build_clap_app() -> App<'static, 'static> {
    App::new("bfi")
        .author("Jesse A. Tov <jesse.tov@gmail.com>")
        .about("A Brainfuck interpreter")
        .arg(Arg::with_name("expr")
            .short("e")
            .long("expr")
            .value_name("CODE")
            .help("BF code to execute")
            .multiple(true)
            .takes_value(true)
            .conflicts_with("INPUT"))
        .arg(Arg::with_name("INPUT")
            .help("The source file(s) to interpret")
            .multiple(true)
            .conflicts_with("expr")
            .index(1))
}

fn error_exit(code: i32, msg: String) -> ! {
    writeln!(io::stderr(), "{}", msg).unwrap();
    exit(code)
}

