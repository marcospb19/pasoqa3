use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Args {
    /// The list of log files to read.
    #[arg(required = true)]
    pub files: Vec<PathBuf>,
}
