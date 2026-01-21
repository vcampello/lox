use crate::interpreter::Value;
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct Env {
    values: HashMap<String, Value>,
}

#[derive(Debug)]
pub enum EnvError {
    UndefinedVariable { name: String },
}

pub type EnvResult<T> = Result<T, EnvError>;

impl Env {
    pub fn define(&mut self, name: &str, value: &Value) {
        // WARNING: override values without any further checks
        self.values.insert(name.to_string(), value.clone());
    }

    pub fn assign(&mut self, name: &str, value: &Value) -> EnvResult<()> {
        // WARNING: override values without any further checks
        if self.values.contains_key(name) {
            self.values.insert(name.to_string(), value.clone());
            return Ok(());
        }

        Err(EnvError::UndefinedVariable {
            name: name.to_string(),
        })
    }

    pub fn get(&self, name: &str) -> EnvResult<&Value> {
        self.values
            .get(name)
            .ok_or_else(|| EnvError::UndefinedVariable {
                name: name.to_string(),
            })
    }
}
