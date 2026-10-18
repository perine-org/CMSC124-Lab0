mod scanner;
mod token;
mod ast;
mod parser;
mod repl;
mod value;
mod interpreter;

use scanner::Scanner;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let args: Vec<&str> = args.iter().map(|s| s.as_str()).collect();

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

// run tokenize
fn run_tokenize(path: &str) {
    let source = fs::read_to_string(path).expect("Failed to read source file");

    let mut s = Scanner::new(&source);
    let tokens = s.scan_tokens();
    for t in tokens {
        println!("{}", t);
    }

    if s.had_error {
        std::process::exit(65);
    }
}

// run parse
fn run_parse(path: &str) {
    let source = fs::read_to_string(path).expect("Failed to read source file");

    let mut s = Scanner::new(&source);
    let tokens = s.scan_tokens().clone();

    if s.had_error {
        std::process::exit(65);
    }

    /* parse() returns Result instead of exiting from inside the parser
    in ref to parser.rs - this is the one place that turns a parse failure
    into the run contract's "nothing on stdout, exit 65". */
    let mut p = parser::Parser::new(tokens);
    let (program, had_error) = p.parse_program();
    if had_error {
        std::process::exit(65);
    }

    let out = program.to_string();
    if !out.is_empty() {
        println!("{}", out);
    }
}



// run eval: parse everything first so static errors happen before any output
fn run_eval(path: &str) {
    let source = fs::read_to_string(path).expect("Failed to read source file"); // Week 3: replace the .expect

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

    for e in program.expressions {
        match interpreter::eval(&e) {
            Ok(v) => println!("{}", v),
            Err(err) => {
                err.report();            // stderr
                std::process::exit(70);  // runtime error
            }
        }
    }
}

// run program, placeholder until lab 4 execution exists)
fn run_program(_path: &str) {
    println!("Hello, world!");
}