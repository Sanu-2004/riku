use std::{cell::RefCell, fmt, process, rc::Rc};

use crate::{
    env::{Env, Value},
    error::{ErrorType, error, line_error},
    stmt::{ControlFlow, Stmt},
    token::{Token, TokenType},
};

#[derive(Debug, Clone)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    And,
    Or,
    Not,
    Ne,
    Eq,
    Gt,
    Ge,
    Lt,
    Le,
}

#[derive(Debug, Clone)]
pub enum Expr {
    Number(f64),
    Bool(bool),
    String(String),
    Binary(Box<Expr>, Op, Box<Expr>),
    Logic(Box<Expr>, Op, Box<Expr>),
    Unary(Op, Box<Expr>),
    Group(Box<Expr>),
    Variable(Token),
    Input(Box<Stmt>),
    Int(Box<Expr>),
    Call { callee: Box<Expr>, args: Vec<Expr> },
}

impl Expr {
    pub fn new(token: Token) -> Self {
        match token.token_type {
            TokenType::Number => {
                let value = token.lexeme.parse::<f64>().unwrap_or(0.0);
                Expr::Number(value)
            }
            TokenType::String => Expr::String(token.lexeme),
            TokenType::True => Expr::Bool(true),
            TokenType::False => Expr::Bool(false),
            TokenType::Ident => Expr::Variable(token),
            _ => {
                line_error(
                    ErrorType::SyntaxError,
                    token.line,
                    format!("expected a expr but found `{}`", token.lexeme),
                );
                process::exit(1);
            }
        }
    }

    pub fn new_call(callee: Expr, args: Vec<Expr>) -> Self {
        Expr::Call {
            callee: Box::new(callee),
            args,
        }
    }

    pub fn new_int(expr: Expr) -> Self {
        Expr::Int(Box::new(expr))
    }

    pub fn new_input(stmt: Stmt) -> Self {
        Expr::Input(Box::new(stmt))
    }

    pub fn new_group(expr: Expr) -> Self {
        Expr::Group(Box::new(expr))
    }

    pub fn new_binary(left: Expr, op: &Token, right: Expr) -> Self {
        let op = Op::new(op);
        Expr::Binary(Box::new(left), op, Box::new(right))
    }

    pub fn new_logic(left: Expr, op: &Token, right: Expr) -> Self {
        let op = Op::new(op);
        Expr::Logic(Box::new(left), op, Box::new(right))
    }

    pub fn new_unary(op: &Token, right: Expr) -> Self {
        let op = match op.token_type {
            TokenType::Minus => Op::Sub,
            TokenType::Bang => Op::Not,
            _ => {
                line_error(
                    ErrorType::SyntaxError,
                    op.line,
                    format!("Only support unary minus operator, found `{}`", op.lexeme),
                );
                process::exit(1);
            }
        };
        Expr::Unary(op, Box::new(right))
    }

    pub fn condition_eval(&self, env: &mut Rc<RefCell<Env>>) -> bool {
        match self.eval(env) {
            Value::Bool(b) => b,
            Value::Number(n) => n > 0.0,
            _ => {
                error(
                    ErrorType::TypeError,
                    "Invalid condition, expected boolean or number".to_string(),
                );
                false
            }
        }
    }

    pub fn eval(&self, env: &mut Rc<RefCell<Env>>) -> Value {
        match self {
            Self::Number(n) => Value::Number(*n),
            Self::Bool(b) => Value::Bool(*b),
            Self::Binary(l, op, r) => {
                let left = l.eval(env);
                let right = r.eval(env);
                let num = op.eval_binary(left, right);
                Value::Number(num)
            }
            Self::Unary(op, r) => op.eval_unary(r.eval(env)),
            Self::Group(expr) => expr.eval(env),
            Self::Logic(l, op, r) => {
                let left = l.eval(env);
                let right = r.eval(env);
                op.eval_logic(left, right)
            }
            Self::Variable(t) => env.borrow().get(&t.lexeme).unwrap_or_else(|| {
                error(
                    ErrorType::RuntimeError,
                    format!("Undefined variable `{}`", t.lexeme),
                );
                process::exit(1);
            }),
            Self::String(s) => Value::String(s.clone()),
            Self::Input(stmt) => {
                let mut input = String::new();
                stmt.eval(env);
                std::io::stdin().read_line(&mut input).unwrap();
                let value = Value::String(input.trim().to_string());
                value
            }
            Self::Int(n) => match n.eval(env) {
                Value::Number(_) => self.eval(env),
                Value::String(s) => {
                    let num = s.parse::<f64>().unwrap_or_else(|_| {
                        error(
                            ErrorType::TypeError,
                            format!("Invalid string `{}` for int", s),
                        );
                        0.0
                    });
                    Value::Number(num)
                }
                Value::Bool(b) => {
                    let num = if b { 1.0 } else { 0.0 };
                    Value::Number(num)
                }
                _ => {
                    error(
                        ErrorType::TypeError,
                        "Invalid operand, expected number or string".to_string(),
                    );
                    Value::Number(0.0)
                }
            },
            Self::Call { callee, args } => {
                let func = callee.eval(env);
                let args = args.iter().map(|a| a.eval(env)).collect::<Vec<_>>();
                match func {
                    Value::Function {
                        params,
                        body,
                        closure,
                        ..
                    } => {
                        if args.len() != params.len() {
                            error(
                                ErrorType::RuntimeError,
                                format!(
                                    "Expected {} arguments but got {}",
                                    params.len(),
                                    args.len()
                                ),
                            );
                            process::exit(1);
                        }
                        let mut child_env = Env::child_env(closure);
                        for (param, arg) in params.iter().zip(args) {
                            child_env.borrow_mut().define(param.clone(), arg);
                        }
                        match body.eval(&mut child_env) {
                            ControlFlow::Return(v) => v,
                            _ => Value::Nil,
                        }
                    }
                    _ => {
                        error(
                            ErrorType::TypeError,
                            format!("`{}` is not a function", func),
                        );
                        Value::Nil
                    }
                }
            }
        }
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Number(n) => write!(f, "{}", n),
            Self::Binary(l, op, r) => write!(f, "{} {} {}", l, op, r),
            Self::Unary(op, r) => write!(f, "{}{}", op, r),
            Self::Group(expr) => write!(f, "({})", expr),
            Self::Bool(b) => write!(f, "{}", b),
            Self::Logic(l, op, r) => write!(f, "({} {} {})", l, op, r),
            Self::Variable(t) => write!(f, "{}", t.lexeme),
            Self::String(s) => write!(f, "{}", s),
            Self::Input(_) => write!(f, "Input box"),
            Self::Int(_) => write!(f, "Int box"),
            Self::Call { callee, args } => {
                let args_str = args
                    .iter()
                    .map(|arg| arg.to_string())
                    .collect::<Vec<_>>()
                    .join(", ");
                write!(f, "{}({})", callee, args_str)
            }
        }
    }
}

impl Op {
    fn new(op: &Token) -> Self {
        match op.token_type {
            TokenType::Plus => Op::Add,
            TokenType::Minus => Op::Sub,
            TokenType::Star => Op::Mul,
            TokenType::Slash => Op::Div,
            TokenType::Modulo => Op::Mod,
            TokenType::Ampersand => Op::And,
            TokenType::Pipe => Op::Or,
            TokenType::Bang => Op::Not,
            TokenType::BangEqual => Op::Ne,
            TokenType::EqualEqual => Op::Eq,
            TokenType::Greater => Op::Gt,
            TokenType::GreaterEqual => Op::Ge,
            TokenType::Less => Op::Lt,
            TokenType::LessEqual => Op::Le,
            _ => {
                line_error(
                    ErrorType::SyntaxError,
                    op.line,
                    format!("Unexpected operator `{}`", op.lexeme),
                );
                process::exit(1);
            }
        }
    }

    fn eval_unary(&self, right: Value) -> Value {
        match self {
            Op::Not => {
                if let Value::Bool(b) = right {
                    Value::Bool(!b)
                } else {
                    error(
                        ErrorType::TypeError,
                        "Invalid operand, expected boolean".to_string(),
                    );
                    Value::Bool(false)
                }
            }
            Op::Sub => {
                if let Value::Number(n) = right {
                    Value::Number(-n)
                } else {
                    error(
                        ErrorType::TypeError,
                        "Invalid operand, expected number".to_string(),
                    );
                    Value::Number(0.0)
                }
            }
            _ => {
                error(
                    ErrorType::TypeError,
                    format!("Invalid unary operator `{}`", self),
                );
                Value::Number(0.0)
            }
        }
    }

    fn eval_binary(&self, left: Value, right: Value) -> f64 {
        let (left, right) = match (left, right) {
            (Value::Number(l), Value::Number(r)) => (l, r),
            _ => {
                error(
                    ErrorType::TypeError,
                    "Invalid operands, expected numbers".to_string(),
                );
                (0.0, 0.0)
            }
        };
        match self {
            Op::Add => left + right,
            Op::Sub => left - right,
            Op::Mul => left * right,
            Op::Div => left / right,
            Op::Mod => left % right,
            _ => {
                error(
                    ErrorType::TypeError,
                    "Invalid operands, expected numbers".to_string(),
                );
                0.0
            }
        }
    }

    fn eval_logic(&self, l: Value, r: Value) -> Value {
        match (&l, &r) {
            (Value::Bool(l), Value::Bool(r)) => {
                let res = self.logic_bool(*l, *r);
                Value::Bool(res)
            }
            (Value::Number(l), Value::Number(r)) => {
                let res = self.logic_num(*l, *r);
                Value::Bool(res)
            }
            (Value::String(l), Value::String(r)) => {
                let res = self.logic_string(l.clone(), r.clone());
                Value::Bool(res)
            }
            _ => {
                error(
                    ErrorType::TypeError,
                    format!(
                        "Invalid Comparison Type: `{}` and `{}` both must be same type",
                        l, r
                    ),
                );
                Value::Number(0.0)
            }
        }
    }

    fn logic_string(&self, l: String, r: String) -> bool {
        match self {
            Op::And => !l.is_empty() && !r.is_empty(),
            Op::Or => !l.is_empty() || !r.is_empty(),
            Op::Eq => l == r,
            Op::Ne => l != r,
            Op::Gt => l > r,
            Op::Ge => l >= r,
            Op::Lt => l < r,
            Op::Le => l <= r,
            _ => {
                error(
                    ErrorType::TypeError,
                    format!("Invalid operator `{}` for string", self),
                );
                false
            }
        }
    }

    fn logic_bool(&self, l: bool, r: bool) -> bool {
        match self {
            Op::And => l && r,
            Op::Or => l || r,
            Op::Eq => l == r,
            Op::Ne => l != r,
            Op::Gt => l > r,
            Op::Ge => l >= r,
            Op::Lt => l < r,
            Op::Le => l <= r,
            _ => {
                error(
                    ErrorType::TypeError,
                    format!("Invalid operator `{}` for boolean", self),
                );
                false
            }
        }
    }

    fn logic_num(&self, l: f64, r: f64) -> bool {
        match self {
            Op::And => l > 0.0 && r > 0.0,
            Op::Or => l > 0.0 || r > 0.0,
            Op::Eq => l == r,
            Op::Ne => l != r,
            Op::Gt => l > r,
            Op::Ge => l >= r,
            Op::Lt => l < r,
            Op::Le => l <= r,
            _ => {
                error(
                    ErrorType::TypeError,
                    format!("Invalid operator `{}` for number", self),
                );
                false
            }
        }
    }
}

impl fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Add => write!(f, "+"),
            Self::Sub => write!(f, "-"),
            Self::Mul => write!(f, "*"),
            Self::Div => write!(f, "/"),
            Self::Mod => write!(f, "%"),
            Self::And => write!(f, "&"),
            Self::Or => write!(f, "|"),
            Self::Not => write!(f, "!"),
            Self::Eq => write!(f, "=="),
            Self::Gt => write!(f, ">"),
            Self::Ge => write!(f, ">="),
            Self::Lt => write!(f, "<"),
            Self::Le => write!(f, "<="),
            Self::Ne => write!(f, "!="),
        }
    }
}
