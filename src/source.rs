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
                ';' => self.add_token(";", TokenType::EOL),
                ',' => self.add_token(",", TokenType::Comma),
                ' ' => self.eat_char(&[' ']),
                '(' => self.add_token("(", TokenType::LParen),
                ')' => self.add_token(")", TokenType::RParen),
                '{' => self.add_token("{", TokenType::LBrace),
                '}' => self.add_token("}", TokenType::RBrace),
                '&' => self.add_token("&", TokenType::Ampersand),
                '|' => self.add_token("|", TokenType::Pipe),
                '<' => {
                    if self.peek_next() == Some('=') {
                        self.advance();
                        self.add_token("<=", TokenType::LessEqual);
                    } else {
                        self.add_token("<", TokenType::Less);
                    }
                }
                '>' => {
                    if self.peek_next() == Some('=') {
                        self.advance();
                        self.add_token(">=", TokenType::GreaterEqual);
                    } else {
                        self.add_token(">", TokenType::Greater);
                    }
                }
                '=' => {
                    if self.peek_next() == Some('=') {
                        self.advance();
                        self.add_token("==", TokenType::EqualEqual);
                    } else {
                        self.add_token("=", TokenType::Equal);
                    }
                }
                '!' => {
                    if self.peek_next() == Some('=') {
                        self.advance();
                        self.add_token("!=", TokenType::BangEqual);
                    } else {
                        self.add_token("!", TokenType::Bang);
                    }
                }
                '0'..='9' => self.numbers(),
                '"' => self.string(),
                _ if c.is_alphabetic() => self.identifier(),
                _ => self.syntaxerror(),
            }
        }
        self.add_token("", TokenType::EOF);
    }

    fn string(&mut self) {
        self.advance();
        let start = self.position;
        while let Some(c) = self.peek() {
            if c == '"' {
                break;
            } else if c == '\n' {
                line_error(
                    ErrorType::SyntaxError,
                    self.line,
                    "Unterminated string".to_string(),
                );
                process::exit(1);
            }
            self.advance();
        }
        let lexeme = &self.input[start..self.position];
        let token = Token::new(lexeme, self.line, TokenType::String);
        self.tokens.push(token);
        self.advance();
        self.eat_char(&[' ']);
    }

    fn identifier(&mut self) {
        let start = self.position;
        while let Some(c) = self.peek() {
            if c.is_alphanumeric() || c == '_' {
                self.advance();
            } else {
                break;
            }
        }
        let lexeme = &self.input[start..self.position];
        let token_type = match lexeme {
            "true" => TokenType::True,
            "false" => TokenType::False,
            "let" => TokenType::Let,
            "print" => TokenType::Print,
            "if" => TokenType::If,
            "else" => TokenType::Else,
            "input" => TokenType::Input,
            "int" => TokenType::Int,
            _ => TokenType::Ident,
        };
        let token = Token::new(lexeme.trim(), self.line, token_type);
        self.tokens.push(token);
        self.eat_char(&[' ']);
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

    pub fn peek_next(&self) -> Option<char> {
        if self.position < self.input.len() {
            self.input[self.position..].chars().skip(1).next()
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
            if c.is_digit(10) || c == '.' {
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
