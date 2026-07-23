use crate::{
    LoxError,
    backend::Interpreter,
    frontend::{Parser, Scanner},
};
use miette::{NamedSource, Report};
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

    pub fn with_writer(stdout: Box<dyn Write>) -> Self {
        Self {
            interpreter: Interpreter::with_writer(stdout),
        }
    }

    /// raw lox pipeline
    pub fn pipeline(&mut self, src: &str) -> LoxResult<()> {
        let mut scanner = Scanner::new(src);
        let tokens = scanner.scan_tokens()?;
        let mut parser = Parser::new(tokens);
        let stmts = parser.parse()?;
        self.interpreter.interpret(&stmts)?;

        Ok(())
    }

    /// wraps the lox pipeline to print errors
    pub fn run(&mut self, src_name: &str, src: &str) -> LoxResult<()> {
        if let Err(error) = self.pipeline(src) {
            let named_source = NamedSource::new(src_name, src.to_string());
            eprintln!(
                "{:?}",
                Report::new(error.clone()).with_source_code(named_source)
            );
            return Err(error);
        }

        Ok(())
    }
}
