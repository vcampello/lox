use std::{
    env, fs,
    io::{self, Write},
    process,
};

use lox::runtime::Runtime;

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);
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
    let mut rtm = Runtime::new();

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

                rtm.run(source);
            }
            Err(e) => {
                eprintln!("Error: {e}");
                process::exit(65)
            }
        };

        // prevent appending contents on next pass
        buf.clear();
    }
}

fn run_file(path: &str) {
    let mut rtm = Runtime::new();

    let Ok(src) = fs::read_to_string(path) else {
        eprintln!("Failed to read {path}");
        process::exit(65)
    };

    rtm.run(&src);
    if rtm.had_error {
        process::exit(65);
    }
    // Chapter 7 adds something along the lines of `had_runtime_error` => exit(70)
}
