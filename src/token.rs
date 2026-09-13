use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Semicolon,
    Plus,
    Minus,
    Divide,
    Multiply,
    Comma,
    Less,
    Greater,
    Assign,

    Identifier,
    Number,
    Str,

    Set,
    Deal,
    Call,
    Flush,
    Fold,
    Bet,
    Bust,
    Round,
    Bluff,
    Draw,
    Raise,
    Show,

    Eof,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: usize,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, line: usize) -> Self {
        Token {
            token_type,
            lexeme,
            line,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Token(type={}, lexeme={}, line={})",
            format_type(&self.token_type),
            self.lexeme,
            self.line
        )
    }
}

fn format_type(t: &TokenType) -> &'static str {
    match t {
        TokenType::LeftParen => "LEFT_PAREN",
        TokenType::RightParen => "RIGHT_PAREN",
        TokenType::LeftBrace => "LEFT_BRACE",
        TokenType::RightBrace => "RIGHT_BRACE",
        TokenType::Semicolon => "SEMICOLON",
        TokenType::Plus => "PLUS",
        TokenType::Minus => "MINUS",
        TokenType::Divide => "DIVIDE",
        TokenType::Multiply => "MULTIPLY",
        TokenType::Comma => "COMMA",
        TokenType::Less => "LESS",
        TokenType::Greater => "GREATER",
        TokenType::Assign => "ASSIGN",

        TokenType::Identifier => "IDENTIFIER",
        TokenType::Number => "NUMBER",
        TokenType::Str => "STRING",

        TokenType::Set => "SET",
        TokenType::Deal => "DEAL",
        TokenType::Call => "CALL",
        TokenType::Flush => "FLUSH",
        TokenType::Fold => "FOLD",
        TokenType::Bet => "BET",
        TokenType::Bust => "BUST",
        TokenType::Round => "ROUND",
        TokenType::Bluff => "BLUFF",
        TokenType::Draw => "DRAW",
        TokenType::Raise => "RAISE",
        TokenType::Show => "SHOW",
        
        TokenType::Eof => "EOF",
    }
}