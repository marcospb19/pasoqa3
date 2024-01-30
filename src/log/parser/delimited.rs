/// Extract a piece of text that's delimited at its two sides.
pub fn extract_text_delimited_by<'input>(
    text: &'input str,
    left_delimiter: &str,
    right_delimiter: &str,
) -> Option<&'input str> {
    let (_left, right) = text.split_once(left_delimiter)?;
    let (left, _right) = right.split_once(right_delimiter)?;

    Some(left)
}
