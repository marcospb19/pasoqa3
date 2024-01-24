mod cli;

use clap::Parser;
use cli::Args;

fn main() {
    let Args { files } = Args::parse();

    for file in files {
        dbg!(file);
    }
}
