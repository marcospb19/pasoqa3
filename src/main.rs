//! Entry point for `pasoqa3`.

use std::{io, path::Path, process::exit};

mod cli;
mod parser;
mod reader;
mod summary;

use cli::Args;
use parser::LogMessageParser;
use reader::line_reader_from_file;
use summary::Summary;

/// Parses `cli::Args` and runs for each file.
fn main() {
    let Args { files } = clap::Parser::parse();

    for file in &files {
        // If there are multiple summaries to show, output the file name
        if files.len() >= 2 {
            println!("----- {file:?} -----");
        }

        let result = run(file);

        if let Err(err) = result {
            eprintln!("Error: {err}");
            exit(1);
        }
    }
}

/// Build and output a kill feed summary for a single log file.
///
/// This function reads from the log file, parses its events, and processes
/// them to build a summary.
fn run(log_path: &Path) -> io::Result<()> {
    let line_reader = line_reader_from_file(log_path)?;

    let mut parser = LogMessageParser::new();
    let mut summary = Summary::new();

    for line in line_reader {
        // Check read line
        let line = line?;

        // Pass line to parser
        let event = parser.parse_line(&line);

        if let Some(event) = event {
            // Pass event to summary
            summary.process(event);
        }
    }

    summary.output();
    Ok(())
}
