use crate::{
    LoxError,
    backend::Interpreter,
    frontend::{Parser, Scanner},
};
use std::io::{Write, stdout};

pub struct Lox {
    interpreter: Interpreter,
}

impl Default for Lox {
    fn default() -> Self {
        Self::with_stdout()
    }
}

pub type LoxResult<T> = Result<T, LoxError>;

impl Lox {
    pub fn with_stdout() -> Self {
        Self::with_writer(Box::new(stdout()))
    }

    pub fn with_writer(w: Box<dyn Write>) -> Self {
        Self {
            interpreter: Interpreter::with_writer(w),
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
