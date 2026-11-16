use crate::callable::Callable;
use crate::environment::Environment;
use crate::interpreter::{Interpreter, RuntimeError};
use crate::token::{Literal, Token, TokenType};
use crate::value::Value;
use std::rc::Rc;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct NativeFunction {
    name: &'static str,
    arity: usize,
    func: fn(&[Value]) -> Result<Value, String>,
}

impl Callable for NativeFunction {
    fn arity(&self) -> usize { self.arity }

    fn call(&self, _interp: &mut Interpreter, args: Vec<Value>) -> Result<Value, RuntimeError> {
        // natives have no call-site token, so build a stand-in to carry the message
        (self.func)(&args).map_err(|msg| {
            let tok = Token::new(TokenType::Identifier, self.name.to_string(), Literal::None, 0);
            RuntimeError::new(&tok, &msg)
        })
    }

    fn name(&self) -> String { self.name.to_string() }
}

fn clock(_: &[Value]) -> Result<Value, String> {
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|_| "System clock is before 1970.".to_string())?
        .as_secs_f64();
    Ok(Value::Number(secs))
}

fn type_of(args: &[Value]) -> Result<Value, String> {
    let name = match &args[0] {
        Value::Number(_) => "number",
        Value::Str(_) => "string",
        Value::Bool(_) => "bool",
        Value::Nil => "nil",
        Value::Function(_) => "function",
    };
    Ok(Value::Str(name.to_string()))
}

// register every native in the global scope (immutable, like `deal`)
pub fn define_all(env: &mut Environment) {
    let natives = [
        NativeFunction { name: "clock", arity: 0, func: clock },
        NativeFunction { name: "type", arity: 1, func: type_of },
    ];
    for n in natives {
        env.define(n.name, Value::Function(Rc::new(n)), false);
    }
}