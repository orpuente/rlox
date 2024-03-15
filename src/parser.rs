use crate::{
    expr::{Expr, Literal},
    token::Token,
    token_type::TokenKind,
};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Expr, ParserError> {
        self.expression()
    }

    fn expression(&mut self) -> Result<Expr, ParserError> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr, ParserError> {
        let mut expr = self.comparison()?;

        use TokenKind::*;
        while self.match_(&[BangEqual, EqualEqual]) {
            let operator = (&self.previous().kind).into();
            let right = self.comparison()?;
            expr = Expr::Binary(operator, expr.into(), right.into());
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, ParserError> {
        let mut expr = self.term()?;

        use TokenKind::*;
        while self.match_(&[Greater, GreaterEqual, Less, LessEqual]) {
            let operator = (&self.previous().kind).into();
            let right = self.term()?;
            expr = Expr::Binary(operator, expr.into(), right.into());
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, ParserError> {
        let mut expr = self.factor()?;

        use TokenKind::*;
        while self.match_(&[Minus, Plus]) {
            let operator = (&self.previous().kind).into();
            let right = self.factor()?;
            expr = Expr::Binary(operator, expr.into(), right.into());
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, ParserError> {
        let mut expr = self.unary()?;

        use TokenKind::*;
        while self.match_(&[Slash, Star]) {
            let operator = (&self.previous().kind).into();
            let right = self.unary()?;
            expr = Expr::Binary(operator, expr.into(), right.into());
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, ParserError> {
        use TokenKind::*;
        if self.match_(&[Bang, Minus]) {
            let operator = (&self.previous().kind).into();
            let right = self.unary()?;
            return Ok(Expr::Unary(operator, right.into()));
        }

        self.primary()
    }

    fn primary(&mut self) -> Result<Expr, ParserError> {
        use TokenKind::*;

        let expr = match &self.peek().kind {
            False => Expr::Literal(Literal::False),
            True => Expr::Literal(Literal::True),
            Nil => Expr::Literal(Literal::Nil),
            Number(n) => Expr::Literal(Literal::Number(n.to_owned())),
            String(s) => Expr::Literal(Literal::String(s.to_owned())),
            LeftParen => {
                let expr = self.expression()?;
                self.consume(RightParen, "Expect ')' after expression.")?;
                Expr::Grouping(expr.into())
            }
            _ => {
                return self
                    .error(self.peek(), "Expect expression.")
                    .map(|_| unreachable!());
            }
        };

        self.current += 1;
        Ok(expr)
    }

    fn match_(&mut self, token_kinds: &[TokenKind]) -> bool {
        for token_kind in token_kinds {
            if self.check(token_kind) {
                self.advance();
                return true;
            }
        }

        false
    }

    fn consume(&mut self, until: TokenKind, error_message: &str) -> Result<&Token, ParserError> {
        if self.check(&until) {
            return Ok(self.advance());
        }

        self.error(self.peek(), error_message)
            .map(|_| unreachable!())
    }

    fn check(&self, token_kind: &TokenKind) -> bool {
        if self.eof() {
            false
        } else {
            self.peek().kind == *token_kind
        }
    }

    fn advance(&mut self) -> &Token {
        if !self.eof() {
            self.current += 1;
        }
        self.previous()
    }

    fn eof(&self) -> bool {
        matches!(self.peek().kind, TokenKind::Eof)
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn error(&self, token: &Token, message: &str) -> Result<Void, ParserError> {
        // TODO: push error to Lox.errors
        println!("{token} : {message}");
        Err(ParserError)
    }

    fn _synchronize(&mut self) {
        self.advance();

        use TokenKind::*;
        while !self.eof() {
            if matches!(self.previous().kind, Semicolon) {
                return;
            }

            match self.peek().kind {
                Class | Fn | Let | For | If | While | Print | Return => return,
                _ => (),
            }

            self.advance();
        }
    }
}

#[derive(Debug)]
pub struct ParserError;

#[allow(dead_code)]
enum Void {}
