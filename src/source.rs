use std::process;

use crate::{
    error::{ErrorType, line_error},
    token::{Token, TokenType},
};

#[derive(Debug)]
pub struct Source {
    input: String,
    position: usize,
    tokens: Vec<Token>,
    line: usize,
}

impl Source {
    pub fn new(input: String) -> Self {
        Source {
            input,
            position: 0,
            tokens: Vec::new(),
            line: 1,
        }
    }

    pub fn get_tokens(&self) -> &Vec<Token> {
        &self.tokens
    }

    pub fn tokenize(&mut self) {
        while let Some(c) = self.peek() {
            // println!("{}", c);
            match c {
                '+' => self.add_token("+", TokenType::Plus),
                '-' => self.add_token("-", TokenType::Minus),
                '*' => self.add_token("*", TokenType::Star),
                '/' => self.add_token("/", TokenType::Slash),
                '\n' => self.eat_char(&['\n']),
                ' ' => self.eat_char(&[' ']),
                '(' => self.add_token("(", TokenType::LParen),
                ')' => self.add_token(")", TokenType::RParen),
                '0'..='9' => self.numbers(),
                _ => self.syntaxerror(),
            }
        }
        self.add_token("", TokenType::EOF);
    }

    fn syntaxerror(&self) {
        let error = ErrorType::SyntaxError;
        let mut syntax = String::new();
        let mut pos = self.position;
        while let Some(c) = self.input[pos..].chars().next() {
            if (c == ' ' || c == '\n') && pos < self.input.len() {
                break;
            }
            syntax.push(c);
            pos += c.len_utf8();
        }
        line_error(error, self.line, format!("Unexpected Syntax `{}`", syntax));
        process::exit(1);
    }

    pub fn peek(&self) -> Option<char> {
        if self.position < self.input.len() {
            Some(self.input[self.position..].chars().next().unwrap())
        } else {
            None
        }
    }

    pub fn eat_char(&mut self, chars: &[char]) {
        while let Some(c) = self.peek() {
            if chars.contains(&c) {
                self.position += c.len_utf8();
                if c == '\n' {
                    self.add_token("\n", TokenType::EOL);
                    self.line += 1;
                }
            } else {
                break;
            }
        }
    }

    pub fn advance(&mut self) -> Option<char> {
        if let Some(c) = self.peek() {
            self.position += c.len_utf8();
            Some(c)
        } else {
            None
        }
    }

    pub fn add_token(&mut self, s: &str, token_type: TokenType) {
        let lexeme = s.trim();
        let token = Token::new(lexeme, self.line, token_type);
        self.tokens.push(token);
        self.advance();
        self.eat_char(&[' ']);
    }

    pub fn numbers(&mut self) {
        let start = self.position;
        while let Some(c) = self.peek() {
            if c.is_digit(10) {
                self.advance();
            } else {
                break;
            }
        }
        let lexeme = &self.input[start..self.position];
        let token = Token::new(lexeme.trim(), self.line, TokenType::Number);
        self.tokens.push(token);
        self.eat_char(&[' ']);
    }
}
