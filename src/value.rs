use crate::token::Literal;
use std::fmt;


// I keep the 'nil spelling here so it can print the same way.
// TODO: we still have no nil literal yet, but i made the spelling indicator here for printing
pub const NIL: &str = "nil";

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(f64),
    Str(String),
    Bool(bool),
    Nil,
}

impl Value {

    // matched each literal so I can keep its value without changing it.
    pub fn from_literal(literal: &Literal) -> Value {
        match literal {
            Literal::Number(n) => Value::Number(*n),
            Literal::Str(s) => Value::Str(s.clone()),
            Literal::Bool(b) => Value::Bool(*b),
            // 'None' has no value, so I treat it as nil here.
            Literal::None => Value::Nil,
        }
    }

    pub fn is_truthy(&self) -> bool{
        match self {
            Value::Nil => false,          // nil is falsey
            Value::Bool(b) => *b,         // true is truthy, false is falsey
            Value::Number(_) => true,     // every number is truthy, even 0
            Value::Str(_) => true,        // every string is truthy, even ""
        }
    }

    //used the derived equality so different types don't compare as equal by accident
    #[allow(dead_code)] // used by '==' / '!=' once comparison operators land
    pub fn is_equal(&self, other: &Value) -> bool {
        self == other
    }
}

impl fmt::Display for Value{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self{
            Value::Number(n) => write!(f, "{}", format_number(*n)),
            Value::Str(s) => write!(f, "{}", s),
            Value::Bool(b) => write!(f, "{}", b),
            Value::Nil => write!(f, "{}", NIL),
        }
    }
}

/// Hard coded these cases myself so the output stays consistent.
fn format_number(n: f64) -> String {
    if n.is_nan() {
        "NaN".to_string()
    } else if n.is_infinite() {
        if n > 0.0 { "Infinity".to_string() } else { "-Infinity".to_string() }
    } else if n == 0.0 {
        "0".to_string() // both zero forms to print as `0`.
    } else {
        format!("{}", n)
    }
}



