use crate::scanner::Scanner;
use crate::parser::Parser;
use std::io::{self, Write};

// lets you run from gitbash
pub fn run_repl() {
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
        // .clone() because scan_tokens() returns a borrowed &Vec<Token>,
        // but Parser::new() needs to own the tokens
        let tokens = s.scan_tokens().clone();

        if s.had_error {
            continue;
        }

        // syntax errors are printed to stderr by the parser
        let mut p = Parser::new(tokens);
        let (exprs, _had_error) = p.parse_program();
        for e in exprs {
            println!("{}", e);
        }
    }
}