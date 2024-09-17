use clap::Parser;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::{
    fs,
    io::{BufRead, BufReader},
    path::PathBuf,
};

/// Finder CLI app.
///
/// Finds all occurences of pattern in file by given path in parallel.
#[derive(Parser)]
#[command(name = "Finder", version)]
#[command(override_usage = "finder --pattern <PATTERN> --path <FILE>")]
#[command(before_help = "It's better to use ripgrep :)")]
#[command(after_help = "Author: @fselischev")]
struct Args {
    /// Pattern to be found.
    ///
    /// Arbitrary string value that will be searched in files.
    #[arg(long)]
    pattern: String,
    /// File path where to search pattern occurrences.
    #[arg(long = "path")]
    paths: Vec<PathBuf>,
}

fn main() {
    let args = Args::parse();

    args.paths.par_iter().for_each(|path| {
        if let Ok(file) = fs::File::open(path) {
            BufReader::new(file)
                .lines()
                .enumerate()
                .filter_map(|(num, line)| {
                    line.ok().and_then(|l| {
                        if l.contains(&args.pattern) {
                            Some(format!("{}:{}: {}", path.display(), num + 1, l))
                        } else {
                            None
                        }
                    })
                })
                .for_each(|line| println!("{}", line));
        } else {
            eprintln!("Error: Could not open file {:?}", path);
        }

        println!()
    });
}
