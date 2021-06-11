use anyhow::{Context, Result};
use structopt::StructOpt;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

// Now you can run: `cargo run foo test.txt`
// Or `cargo build --release` then `target/release/rust-workshop foo test.txt`
fn main() -> Result<()> {
    let args = Cli::from_args();
    let pattern = args.pattern;
    let path = args.path.as_path().to_str().unwrap();

    println!("Args: {{ string: \"{}\", file: \"{}\" }}", pattern, path);

    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("Could not read he file \"{}\"", path))?;

    for line in content.lines() {
        if line.contains(&pattern) {
            println!("{}", line);
        }
    }

    Ok(())
}
