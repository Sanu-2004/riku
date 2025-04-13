use std::{cell::RefCell, collections::HashMap, process, rc::Rc};

use crate::{
    error::{ErrorType, error},
    expr::Expr,
};

#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    Bool(bool),
}

#[derive(Debug)]
pub struct Env {
    pub map: HashMap<String, Value>,
    pub parent: Option<Rc<RefCell<Env>>>,
}

impl Env {
    pub fn new() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Env {
            map: HashMap::new(),
            parent: None,
        }))
    }

    pub fn child_env(parent: Rc<RefCell<Self>>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Env {
            map: HashMap::new(),
            parent: Some(parent),
        }))
    }

    pub fn define(&mut self, name: String, value: Value) {
        self.map.insert(name, value);
    }

    pub fn assign(&mut self, name: String, value: Value) {
        if let Some(v) = self.map.get_mut(&name) {
            *v = value;
        } else if let Some(parent) = &self.parent {
            parent.borrow_mut().assign(name, value);
        } else {
            error(
                ErrorType::RuntimeError,
                format!("Undefined variable `{}`", name),
            );
            process::exit(1);
        }
    }

    pub fn get(&self, name: &str) -> Option<Value> {
        if let Some(value) = self.map.get(name) {
            Some(value.clone())
        } else if let Some(parent) = &self.parent {
            parent.borrow().get(name)
        } else {
            None
        }
    }
}
