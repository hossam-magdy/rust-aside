use crate::{config::Config, search};
use std::{error::Error, fs};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file_content = fs::read_to_string(config.filename)?;
    let mut i = 0;
    let search_result = if config.case_sensitive {
        search::search(&config.query, &file_content)
    } else {
        search::search_case_insensitive(&config.query, &file_content)
    };

    println!("Found {} results", search_result.len());

    for line in search_result {
        i = i + 1;
        print!("Line #{}: {:?}\n", i, line);
    }

    Ok(())
}
