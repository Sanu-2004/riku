use std::process;

use crate::{
    error::{ErrorType, line_error},
    expr::Expr,
    stmt::Stmt,
    token::{Token, TokenType},
};

#[derive(Debug)]
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
    stmts: Vec<Stmt>,
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

    fn peek_next(&self) -> Option<&Token> {
        if self.current + 1 < self.tokens.len() {
            Some(&self.tokens[self.current + 1])
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

    fn advance(&mut self) -> Option<&Token> {
        if self.current < self.tokens.len() {
            self.current += 1;
            self.peek_back(1)
        } else {
            None
        }
    }

    fn check(&mut self, s: &str) -> bool {
        if self.current < self.tokens.len() {
            if self.peek().unwrap().lexeme == s {
                return true;
            }
        }
        false
    }

    fn check1(&mut self, s: &str) -> Result<(), ErrorType> {
        if self.check(s) {
            Ok(())
        } else {
            Err(ErrorType::SyntaxError)
        }
    }

    fn next(&mut self) {
        if self.current < self.tokens.len() {
            self.current += 1;
        }
    }

    pub fn get_stmts(&self) -> &Vec<Stmt> {
        &self.stmts
    }

    pub fn parse(&mut self) {
        self.parse_eof();
    }

    fn parse_eof(&mut self) {
        let (stmts, _) = self.parse_till(TokenType::EOF);
        self.stmts = stmts;
    }

    fn parse_till(&mut self, till: TokenType) -> (Vec<Stmt>, bool) {
        let mut stmts = Vec::new();
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
                TokenType::Let => {
                    let stmt = self.parse_let();
                    stmts.push(stmt);
                }
                TokenType::Ident => {
                    let stmt = self.parse_ident();
                    stmts.push(stmt);
                }
                _ => {
                    let Some(expr) = self.parse_expr() else {
                        return (stmts, found);
                    };
                    stmts.push(Stmt::Expr(expr));
                }
            }
            self.next()
        }
        (stmts, found)
    }

    fn parse_ident(&mut self) -> Stmt {
        if self.peek_next().is_some() {
            if self.peek_next().unwrap().token_type == TokenType::Equal {
                let token = self.peek().unwrap().clone();
                return self.parse_assign(token);
            }
        }
        Stmt::Expr(self.parse_expr().unwrap())
    }

    fn parse_assign(&mut self, name: Token) -> Stmt {
        self.next(); // consume the identifier
        self.next(); // consume the equal sign
        let expr = self.parse_expr();
        if expr.is_none() {
            line_error(
                ErrorType::SyntaxError,
                name.line,
                format!(
                    "Expected expression, found `{}`",
                    self.peek().unwrap().lexeme
                ),
            );
            process::exit(1);
        }
        let expr = expr.unwrap();
        Stmt::Assign(name, expr)
    }

    fn parse_let(&mut self) -> Stmt {
        self.next();
        let name = self.advance().unwrap();
        let name = name.clone();
        if name.token_type != TokenType::Ident {
            line_error(
                ErrorType::SyntaxError,
                name.line,
                format!("Expected identifier, found `{}`", name.lexeme),
            );
            process::exit(1);
        }
        if self.check1("=").is_err() {
            line_error(
                ErrorType::SyntaxError,
                name.line,
                format!("Expected `=`, found `{}`", self.peek().unwrap().lexeme),
            );
            process::exit(1);
        }
        self.next();
        let expr = self.parse_expr();
        if expr.is_none() {
            line_error(
                ErrorType::SyntaxError,
                name.line,
                format!(
                    "Expected expression, found `{}`",
                    self.peek().unwrap().lexeme
                ),
            );
            process::exit(1);
        }
        let expr = expr.unwrap();
        Stmt::Let(name, expr)
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
            TokenType::Ident => {
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
