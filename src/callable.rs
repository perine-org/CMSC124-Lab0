use crate::ast::Stmt;
use crate::environment::Environment;
use crate::interpreter::{Interpreter, RuntimeError};
use crate::token::Token;
use crate::value::Value;
use std::cell::RefCell;
use std::rc::Rc;

// anything that can be called: user functions now, native functions later.
// the call site checks arity() once, so no implementation repeats that check
pub trait Callable {
    // how many arguments a call must pass
    fn arity(&self) -> usize;

    // runs the callable with already-evaluated arguments and returns its result
    fn call(&self, interpreter: &mut Interpreter, arguments: Vec<Value>) -> Result<Value, RuntimeError>;

    // used when the function is printed or shown in debug output
    fn name(&self) -> String;
}

// a function declared in the program: its code plus the scope it was declared in
pub struct UserFunction {
    name: String,
    params: Vec<Token>,
    body: Vec<Stmt>,
    // the environment where the declaration ran (NOT the call site), so the body
    // sees the names around where it was written
    closure: Rc<RefCell<Environment>>,
}

impl UserFunction {
    pub fn new(
        name: String,
        params: Vec<Token>,
        body: Vec<Stmt>,
        closure: Rc<RefCell<Environment>>,
    ) -> Self {
        UserFunction { name, params, body, closure }
    }
}

impl Callable for UserFunction {
    fn arity(&self) -> usize {
        self.params.len()
    }

    fn call(&self, interpreter: &mut Interpreter, arguments: Vec<Value>) -> Result<Value, RuntimeError> {
        // a fresh scope for every call, parented to the closure, so calls never share locals
        let env = Rc::new(RefCell::new(Environment::with_enclosing(Rc::clone(&self.closure))));
        for (param, arg) in self.params.iter().zip(arguments) {
            // parameters are local to this call and can be reassigned inside it
            env.borrow_mut().define(&param.lexeme, arg, true);
        }

        interpreter.execute_in(&self.body, env)?;

        // no return statement yet (Week 2), so falling off the end produces nil
        Ok(Value::Nil)
    }

    fn name(&self) -> String {
        self.name.clone()
    }
}