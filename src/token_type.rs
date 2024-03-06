use crate::LoxNumber;

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
    Bang, BangEqual,
    Equal, EqualEqual,
    Greater, GreaterEqual,
    Less, LessEqual,

    // Literals
    Identifier(String),
    String(String),
    Number(LoxNumber),

    // Keywords,
    And, Class, Else, False, Fn, For, If, Let, Nil,
    Or, Print, Return, Self_, Super, True, While,

    // EOF
    Eof,
}