use std::collections::HashMap;

use crate::{error::UnboundVariable, expr::eval::Value, Identifier};

#[derive(Default)]
pub struct Environment {
    table: HashMap<Identifier, Value>
}

impl Environment {
    pub fn bind(&mut self, name: Identifier, value: Value) {
        self.table.insert(name, value);
    }

    pub fn get(&self, name: Identifier) -> Result<&Value, UnboundVariable> {
        self.table.get(&name).ok_or(UnboundVariable(name))
    }
}