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

    // Compile the regex pattern (handles error if the regex is invalid)
    let re = Regex::new(&args.keyword)?;

    // Open the file using the provided path
    let file = File::open(&args.path)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        // If valid we unwrap the String otherwise we ignore the line
        let line = match line {
            Ok(line) => line,
            Err(_) => continue,
        };

        // Pattern matching
        if re.is_match(&line) {
            println!("{}", line);
        }
    }

    Ok(())
}
