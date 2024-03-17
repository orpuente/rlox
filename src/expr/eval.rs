use std::fmt::Display;

use crate::{environment::Environment, error::{RuntimeError, TypeError}, LoxNumber};

use super::{Expr, Literal, UnaryOp, BinaryOp};

impl Expr {
    pub fn eval(self, env: &Environment) -> Result<Value, RuntimeError> {
        match self {
            Expr::Binary(op, expr1, expr2) => Ok(op.eval(expr1.eval(env)?, expr2.eval(env)?)?),
            Expr::Grouping(expr) => expr.eval(env),
            Expr::Literal(lit) =>   Ok(lit.into()),
            Expr::Unary(op, expr) => Ok(op.eval(expr.eval(env)?)?),

            // There is an oportunity for optimization here. Distinguish between OwenedValue, RefValue, and MutRefValue.
            Expr::Variable(name) => Ok(env.get(name)?.clone())
        }
    }
}

#[derive(Clone)]
pub enum Value {
    Boolean(bool),
    Number(LoxNumber),
    String(String),
    Nil,
}

impl UnaryOp {
    fn eval(self, x: Value) -> Result<Value, TypeError> {
        use Value::*;
        let res: Value = match (self, x) {
            (UnaryOp::Minus, Number(n)) => Number(-n),
            (UnaryOp::Not, Boolean(b))  => Boolean(!b),
            _ => return Err(TypeError),
        };
        Ok(res)
    }
}

impl BinaryOp {
    fn eval(self, x: Value, y: Value) -> Result<Value, TypeError> {
        use Value::*;
        let res: Value = match (self, x, y) {
            (BinaryOp::Minus, Number(x), Number(y))        => Number(x - y),
            (BinaryOp::Plus,  Number(x), Number(y))        => Number(x + y),
            (BinaryOp::Plus,  String(x), String(y))        => String(x + &y),
            (BinaryOp::Div,   Number(x), Number(y))        => Number(x / y),
            (BinaryOp::Mul,   Number(x), Number(y))        => Number(x * y),
            (BinaryOp::NotEqual, Boolean(x), Boolean(y))   => Boolean(x != y),
            (BinaryOp::NotEqual, Number(x), Number(y))     => Boolean(x != y),
            (BinaryOp::NotEqual, String(x), String(y))     => Boolean(x != y),
            (BinaryOp::NotEqual, Nil, Nil)                 => Boolean(false),
            (BinaryOp::NotEqual, _, _)                     => Boolean(true),
            (BinaryOp::Equal, Boolean(x), Boolean(y))      => Boolean(x == y),
            (BinaryOp::Equal, Number(x), Number(y))        => Boolean(x == y),
            (BinaryOp::Equal, String(x), String(y))        => Boolean(x == y),
            (BinaryOp::Equal, Nil, Nil)                    => Boolean(true),
            (BinaryOp::Equal, _, _)                        => Boolean(false),
            (BinaryOp::Less,         Number(x), Number(y)) => Boolean(x < y),
            (BinaryOp::Greater,      Number(x), Number(y)) => Boolean(x > y),
            (BinaryOp::LessEqual,    Number(x), Number(y)) => Boolean(x <= y),
            (BinaryOp::GreaterEqual, Number(x), Number(y)) => Boolean(x >= y),
            (BinaryOp::And, Boolean(x), Boolean(y))        => Boolean(x && y),
            (BinaryOp::Or,  Boolean(x), Boolean(y))        => Boolean(x || y),
            _ => return Err(TypeError),
        };
        Ok(res)
    }
}

impl From<Literal> for Value {
    fn from(value: Literal) -> Self {
        match value {
            Literal::Boolean(b) => Self::Boolean(b),
            Literal::Number(n) => Self::Number(n),
            Literal::String(s) => Self::String(s),
            Literal::Nil => Self::Nil,
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Value::Boolean(value) => value.to_string(),
                Value::Number(value) => value.to_string(),
                Value::String(value) => value.to_owned(),
                Value::Nil => "nil".to_string(),
            }
        )
    }
}