use std::{
    env, fs,
    io::{self, Write},
    process,
};

use lox::Lox;
use miette::{NamedSource, Report};

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => run_prompt(),
        2 => run_file(&args[1]),
        _ => {
            println!("Usage: lox [script]");
            process::exit(54)
        }
    }
}

fn run_prompt() {
    println!("Lox REPL");
    let mut lox = Lox::new();

    let mut buf = String::new();
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    loop {
        print!("> ");
        stdout.flush().unwrap();

        match stdin.read_line(&mut buf) {
            Ok(0) => break, // EOL
            Ok(_) => {
                let source = &buf.trim();

                // no-op on  whitespace
                if source.is_empty() {
                    continue;
                }

                // TODO: consolidate the reporter. There's a lot more work to be done
                if let Err(error) = lox.run(source) {
                    let source_code = NamedSource::new("REPL", source.to_string());
                    eprintln!("{:?}", Report::new(error).with_source_code(source_code));
                }
            }
            Err(e) => {
                eprintln!("REPL error: {e}");
                process::exit(65)
            }
        };

        // prevent appending contents on next pass
        buf.clear();
    }
}

fn run_file(path: &str) {
    let mut lox = Lox::new();

    let Ok(source) = fs::read_to_string(path) else {
        eprintln!("Failed to read {path}");
        process::exit(65)
    };

    // TODO: consolidate the reporter. There's a lot more work to be done
    if let Err(error) = lox.run(&source) {
        let source_code = NamedSource::new(path, source.to_string());
        eprintln!("{:?}", Report::new(error).with_source_code(source_code));
        process::exit(65);
    }
    // Chapter 7 adds something along the lines of `had_runtime_error` => exit(70)
}
