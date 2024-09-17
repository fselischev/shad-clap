use clap::Parser;
use std::{
    fs,
    io::{BufRead, BufReader},
    path::PathBuf,
};

#[derive(Parser)]
struct Args {
    #[arg(long)]
    pattern: String,
    #[arg(long)]
    path: PathBuf,
}

fn main() {
    let args = Args::parse();

    let file = fs::File::open(args.path).expect("file not exists");
    let reader = BufReader::new(file);

    reader
        .lines()
        .enumerate()
        .filter(|(_, line)| line.as_ref().unwrap().contains(&args.pattern))
        .for_each(|(num, line)| println!("{num}: {}", line.unwrap()));
}
