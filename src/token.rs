use crate::token_kind::TokenKind;

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub lexeme: String,
    pub span: Span,
}

impl Token {
    pub fn new(kind: TokenKind, lexeme: &str, span: Span) -> Self {
        Self {
            kind,
            lexeme: lexeme.to_string(),
            span,
        }
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} \"{}\"", self.kind, self.lexeme,)
    }
}

#[derive(Debug, Default, Clone)]
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
