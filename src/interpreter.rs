use crate::ast::Expr;
use crate::token::{Token, TokenType};
use crate::value::Value;

//  this is  when the code is valid, but the value causes a problem at runtime
// keeping the token to show where the error happened
#[derive(Debug)]
pub struct RuntimeError {
    pub token: Token,
    pub message: String,
}

impl RuntimeError {
    fn new(token: &Token, message: &str) -> Self {
        RuntimeError { token: token.clone(), message: message.to_string() }
    }

    // sending errors to stderr so normal output stays clean.
    pub fn report(&self) {
        eprintln!("[line {}] Runtime error: {}", self.token.line, self.message);
    }
}

// evaluation of the expression from the inside out
// If something fails, return the error instead of printing it here
pub fn eval(expr: &Expr) -> Result<Value, RuntimeError> {
    match expr {
        // the scanner already gave the value, so just convert it
        Expr::Literal(literal) => Ok(Value::from_literal(literal)),

        // the parser already handled the parentheses, so I just evaluate what is inside
        Expr::Grouping { expression } => eval(expression),

        Expr::Unary { operator, right } => {
            let right = eval(right)?; // value is needed before using the operator
            match operator.token_type {
                TokenType::Minus => {
                    let n = num(operator, &right)?;
                    Ok(Value::Number(-n))
                }
                TokenType::Not => Ok(Value::Bool(!right.is_truthy())),
                _ => Err(RuntimeError::new(operator, "Unknown unary operator.")),
            }
        }

        Expr::Binary { left, operator, right } => {
            // evaluate left first so its error is reported first
            let left = eval(left)?;
            let right = eval(right)?;
            binary(operator, left, right)
        }
    }
}

// keeps binary operator handling in one place so type checking stays consistent
fn binary(operator: &Token, left: Value, right: Value) -> Result<Value, RuntimeError> {
    match operator.token_type {
        TokenType::Plus => match (&left, &right) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a + b)),
            (Value::Str(a), Value::Str(b)) => Ok(Value::Str(format!("{}{}", a, b))),
            // reject mixed types instead of converting them automatically
            _ => Err(RuntimeError::new(operator, "Operands must be two numbers or two strings.")),
        },
        TokenType::Minus => {
            let (a, b) = nums(operator, &left, &right)?;
            Ok(Value::Number(a - b))
        }
        TokenType::Multiply => {
            let (a, b) = nums(operator, &left, &right)?;
            Ok(Value::Number(a * b))
        }
        TokenType::Divide => {
            let (a, b) = nums(operator, &left, &right)?;
            // check zero first so it can return its own error
            if b == 0.0 {
                return Err(RuntimeError::new(operator, "Division by zero."));
            }
            Ok(Value::Number(a / b))
        }
        TokenType::Modulo => {
            let (a, b) = nums(operator, &left, &right)?;
            // check zero first so `%` gives my runtime error instead of `NaN`
            if b == 0.0 {
                return Err(RuntimeError::new(operator, "Modulo by zero."));
            }
            // keep the left operand's sign for the remainder
            Ok(Value::Number(a % b))
        }
                
        // equality works on any pair of types; different types are simply not equal
        // (1 == "1" is false, not an error), so there is no type check here
        TokenType::Equal => Ok(Value::Bool(left.is_equal(&right))),
        TokenType::NotEqual => Ok(Value::Bool(!left.is_equal(&right))),

        // ordering only makes sense for numbers, so non-number operands are errors
        TokenType::Less => {
            let (a, b) = nums(operator, &left, &right)?;
            Ok(Value::Bool(a < b))
        }
        TokenType::LessEqual => {
            let (a, b) = nums(operator, &left, &right)?;
            Ok(Value::Bool(a <= b))
        }
        TokenType::Greater => {
            let (a, b) = nums(operator, &left, &right)?;
            Ok(Value::Bool(a > b))
        }
        TokenType::GreaterEqual => {
            let (a, b) = nums(operator, &left, &right)?;
            Ok(Value::Bool(a >= b))
        }
        // return an error here because these operators are not ready yet
        _ => Err(RuntimeError::new(
            operator,
            &format!("Operator '{}' is not supported yet.", operator.lexeme),
        )),
    }
}

// I keep the number checks here so I can reuse the same rule.
// I check the values after evaluation, not the expression tree.
// shared type checks for operators that only accept numbers
fn num(operator: &Token, value: &Value) -> Result<f64, RuntimeError> {
    match value {
        Value::Number(n) => Ok(*n),
        _ => Err(RuntimeError::new(operator, "Operand must be a number.")),
    }
}

// shared type checks for operators that only accept numbers
fn nums(operator: &Token, left: &Value, right: &Value) -> Result<(f64, f64), RuntimeError> {
    match (left, right) {
        (Value::Number(a), Value::Number(b)) => Ok((*a, *b)),
        _ => Err(RuntimeError::new(operator, "Operands must be numbers.")),
    }
}
