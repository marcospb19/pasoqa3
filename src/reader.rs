//! Provide a file line reader.

use std::{
    io::{BufRead, BufReader, Lines},
    path::Path,
};

use fs_err as fs;

use crate::{Result, WrapErr};

type LineReader = Lines<BufReader<fs::File>>;

/// A buffered line reader from a file.
pub fn line_reader_from_file(path: &Path) -> Result<LineReader> {
    let file = fs::OpenOptions::new()
        .read(true)
        .open(path)
        .wrap_err("Failed to open log file for reading")?;

    let reader = BufReader::with_capacity(1024 * 8, file);
    Ok(reader.lines())
}
