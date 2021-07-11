// cargo run -q foo test.txt

use std::{env, fs, process};

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|error| {
        println!("Problem parsing arguments: {}", error);
        process::exit(1);
    });

    let file_content = fs::read_to_string(config.filename)?;
    let mut i = 0;

    for line in file_content.lines() {
        i = i + 1;
        if line.contains(config.query) {
            print!("Line #{}: {:?}\n", i, line);
        }
    }

    Ok(())
}

struct Config<'a> {
    query: &'a String,
    filename: &'a String,
}

impl<'a> Config<'a> {
    fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args
            .get(1)
            .expect("Search string is required as first argument");

        let filename = args
            .get(2)
            .expect("File name is required as second argument");

        Ok(Config { query, filename })
    }
}
