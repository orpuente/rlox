use crate::{
    error::ParserError, expr::{Expr, Literal}, statement::Statement, token::Token, token_kind::TokenKind
};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Vec<Statement>, ParserError> {
        let mut statements = Vec::new();
        let mut error = false;

        while !self.eof() {
            if let Ok(stmt) = self.statement() {
                statements.push(stmt);
            } else {
                self.synchronize();
                error = true;
            }
        }

        if error { Err(ParserError) } else { Ok(statements) }
    }

    fn statement(&mut self) -> Result<Statement, ParserError> {
        if self.match_(&[TokenKind::Print]) {            
            self.print_statement()
        } else if self.match_(&[TokenKind::Let]) {
            self.binding_statement()
        } else if self.match_(&[TokenKind::LeftBrace]) {
            self.block_statement()
        } else {
            self.expression_statement()
        }
    }

    fn print_statement(&mut self) -> Result<Statement, ParserError> {
        let expr = self.expression()?;
        self.consume(TokenKind::Semicolon, "Expected ';' after expression.")?;
        Ok(Statement::Print(expr))
    }

    fn binding_statement(&mut self) -> Result<Statement, ParserError> {
        // Can't mut borrow twice, so we clone the value.
        let name = self
            .consume(TokenKind::Identifier("".into()), "Expected variable name.")
            .cloned();

        if self.match_(&[TokenKind::Equal]) {
            let value = self.expression();
            // We want to consume the semicolon before emiting any errors to match the book's logic.
            self.consume(TokenKind::Semicolon, "Expected ';' after let binding.")?;
            let name = name?;
            let value = value?;
            Ok(Statement::Let(name.lexeme, value))
        } else {
            self.consume(TokenKind::Semicolon, "Expected ';' after let binding.")?;
            Err(ParserError)
        }
    }

    fn expression_statement(&mut self) -> Result<Statement, ParserError> {
        let expr = self.expression()?;
        self.consume(TokenKind::Semicolon, "Expected ';' after expression.")?;
        Ok(Statement::Expr(expr))
    }

    fn block_statement(&mut self) -> Result<Statement, ParserError> {
        let mut stmts = Vec::new();

        while !self.check(&TokenKind::RightBrace) && !self.eof() {
            stmts.push(self.statement()?);
        }

        self.consume(TokenKind::RightBrace, "Expected '}' after block.")?;
        Ok(Statement::Block(stmts))
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
            False => Expr::Literal(Literal::Boolean(false)),
            True => Expr::Literal(Literal::Boolean(true)),
            Nil => Expr::Literal(Literal::Nil),
            Number(n) => Expr::Literal(Literal::Number(n.to_owned())),
            String(s) => Expr::Literal(Literal::String(s.to_owned())),
            Identifier(n) => Expr::Variable(n.to_owned()),
            LeftParen => {
                let expr = self.expression()?;
                self.consume(RightParen, "Expected ')' after expression.")?;
                Expr::Grouping(expr.into())
            }
            _ => {
                return self
                    .error(self.peek(), "Expected expression.")
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
            self.peek().kind.same_kind(token_kind)
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

    fn synchronize(&mut self) {
        self.advance();

        use TokenKind::*;
        while !self.eof() {
            if matches!(self.previous().kind, Semicolon) { return; }
            match self.peek().kind {
                Class | Fn | Let | For | If | While | Print | Return => return,
                _ => (),
            }
            self.advance();
        }
    }
}

#[allow(dead_code)]
enum Void {}
