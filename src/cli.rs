//! CLI argparsing definitions with `clap`.

use std::path::PathBuf;

use clap::Parser;

const REPOSITORY: &str = "Repository: https://github.com/marcospb19/pasoqa3";

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about,
    after_help = REPOSITORY,
)]
pub struct Args {
    /// The log file to read from
    #[arg(required = true)]
    pub file: PathBuf,

    /// The number of the match to be shown
    #[arg(short, long = "game", value_name = "GAME")]
    pub game_to_show: Option<u32>,
}
