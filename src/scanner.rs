use crate::token::{Literal, Token, TokenType};

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

    // loops over the whole input, one token at a time
    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens
            .push(Token::new(TokenType::Eof, String::new(), Literal::None, self.line));
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
            '<' => {
                if self.peek() == '=' {
                    self.advance();
                    self.add_token(TokenType::LessEqual);
                } else {
                    self.add_token(TokenType::Less);
                }
            }
            '>' => {
                if self.peek() == '=' {
                    self.advance();
                    self.add_token(TokenType::GreaterEqual);
                } else {
                    self.add_token(TokenType::Greater);
                }
            }
            '=' => {
                if self.peek() == '=' {
                    self.advance();
                    self.add_token(TokenType::Equal);
                } else {
                    self.add_token(TokenType::Assign);
                }
            }

            // maximal munch: check for the longer !!! form before falling back to !
            '!' => {
                if self.peek() == '!' && self.peek_next() == '!' {
                    self.advance(); 
                    self.advance(); 
                    self.block_comment();
                } else {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                }
            }

            '"' => self.string(),

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

        let text: String = self.source[self.start..self.current].iter().collect();
        let token_type = match text.as_str() {
            "set" => TokenType::Set,
            "deal" => TokenType::Deal,
            "call" => TokenType::Call,
            "flush" => TokenType::Flush,
            "fold" => TokenType::Fold,
            "bet" => TokenType::Bet,
            "bust" => TokenType::Bust,
            "round" => TokenType::Round,
            "bluff" => TokenType::Bluff,
            "draw" => TokenType::Draw,
            "raise" => TokenType::Raise,
            "show" => TokenType::Show,
            _ => TokenType::Identifier,
        };

        self.add_token(token_type);
    }

    // handles numbers, including decimals
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

        let text: String = self.source[self.start..self.current].iter().collect();
        let value: f64 = text.parse().expect("Failed to parse number literal");
        self.add_token_with_literal(TokenType::Number, Literal::Number(value));
    }

        // handles string literals
    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            self.error("Unterminated string.");
            return;
        }

        self.advance(); // consume closing "

        // extract the string value between the quotes
        let value: String = self.source[self.start + 1..self.current - 1].iter().collect();
        self.add_token_with_literal(TokenType::Str, Literal::Str(value));
    }

    // function for reading block comments
    fn block_comment(&mut self) {
        while !self.is_at_end() {
            if self.peek() == '!' && self.peek_next() == '!' && self.peek_next_next() == '!' {
                self.advance(); 
                self.advance();
                self.advance();
                return; 
            }

            if self.peek() == '\n' {
                self.line += 1; // keep line numbers through the comment
            }
            self.advance();
        }

        // fell through the loop: hit EOF without finding a closing !!!
        self.error("Unterminated block comment.");
    }

    // a helper that lets the scanner "look ahead"
    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            '\0'
        } else {
            self.source[self.current + 1]
        }
    }

    // looks two characters ahead without consuming, needed for !!!
    fn peek_next_next(&self) -> char {
        if self.current + 2 >= self.source.len() {
            '\0'
        } else {
            self.source[self.current + 2]
        }
    }

    // a simple check: has the scanner run out of input
    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    // moves forward one character and returns it
    fn advance(&mut self) -> char {
        let c = self.source[self.current];
        self.current += 1;
        c
    }

    // looks at the next character without consuming it
    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source[self.current]
        }
    }

    // for tokens with no literal value
    fn add_token(&mut self, token_type: TokenType) {
        self.add_token_with_literal(token_type, Literal::None);
    }

    // for tokens that carry an actual value
    fn add_token_with_literal(&mut self, token_type: TokenType, literal: Literal) {
        let text: String = self.source[self.start..self.current].iter().collect();
        self.tokens.push(Token::new(token_type, text, literal, self.line));
    }

    // scanning didn't go cleanly
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