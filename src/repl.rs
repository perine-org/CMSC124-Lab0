use crate::interpreter;
use crate::scanner::Scanner;
use crate::parser::Parser;
use std::io::{self, Write};

#[derive(Clone, Copy)]
pub enum Mode {
    Tokenize,
    Parse,
    Eval,
}
// handles one input line at a time through scanning, parsing, and evaluation
pub fn run_repl(mode: Mode) {
    let stdin = io::stdin();
    loop {
        print!("> ");
        // stop the REPL cleanly if stdout fails instead of panicking
        if let Err(err) = io::stdout().flush() {
            eprintln!("Error flushing stdout: {}", err);
            break;
        }

        let mut line = String::new();
        // handle input errors without crashing the session
        let bytes_read = match stdin.read_line(&mut line) {
            Ok(n) => n,
            Err(err) => {
                eprintln!("Error reading input: {}", err);
                break;
            }
        };

        // exit on EOF (Ctrl+D / Ctrl+Z)
        if bytes_read == 0 {
            break;
        }

        let mut s = Scanner::new(&line);
        let tokens = s.scan_tokens().clone();

        // tokenize mode: show the tokens and stop here
        if let Mode::Tokenize = mode {
            for t in &tokens {
                println!("{}", t);
            }
            continue;
        }

        if s.had_error {
            continue;
        }

        // parsing happens after scanning so the REPL works with the same parser as file mode
        let mut p = Parser::new(tokens);
        // parse_program now returns one Program root
        let (program, had_error) = p.parse_program();

        if had_error {
            continue;
        }

        // the selected mode decides whether to print the AST or evaluate it
        match mode {
            Mode::Parse => {
                // prints one line per expression, same format as before
                let out = program.to_string();
                if !out.is_empty() {
                    println!("{}", out);
                }
            }
            Mode::Eval => {
                for e in &program.expressions {
                    match interpreter::eval(e) {
                        Ok(v) => println!("{}", v),
                        Err(err) => {
                            err.report();
                            break; // runtime error: stop this line, keep the session alive
                        }
                    }
                }
            }
            // already handled above, kept as an empty arm to avoid panicking macros
            Mode::Tokenize => {}
        }
    }
}