use std::{fmt, process};

use crate::{
    error::{ErrorType, error, line_error},
    token::{Token, TokenType},
};

#[derive(Debug, Clone)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
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
    Binary(Box<Expr>, Op, Box<Expr>),
    Logic(Box<Expr>, Op, Box<Expr>),
    Unary(Op, Box<Expr>),
    Group(Box<Expr>),
}

impl Expr {
    pub fn new(token: Token) -> Self {
        match token.token_type {
            TokenType::Number => {
                let value = token.lexeme.parse::<f64>().unwrap_or(0.0);
                Expr::Number(value)
            }
            TokenType::True => Expr::Bool(true),
            TokenType::False => Expr::Bool(false),
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

    pub fn eval(&self) -> Self {
        match self {
            Self::Number(n) => Self::Number(*n),
            Self::Bool(b) => Self::Bool(*b),
            Self::Binary(l, op, r) => {
                let left = l.eval();
                let right = r.eval();
                let num = op.eval_binary(left, right);
                Self::Number(num)
            }
            Self::Unary(op, r) => op.eval_unary(r.eval()),
            Self::Group(expr) => expr.eval(),
            Self::Logic(l, op, r) => {
                let left = l.eval();
                let right = r.eval();
                op.eval_logic(left, right)
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

    fn eval_unary(&self, right: Expr) -> Expr {
        match self {
            Op::Not => {
                if let Expr::Bool(b) = right {
                    Expr::Bool(!b)
                } else {
                    error(
                        ErrorType::TypeError,
                        "Invalid operand, expected boolean".to_string(),
                    );
                    Expr::Bool(false)
                }
            }
            Op::Sub => {
                if let Expr::Number(n) = right {
                    Expr::Number(-n)
                } else {
                    error(
                        ErrorType::TypeError,
                        "Invalid operand, expected number".to_string(),
                    );
                    Expr::Number(0.0)
                }
            }
            _ => {
                error(
                    ErrorType::TypeError,
                    format!("Invalid unary operator `{}`", self),
                );
                Expr::Number(0.0)
            }
        }
    }

    fn eval_binary(&self, left: Expr, right: Expr) -> f64 {
        let (left, right) = match (left, right) {
            (Expr::Number(l), Expr::Number(r)) => (l, r),
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
            _ => {
                error(
                    ErrorType::TypeError,
                    "Invalid operands, expected numbers".to_string(),
                );
                0.0
            }
        }
    }

    fn eval_logic(&self, l: Expr, r: Expr) -> Expr {
        match (&l, &r) {
            (Expr::Bool(l), Expr::Bool(r)) => {
                let res = self.logic_bool(*l, *r);
                Expr::Bool(res)
            }
            (Expr::Number(l), Expr::Number(r)) => {
                let res = self.logic_num(*l, *r);
                Expr::Bool(res)
            }
            _ => {
                error(
                    ErrorType::TypeError,
                    format!(
                        "Invaild Operator: `{}` and `{}` must be a number or boolean",
                        l, r
                    ),
                );
                Expr::Number(0.0)
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
