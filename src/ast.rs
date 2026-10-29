use crate::token::{Literal, Token};
use std::fmt;

// root of the tree: one node that owns every top-level statement
#[derive(Debug, Clone)]
pub struct Program {
    pub statements: Vec<Stmt>,
}

// each variant represents a different kind of expression the parser can build
#[derive(Debug, Clone)]
pub enum Expr {
    Literal(Literal),

    Grouping {
        expression: Box<Expr>,
    },

    // keeps both sides and the operator so the interpreter can evaluate the expression later
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
    },
}

// represents executable statements, separate from value-producing expressions
#[derive(Debug, Clone)]
pub enum Stmt {
    // evaluates an expression for its side effects and discards the value
    Expression(Expr),

    // evaluates an expression and prints its runtime value to stdout
    Show(Expr),
}

// one statement per line, so --parse output stays readable
impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, s) in self.statements.iter().enumerate() {
            if i > 0 {
                writeln!(f)?;
            }
            write!(f, "{}", s)?;
        }
        Ok(())
    }
}

// keeps AST output in a readable form for parse mode and debugging
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

// keeps statement AST nodes printable for debugging and inspection
// a bare expression prints as itself, so existing --parse output is unchanged
impl fmt::Display for Stmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Stmt::Expression(expr) => write!(f, "{}", expr),
            Stmt::Show(expr) => write!(f, "(show {})", expr),
        }
    }
}