use std::{error::Error, fs};

pub struct Config<'a> {
    pub query: &'a String,
    pub filename: &'a String,
}

impl<'a> Config<'a> {
    pub fn new(args: &[String]) -> Result<Config, &str> {
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

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
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
