//! The Brainfuck interpreter executable.
//!
//! ```
//! USAGE:
//!     bfi [FLAGS] [OPTIONS] [--] [FILE]...
//!
//! FLAGS:
//!         --ast          Interpret the unoptimized AST
//!         --byte         Compile AST to bytecode
//!     -h, --help         Prints help information
//!         --jit          JIT to native x64 (default)
//!         --llvm         JIT using LLVM
//!         --peep         Interpret the peephole-optimized AST
//!         --rle          Interpret the run-length encoded the AST
//!     -u, --unchecked    Omit memory bounds checks in JIT
//!     -V, --version      Prints version information
//!
//! OPTIONS:
//!     -e, --expr <CODE>...    BF code to execute
//!     -s, --size <SIZE>       Memory size in bytes (default 30,000)
//!
//! ARGS:
//!     <FILE>...    The source file(s) to interpret
//! ```
//!
//! See [the library crate documentation](../bf/index.html) for more.

extern crate bf;
extern crate clap;

use std::io::{self, Read, Write};
use std::fs::File;
use std::process::exit;

use clap::{Arg, App};

use bf::ast;
use bf::traits::*;

const VERSION: &str = "0.4.6";

#[derive(Debug, Clone)]
struct Options {
    program_text:  Vec<u8>,
    memory_size:   Option<usize>,
    compiler_pass: Pass,
    unchecked:     bool,
}

#[derive(Debug, Clone, Copy)]
enum Pass {
    Ast,
    Rle,
    Bytecode,
    Peephole,
    #[cfg(feature = "jit")]
    Jit,
    #[cfg(feature = "llvm")]
    Llvm,
}

fn main() {
    let options = get_options();

    let program = parse(&options);

    match options.compiler_pass {
        Pass::Ast => {
            interpret(&*program, &options);
        }

        Pass::Rle => {
            let program = program.rle_compile();
            interpret(&*program, &options);
        }

        Pass::Peephole => {
            let program = program.peephole_compile();
            interpret(&*program, &options);
        }

        Pass::Bytecode => {
            let program = program.bytecode_compile();
            interpret(&*program, &options);
        }

        #[cfg(feature = "jit")]
        Pass::Jit => {
            let program = program.jit_compile(!options.unchecked);
            interpret(&program, &options);
        }

        #[cfg(feature = "llvm")]
        Pass::Llvm => {
            program.llvm_run(options.memory_size)
                .unwrap_or_else(|e| error_exit(3, format!("runtime error: {}.", e)));
        }
    }
}

fn parse(options: &Options) -> Box<ast::Program> {
    ast::parse_program(&options.program_text)
        .unwrap_or_else(|e| error_exit(2, format!("syntax error: {}.", e)))
}

fn interpret<P: Interpretable + ?Sized>(program: &P, options: &Options) {
    program.interpret_stdin(options.memory_size)
        .unwrap_or_else(|e| error_exit(3, format!("runtime error: {}.", e)))
}

#[cfg(feature = "jit")]
const DEFAULT_PASS: Pass = Pass::Jit;

#[cfg(not(feature = "jit"))]
const DEFAULT_PASS: Pass = Pass::Peephole;

fn get_options() -> Options {
    let mut result = Options {
        program_text:  Vec::new(),
        memory_size:   None,
        compiler_pass: DEFAULT_PASS,
        unchecked:     false,
    };

    let matches = build_clap_app().get_matches();

    if let Some(size) = matches.value_of("size") {
        let size = size.parse()
            .unwrap_or_else(|e|
                error_exit(1, format!("error: could not parse memory size: {}.", e)));
        if size == 0 {
            error_exit(1, "error: memory size must be at least 1.".to_owned());
        }
        result.memory_size = Some(size);
    }

    if matches.is_present("jit") {
        #[cfg(feature = "jit")]
        let _ = result.compiler_pass = Pass::Jit;
    } else if matches.is_present("llvm") {
        #[cfg(feature = "llvm")]
        let _ = result.compiler_pass = Pass::Llvm;
    } else if matches.is_present("byte") {
        result.compiler_pass = Pass::Bytecode;
    } else if matches.is_present("peep") {
        result.compiler_pass = Pass::Peephole;
    } else if matches.is_present("rle") {
        result.compiler_pass = Pass::Rle;
    } else if matches.is_present("ast") {
        result.compiler_pass = Pass::Ast;
    }

    if matches.is_present("unchecked") {
        result.unchecked = true;
    }

    if let Some(exprs) = matches.values_of("expr") {
        for e in exprs {
            result.program_text.extend(e.as_bytes());
        }
    } else if let Some(files) = matches.values_of("FILE") {
        for f in files {
            let mut file = File::open(f)
                .unwrap_or_else(|e| error_exit(1, format!("{}: ‘{}’.", e, f)));
            file.read_to_end(&mut result.program_text)
                .unwrap_or_else(|e| error_exit(1, format!("{}: ‘{}’.", e, f)));
        }
    } else {
        error_exit(1, "error: no program given.".to_owned());
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
            .conflicts_with("FILE"))
        .arg(Arg::with_name("FILE")
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
            .conflicts_with_all(&["rle", "peep", "byte", "jit", "llvm"]))
        .arg(Arg::with_name("rle")
            .long("rle")
            .help("Interpret the run-length encoded the AST")
            .conflicts_with_all(&["ast", "peep", "byte", "jit", "llvm"]))
        .arg(Arg::with_name("peep")
            .long("peep")
            .help(
                if cfg!(feature = "jit") {
                    "Interpret the peephole-optimized AST"
                } else {
                    "Interpret the peephole-optimized AST (default)"
                })
            .conflicts_with_all(&["ast", "rle", "byte", "jit", "llvm"]))
        .arg(Arg::with_name("byte")
            .long("byte")
            .help("Compile AST to bytecode")
            .conflicts_with_all(&["ast", "rle", "peep", "jit", "llvm"]));

    #[cfg(feature = "llvm")]
    let app = app
        .arg(Arg::with_name("llvm")
            .long("llvm")
            .help("JIT using LLVM")
            .conflicts_with_all(&["ast", "rle", "peep", "byte", "jit"]));

    #[cfg(feature = "jit")]
    let app = app
        .arg(Arg::with_name("jit")
            .long("jit")
            .help("JIT to native x64 (default)")
            .conflicts_with_all(&["ast", "rle", "peep", "byte", "llvm"]));

    #[cfg(any(feature = "jit"))]
    let app = app
        .arg(Arg::with_name("unchecked")
            .short("u")
            .long("unchecked")
            .help("Omit memory bounds checks in JIT")
            .conflicts_with_all(&["ast", "rle", "peep", "byte", "llvm"]));

    app
}

fn error_exit(code: i32, msg: String) -> ! {
    writeln!(io::stderr(), "bfi: {}", msg).unwrap();
    exit(code)
}

