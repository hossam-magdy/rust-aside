use crate::{config::Config, search};
use std::{error::Error, fs};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file_content = fs::read_to_string(config.filename)?;
    let mut i = 0;
    let search_result = search::search(config.query, &file_content);

    println!("Found {} results", search_result.len());

    for line in search_result {
        i = i + 1;
        print!("Line #{}: {:?}\n", i, line);
    }

    Ok(())
}
