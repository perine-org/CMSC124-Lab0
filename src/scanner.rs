use crate::token::{Token, TokenType};

pub struct Scanner {
    source: Vec<char>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    pub had_error: bool,
}

impl Scanner {
    pub fn new(source: &str) -> Self {
        Scanner {
            source: source.chars().collect(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            had_error: false,
        }
    }

    // loops over the whole input, one token at a time,
    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens
            .push(Token::new(TokenType::Eof, String::new(), self.line));
        &self.tokens
    }

    // determines what type of token
    fn scan_token(&mut self) {
        let c = self.advance();

        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ';' => self.add_token(TokenType::Semicolon),
            '+' => self.add_token(TokenType::Plus),
            '-' => self.add_token(TokenType::Minus),
            '/' => self.add_token(TokenType::Divide),
            '*' => self.add_token(TokenType::Multiply),
            ',' => self.add_token(TokenType::Comma),
            '<' => self.add_token(TokenType::Less),
            '>' => self.add_token(TokenType::Greater),
            '=' => self.add_token(TokenType::Assign),

            // handles tabs whitespaces chuchu
            ' ' | '\r' | '\t' => {}
            '\n' => self.line += 1,

            _ => {
                if c.is_ascii_digit() {
                    self.number();
                } else if is_identifier_start(c) {
                    self.identifier();
                } else {
                    self.error(&format!("Unexpected character '{}'.", c));
                }
            }
        }
    }

    // handles words and names
    fn identifier(&mut self) {
        while is_identifier_continue(self.peek()) {
            self.advance();
        }

        self.add_token(TokenType::Identifier);
    }

    // handles numbers
    fn number(&mut self) {
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            self.advance(); 
            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }

        self.add_token(TokenType::Number);
    }


    // a helper that lets the scanner "look ahead"
    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            '\0'
        } else {
            self.source[self.current + 1]
        }
    }

    // a simple check:has the scanner run out of input
    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    // moves forward one character and returns it

    fn advance(&mut self) -> char {
        let c = self.source[self.current];
        self.current += 1;
        c
    }

    // loks at the next character without consuming it
    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source[self.current]
        }
    }

    fn add_token(&mut self, token_type: TokenType) {
        let text: String = self.source[self.start..self.current].iter().collect();
        self.tokens.push(Token::new(token_type, text, self.line));
    }

    // scanning didn't go cleanly.

    fn error(&mut self, message: &str) {
        self.had_error = true;
        eprintln!("[line {}] Error: {}", self.line, message);
    }
}

fn is_identifier_start(c: char) -> bool {
    c.is_alphabetic() || c == '~'
}

fn is_identifier_continue(c: char) -> bool {
    c.is_alphanumeric() || c == '~'
}