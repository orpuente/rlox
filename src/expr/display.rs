use std::fmt::Display;

use crate::expr::{Literal, UnaryOp};

use super::{BinaryOp, Expr};

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Binary(l, op, r) => write!(f, "({op} {l} {r})"),
            Expr::Grouping(expr) => write!(f, "(group {expr})"),
            Expr::Literal(lit) => write!(f, "{lit}"),
            Expr::Unary(op, expr) => write!(f, "({op} {expr})"),
        }
    }
}

impl Display for BinaryOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                BinaryOp::Minus => "-",
                BinaryOp::Plus => "+",
                BinaryOp::Div => "/",
                BinaryOp::Mul => "*",
                BinaryOp::NotEqual => "!=",
                BinaryOp::Equal => "=",
                BinaryOp::Greater => ">",
                BinaryOp::GreaterEqual => ">=",
                BinaryOp::Less => "<",
                BinaryOp::LessEqual => "<=",
                BinaryOp::And => "&",
                BinaryOp::Or => "|",
            }
        )
    }
}

impl Display for UnaryOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                UnaryOp::Minus => '-',
                UnaryOp::Not => '!',
            }
        )
    }
}

impl Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Literal::Boolean(value) => value.to_string(),
                Literal::Number(value) => value.to_string(),
                Literal::String(value) => value.to_owned(),
                Literal::Nil => "nil".to_string(),
            }
        )
    }
}
