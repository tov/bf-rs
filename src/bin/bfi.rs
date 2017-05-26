extern crate bf;
extern crate clap;

use std::io::{self, Read, Write};
use std::fs::File;
use std::process::exit;

use clap::{Arg, App};

use bf::ast;
use bf::rle_ast;
use bf::flat;
use bf::peephole;

#[cfg(feature = "jit")]
use bf::jit;

use bf::traits::Interpretable;

const VERSION: &'static str = "0.2.0";

#[derive(Debug, Clone)]
struct Options {
    program_text:  Vec<u8>,
    memory_size:   Option<usize>,
    compiler_pass: Pass,
}

#[derive(Debug, Clone, Copy)]
enum Pass {
    Ast,
    Rle,
    Flat,
    Peep,
    #[cfg(feature = "jit")]
    Jit,
}

fn main() {
    // Process command-line options:
    let options = get_options();

    // Parse the program to AST:
    let program = parse(&options);

    match options.compiler_pass {
        Pass::Ast => {
            interpret(&*program, &options);
        }

        Pass::Rle => {
            let program = rle_ast::compile(&program);
            interpret(&*program, &options);
        }

        Pass::Peep => {
            let program = rle_ast::compile(&program);
            let program = peephole::compile(&program);
            interpret(&*program, &options);
        }

        Pass::Flat => {
            let program = rle_ast::compile(&program);
            let program = peephole::compile(&program);
            let program = flat::compile(&program);
            interpret(&*program, &options);
        }

        #[cfg(feature = "jit")]
        Pass::Jit => {
            let program = rle_ast::compile(&program);
            let program = peephole::compile(&program);
            let program = jit::compile(&program);
            interpret(&program, &options);
        }
    }
}

fn parse(options: &Options) -> Box<ast::Program> {
    ast::parse_program(&options.program_text)
        .unwrap_or_else(|e| error_exit(2, format!("Syntax error: {}.", e)))
}

fn interpret<P: Interpretable + ?Sized>(program: &P, options: &Options) {
    program.interpret_stdin(options.memory_size)
        .unwrap_or_else(|e| error_exit(3, format!("Runtime error: {}.", e)))
}

fn get_options() -> Options {
    let mut result = Options {
        program_text:  Vec::new(),
        memory_size:   None,
        compiler_pass: Pass::Peep,
    };

    let matches = build_clap_app().get_matches();

    if let Some(size) = matches.value_of("size") {
        let size = size.parse()
            .unwrap_or_else(|e| error_exit(1, format!("Could not parse memory size: {}", e)));
        result.memory_size = Some(size);
    }

    if matches.is_present("ast") {
        result.compiler_pass = Pass::Ast;
    } else if matches.is_present("jit") {
        #[cfg(feature = "jit")]
        let _ = result.compiler_pass = Pass::Jit;
    } else if matches.is_present("flat") {
        result.compiler_pass = Pass::Flat;
    } else if matches.is_present("peep") {
        result.compiler_pass = Pass::Peep;
    } else if matches.is_present("rle") {
        result.compiler_pass = Pass::Rle;
    }

    if let Some(exprs) = matches.values_of("expr") {
        for e in exprs {
            result.program_text.extend(e.as_bytes());
        }
    } else if let Some(files) = matches.values_of("INPUT") {
        for f in files {
            let mut file = File::open(f)
                .unwrap_or_else(|e| error_exit(1, format!("{}: {}", e, f)));
            file.read_to_end(&mut result.program_text)
                .unwrap_or_else(|e| error_exit(1, format!("{}: {}", e, f)));
        }
    } else {
        error_exit(1, "No program given.".to_owned());
    }

    result
}

fn build_clap_app() -> App<'static, 'static> {
    let app = App::new("bfi")
        .version(VERSION)
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
        .arg(Arg::with_name("size")
            .short("s")
            .long("size")
            .value_name("SIZE")
            .help("Memory size in bytes (default 30,000)")
            .takes_value(true))
        .arg(Arg::with_name("ast")
            .long("ast")
            .help("Interpret the unoptimized AST")
            .conflicts_with_all(&["rle", "peep", "flat", "jit"]))
        .arg(Arg::with_name("rle")
            .long("rle")
            .help("Run-length encode the AST"))
        .arg(Arg::with_name("peep")
            .long("peep")
            .help("Peephole optimize (default, implies --rle)"))
        .arg(Arg::with_name("flat")
            .long("flat")
            .help("Flatten to bytecode (implies --peep)")
            .conflicts_with("jit"));

    #[cfg(feature = "jit")]
    let app = app
        .arg(Arg::with_name("jit")
            .long("jit")
            .help("JIT to native x64 (implies --peep)")
            .conflicts_with("flat"));

    app
}

fn error_exit(code: i32, msg: String) -> ! {
    writeln!(io::stderr(), "{}", msg).unwrap();
    exit(code)
}

