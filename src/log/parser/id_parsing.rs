use std::{marker::PhantomData, str::FromStr};

use super::super::Id;

/// Convenient shorthand for using `IdSequenceParser` once.
pub fn parse_id(text: &str) -> Option<Id> {
    IdSequenceParser::new(text).next()
}

/// A struct that parses a sequence of IDs from a string.
pub struct IdSequenceParser<'a, T = Id> {
    text: &'a str,
    phantom: PhantomData<T>,
}

impl<'a, T> IdSequenceParser<'a, T>
where
    T: FromStr,
{
    pub fn new(text: &'a str) -> Self {
        Self {
            text,
            phantom: PhantomData,
        }
    }
}

impl<T> Iterator for IdSequenceParser<'_, T>
where
    T: FromStr,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.text = self.text.trim();

        let next_piece = self
            .text
            .split_terminator(|ch| ch == ' ' || ch == ':')
            .next()?;

        let (_, remainder) = self.text.split_at(next_piece.len());
        self.text = remainder;

        next_piece.parse::<T>().ok()
    }
}
