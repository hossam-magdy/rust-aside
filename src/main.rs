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
fn main() {
    let args = Cli::from_args();

    println!(
        "Args: {{ string: \"{}\", file: \"{}\" }}",
        args.pattern,
        args.path.as_path().to_str().unwrap()
    );

    let result = std::fs::read_to_string(&args.path);

    // The patter matching is equivalent to:
    // let content = result.unwrap();
    // @see documentation during autocompletion in VSCode
    let content = match result {
        Ok(content) => content,
        Err(error) => {
            panic!("Can't deal with {}, just exit here", error);
        }
    };

    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }
}
