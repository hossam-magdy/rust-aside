use crate::config::Config;
use std::{error::Error, fs};

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
