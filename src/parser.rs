use std::process;

use crate::{
    error::{ErrorType, line_error},
    expr::Expr,
    token::{Token, TokenType},
};

#[derive(Debug)]
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
    stmts: Vec<Expr>,
}

impl Parser {
    pub fn new(tokens: &[Token]) -> Self {
        Parser {
            tokens: tokens.to_vec(),
            current: 0,
            stmts: Vec::new(),
        }
    }

    fn peek(&self) -> Option<&Token> {
        if self.current < self.tokens.len() {
            Some(&self.tokens[self.current])
        } else {
            None
        }
    }

    fn peek_back(&self, offset: usize) -> Option<&Token> {
        if self.current >= offset {
            Some(&self.tokens[self.current - offset])
        } else {
            None
        }
    }

    fn next(&mut self) {
        if self.current < self.tokens.len() {
            self.current += 1;
        }
    }

    pub fn get_stmts(&self) -> &Vec<Expr> {
        &self.stmts
    }

    pub fn parse(&mut self) {
        self.parse_eof();
    }

    fn parse_eof(&mut self) {
        let (exprs, _) = self.parse_till(TokenType::EOF);
        self.stmts = exprs;
    }

    fn parse_till(&mut self, till: TokenType) -> (Vec<Expr>, bool) {
        let mut exprs = Vec::new();
        let mut found = false;
        while let Some(t) = self.peek() {
            if t.token_type == till {
                found = true;
                break;
            }
            match t.token_type {
                TokenType::EOL => {
                    self.next();
                    continue;
                }
                _ => {
                    let Some(expr) = self.parse_expr() else {
                        return (exprs, found);
                    };
                    exprs.push(expr);
                }
            }
            self.next()
        }
        (exprs, found)
    }

    fn parse_expr(&mut self) -> Option<Expr> {
        self.expr_logic()
    }

    fn expr_logic(&mut self) -> Option<Expr> {
        let mut left = self.expr_equality()?;
        while self.peek()?.token_type == TokenType::Ampersand
            || self.peek()?.token_type == TokenType::Pipe
        {
            let op = self.peek()?;
            let op = op.clone();
            self.next();
            let right = self.expr_equality()?;
            let expr = Expr::new_logic(left, &op, right);
            left = expr;
        }
        Some(left)
    }

    fn expr_equality(&mut self) -> Option<Expr> {
        let mut left = self.expr_relation()?;
        while self.peek()?.token_type == TokenType::EqualEqual
            || self.peek()?.token_type == TokenType::BangEqual
        {
            let op = self.peek()?;
            let op = op.clone();
            self.next();
            let right = self.expr_relation()?;
            let expr = Expr::new_logic(left, &op, right);
            left = expr;
        }
        Some(left)
    }

    fn expr_relation(&mut self) -> Option<Expr> {
        let mut left = self.expr_term()?;
        while self.peek()?.token_type == TokenType::Less
            || self.peek()?.token_type == TokenType::LessEqual
            || self.peek()?.token_type == TokenType::Greater
            || self.peek()?.token_type == TokenType::GreaterEqual
        {
            let op = self.peek()?;
            let op = op.clone();
            self.next();
            let right = self.expr_term()?;
            let expr = Expr::new_logic(left, &op, right);
            left = expr;
        }
        Some(left)
    }

    fn expr_term(&mut self) -> Option<Expr> {
        let mut left = self.expr_factor()?;
        while self.peek()?.token_type == TokenType::Plus
            || self.peek()?.token_type == TokenType::Minus
        {
            let op = self.peek()?;
            let op = op.clone();
            self.next();
            let right = self.expr_factor()?;
            let expr = Expr::new_binary(left, &op, right);
            left = expr;
        }
        Some(left)
    }

    fn expr_factor(&mut self) -> Option<Expr> {
        let mut left = self.expr_unary()?;
        while self.peek()?.token_type == TokenType::Star
            || self.peek()?.token_type == TokenType::Slash
        {
            let op = self.peek()?;
            let op = op.clone();
            self.next();
            let right = self.expr_unary()?;
            let expr = Expr::new_binary(left, &op, right);
            left = expr;
        }
        Some(left)
    }

    fn expr_unary(&mut self) -> Option<Expr> {
        if self.peek()?.token_type == TokenType::Minus || self.peek()?.token_type == TokenType::Bang
        {
            let op = self.peek()?;
            let op = op.clone();
            self.next();
            let right = self.expr_unary()?;
            return Some(Expr::new_unary(&op, right));
        }
        self.expr_group()
    }

    fn expr_group(&mut self) -> Option<Expr> {
        if self.peek()?.token_type == TokenType::LParen {
            self.next();
            let expr = self.parse_expr()?;
            if self.peek()?.token_type == TokenType::RParen {
                self.next();
                return Some(Expr::new_group(expr));
            } else {
                line_error(
                    ErrorType::SyntaxError,
                    self.peek_back(1)?.line,
                    "Missing closing parenthesis".to_string(),
                );
                process::exit(1);
            }
        }
        self.expr_primary()
    }

    fn expr_primary(&mut self) -> Option<Expr> {
        match self.peek()?.token_type {
            TokenType::Number => {
                self.next();
                Some(Expr::new(self.peek_back(1)?.clone()))
            }
            TokenType::True | TokenType::False => {
                self.next();
                Some(Expr::new(self.peek_back(1)?.clone()))
            }
            _ => {
                line_error(
                    ErrorType::SyntaxError,
                    self.peek_back(1)?.line,
                    format!("Unexpected token `{}`", self.peek()?.lexeme),
                );
                process::exit(1);
            }
        }
    }
}
