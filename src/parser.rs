use crate::ast::{Expr, Program, Stmt};
use crate::token::{Literal, Token, TokenType};

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

    // * GRAMMAR RULE: <program> ::= <declaration>* EOF
    // parses a whole file or REPL line into statements.
    // on an error, synchronize to the next ';' and continue, so one bad
    // statement doesn't lose the rest of the file.
    pub fn parse_program(&mut self) -> (Program, bool) {
        let mut statements = Vec::new();
        while !self.is_at_end() {
            match self.declaration() {
                Ok(stmt) => statements.push(stmt),
                Err(_) => self.synchronize(),
            }
        }
        (Program { statements }, self.had_error)
    }

    // * <declaration> ::= <set_decl> | <deal_decl> | <statement>
    fn declaration(&mut self) -> Result<Stmt, ParseError> {
        if self.match_types(&[TokenType::Set]) {
            return self.var_declaration(true);
        }
        if self.match_types(&[TokenType::Deal]) {
            return self.var_declaration(false);
        }
        self.statement()
    }

    // * <set_decl>  ::= "set" IDENTIFIER ( "=" <expression> )? ";"
    // * <deal_decl> ::= "deal" IDENTIFIER "=" <expression> ";"
    fn var_declaration(&mut self, mutable: bool) -> Result<Stmt, ParseError> {
        let name = self.consume(TokenType::Identifier, "Expect variable name.")?;

        let initializer = if mutable {
            if self.match_types(&[TokenType::Assign]) {
                Some(self.parse_expression()?)
            } else {
                None
            }
        } else {
            // `deal` bindings cannot be reassigned later, so an initializer is required
            self.consume(TokenType::Assign, "Expect '=' after immutable variable name.")?;
            Some(self.parse_expression()?)
        };

        self.consume(TokenType::Semicolon, "Expect ';' after variable declaration.")?;
        Ok(Stmt::VarDecl {
            name,
            initializer,
            mutable,
        })
    }

    // * <statement> ::= <if_stmt> | <while_stmt> | <for_stmt> | <show_stmt> | <block> | <expr_stmt>
    // TODO: Control-flow keywords (if, else, while, for) are for template purposes in Lab 5 and will change once we decide on our final programming language ideas.
    fn statement(&mut self) -> Result<Stmt, ParseError> {
        if self.match_types(&[TokenType::If]) {
            return self.if_statement();
        }
        if self.match_types(&[TokenType::While]) {
            return self.while_statement();
        }
        if self.match_types(&[TokenType::For]) {
            return self.for_statement();
        }
        if self.match_types(&[TokenType::Show]) {
            return self.show_statement();
        }
        // an opening '{' starts a nested block scope; consume it and parse until '}'
        if self.match_types(&[TokenType::LeftBrace]) {
            return Ok(Stmt::Block(self.block()?));
        }
        self.expr_statement()
    }

    // * <if_stmt> ::= "if" "(" <expression> ")" <statement> ( "else" <statement> )?
    // matches `else` right away so it attaches to the closest `if`
    fn if_statement(&mut self) -> Result<Stmt, ParseError> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'if'.")?;
        let condition = self.parse_expression()?;
        self.consume(TokenType::RightParen, "Expect ')' after if condition.")?;

        let then_branch = Box::new(self.statement()?);
        let else_branch = if self.match_types(&[TokenType::Else]) {
            Some(Box::new(self.statement()?))
        } else {
            None
        };

        Ok(Stmt::If {
            condition,
            then_branch,
            else_branch,
        })
    }

    // * <while_stmt> ::= "while" "(" <expression> ")" <statement>
    fn while_statement(&mut self) -> Result<Stmt, ParseError> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'while'.")?;
        let condition = self.parse_expression()?;
        self.consume(TokenType::RightParen, "Expect ')' after condition.")?;
        let body = Box::new(self.statement()?);

        Ok(Stmt::While { condition, body })
    }

    // * <for_stmt> ::= "for" "(" ( <set_decl> | <deal_decl> | <expr_stmt> | ";" ) <expression>? ";" <expression>? ")" <statement>
    // turns a `for` loop into a `while` loop inside a block so the interpreter doesn't need extra `for` logic
    fn for_statement(&mut self) -> Result<Stmt, ParseError> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'for'.")?;

        // 1. starting part: can be empty `;`, a `set`/`deal` variable, or an expression
        let initializer = if self.match_types(&[TokenType::Semicolon]) {
            None
        } else if self.match_types(&[TokenType::Set]) {
            Some(self.var_declaration(true)?)
        } else if self.match_types(&[TokenType::Deal]) {
            Some(self.var_declaration(false)?)
        } else {
            Some(self.expr_statement()?)
        };

        // 2. condition: if left empty (like `for (;;)`), treat it as `true` so it loops forever
        let condition = if self.check(&TokenType::Semicolon) {
            Expr::Literal(Literal::Bool(true))
        } else {
            self.parse_expression()?
        };
        self.consume(TokenType::Semicolon, "Expect ';' after loop condition.")?;

        // 3. step/update part: runs at the end of every loop turn if provided
        let increment = if self.check(&TokenType::RightParen) {
            None
        } else {
            Some(self.parse_expression()?)
        };
        self.consume(TokenType::RightParen, "Expect ')' after for clauses.")?;

        // 4. build the `while` loop: put the step at the end of the body, then put the starting variable outside
        let mut body = self.statement()?;

        if let Some(inc) = increment {
            body = Stmt::Block(vec![body, Stmt::Expression(inc)]);
        }

        let while_loop = Stmt::While {
            condition,
            body: Box::new(body),
        };

        if let Some(init) = initializer {
            Ok(Stmt::Block(vec![init, while_loop]))
        } else {
            Ok(while_loop)
        }
    }

    // * <block> ::= "{" <declaration>* "}"
    // collects declarations and statements inside braces so blocks can nest to any depth
    fn block(&mut self) -> Result<Vec<Stmt>, ParseError> {
        let mut statements = Vec::new();

        // keep parsing inner declarations until we see the closing '}' or hit EOF
        while !self.check(&TokenType::RightBrace) && !self.is_at_end() {
            match self.declaration() {
                Ok(stmt) => statements.push(stmt),
                Err(_) => {
                    // only synchronize if the next token isn't already the closing '}',
                    // so a missing ';' right before '}' doesn't eat the block's closing brace
                    if !self.check(&TokenType::RightBrace) {
                        self.synchronize();
                    }
                }
            }
        }

        // report an unclosed block if we hit EOF without finding '}'
        self.consume(TokenType::RightBrace, "Expect '}' after block.")?;
        Ok(statements)
    }

    // * <show_stmt> ::= "show" <expression> ";"
    fn show_statement(&mut self) -> Result<Stmt, ParseError> {
        let value = self.parse_expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after value.")?;
        Ok(Stmt::Show(value))
    }

    // * <expr_stmt> ::= <expression> ";"
    fn expr_statement(&mut self) -> Result<Stmt, ParseError> {
        let expr = self.expression_statement()?;
        Ok(Stmt::Expression(expr))
    }

    // expression ";"
    fn expression_statement(&mut self) -> Result<Expr, ParseError> {
        let expr = self.parse_expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after expression.")?;
        Ok(expr)
    }

    // *GRAMMAR RULE: <expression> ::= <assignment>
    pub fn parse_expression(&mut self) -> Result<Expr, ParseError> {
        self.assignment()
    }

    // *GRAMMAR RULE: <assignment> ::= IDENTIFIER "=" <assignment> | <logic_or>
    // parses the left side as a general expression first, then validates the L-value if '=' follows
    fn assignment(&mut self) -> Result<Expr, ParseError> {
        let expr = self.logic_or()?;

        if self.match_types(&[TokenType::Assign]) {
            let equals = self.previous();
            // recurses into assignment() so chained assignments like `a = b = 5` are right-associative
            let value = self.assignment()?;

            if let Expr::Variable(name) = expr {
                return Ok(Expr::Assign {
                    name,
                    value: Box::new(value),
                });
            }

            return Err(self.error(equals, "Invalid assignment target."));
        }

        Ok(expr)
    }

    // *GRAMMAR RULE: <logic_or> ::= <logic_and> { "or" <logic_and> }
    fn logic_or(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.logic_and()?;
        while self.match_types(&[TokenType::Or]) {
            let operator = self.previous();
            let right = self.logic_and()?;
            expr = Expr::Logical {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    // *GRAMMAR RULE: <logic_and> ::= <equality> { "and" <equality> }
    fn logic_and(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.equality()?;
        while self.match_types(&[TokenType::And]) {
            let operator = self.previous();
            let right = self.equality()?;
            expr = Expr::Logical {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    // *GRAMMAR RULE: <equality> ::= <comparison> { ( "==" | "!=" ) <comparison> }
    fn equality(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.comparison()?;
        while self.match_types(&[TokenType::Equal, TokenType::NotEqual]) {
            let operator = self.previous();
            let right = self.comparison()?;
            expr = Expr::Binary { left: Box::new(expr), operator, right: Box::new(right) };
        }
        Ok(expr)
    }

    // *GRAMMAR RULE: <comparison> ::= <term> { ( "<" | "<=" | ">" | ">=" ) <term> }
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

    // *GRAMMAR RULE: <term> ::= <factor> { ( "+" | "-" ) <factor> }
    fn term(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.factor()?;
        while self.match_types(&[TokenType::Plus, TokenType::Minus]) {
            let operator = self.previous();
            let right = self.factor()?;
            expr = Expr::Binary { left: Box::new(expr), operator, right: Box::new(right) };
        }
        Ok(expr)
    }

    // *GRAMMAR RULE: <factor> ::= <unary> { ( "*" | "/" | "%" ) <unary> }
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

    // *GRAMMAR RULE: <unary> ::= ( "not" | "-" ) <unary> | <primary>
    // calls itself recursively to handle prefix signs and allow chained operators like --5 or not not true
    fn unary(&mut self) -> Result<Expr, ParseError> {
        if self.match_types(&[TokenType::Not, TokenType::Minus]) {
            let operator = self.previous();
            let right = self.unary()?;
            return Ok(Expr::Unary { operator, right: Box::new(right) });
        }

        self.primary()
    }

    // *GRAMMAR RULE: <primary> ::= NUMBER | STRING | "true" | "false" | IDENTIFIER | "(" <expression> ")"
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

        if self.match_types(&[TokenType::Identifier]) {
            return Ok(Expr::Variable(self.previous()));
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

    /// Error recovery: discards tokens until a statement or declaration boundary.
    /// Always advances at least one token first so malformed tokens (like a stray '}')
    /// can never cause an infinite loop.
    fn synchronize(&mut self) {
        // always consume at least one token so we never get stuck on a bad token (15s timeout)
        self.advance();

        while !self.is_at_end() {
            // stop right after consuming a ';' since the next token starts a new statement
            if self.previous().token_type == TokenType::Semicolon {
                return;
            }

            // or stop right before a keyword or brace that can begin or close a statement/block
            match self.peek().token_type {
                TokenType::Set
                | TokenType::Deal
                | TokenType::Show
                | TokenType::If
                | TokenType::While
                | TokenType::For
                | TokenType::LeftBrace
                | TokenType::RightBrace
                | TokenType::Call
                | TokenType::Flush
                | TokenType::Fold
                | TokenType::Bet
                | TokenType::Bust
                | TokenType::Round => return,
                _ => {}
            }

            self.advance();
        }
    }
}