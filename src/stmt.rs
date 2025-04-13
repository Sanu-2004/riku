use crate::env::Env;
use crate::expr::Expr;
use crate::token::Token;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub enum Stmt {
    Expr(Expr),
    Let(Token, Expr),
    Assign(Token, Expr),
    Group(Vec<Stmt>),
    Print(Vec<Expr>),
    If(Expr, Box<Stmt>, Option<Box<Stmt>>),
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
            Stmt::Group(stmts) => {
                let mut child_env = Env::child_env(env.clone());
                for stmt in stmts {
                    stmt.eval(&mut child_env);
                }
            }
            Stmt::Print(exprs) => {
                for expr in exprs {
                    print!("{}", expr.eval(&env.borrow()));
                }
                println!();
            }
            Stmt::If(con, then, else_stmt) => {
                if con.condition_eval(&env.borrow()) {
                    then.eval(env);
                } else if let Some(else_stmt) = else_stmt {
                    else_stmt.eval(env);
                }
            }
        }
    }
}
