use crate::LoxNumber;

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
}

pub enum UnaryOp {
    Minus,
    Not,
}

pub enum Literal {
    Number(LoxNumber),
    String(String),
    Nil,
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
                BinaryOp::Minus => '-',
                BinaryOp::Plus => '+',
                BinaryOp::Div => '/',
                BinaryOp::Mul => '*',
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