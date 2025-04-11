use crate::{
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
        self.expr_term()
    }

    fn expr_term(&mut self) -> Option<Expr> {
        let left = self.expr_factor()?;
        let op = self.peek()?;
        let op = op.clone();
        while op.token_type == TokenType::Plus || op.token_type == TokenType::Minus {
            self.current += 1;
            let right = self.expr_factor()?;
            let expr = Expr::new_binary(left, &op, right);
            return Some(expr);
        }
        Some(left)
    }

    fn expr_factor(&mut self) -> Option<Expr> {
        let left = self.expr_unary()?;
        let op = self.peek()?;
        let op = op.clone();
        while op.token_type == TokenType::Star || op.token_type == TokenType::Slash {
            self.current += 1;
            let right = self.expr_unary()?;
            let expr = Expr::new_binary(left, &op, right);
            return Some(expr);
        }
        Some(left)
    }

    fn expr_unary(&mut self) -> Option<Expr> {
        let op = self.peek()?;
        let op = op.clone();
        if op.token_type == TokenType::Minus {
            self.current += 1;
            let right = self.expr_unary()?;
            return Some(Expr::new_unary(&op, right));
        }
        self.expr_primary()
    }

    fn expr_primary(&mut self) -> Option<Expr> {
        let token = self.peek()?;
        let token = token.clone();
        match token.token_type {
            TokenType::Number => {
                self.current += 1;
                Some(Expr::new(token))
            }
            _ => None,
        }
    }
}
