mod parse;

use crate::parse::parse;
use clap::Parser;

#[derive(Parser, Default, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// String format for what to gennerate
    format: String,
}

fn main() {
    let args = Cli::parse();
    println!("{}", parse(&args.format));
}
