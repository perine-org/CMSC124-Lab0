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
    LessEqual,
    Greater,
    GreaterEqual,
    Assign,
    Equal,
    NotEqual,
    And,     
    Or,     
    Not,

    Number,
    Identifier,
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

// represents the literal value a token carries, if any.
#[derive(Debug, Clone)]
pub enum Literal {
    Number(f64),
    Str(String),
    None, // keywords, punctuation, operators carry no literal
}

// 
impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Literal::Number(n) => write!(f, "{}", n),
            Literal::Str(s) => write!(f, "{}", s),
            Literal::None => write!(f, "null"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String, // actual text
    pub literal: Literal, // actual value (numbers/strings)
    pub line: usize, 
}

// constructor
impl Token {
    pub fn new(token_type: TokenType, lexeme: String, literal: Literal, line: usize) -> Self {
        Token {
            token_type,
            lexeme,
            literal,
            line,
        }
    }
}

// display formatting for tokens
impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Token(type={}, lexeme={}, literal={}, line={})",
            format_type(&self.token_type),
            self.lexeme,
            self.literal,
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
        TokenType::LessEqual => "LESS_EQUAL",
        TokenType::Greater => "GREATER",
        TokenType::GreaterEqual => "GREATER_EQUAL",
        TokenType::Assign => "ASSIGN",
        TokenType::Equal => "EQUAL",
        TokenType::NotEqual => "NOT_EQUAL",
        TokenType::And => "AND",
        TokenType::Or => "OR",
        TokenType::Not => "NOT",
        TokenType::Number => "NUMBER",
        TokenType::Identifier => "IDENTIFIER",
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