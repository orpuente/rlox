pub mod eval;

use crate::{expr::Expr, Identifier};


pub enum Statement {
    Expr(Expr),
    Print(Expr),
    Let(Identifier, Expr),
}
