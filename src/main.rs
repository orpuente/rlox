use std::env;

use rlox::lox::Lox;

fn main() {
    let args: Vec<_> = env::args().collect();
    let mut lox = Lox::default();
    lox.entry_point(&args);
}
