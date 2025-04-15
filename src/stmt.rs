use crate::env::Env;
use crate::env::Value;
use crate::expr::Expr;
use crate::token::Token;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub enum ControlFlow {
    Value(Value),
    Break,
    Continue,
    Return(Value),
    None,
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Expr(Expr),
    Let(Token, Expr),
    Assign(Token, Expr),
    Group(Vec<Stmt>),
    Print(Vec<Expr>),
    If(Expr, Box<Stmt>, Option<Box<Stmt>>),
    While(Expr, Box<Stmt>),
    Function(Token, Vec<Token>, Box<Stmt>),
    Break,
    Continue,
    Return(Option<Expr>),
}

impl Stmt {
    pub fn eval(&self, env: &mut Rc<RefCell<Env>>) -> ControlFlow {
        match self {
            Stmt::Expr(expr) => ControlFlow::Value(expr.eval(env)),
            Stmt::Let(token, expr) => {
                let value = expr.eval(env);
                env.borrow_mut().define(token.lexeme.clone(), value.clone());
                ControlFlow::Value(value)
            }
            Stmt::Assign(token, expr) => {
                let value = expr.eval(env);
                env.borrow_mut().assign(token.lexeme.clone(), value);
                ControlFlow::None
            }
            Stmt::Group(stmts) => {
                let mut child_env = Env::child_env(env.clone());
                for stmt in stmts {
                    let res = stmt.eval(&mut child_env);
                    match res {
                        ControlFlow::Break | ControlFlow::Continue | ControlFlow::Return(_) => {
                            return res;
                        }
                        _ => {}
                    }
                }
                ControlFlow::None
            }
            Stmt::Print(exprs) => {
                for expr in exprs {
                    print!("{}", expr.eval(env));
                }
                println!();
                ControlFlow::None
            }
            Stmt::If(con, then, else_stmt) => {
                if con.condition_eval(env) {
                    return then.eval(env);
                } else if let Some(else_stmt) = else_stmt {
                    return else_stmt.eval(env);
                }
                ControlFlow::None
            }
            Stmt::Break => ControlFlow::Break,
            Stmt::Continue => ControlFlow::Continue,
            Stmt::While(expr, then) => {
                while expr.condition_eval(env) {
                    let res = then.eval(env);
                    match res {
                        ControlFlow::Break => break,
                        ControlFlow::Continue => continue,
                        ControlFlow::Return(_) => return res,
                        _ => {}
                    }
                }
                ControlFlow::None
            }
            Stmt::Return(expr) => {
                if let Some(expr) = expr {
                    return ControlFlow::Return(expr.eval(env));
                }
                ControlFlow::Return(Value::Nil)
            }
            Stmt::Function(name, args, body) => {
                let function = Value::Function {
                    name: name.lexeme.clone(),
                    params: args.iter().map(|arg| arg.lexeme.clone()).collect(),
                    body: body.clone(),
                    closure: env.clone(),
                };
                env.borrow_mut().define(name.lexeme.clone(), function);
                ControlFlow::None
            }
        }
    }
}
