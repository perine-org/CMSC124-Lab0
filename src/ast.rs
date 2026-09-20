use crate::token::{Literal as TokenLiteral, Token};

#[derive(Debug, Clone)]
pub enum Expr {
    Literal(TokenLiteral),

    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
} 


