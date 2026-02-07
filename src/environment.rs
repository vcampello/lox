use crate::interpreter::Value;

type Scope = Vec<Value>;

#[derive(Debug, Default)]
pub struct Env {
    scopes: Vec<Scope>,
}

#[derive(Debug)]
pub enum EnvError {
    // UndefinedVariable { name: String },
    // TODO: how can I support a variable name in this case?
    UndefinedVariable,
    ScopeOutOfBounds { distance: usize },
    UndefinedGlobalScope,
}

pub type EnvResult<T> = Result<T, EnvError>;

impl Env {
    pub fn new() -> Self {
        Self {
            scopes: vec![Vec::new()],
        }
    }

    pub fn begin_scope(&mut self) {
        self.scopes.push(Vec::new());
    }

    pub fn end_scope(&mut self) {
        // Never end the global scope
        if !self.scopes.is_empty() {
            self.scopes.pop();
        }
    }

    pub fn define(&mut self, value: Value) -> EnvResult<()> {
        if let Some(scope) = self.scopes.last_mut() {
            scope.push(value);
            return Ok(());
        }

        // This should never happen!
        Err(EnvError::UndefinedGlobalScope)
    }

    pub fn get(&mut self, distance: usize, var_index: usize) -> EnvResult<&Value> {
        let Some(scope) = self.scopes.get_mut(distance) else {
            return Err(EnvError::ScopeOutOfBounds { distance });
        };

        match scope.get(var_index) {
            Some(value) => Ok(value),
            None => Err(EnvError::UndefinedVariable),
        }
    }

    pub fn set(&mut self, distance: usize, value: Value) -> EnvResult<()> {
        if distance > self.scopes.len() {
            return Err(EnvError::ScopeOutOfBounds { distance });
        }

        let scope_idx = self.scopes.len() - distance;
        let Some(scope) = self.scopes.get_mut(scope_idx) else {
            Err(EnvError::UndefinedVariable)
        };
    }
}
