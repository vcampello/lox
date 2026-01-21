use crate::interpreter::Value;
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct Env {
    values: HashMap<String, Value>,
    enclosing: Option<Box<Env>>,
}

#[derive(Debug)]
pub enum EnvError {
    UndefinedVariable { name: String },
}

pub type EnvResult<T> = Result<T, EnvError>;

impl Env {
    pub fn new(enclosing: Option<Env>) -> Self {
        Self {
            values: HashMap::new(),
            enclosing: enclosing.map(Box::new),
        }
    }

    pub fn define(&mut self, name: &str, value: &Value) {
        self.values.insert(name.to_string(), value.clone());
    }

    pub fn assign(&mut self, name: &str, value: &Value) -> EnvResult<()> {
        if self.values.contains_key(name) {
            self.values.insert(name.to_string(), value.clone());
            return Ok(());
        }

        if let Some(enclosing) = &mut self.enclosing {
            return enclosing.assign(name, value);
        }

        Err(EnvError::UndefinedVariable {
            name: name.to_string(),
        })
    }

    pub fn get(&self, name: &str) -> EnvResult<&Value> {
        if let Some(value) = self.values.get(name) {
            return Ok(value);
        }

        if let Some(enclosing) = &self.enclosing {
            return enclosing.get(name);
        }

        Err(EnvError::UndefinedVariable {
            name: name.to_string(),
        })
    }
}
