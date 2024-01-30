//! Entry point for `pasoqa3`.

mod cli;
mod error;
mod log;
mod reader;

use cli::Args;
use error::{err, Result, WrapErr, WrapNone};
use log::{LogMessageParser, SummaryProcessor};
use reader::line_reader_from_file;

/// Build and output a summary for the provided log file.
///
/// Reads from the log file, parses its events, and processes them to build
/// summaries.
fn main() -> Result<()> {
    // Install panic and error report handlers
    color_eyre::install()?;

    let Args { file, game_to_show } = clap::Parser::parse();

    let line_reader = line_reader_from_file(&file)?;

    let mut parser = LogMessageParser::new();
    let mut summaries = SummaryProcessor::new(game_to_show);

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
