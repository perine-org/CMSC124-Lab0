use crate::scanner::Scanner;
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
        let tokens = s.scan_tokens();
        for t in tokens {
            println!("{}", t);
        }
    }
}