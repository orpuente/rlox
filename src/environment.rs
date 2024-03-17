use std::collections::HashMap;

use crate::{error::UnboundVariable, expr::eval::Value, Identifier};

#[derive(Default)]
pub struct Environment<'a> {
    table: HashMap<Identifier, Value>,
    enclosing: Option<&'a Environment<'a>>,
}

impl<'a> Environment<'a> {
    pub fn push(&'a self) -> Self {
        Self {
            table: Default::default(),
            enclosing: Some(self)
        }
    }

    pub fn bind(&mut self, name: Identifier, value: Value) {
        self.table.insert(name, value);
    }

    pub fn get(&self, name: Identifier) -> Result<&Value, UnboundVariable> {
        self.get_rec(&name).ok_or(UnboundVariable(name))
    }

    fn get_rec(&self, name: &Identifier) -> Option<&Value> {
        self.table.get(name).or(if let Some(ref env) = self.enclosing {
            (&*env).get_rec(name)
        } else {
            None
        })
    }
}