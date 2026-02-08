use crate::interpreter::Value;
use std::collections::HashMap;

// #[derive(Debug, Copy)]
pub type Scope = HashMap<String, Value>;

#[derive(Debug)]
pub enum EnvError {
    UndefinedVariable { name: String },
}

pub type EnvResult<T> = Result<T, EnvError>;

#[derive(Debug, Default)]
pub struct Env {
    scopes: Vec<Scope>,
}

impl Env {
    pub fn new() -> Self {
        Self {
            scopes: vec![HashMap::new()],
        }
    }

    pub fn begin_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    pub fn end_scope(&mut self) {
        if self.scopes.len() > 1 {
            self.scopes.pop();
        }
    }

    fn last_scope(&mut self) -> &mut Scope {
        self.scopes
            .last_mut()
            .expect("Scope is undefined. This should never happen")
    }

    pub fn define(&mut self, name: &str, value: &Value) {
        self.last_scope().insert(name.to_string(), value.clone());
    }

    pub fn assign(&mut self, name: &str, value: &Value) -> EnvResult<()> {
        for scope in self.scopes.iter_mut().rev() {
            if scope.contains_key(name) {
                scope.insert(name.to_string(), value.clone());
                return Ok(());
            }
        }

        Err(EnvError::UndefinedVariable {
            name: name.to_string(),
        })
    }

    pub fn get(&self, name: &str) -> EnvResult<&Value> {
        for scope in self.scopes.iter().rev() {
            if let Some(value) = scope.get(name) {
                return Ok(value);
            }
        }

        Err(EnvError::UndefinedVariable {
            name: name.to_string(),
        })
    }
}
