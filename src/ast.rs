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
    
    /*
    Unary: Needed for "not x" and "-x". Before, 
    factor() just called primary() so there was no node
    type for these even though scanner already makes not/minus tokens
     */
    Unary {
        operator: Token,
        right: Box<Expr>,
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Literal(lit) => match lit {
                // AST printer always shows a decimal point: 1 -> 1.0, 3.14 -> 3.14
                Literal::Number(n) if n.fract() == 0.0 => write!(f, "{:.1}", n),
                other => write!(f, "{}", other),
            },
            Expr::Grouping { expression } => write!(f, "(group {})", expression),
            Expr::Binary { left, operator, right } => {
                write!(f, "({} {} {})", operator.lexeme, left, right)
            }
            Expr::Unary { operator, right } => {
                write!(f, "({} {})", operator.lexeme, right)
            }
        }
    }
}