use crate::{environment::Environment, error::RuntimeError};
use super::Statement;



impl Statement {
    pub fn eval(self, env: &mut Environment) -> Result<(), RuntimeError> {
        match self {
            Statement::Expr(expr) => { expr.eval(env)?; },
            Statement::Let(name, expr) => env.bind(name, expr.eval(env)?),
            Statement::Print(expr) => {
                println!("{}", expr.eval(env)?)
            },
        };

        Ok(())
    }
}