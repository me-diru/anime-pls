use core::fmt;
use std::io::BufRead;

use clap::Parser;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
}

// impl fmt::Display for std::path::PathBuf {
//     fn fmt(&self, f: &fmt::Formatter) -> fmt::Result {
//         write!(f, "{}", self.0)
//     }
// }
fn main() {
    let args = Cli::parse();

    println!("{} {:?}", args.pattern, args.path);
    let file = std::fs::File::open(&args.path).expect("wrong file path");

    let reader = std::io::BufReader::new(file);

    for line in reader.lines() {
        let line = line.expect("not able to read line");
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }
}
