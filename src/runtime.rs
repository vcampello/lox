use crate::{ast::expression::Expr, interpreter::Interpreter, parser::Parser, scanner::Scanner};

#[derive(Debug)]
pub struct Runtime {
    // REVIEW: convert into Result and let the caller decide what to do with each error
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
    pub fn run(&mut self, src: &str) {
        self.had_error = false;

        let mut scanner = Scanner::new(src);
        let tokens = scanner.scan_tokens();

        // TODO: add a flag to enable this once the REPL can evaluate things
        // println!("Scanned tokens:");
        // for token in tokens {
        //     println!("{token}");
        // }

        let mut parser = Parser::new(tokens);
        let interpreter = Interpreter;

        let ast = match parser.parse() {
            Ok(ast) => ast,
            Err(e) => {
                self.had_error = true;
                eprintln!("Failed to parse: {e}");
                return;
            }
        };

        // println!("{}", Expr::print(&ast));
        match interpreter.visit(&ast) {
            Err(e) => {
                self.had_error = true;
                eprintln!("Failed to parse: {e}");
            }
            Ok(result) => println!("{result}"),
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
}
