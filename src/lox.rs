use crate::{
    LoxError,
    backend::Interpreter,
    frontend::{Parser, Scanner},
};

#[derive(Debug)]
pub struct Lox {
    interpreter: Interpreter,
}

impl Default for Lox {
    fn default() -> Self {
        Self::new()
    }
}

pub type LoxResult<T> = Result<T, LoxError>;

impl Lox {
    pub fn new() -> Self {
        Self {
            interpreter: Interpreter::new(),
        }
    }

    pub fn run(&mut self, src: &str) -> LoxResult<()> {
        let mut scanner = Scanner::new(src);
        let tokens = scanner.scan_tokens()?;
        let mut parser = Parser::new(tokens);
        let stmts = parser.parse()?;
        self.interpreter.interpret(&stmts)?;

        Ok(())
    }
}
