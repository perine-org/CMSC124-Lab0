mod scanner;
mod token;
mod ast;
mod parser;
mod repl;
mod value;
mod interpreter;

use ast::{Program, Stmt};
use scanner::Scanner;
use std::env;
use std::fs;

// entry point that decides whether to run the REPL or process a source file
fn main() {
    let args: Vec<String> = env::args().collect();
    let args: Vec<&str> = args.iter().map(|s| s.as_str()).collect();

    // command-line arguments determine which stage of the interpreter will run
    match args.as_slice() {
        // ./run
        [_] => repl::run_repl(repl::Mode::Eval),

        // ./run --tokenize | --parse | --eval   (REPL modes)
        [_, "--tokenize"] => repl::run_repl(repl::Mode::Tokenize),
        [_, "--parse"] => repl::run_repl(repl::Mode::Parse),
        [_, "--eval"] => repl::run_repl(repl::Mode::Eval),

        // ./run --tokenize | --parse | --eval <path>   (file modes)
        [_, "--tokenize", path] => run_tokenize(path),
        [_, "--parse", path] => run_parse(path),
        [_, "--eval", path] => run_eval(path),

        // ./run file.<ext>
        [_, path] => run_program(path),

        // anything else
        _ => {
            eprintln!("Usage: {} [--tokenize | --parse | --eval] [<path>]", args[0]);
            std::process::exit(64);
        }
    }
}

// reads the source file or exits with an error message instead of panicking
fn read_source(path: &str) -> String {
    match fs::read_to_string(path) {
        Ok(source) => source,
        Err(err) => {
            eprintln!("Error reading '{}': {}", path, err);
            std::process::exit(65);
        }
    }
}

// scan + parse a file; any static error exits 65 before anything runs.
// shared by --parse, --eval and plain ./run so the front end exists only once
fn load_program(path: &str) -> Program {
    let source = read_source(path);

    let mut s = Scanner::new(&source);
    let tokens = s.scan_tokens().clone();
    if s.had_error {
        std::process::exit(65);
    }

    let mut p = parser::Parser::new(tokens);
    let (program, had_error) = p.parse_program();
    if had_error {
        std::process::exit(65);
    }
    program
}

// runs only the scanner and prints the tokens produced from the source file
fn run_tokenize(path: &str) {
    let source = read_source(path);

    let mut s = Scanner::new(&source);
    let tokens = s.scan_tokens();
    for t in tokens {
        println!("{}", t);
    }

    if s.had_error {
        std::process::exit(65);
    }
}

// scans and parses the file, then prints the resulting AST
fn run_parse(path: &str) {
    let program = load_program(path);

    let out = program.to_string();
    if !out.is_empty() {
        println!("{}", out);
    }
}

// --eval keeps the Lab 3 behavior: every expression statement prints its value.
// show statements still go through execute.
fn run_eval(path: &str) {
    let program = load_program(path);
    let mut interp = interpreter::Interpreter::new();

    for stmt in &program.statements {
        let result = match stmt {
            Stmt::Expression(e) => interpreter::eval(e).map(|v| println!("{}", v)),
            other => interp.execute(other),
        };
        if let Err(err) = result {
            err.report();            // stderr
            std::process::exit(70);  // runtime error
        }
    }
}

// plain ./run <file>: execute statements in order; only show produces output
fn run_program(path: &str) {
    let program = load_program(path);
    let mut interp = interpreter::Interpreter::new();

    for stmt in &program.statements {
        if let Err(err) = interp.execute(stmt) {
            err.report();            // stderr
            std::process::exit(70);  // runtime error
        }
    }
}