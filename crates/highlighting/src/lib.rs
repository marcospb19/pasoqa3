use std::fmt::Write;

use owo_colors::{OwoColorize, Rgb};
use syntect::{
    easy::HighlightLines,
    highlighting::{Color, Style, ThemeSet},
    parsing::SyntaxSet,
};

/// Highlights JSON using ASCII escape sequences colors.
///
/// # Panics
///
/// This function panics if the given `json` is malformed.
pub fn highlight_json(json: &str) -> String {
    let syntax_set = SyntaxSet::load_defaults_newlines();
    let syntax = syntax_set
        .syntaxes()
        .iter()
        .find(|syntax| syntax.name == "JSON")
        .expect("JSON is included with the `default-syntaxes` feature");

    let theme = &ThemeSet::load_defaults().themes["base16-eighties.dark"];

    let mut highlighter = HighlightLines::new(syntax, theme);
    let mut result = String::new();

    for line in json.lines() {
        let line_pieces = highlighter
            .highlight_line(line, &syntax_set)
            .expect("Malformed JSON");

        for (style, text) in line_pieces {
            let Style { foreground, .. } = style;
            let Color { r, g, b, .. } = foreground;
            let color = Rgb(r, g, b);
            write!(result, "{}", text.color(color)).unwrap();
        }
        writeln!(result).unwrap();
    }

    result
}
