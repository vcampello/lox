use crate::{ast::expression::Expr, interpreter::Interpreter, parser::Parser, scanner::Scanner};

#[derive(Debug)]
pub struct Runtime {
    pub had_error: bool,
}

impl Default for Runtime {
    fn default() -> Self {
        Self::new()
    }
}

impl Runtime {
    pub fn new() -> Self {
        Self { had_error: false }
    }

    // TODO: accept a stream
    pub fn run(&self, src: &str) {
        let mut scanner = Scanner::new(src);
        let tokens = scanner.scan_tokens();

        // TODO: add a flag to enable this once the REPL can evaluate things
        // println!("Scanned tokens:");
        // for token in tokens {
        //     println!("{token}");
        // }

        let mut parser = Parser::new(tokens);

        match parser.parse() {
            Err(e) => eprintln!("Failed to parse: {e}"),
            Ok(ast) => {
                // println!("{}", Expr::print(&ast));
                let result = Interpreter::visit(&ast).unwrap();
                println!("{result}");
            }
        };
    }

    fn error(&mut self, line: &u32, msg: &str) {
        // I don't love this pattern but I don't know where the book wants to take it yet
        self.report(line, "", msg);
    }

    fn report(&mut self, line: &u32, location: &str, msg: &str) {
        // REFACTOR: there's some shared error handling between the scanner and the runtime
        eprintln!(" {line} | Error {location}: {msg}");
        self.had_error = true;
    }

    pub fn clear_error_flag(&mut self) {
        self.had_error = false;
    }
}
