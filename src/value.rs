use crate::callable::Callable;
use crate::token::Literal;
use std::fmt;
use std::rc::Rc;


// I keep the 'nil spelling here so it can print the same way.
// TODO: we still have no nil literal yet, but i made the spelling indicator here for printing
pub const NIL: &str = "nil";

// represents the values that expressions can produce during evaluation
// Debug and PartialEq, a function holds an environment and
// code, which can't be derived or compared by content
#[derive(Clone)]
pub enum Value {
    Number(f64),
    Str(String),
    Bool(bool),
    Nil,
    // functions are ordinary values, so they can be stored, passed and returned
    Function(Rc<dyn Callable>),
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Number(n) => write!(f, "Number({:?})", n),
            Value::Str(s) => write!(f, "Str({:?})", s),
            Value::Bool(b) => write!(f, "Bool({:?})", b),
            Value::Nil => write!(f, "Nil"),
            Value::Function(func) => write!(f, "Function({})", func.name()),
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Value) -> bool {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => a == b,
            (Value::Str(a), Value::Str(b)) => a == b,
            (Value::Bool(a), Value::Bool(b)) => a == b,
            (Value::Nil, Value::Nil) => true,
            // two function values are equal only if they are the very same function object
            (Value::Function(a), Value::Function(b)) => Rc::ptr_eq(a, b),
            _ => false,
        }
    }
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

    // defines how each runtime value behaves when used as a boolean
    pub fn is_truthy(&self) -> bool{
        match self {
            Value::Nil => false,          // nil is falsey
            Value::Bool(b) => *b,         // true is truthy, false is falsey
            Value::Number(_) => true,     // every number is truthy, even 0
            Value::Str(_) => true,        // every string is truthy, even ""
            Value::Function(_) => true,   // functions are truthy
        }
    }

    // used for == and !=; different types don't match
    // so 1 == "1" is false, and NaN doesn't equal itself
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
            Value::Function(func) => write!(f, "<fn {}>", func.name()),
        }
    }
}

// keeps number output consistent across the interpreter
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