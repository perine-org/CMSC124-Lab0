use crate::value::Value;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

// why an assignment failed; the interpreter turns these into runtime errors (exit 70)
pub enum AssignError {
    Undefined,
    Immutable,
}

// a stored value plus whether it can be reassigned (`set` = true, `deal` = false)
struct Binding {
    value: Value,
    mutable: bool,
}

// one scope's bindings; `enclosing` links to the outer scope (used by blocks and Lab 5 closures)
pub struct Environment {
    values: HashMap<String, Binding>,
    enclosing: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn new() -> Self {
        Environment { values: HashMap::new(), enclosing: None }
    }

    // makes a child scope whose lookups fall back to `enclosing` (used by blocks, and by lab 5 closures)
    pub fn with_enclosing(enclosing: Rc<RefCell<Environment>>) -> Self {
        Environment { values: HashMap::new(), enclosing: Some(enclosing) }
    }

    // creates a binding in this scope; redeclaring the same name replaces it
    pub fn define(&mut self, name: &str, value: Value, mutable: bool) {
        self.values.insert(name.to_string(), Binding { value, mutable });
    }

    // looks in this scope, then outward; None = undefined
    pub fn get(&self, name: &str) -> Option<Value> {
        if let Some(binding) = self.values.get(name) {
            return Some(binding.value.clone());
        }
        match &self.enclosing {
            Some(outer) => outer.borrow().get(name),
            None => None,
        }
    }

    // updates the nearest existing binding; fails if undefined or declared with `deal`
    pub fn assign(&mut self, name: &str, value: Value) -> Result<(), AssignError> {
        if let Some(binding) = self.values.get_mut(name) {
            if !binding.mutable {
                return Err(AssignError::Immutable);
            }
            binding.value = value;
            return Ok(());
        }
        match &self.enclosing {
            Some(outer) => outer.borrow_mut().assign(name, value),
            None => Err(AssignError::Undefined),
        }
    }
}

impl Default for Environment {
    fn default() -> Self {
        Self::new()
    }
}