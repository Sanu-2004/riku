use crate::env::{Env, Value};
use crate::expr::Expr;
use crate::token::Token;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub enum Stmt {
    Expr(Expr),
    Let(Token, Expr),
    Assign(Token, Expr),
}

impl Stmt {
    pub fn eval(&self, env: &mut Rc<RefCell<Env>>) {
        match self {
            Stmt::Expr(expr) => {
                expr.eval(&env.borrow());
            }
            Stmt::Let(token, expr) => {
                let value = expr.eval(&env.borrow());
                env.borrow_mut().define(token.lexeme.clone(), value);
            }
            Stmt::Assign(token, expr) => {
                let value = expr.eval(&env.borrow());
                env.borrow_mut().assign(token.lexeme.clone(), value);
            }
        }
    }
}
