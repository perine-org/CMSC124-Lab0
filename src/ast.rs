use crate::token::{Literal, Token};
use std::fmt;

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

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Literal(lit) => write!(f, "{}", lit),
            Expr::Grouping { expression } => write!(f, "(group {})", expression),
            Expr::Binary { left, operator, right } => {
                write!(f, "({} {} {})", operator.lexeme, left, right)
            }
        }
    }
}