use clap::{Parser, Subcommand};
use colored::Colorize;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::{
    fs,
    io::{BufRead, BufReader},
    path::PathBuf,
};

/// Finder CLI app.
///
/// Finds all occurrences of pattern in file by given path in parallel.
#[derive(Parser)]
#[command(name = "Finder", version)]
#[command(override_usage = "finder --pattern <PATTERN> --path <FILE> [COMMAND]")]
#[command(before_help = "It's better to use ripgrep :)")]
#[command(after_help = "Author: @fselischev")]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,
    /// Pattern to be found.
    ///
    /// Arbitrary string value that will be searched in files.
    #[arg(long)]
    pattern: String,
    /// File path where to search pattern occurrences.
    #[arg(long = "path")]
    paths: Vec<PathBuf>,
}

#[derive(Subcommand)]
enum Commands {
    /// Colorizes pattern in lines for readability (performance overhead).
    Colorize,
}

fn process_file(path: &PathBuf, pattern: &str, colorize: bool) {
    if let Ok(file) = fs::File::open(path) {
        BufReader::new(file)
            .lines()
            .enumerate()
            .filter_map(|(num, line)| {
                line.ok().and_then(|l| {
                    if l.contains(pattern) {
                        let formatted_line = if colorize {
                            let colored_line = l.replace(pattern, &pattern.red().to_string());
                            format!(
                                "{}:{}: {}",
                                path.display().to_string().magenta(),
                                (num + 1).to_string().green(),
                                colored_line
                            )
                        } else {
                            format!("{}:{}: {}", path.display(), num + 1, l)
                        };
                        Some(formatted_line)
                    } else {
                        None
                    }
                })
            })
            .for_each(|line| println!("{}", line));
    } else {
        eprintln!("Error: Could not open file {:?}", path);
    }
}

fn main() {
    let args = Args::parse();
    let colorize = matches!(args.command, Some(Commands::Colorize));

    args.paths.par_iter().for_each(|path| {
        process_file(path, &args.pattern, colorize);
    });
}
