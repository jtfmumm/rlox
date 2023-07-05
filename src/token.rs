use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub ttype: TokenType,
    pub lexeme: String,
    pub literal: String,
    pub line: u32,
}

impl Token {
    pub fn new(ttype: TokenType, lexeme: String, literal: String, line: u32) -> Self {
        Token {
            ttype,
            lexeme,
            literal,
            line,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.lexeme)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    // single-character tokens.,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // one or two character tokens.,
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // literals.,
    Identifier(String),
    StringLit(String),
    Number(f64),

    // keywords.,
    And,
    Class,
    Elif,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof,
    Sof,
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = format!("{:?}", self);
        write!(f, "{}", s)
    }
}
