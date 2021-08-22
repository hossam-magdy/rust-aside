// cargo run -q foo test.txt
use std::{env, process};

fn main() {
    let config = minigrep::config::Config::new(env::args()).unwrap_or_else(|error| {
        eprintln!("Problem parsing arguments: {}", error);
        process::exit(1);
    });

    if let Err(e) = minigrep::run::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
