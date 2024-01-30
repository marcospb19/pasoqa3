use std::{
    io::{self, BufRead, BufReader, Lines},
    path::Path,
};

use fs_err as fs;

type LineReader = Lines<BufReader<fs::File>>;

pub fn line_reader_from_file(path: &Path) -> io::Result<LineReader> {
    let file = fs::OpenOptions::new().read(true).open(path)?;
    let reader = BufReader::with_capacity(1024 * 8, file);
    Ok(reader.lines())
}
