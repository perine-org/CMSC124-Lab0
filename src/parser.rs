use crate::ast::Expr;
use crate::token::{Token, TokenType};

/*
program    → ( expression ";" )* EOF
expression → equality
equality   → comparison ( ( "==" | "!=" ) comparison )*
comparison → term ( ( "<" | "<=" | ">" | ">=" ) term )*
term       → factor ( ( "+" | "-" ) factor )*
factor     → unary ( ( "*" | "/" | "%" ) unary )*
unary      → ( "not" | "-" ) unary | primary
primary    → NUMBER | STRING | "true" | "false" | "(" expression ")"
*/

#[derive(Debug)]
// empty error type so functions can use '?' to
// instantly exit and pass errors up the chain on failure
pub struct ParseError;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
    // tracks if any error occurred so the program can still exit with code 65 even if it recovered
    pub had_error: bool,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0, had_error: false }
    }

    /// Parses a whole file: a sequence of `expression ";"`. On a syntax error we
    /// synchronize to the next ';' and continue, so one bad expression doesn't
    /// lose the rest of the file. Returns the trees and whether any error occurred.
    pub fn parse_program(&mut self) -> (Vec<Expr>, bool) {
        let mut exprs = Vec::new();
        while !self.is_at_end() {
            match self.expression_statement() {
                Ok(e) => exprs.push(e),
                Err(_) => self.synchronize(),
            }
        }
        (exprs, self.had_error)
    }

    // expression ";"
    fn expression_statement(&mut self) -> Result<Expr, ParseError> {
        let expr = self.parse_expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after expression.")?;
        Ok(expr)
    }

    pub fn parse_expression(&mut self) -> Result<Expr, ParseError> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.comparison()?;
        while self.match_types(&[TokenType::Equal, TokenType::NotEqual]) {
            let operator = self.previous();
            let right = self.comparison()?;
            expr = Expr::Binary { left: Box::new(expr), operator, right: Box::new(right) };
        }
        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.term()?;
        while self.match_types(&[
            TokenType::Greater, TokenType::GreaterEqual,
            TokenType::Less, TokenType::LessEqual,
        ]) {
            let operator = self.previous();
            let right = self.term()?;
            expr = Expr::Binary { left: Box::new(expr), operator, right: Box::new(right) };
        }
        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.factor()?;
        while self.match_types(&[TokenType::Plus, TokenType::Minus]) {
            let operator = self.previous();
            let right = self.factor()?;
            expr = Expr::Binary { left: Box::new(expr), operator, right: Box::new(right) };
        }
        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, ParseError> {
        // calls unary() to evaluate prefix signs (e.g. -5) before multiplying or dividing
        let mut expr = self.unary()?;

        while self.match_types(&[TokenType::Divide, TokenType::Multiply, TokenType::Modulo]) {
            let operator = self.previous();
            let right = self.unary()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    // calls itself recursively to handle prefix signs and allow chained operators like --5 or not not true
    fn unary(&mut self) -> Result<Expr, ParseError> {
        if self.match_types(&[TokenType::Not, TokenType::Minus]) {
            let operator = self.previous();
            let right = self.unary()?;
            return Ok(Expr::Unary { operator, right: Box::new(right) });
        }

        self.primary()
    }

    fn primary(&mut self) -> Result<Expr, ParseError> {
        if self.match_types(&[TokenType::Number]) {
            return Ok(Expr::Literal(self.previous().literal.clone()));
        }

        if self.match_types(&[TokenType::Str]) {
            return Ok(Expr::Literal(self.previous().literal.clone()));
        }

        if self.match_types(&[TokenType::True, TokenType::False]) {
            return Ok(Expr::Literal(self.previous().literal.clone()));
        }

        if self.match_types(&[TokenType::LeftParen]) {
            let expr = self.parse_expression()?;
            self.consume(TokenType::RightParen, "Expect ')' after expression.")?;
            return Ok(Expr::Grouping { expression: Box::new(expr) });
        }

        let peek_token = self.peek().clone();
        Err(self.error(peek_token, "Expect expression."))
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::Eof
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn previous(&self) -> Token {
        self.tokens[self.current - 1].clone()
    }

    fn check(&self, token_type: &TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        &self.peek().token_type == token_type
    }

    fn match_types(&mut self, types: &[TokenType]) -> bool {
        for t in types {
            if self.check(t) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn consume(&mut self, token_type: TokenType, message: &str) -> Result<Token, ParseError> {
        if self.check(&token_type) {
            return Ok(self.advance());
        }
        let error_token = self.peek().clone();
        Err(self.error(error_token, message))
    }

    // prints error details to stderr and returns ParseError to let caller code unwind safely instead of crashing
    fn error(&mut self, token: Token, message: &str) -> ParseError {
        self.had_error = true;
        if token.token_type == TokenType::Eof {
            eprintln!("[line {}] Error at end: {}", token.line, message);
        } else {
            eprintln!("[line {}] Error at '{}': {}", token.line, token.lexeme, message);
        }
        ParseError
    }

    /// Error recovery: discards tokens up to and including the next ';', so
    /// parsing resumes at the start of the next expression. It always consumes
    /// at least one token unless already at Eof, so it can't loop forever.
    fn synchronize(&mut self) {
        while !self.is_at_end() {
            if self.peek().token_type == TokenType::Semicolon {
                self.advance();
                return;
            }
            self.advance();
        }
    }
}