pub mod display;
pub mod eval;

use crate::{token_kind::TokenKind, Identifier, LoxNumber};

pub enum Expr {
    Binary(BinaryOp, Box<Expr>, Box<Expr>),
    Grouping(Box<Expr>),
    Literal(Literal),
    Unary(UnaryOp, Box<Expr>),
    Variable(Identifier),
}

pub enum BinaryOp {
    Minus,
    Plus,
    Div,
    Mul,
    NotEqual,
    Equal,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    And,
    Or,
}

pub enum UnaryOp {
    Minus,
    Not,
}

pub enum Literal {
    Boolean(bool),
    Number(LoxNumber),
    String(String),
    Nil,
}

impl From<&TokenKind> for BinaryOp {
    fn from(value: &TokenKind) -> Self {
        match value {
            TokenKind::Minus => BinaryOp::Minus,
            TokenKind::Plus => BinaryOp::Plus,
            TokenKind::Slash => BinaryOp::Div,
            TokenKind::Star => BinaryOp::Mul,
            TokenKind::BangEqual => BinaryOp::NotEqual,
            TokenKind::EqualEqual => BinaryOp::Equal,
            TokenKind::Greater => BinaryOp::Greater,
            TokenKind::GreaterEqual => BinaryOp::GreaterEqual,
            TokenKind::Less => BinaryOp::Less,
            TokenKind::LessEqual => BinaryOp::LessEqual,
            TokenKind::And => BinaryOp::And,
            TokenKind::Or => BinaryOp::Or,
            _ => panic!("{:?} should be a binary operator", value),
        }
    }
}

impl From<&TokenKind> for UnaryOp {
    fn from(value: &TokenKind) -> Self {
        match value {
            TokenKind::Minus => UnaryOp::Minus,
            TokenKind::Bang => UnaryOp::Not,
            _ => panic!("{:?} should be a unary operator", value),
        }
    }
}

#[test]
fn print_ast() {
    let expr = Expr::Binary(
        BinaryOp::Mul,
        Expr::Unary(UnaryOp::Minus, Expr::Literal(Literal::Number(123.)).into()).into(),
        Expr::Grouping(Expr::Literal(Literal::Number(45.67)).into()).into(),
    );

    println!("{expr}");
}
