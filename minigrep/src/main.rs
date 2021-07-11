use std::{env, fs};

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();

    let search_string = args
        .get(1)
        .expect("Search string is required as first argument");

    let filename = args
        .get(2)
        .expect("File name is required as second argument");

    let file_content = fs::read_to_string(filename)?;
    let mut i = 0;

    for line in file_content.lines() {
        i = i + 1;
        if line.contains(search_string) {
            print!("Line #{}: {:?}\n", i, line);
        }
    }

    Ok(())
}
