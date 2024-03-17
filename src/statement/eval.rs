use crate::{environment::Environment, error::RuntimeError};
use super::Statement;



impl Statement {
    pub fn eval(self, env: &mut Environment) -> Result<(), RuntimeError> {
        match self {
            Statement::Block(stmts) => {
                let mut new_env = env.push();
                for stmt in stmts { stmt.eval(&mut new_env)? }
            }
            Statement::Expr(expr) => { expr.eval(env)?; },
            Statement::Let(name, expr) => env.bind(name, expr.eval(env)?),
            Statement::Print(expr) => println!("{}", expr.eval(env)?),
        };

        Ok(())
    }
}