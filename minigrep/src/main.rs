// cargo run -q foo test.txt
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = minigrep::config::Config::new(&args).unwrap_or_else(|error| {
        eprintln!("Problem parsing arguments: {}", error);
        process::exit(1);
    });

    if let Err(e) = minigrep::run::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
