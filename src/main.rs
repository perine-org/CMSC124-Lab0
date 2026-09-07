mod scanner;
mod token;

use scanner::Scanner;
use std::io::{self, Write};

fn main() {
    print!("Enter : ");
    io::stdout().flush().expect("Failed to flush stdout");

    let mut source = String::new();
    io::stdin()
        .read_line(&mut source)
        .expect("Failed to read line");

    let mut s = Scanner::new(&source);
    let tokens = s.scan_tokens();
    for t in tokens {
        println!("{}", t);
    }
}