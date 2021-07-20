// cargo run -q foo test.txt
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = minigrep::config::Config::new(&args).unwrap_or_else(|error| {
        println!("Problem parsing arguments: {}", error);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = minigrep::run::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
