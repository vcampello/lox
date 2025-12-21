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
        1 => run_file(&args[0]),
        x if x > 2 => {
            println!("Usage: lox [script]");
            process::exit(54)
        }
        _ => run_prompt(),
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
                println!("Got: {buf}");
                rtm.run(&buf);
                rtm.clear_error_flag(); // don't kill the session
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
    println!("Loading ${path}");
    let rtm = Runtime::new();

    // REFACTOR: let errors bubble up to the caller
    let Ok(src) = fs::read_to_string(path) else {
        eprintln!("Failed to read {path}");
        process::exit(65)
    };

    rtm.run(&src);
    if rtm.had_error {
        process::exit(65)
    }
}
