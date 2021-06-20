use rust_workshop::p3_reverse_string::reverse_string2;

/*
use anyhow::{Context, Result};
use log::info;
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
fn mainX() -> Result<()> {
    env_logger::init();

    let args = Cli::from_args();
    let pattern = args.pattern;
    let path = args.path.as_path().to_str().unwrap();

    info!("Args: {{ string: \"{}\", file: \"{}\" }}", pattern, path);

    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("Could not read he file \"{}\"", path))?;

    for line in content.lines() {
        if line.contains(&pattern) {
            println!("{}", line);
        }
    }

    Ok(())
}
*/

// fn main() {
//     println!(
//         "output:   {:?}\nexpected: {:?}\n",
//         rust_workshop::p2_two_sum::two_sum2(vec![2, 7, 11, 15], 9),
//         vec![0, 1]
//     );
//     println!(
//         "output:   {:?}\nexpected: {:?}\n",
//         rust_workshop::p2_two_sum::two_sum2(vec![3, 2, 4], 6),
//         vec![1, 2]
//     );
//     println!(
//         "output:   {:?}\nexpected: {:?}\n",
//         rust_workshop::p2_two_sum::two_sum2(vec![3, 3], 6),
//         vec![0, 1]
//     );
// }

fn main() {
    // println!("{}", reverse_string1("abc"));
    println!("{}", reverse_string2("abc"));
}
