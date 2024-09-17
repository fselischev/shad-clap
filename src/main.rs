use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
struct Args {
    #[arg(long)]
    pattern: String,
    #[arg(long)]
    path: PathBuf,
}

fn main() {
    let args = Args::parse();
    // other logic
}
