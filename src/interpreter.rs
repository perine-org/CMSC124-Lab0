use crate::ast::{Expr, Program, Stmt};
use crate::environment::{AssignError, Environment};
use crate::token::{Token, TokenType};
use crate::value::Value;
use std::cell::RefCell;
use std::rc::Rc;

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

// holds the evaluator's state. empty danay; week 2 will add enivormnetn
// so file mode and the REPL can share one instance and variables persist.
#[derive(Default)]
pub struct Interpreter {
    env: Rc<RefCell<Environment>>,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter { env: Rc::new(RefCell::new(Environment::new())) }
    }

    // walks the whole program 
    // echo = true: bare expressions print their value (--eval and REPL)
    // echo = false: only `show` prints (plain ./run <file>)
    pub fn run(&mut self, program: &Program, echo: bool) -> Result<(), RuntimeError> {
        for stmt in &program.statements {
            match stmt {
                Stmt::Expression(e) if echo => println!("{}", self.eval(e)?),
                other => self.execute(other)?,
            }
        }
        Ok(())
    }

    // runs one statement for its effect
    // Expression: evaluate and discard the value; Show: print the value to stdout
    pub fn execute(&mut self, stmt: &Stmt) -> Result<(), RuntimeError> {
        match stmt {
            Stmt::Expression(e) => {
                self.eval(e)?;
                Ok(())
            }
            Stmt::Show(e) => {
                let v = self.eval(e)?;
                println!("{}", v);
                Ok(())
            }
            Stmt::VarDecl { name, initializer, mutable } => {
                // evaluate first, so `set a = a;` fails as an undefined read
                let value = match initializer {
                    Some(e) => self.eval(e)?,
                    None => Value::Nil, // `set a;` holds nil
                };
                self.env.borrow_mut().define(&name.lexeme, value, *mutable);
                Ok(())
            }
        }
    }

    // evaluation of the expression from the inside out
    // If something fails, return the error instead of printing it here
    pub fn eval(&mut self, expr: &Expr) -> Result<Value, RuntimeError> {
        match expr {
            // the scanner already gave the value, so just convert it
            Expr::Literal(literal) => Ok(Value::from_literal(literal)),

            // the parser already handled the parentheses, so I just evaluate what is inside
            Expr::Grouping { expression } => self.eval(expression),

            Expr::Unary { operator, right } => {
                let right = self.eval(right)?; // value is needed before using the operator
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
                let left = self.eval(left)?;
                let right = self.eval(right)?;
                binary(operator, left, right)
            }

            // lookup: an undefined name is a runtime error
            Expr::Variable(name) => {
                let found = self.env.borrow().get(&name.lexeme);
                match found {
                    Some(value) => Ok(value),
                    None => Err(RuntimeError::new(
                        name,
                        &format!("Undefined variable '{}'.", name.lexeme),
                    )),
                }
            }

            // assignment evaluates the right side first, then updates the binding
            Expr::Assign { name, value } => {
                let value = self.eval(value)?;
                let result = self.env.borrow_mut().assign(&name.lexeme, value.clone());
                match result {
                    Ok(()) => Ok(value),
                    Err(AssignError::Undefined) => Err(RuntimeError::new(
                        name,
                        &format!("Undefined variable '{}'.", name.lexeme),
                    )),
                    Err(AssignError::Immutable) => Err(RuntimeError::new(
                        name,
                        &format!("Cannot assign to immutable binding '{}'.", name.lexeme),
                    )),
                }
            }
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