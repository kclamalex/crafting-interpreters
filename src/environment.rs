use std::collections::HashMap;
use std::default::Default;

use crate::common::{LiteralValue, Token};
use crate::error::RuntimeError;

#[derive(Clone)]
pub struct Environment {
    var_map: HashMap<String, LiteralValue>,
    enclosing: Option<Box<Environment>>,
}

impl Default for Environment {
    fn default() -> Self {
        Environment {
            var_map: HashMap::new(),
            enclosing: None,
        }
    }
}
impl Environment {
    pub fn new(enclosing: Option<Box<Environment>>) -> Self {
        Environment {
            var_map: HashMap::new(),
            enclosing,
        }
    }
    pub fn define(&mut self, name: Token, value: LiteralValue) {
        self.var_map.insert(name.lexeme, value);
    }
    pub fn get(&self, name: Token) -> Result<LiteralValue, RuntimeError> {
        if let Some(value) = self.var_map.get(&name.lexeme) {
            Ok(value.clone())
        } else if let Some(enclosing) = &self.enclosing {
            enclosing.get(name)
        } else {
            Err(RuntimeError {
                message: format!("Undefined variable '{}'.", name.lexeme),
            })
        }
    }
    pub fn assign(&mut self, name: Token, value: LiteralValue) -> Result<(), RuntimeError> {
        if self.var_map.contains_key(&name.lexeme) {
            self.var_map.insert(name.lexeme, value);
            Ok(())
        } else if let Some(enclosing) = &self.enclosing {
            let cloned_enclosing = enclosing.clone();
            let mut mut_env = *cloned_enclosing;
            let _ = &mut_env.assign(name, value);
            Ok(())
        } else {
            let name_lexeme = name.lexeme;
            Err(RuntimeError {
                message: format!("Undefined variable '{name_lexeme}'."),
            })
        }
    }
}
