use crate::expr::Expr;
use crate::token::Token;

#[derive(Debug, Clone)]
pub enum Stmt {
    Expr(Expr),
    Let(Token, Expr),
}
