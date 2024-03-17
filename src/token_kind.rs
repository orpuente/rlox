use crate::{Identifier, LoxNumber};

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    // Single-character tokens
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

    // One or two character tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals
    Identifier(Identifier),
    String(String),
    Number(LoxNumber),

    // Keywords,
    And,
    Class,
    Else,
    False,
    Fn,
    For,
    If,
    Let,
    Nil,
    Or,
    Print,
    Return,
    Self_,
    Super,
    True,
    While,

    // EOF
    Eof,
}

impl TokenKind {
    pub fn same_kind(&self, rhs: &Self) -> bool {
        match (self, rhs) {
            (TokenKind::Identifier(_), TokenKind::Identifier(_)) => true,
            _ => self == rhs,
        }
    }
}