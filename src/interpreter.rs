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
        eprintln!("[line {}] Poker error: {}", self.token.line, self.message);
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
        // return an error here because these operators are not ready yet
        _ => Err(RuntimeError::new(
            operator,
            &format!("Operator '{}' is not supported yet.", operator.lexeme),
        )),
    }
}

// I keep the number checks here so I can reuse the same rule.
 // I check the values after evaluation, not the expression tree.

fn num(operator: &Token, value: &Value) -> Result<f64, RuntimeError> {
    match value {
        Value::Number(n) => Ok(*n),
        _ => Err(RuntimeError::new(operator, "Operand must be a number.")),
    }
}

fn nums(operator: &Token, left: &Value, right: &Value) -> Result<(f64, f64), RuntimeError> {
    match (left, right) {
        (Value::Number(a), Value::Number(b)) => Ok((*a, *b)),
        _ => Err(RuntimeError::new(operator, "Operands must be numbers.")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::Parser;
    use crate::scanner::Scanner;

    // running scan, parse, and eval here so each test checks the whole process.
    fn run(source: &str) -> Result<String, String> {
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens().clone();
        let expr = Parser::new(tokens).parse_expression().expect("test source should parse");
        eval(&expr).map(|v| v.to_string()).map_err(|e| e.message)
    }

    fn ok(source: &str, expected: &str) {
        assert_eq!(run(source), Ok(expected.to_string()), "source: {}", source);
    }

    fn err(source: &str, expected: &str) {
        assert_eq!(run(source), Err(expected.to_string()), "source: {}", source);
    }

    #[test]
    fn literals() {
        ok("42", "42");
        ok("3.14", "3.14");
        ok("\"hi\"", "hi");
        ok("true", "true");
        ok("false", "false");
    }

    #[test]
    fn arithmetic() {
        ok("2 + 3 * 4", "14");
        ok("(2 + 3) * 4", "20");
        ok("10 - 4 - 3", "3"); // expectation: the left side to run first
        ok("100 / 10 / 5", "2"); // expectation: the left side to run first
        ok("10 % 3 + 1", "2");
        ok("7 - 2 * 3 + 8 / 4", "3"); // checking if the precedence is correct , mul before add
        ok("1 / 4", "0.25");
    }

    #[test]
    fn number_format() {
        ok("2 + 3", "5");
        ok("0.5 + 0.5", "1");
    }

    #[test]
    fn unary() {
        ok("-5", "-5");
        ok("--5", "5");
        ok("-(1 + 2)", "-3");
        ok("1 - -2", "3");
        ok("-0", "0"); //  zero to print consistently
        ok("not true", "false");
        ok("not not true", "true");
        ok("not 0", "false"); //  0 as truthy
        ok("not \"\"", "false"); // empty string as truthy
    }

    #[test]
    fn modulo_sign() {
        ok("-7 % 3", "-1");
        ok("7 % -3", "1");
    }

    #[test]
    fn string_plus() {
        ok("\"a\" + \"b\"", "ab");
        err("\"a\" + 1", "Operands must be two numbers or two strings.");
        err("1 + true", "Operands must be two numbers or two strings.");
    }

    #[test]
    fn type_errors() {
        err("-\"a\"", "Operand must be a number.");
        err("-true", "Operand must be a number.");
        err("\"a\" - 1", "Operands must be numbers.");
        err("true * 2", "Operands must be numbers.");
        err("\"a\" / \"b\"", "Operands must be numbers.");
    }

    #[test]
    fn zero_division() {
        err("1 / 0", "Division by zero.");
        err("1 / (2 - 2)", "Division by zero.");
        err("5 % 0", "Modulo by zero.");
        ok("0 / 5", "0");
    }

    #[test]
    fn eval_order() {
        // expectation: the left error first because evaluation is left to right
        err("(1 / 0) + (\"a\" - 1)", "Division by zero.");
        err("(\"a\" - 1) + (1 / 0)", "Operands must be numbers.");
    }

    #[test]
    fn operands_first() {
        // expectation: the inner error first because the operand runs first
        err("-(\"a\" - 1)", "Operands must be numbers.");
    }

    #[test]
    fn unsupported_ops() {
        err("1 < 2", "Operator '<' is not supported yet.");
        err("1 == 1", "Operator '==' is not supported yet.");
    }

    #[test]
    fn error_line() {
        let mut scanner = Scanner::new("1 +\n  1 / 0");
        let tokens = scanner.scan_tokens().clone();
        let expr = Parser::new(tokens).parse_expression().unwrap();
        let error = eval(&expr).unwrap_err();
        assert_eq!(error.token.line, 2); // expectation: the `/` error to point to line 2.
    }
}
