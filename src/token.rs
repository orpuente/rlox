use crate::token_type::TokenType;

#[derive(Debug)]
pub struct Token {
    pub type_: TokenType,
    pub lexeme: String,
    pub span: Span,
}

impl Token {
    pub fn new(type_: TokenType, lexeme: &str, span: Span) -> Self {
        Self { type_, lexeme: lexeme.to_string(), span }
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?} \"{}\"",
            self.type_,
            self.lexeme,
        )
    }
}

#[derive(Debug, Default)]
pub struct Span {
    offset: usize,
    length: usize,
}

impl Span {
    pub fn new(offset: usize, length: usize) -> Self {
        Self { offset, length }
    }
}

impl std::fmt::Display for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[offset: {}; length: {}]", self.offset, self.length)
    }
}