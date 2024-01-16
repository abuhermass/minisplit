use minisplit::Cli;
use clap::Parser;

fn main() {
    let cli = Cli::parse();
    let buffer = minisplit::read_stdin();

    let result = minisplit::split(buffer, &cli);
    println!("{}", result);
}