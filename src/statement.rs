pub mod eval;

use crate::{expr::Expr, Identifier};


pub enum Statement {
    Block(Vec<Statement>),
    Expr(Expr),
    Print(Expr),
    Let(Identifier, Expr),
}
