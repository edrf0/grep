use std::error::Error;
use clap::Parser;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

/// Simple program to find lines in a file that match a regex pattern
#[derive(Parser,Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the file to read
    #[arg(short, long)]
    path: PathBuf,

    /// Regex keyword or pattern to search for
    #[arg(short, long)]
    keyword: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    // Parse command line arguments
    let args = Args::parse();

    // Compile the regex pattern
    let re = Regex::new(&args.keyword)?;

    // Open the file using the provided path
    let file = File::open(&args.path)?;
    let mut reader = BufReader::new(file);

    // Reusable buffer to avoid repeated heap allocations per line read
    let mut line = String::new();

    // Reads lines until Ok(0) is returned meaning the stream has reached EOF
    while reader.read_line(&mut line)? > 0 {
        // Keyword matching
        if re.is_match(&line) {
            // print! is used because the read_line method includes the \n character
            print!("{line}");
        }
        // String is cleared so its current capacity be reused without further heap allocations
        line.clear();
    }

    Ok(())
}
