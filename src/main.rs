//! Entry point for `pasoqa3`.

use std::path::Path;

mod cli;
mod error;
mod log;
mod reader;

use cli::Args;
use error::{err, Result, WrapErr, WrapNone};
use log::{LogMessageParser, SummaryProcessor};
use reader::line_reader_from_file;

/// Parses `cli::Args` and runs for each file.
fn main() -> Result<()> {
    // Install panic and error report handlers
    color_eyre::install()?;

    let Args { files } = clap::Parser::parse();

    for file in &files {
        // If there are multiple summaries to show, output the file name
        if files.len() >= 2 {
            println!("----- {file:?} -----");
        }

        // `eyre::Report` uses `Debug`, it's OK to return it from `main`
        run(file)?;
    }

    Ok(())
}

/// Build and output a kill feed summary for a single log file.
///
/// This function reads from the log file, parses its events, and processes
/// them to build a summary.
fn run(log_path: &Path) -> Result<()> {
    let line_reader = line_reader_from_file(log_path)?;

    let mut parser = LogMessageParser::new();
    let mut summaries = SummaryProcessor::new();

    for (i, line) in line_reader.enumerate() {
        let line_number = i + 1;

        // Check read line
        let line =
            line.wrap_err_with(|| err!("Failed to read line {line_number}"))?;

        // Pass line to parser
        let event = parser
            .parse_line(&line)
            .wrap_err_with(|| err!("Failed to parse line {line_number}"))?;

        // If the current line generated a event to be processed
        if let Some(event) = event {
            // Pass event to the summary processor. If the match summary is
            // finished, this will output it to the screen.
            summaries.process(event);
        }
    }

    Ok(())
}
