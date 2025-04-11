use std::{fmt, process};

use crate::{
    error::{ErrorType, error, line_error},
    token::{Token, TokenType},
};

#[derive(Debug)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug)]
pub enum Expr {
    Number(f64),
    Binary(Box<Expr>, Op, Box<Expr>),
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

    pub fn new_unary(op: &Token, right: Expr) -> Self {
        dbg!(op);
        let op = match op.token_type {
            TokenType::Minus => Op::Sub,
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
            Self::Binary(l, op, r) => {
                let left = l.eval();
                let right = r.eval();
                let num = op.eval_binary(left, right);
                Self::Number(num)
            }
            Self::Unary(_, r) => {
                if let Self::Number(r) = r.eval() {
                    Self::Number(r * -1.0)
                } else {
                    error(
                        ErrorType::TypeError,
                        "Invalid operand for unary operator".to_string(),
                    );
                    Self::Number(0.0)
                }
            }
            Self::Group(expr) => expr.eval(),
        }
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Number(n) => write!(f, "{}", n),
            Self::Binary(l, op, r) => write!(f, "({} {} {})", l, op, r),
            Self::Unary(op, r) => write!(f, "({} {})", op, r),
            Self::Group(expr) => write!(f, "{}", expr),
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
        }
    }
}
