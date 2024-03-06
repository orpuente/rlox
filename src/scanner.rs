use crate::{token::{Span, Token}, token_type::TokenKind, LoxNumber};
use std::collections::HashMap;
use once_cell::sync::Lazy;

#[derive(Default)]
pub struct Scanner {
    source: String,
    pub tokens: Vec<Token>,
    start_of_lexeme: usize,
    current: usize,
    line: usize,
    errors: Vec<ScannerError>,
}

impl Scanner {
    pub fn new(source: &str) -> Self {
        Self {
            source: source.to_string(),
            line: 1,
            ..Default::default()
        }
    }

    pub fn scan_tokens(&mut self) -> Result<&[Token], &[ScannerError]> {
        while !self.eof() {
            self.start_of_lexeme = self.current;
            self.scan_token();
        }

        self.start_of_lexeme = self.current;
        self.add_token(TokenKind::Eof);

        if self.errors.is_empty() {
            Ok(&self.tokens)
        } else {
            Err(&self.errors)
        }
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        use TokenKind::*;
        match c {
            '(' => self.add_token(LeftParen),
            ')' => self.add_token(RightParen),
            '{' => self.add_token(LeftBrace),
            '}' => self.add_token(RightBrace),
            ',' => self.add_token(Comma),
            '.' => self.add_token(Dot),
            '-' => self.add_token(Minus),
            '+' => self.add_token(Plus),
            ';' => self.add_token(Semicolon),
            '*' => self.add_token(Star),
            '!' => if self.match_advance('=') { self.add_token(BangEqual) } else { self. add_token(Bang)},
            '=' => if self.match_advance('=') { self.add_token(EqualEqual) } else { self. add_token(Equal)},
            '<' => if self.match_advance('=') { self.add_token(LessEqual) } else { self. add_token(Less)},
            '>' => if self.match_advance('=') { self.add_token(GreaterEqual) } else { self. add_token(Greater)},
            '/' => {
                if self.match_advance('/') {
                    // A comment goes until the end of the line.
                    while self.peek() != '\n' && !self.eof() { self.advance(); }
                } else {
                    self.add_token(Slash);
                }
            }
            ' ' | '\r' | '\t' => (), // Ignore  whitespace.
            '\n' => self.line += 1,

            // Literals:
            '"' => self.literal_string(),
            '0'..='9' => self.literal_number(),

            // Identifiers:
            'a'..='z' | 'A'..='Z' => self.identifier(),

            _ => self.push_error("Unexpected character"),
        };
    }

    fn literal_string(&mut self) {
        while self.peek() != '"' && self.eof() {
            if self.peek() == '\n' { self.line += 1 }
            self.advance();
        }

        if self.eof() {
            self.push_error("Unterminated string.");
            return;
        }

        // Consume the closing '"'.
        self.advance();

        let value = self.source.as_str()[self.start_of_lexeme + 1 .. self.current - 1].to_string();
        self.add_token(TokenKind::String(value));
    }

    fn literal_number(&mut self) {
        while self.peek().is_digit(10) { self.advance(); }

        // Look for a fractional part.
        if self.peek() == '.' && self.peek_next().is_digit(10) {
            // Consume the '.'
            self.advance();

            // Consume the decimal part.
            while self.peek().is_digit(10) { self.advance(); }
        }

        let value = self.source.as_str()[self.start_of_lexeme .. self.current].to_string();

        // SAFETY: If we got here is because the lexeme is of the form
        // [0-9]*(.[0-9]*) which is a valid floating number representation.
        let value: LoxNumber = value.parse().unwrap();

        self.add_token(TokenKind::Number(value));
    }

    fn identifier(&mut self) {
        while is_alpha(self.peek()) { self.advance(); }

        let value = self.source.as_str()[self.start_of_lexeme .. self.current].to_string();
        if let Some(token) = KEYWORDS.get(value.as_str()) {
            self.add_token(token.clone());
        } else {
            self.add_token(TokenKind::Identifier(value));
        }
    }

    fn advance(&mut self) -> char {
        let c = self.current();
        self.current += 1;
        c as char
    }
    
    /// Returns `true` if the next character matches the `expected` character.
    /// Only advances the scanner if the match succeeds.
    fn match_advance(&mut self, expected: char) -> bool {
        if self.eof() || self.current() != expected {
            false
        } else {
            self.current += 1;
            true
        }
    }

    fn peek(&self) -> char {
        self.char_at(self.current)
    }

    fn peek_next(&self) -> char {
        self.char_at(self.current + 1)
    }
    
    fn add_token(&mut self, token: TokenKind) {
        let lexeme = &self.source.as_str()[self.start_of_lexeme .. self.current];
        let span = Span::new(self.start_of_lexeme, self.current - self.start_of_lexeme);
        self.tokens.push(Token::new(token, lexeme, span));
    }

    fn current(&self) -> char {
        self.source.as_bytes()[self.current] as char
    }

    fn char_at(&self, index: usize) -> char {
        if index >= self.source.len() {
            '\0'
        } else {
            self.source.as_bytes()[index] as char
        }
    }

    fn eof(&self) -> bool {
        self.current >= self.source.len()
    }

    fn push_error(&mut self, message: &str) {
        self.errors.push(ScannerError {
            line: self.line,
            message: message.to_string(),
        })
    }
}

pub struct ScannerError {
    pub line: usize,
    pub message: String,
}

fn is_alpha(c: char) -> bool {
    matches!(c, 'a'..='z' | 'A'..='Z' | '_')
}

const KEYWORDS: Lazy<HashMap<&str, TokenKind>> = Lazy::new(||
    HashMap::from([
        ("and",    TokenKind::And),
        ("class",  TokenKind::Class),
        ("else",   TokenKind::Else),
        ("false",  TokenKind::False),
        ("for",    TokenKind::For),
        ("fn",    TokenKind::Fn),
        ("if",     TokenKind::If),
        ("nil",    TokenKind::Nil),
        ("or",     TokenKind::Or),
        ("print",  TokenKind::Print),
        ("return", TokenKind::Return),
        ("super",  TokenKind::Super),
        ("self",   TokenKind::Self_),
        ("true",   TokenKind::True),
        ("let",    TokenKind::Let),
        ("while",  TokenKind::While),
    ])
);
