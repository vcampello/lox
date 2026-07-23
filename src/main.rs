use lox::Lox;
use std::{
    env, fs,
    io::{self, Write},
    process,
};

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
    let mut lox = Lox::with_stdout();

    let mut buf = String::new();
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    loop {
        print!("> ");
        stdout.flush().unwrap();

        match stdin.read_line(&mut buf) {
            Ok(0) => break, // EOL
            Ok(_) => {
                let src = &buf.trim();

                // no-op on  whitespace
                if src.is_empty() {
                    continue;
                }

                let _ = lox.run("REPL", src);
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
    let mut lox = Lox::with_stdout();

    let Ok(source) = fs::read_to_string(path) else {
        eprintln!("Failed to read {path}");
        process::exit(65)
    };

    if lox.run(path, &source).is_err() {
        process::exit(65);
    }
}
