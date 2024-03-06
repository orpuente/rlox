use crate::{token_type::TokenKind, LoxNumber};

pub enum Expr {
    Binary(Box<Expr>, BinaryOp, Box<Expr>),
    Grouping(Box<Expr>),
    Literal(Literal),
    Unary(UnaryOp, Box<Expr>),
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
    Number(LoxNumber),
    String(String),
    Nil,
    False,
    True,
}

impl From<&TokenKind> for BinaryOp {
    fn from(value: &TokenKind) -> Self {
        match value {
            TokenKind::Minus        => BinaryOp::Minus,
            TokenKind::Plus         => BinaryOp::Plus,
            TokenKind::Slash        => BinaryOp::Div,
            TokenKind::Star         => BinaryOp::Mul,
            TokenKind::BangEqual    => BinaryOp::NotEqual,
            TokenKind::EqualEqual   => BinaryOp::Equal,
            TokenKind::Greater      => BinaryOp::Greater,
            TokenKind::GreaterEqual => BinaryOp::GreaterEqual,
            TokenKind::Less         => BinaryOp::Less,
            TokenKind::LessEqual    => BinaryOp::LessEqual,
            TokenKind::And          => BinaryOp::And,
            TokenKind::Or           => BinaryOp::Or,
            _ => panic!("{:?} should be a binary operator", value)
        }
    }
}

impl From<&TokenKind> for UnaryOp {
    fn from(value: &TokenKind) -> Self {
        match value {
            TokenKind::Minus => UnaryOp::Minus,
            TokenKind::Bang => UnaryOp::Not,
            _ => panic!("{:?} should be a unary operator", value)
        }
    }
}

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Binary(l, op, r) => write!(f, "({op} {l} {r})"),
            Expr::Grouping(expr) => write!(f, "(group {expr})"),
            Expr::Literal(lit) => write!(f, "{lit}"),
            Expr::Unary(op, expr) => write!(f, "({op} {expr})"),
        }
    }
}

impl std::fmt::Display for BinaryOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f, "{}",
            match self {
                BinaryOp::Minus        => "-",
                BinaryOp::Plus         => "+",
                BinaryOp::Div          => "/",
                BinaryOp::Mul          => "*",
                BinaryOp::NotEqual     => "!=",
                BinaryOp::Equal        => "=",
                BinaryOp::Greater      => ">",
                BinaryOp::GreaterEqual => ">=",
                BinaryOp::Less         => "<",
                BinaryOp::LessEqual    => "<=",
                BinaryOp::And          => "&",
                BinaryOp::Or           => "|",
            }
        )
    }
}

impl std::fmt::Display for UnaryOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f, "{}",
            match self {
                UnaryOp::Minus => '-',
                UnaryOp::Not => '!',
            }
        )
    }
}

impl std::fmt::Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {        
        write!(
            f, "{}",
            match self {
                Literal::Number(value) => value.to_string(),
                Literal::String(value) => value.to_owned(),
                Literal::Nil => "nil".to_string(),
                Literal::False => "false".to_string(),
                Literal::True => "true".to_string(),
            }
        )
    }
}

#[test]
fn print_ast() {
    let expr = Expr::Binary(
        Expr::Unary(
            UnaryOp::Minus,
            Expr::Literal(Literal::Number(123.)).into()
        ).into(),
        BinaryOp::Mul,
        Expr::Grouping(
            Expr::Literal(Literal::Number(45.67)).into()
        ).into()
    );

    println!("{expr}");
}