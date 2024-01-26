mod cli;
mod log;

use std::{
    fs,
    io::{self, BufReader},
    path::Path,
};

use clap::Parser;
use cli::Args;
use log::LogReader;

fn main() {
    let Args { files } = Args::parse();

    for file in &files {
        let reader = reader_from_file(file).unwrap();

        let log_parser = LogReader::new(reader);

        for log_line in log_parser {
            dbg!(log_line);
        }
    }
}

fn reader_from_file(path: &Path) -> io::Result<BufReader<fs::File>> {
    let file = fs::OpenOptions::new().read(true).open(path)?;
    let reader = BufReader::with_capacity(1024 * 8, file);
    Ok(reader)
}
