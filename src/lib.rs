pub mod environment;
pub mod error;
pub mod expr;
pub mod lox;
pub mod parser;
pub mod scanner;
pub mod statement;
pub mod token;
pub mod token_kind;

type LoxNumber = f64;
type Identifier = String;