mod scanner;
mod token;
mod ast;
mod parser;
mod repl;

use scanner::Scanner;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    // ./run
    if args.len() == 1 {
        repl::run_repl();
    } else if args.len() == 3 && args[1] == "--tokenize" {
        // ./run --tokenize <path>
        run_tokenize(&args[2]);
    } else if args.len() == 2 {
        // ./run file.<ext>
        run_program(&args[1]);
        // ./run --parse
    } else if args.len() == 3 && args[1] == "--parse" {
        run_parse(&args[2]);
    } else {
        eprintln!("Usage: {} [--tokenize <path>] | <path>", args[0]);
        std::process::exit(64);
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

    let mut p = parser::Parser::new(tokens);
    let expr = p.parse();
    println!("{}", expr);
}

// run program, placeholder until lab 4 execution exists)
fn run_program(_path: &str) {
    println!("Hello, world!");
}