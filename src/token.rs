#[derive(Debug, Clone, PartialEq, Copy, Eq)]
pub enum TokenType {
    Number,
    Plus,
    Minus,
    Star,
    Slash,
    LParen,
    RParen,
    // LBrace,
    // RBrace,
    // Semi,
    // Comma,
    // Dot,
    // String,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Bang,
    BangEqual,
    Ampersand,
    Pipe,
    Let,
    Ident,
    True,
    False,
    EOL,
    EOF,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: usize,
}

impl Token {
    pub fn new(s: &str, line: usize, token_type: TokenType) -> Self {
        Token {
            token_type,
            lexeme: s.to_string(),
            line,
        }
    }
}
