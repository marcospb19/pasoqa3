//! Parsing util: extracting delimited text.

/// Find and extract a piece of text that is delimited at both sides.
pub fn extract_text_delimited_by<'input>(
    text: &'input str,
    left_delimiter: &str,
    right_delimiter: &str,
) -> Option<&'input str> {
    let (_left, right) = text.split_once(left_delimiter)?;
    let (left, _right) = right.split_once(right_delimiter)?;

    Some(left)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_text_delimited_by() {
        let haystack = "check @@ this ## out";

        let left = "@@";
        let right = "##";

        let extracted = extract_text_delimited_by(haystack, left, right);

        assert_eq!(extracted, Some(" this "));
    }
}
