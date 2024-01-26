use std::io::{BufRead, Lines};

pub struct LogReader<R>
where
    R: BufRead,
{
    line_reader: Lines<R>,
}

impl<R> LogReader<R>
where
    R: BufRead,
{
    pub fn new(reader: R) -> Self {
        Self {
            line_reader: reader.lines(),
        }
    }
}

impl<R> Iterator for LogReader<R>
where
    R: BufRead,
{
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let line = self.line_reader.next()?.expect("io error");

        let should_skip = !line.contains(" Kill: ");

        if should_skip {
            self.next()
        } else {
            Some(line)
        }
    }
}
