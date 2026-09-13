mod scanner;
mod token;

use scanner::Scanner;
use std::env;
use std::fs;
use std::io::{self, Write};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        run_repl();
    } else if args.len() == 3 && args[1] == "--tokenize" {
        // ./run --tokenize <path>
        run_tokenize(&args[2]);
    } else if args.len() == 2 {
        // ./run 
        let source = fs::read_to_string(&args[1]).expect("Failed to read source file");
        print!("{source}");
    } else {
        eprintln!("Usage: {} [--tokenize <path>] | <path>", args[0]);
        std::process::exit(64);
    }
}

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

fn run_repl() {
    let stdin = io::stdin();
    loop {
        print!("> ");
        io::stdout().flush().expect("Failed to flush stdout");

        let mut line = String::new();
        let bytes_read = stdin.read_line(&mut line).expect("Failed to read line");

        if bytes_read == 0 {
            // EOF 
            break;
        }

        let mut s = Scanner::new(&line);
        let tokens = s.scan_tokens();
        for t in tokens {
            println!("{}", t);
        }

    }
}