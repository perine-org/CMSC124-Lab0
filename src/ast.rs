use crate::token::{Literal, Token};

#[derive(Debug, Clone)]
pub enum Expr {
    Literal(Literal),

    Grouping {
        expression: Box<Expr>,
    },

    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
}