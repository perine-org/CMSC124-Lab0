use crate::ast::Expr;
use crate::token::{Token, TokenType};

/*
expression → equality
equality   → comparison ( "==" comparison )*
comparison → term ( ( "<" | "<=" | ">" | ">=" ) term )*
term       → factor ( ( "+" | "-" ) factor )*
factor     → primary ( ( "*" | "/" ) primary )*
primary    → NUMBER | STRING | "(" expression ")"
*/
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    pub fn parse_expression(&mut self) -> Expr {
        self.equality()
    }

    fn term(&mut self) -> Expr {
        let mut expr = self.factor();
        while self.match_types(&[TokenType::Plus, TokenType::Minus]){
            let operator = self.previous();
            let right = self.factor();
            expr = Expr::Binary { left: Box::new(expr), operator, right: Box::new(right) };
        }
        expr

    }

    fn comparison(&mut self) -> Expr {
        let mut expr = self.term();
        while self.match_types(&[
           TokenType::Greater, TokenType::GreaterEqual,
           TokenType:: Less, TokenType::LessEqual,
        ]) {
            let operator = self.previous();
            let right = self.term();
            expr = Expr::Binary {left: Box::new(expr), operator, right: Box::new(right)};
        }
        expr
    }

    fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();
        while self.match_types(&[TokenType::Equal, TokenType::NotEqual]) {
            let operator = self.previous();
            let right = self.comparison();
            expr = Expr::Binary { left: Box::new(expr), operator, right: Box::new(right) };
        }
        expr
    }

    fn factor(&mut self) -> Expr {
        let mut expr = self.primary();

        while self.match_types(&[TokenType::Divide, TokenType::Multiply]) {
            let operator = self.previous();
            let right = self.primary();
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        expr
    }

    // primary 
    fn primary(&mut self) -> Expr {
        if self.match_types(&[TokenType::Number]) {
            return Expr::Literal(self.previous().literal.clone());
        }

        if self.match_types(&[TokenType::Str]) {
            return Expr::Literal(self.previous().literal.clone());
        }

        if self.match_types(&[TokenType::LeftParen]) {
            let expr = self.parse_expression();
            self.consume(TokenType::RightParen, "Expect ')' after expression.");
            return Expr::Grouping {
                expression: Box::new(expr),
            };
        }

        self.error(self.peek(), "Expect expression.")
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

    fn consume(&mut self, token_type: TokenType, message: &str) -> Token {
        if self.check(&token_type) {
            return self.advance();
        }
        self.error(self.peek(), message)
    }

    fn error(&self, token: &Token, message: &str) -> ! {
        if token.token_type == TokenType::Eof {
            eprintln!("[line {}] Error at end: {}", token.line, message);
        } else {
            eprintln!("[line {}] Error at '{}': {}", token.line, token.lexeme, message);
        }
        std::process::exit(65);
    }
}