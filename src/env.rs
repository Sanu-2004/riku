use std::{cell::RefCell, collections::HashMap, fmt, process, rc::Rc};

use crate::{
    error::{ErrorType, error},
    std_fn::std_fn,
    stmt::Stmt,
};

#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    Bool(bool),
    String(String),
    Function {
        name: String,
        params: Vec<String>,
        body: Box<Stmt>,
        closure: Rc<RefCell<Env>>,
    },
    FuncBuiltIn {
        name: String,
        body: fn(Vec<Value>) -> Value,
    },
    Nil,
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{}", n),
            Value::Bool(b) => write!(f, "{}", b),
            Value::String(s) => write!(f, "{}", s),
            Value::Nil => write!(f, "nil"),
            Value::Function { name, .. } => write!(f, "<function {}>", name),
            Value::FuncBuiltIn { name, .. } => write!(f, "<builtin function {}>", name),
        }
    }
}

#[derive(Debug)]
pub struct Env {
    pub map: HashMap<String, Value>,
    pub parent: Option<Rc<RefCell<Env>>>,
}

impl Env {
    pub fn new() -> Rc<RefCell<Self>> {
        let mut env = Env {
            map: HashMap::new(),
            parent: None,
        };
        std_fn(&mut env);
        Rc::new(RefCell::new(env))
    }

    pub fn child_env(parent: Rc<RefCell<Self>>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Env {
            map: HashMap::new(),
            parent: Some(parent.clone()),
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
