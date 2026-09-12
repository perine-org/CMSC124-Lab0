mod scanner;
mod token;

use scanner::Scanner;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <path-to-source-file>", args[0]);
        std::process::exit(64);
    }

    let source = fs::read_to_string(&args[1]).expect("Failed to read source file");