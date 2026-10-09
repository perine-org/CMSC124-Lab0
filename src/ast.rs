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
            Expr::Literal(Literal::Number(n)) => write!(f, "{:?}", n),
            Expr::Literal(lit) => write!(f, "{}", lit),
            Expr::Grouping { expression } => write!(f, "(group {})", expression),
            Expr::Binary { left, operator, right } => {
                write!(f, "({} {} {})", operator.lexeme, left, right)
            }
            /*
            Same prefix with Binary but only one operand.
            This keeps the printer's output consistene e.i (not true) or (-2.0)
             */
            Expr::Unary { operator, right } => {
                write!(f, "({} {})", operator.lexeme, right)
            }
        }
    }
}